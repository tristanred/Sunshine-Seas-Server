// Using temporary allows for cleaner development.
#![allow(unused_imports)]
#![allow(dead_code)]

use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::string::*;
use std::borrow::*;
use std::rc::*;
use std::cell::*;
use std::sync::*;
use std::ops::Deref;
use std::iter::*;

mod sessions;
mod commands;
mod testclients;
mod utils;

use utils::*;
use commands::*;
use sessions::*;

// Hosts various server objects.
struct ServerContext {
    pub sessions: Mutex<SessionManager>
}

fn create_server_context() -> ServerContext {
    ServerContext {
        sessions: Mutex::new(SessionManager::new())
    }
}

fn main() {

    start_server_thread();

    // Wait 1 sec before starting the clients
    std::thread::sleep(std::time::Duration::from_secs(1));

    testclients::start_client_start_stop();

    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn start_server_thread() {

    let context = Arc::new(create_server_context());

    // Server Accept thread
    std::thread::spawn(move || {
        let t = std::net::TcpListener::bind("127.0.0.1:5555").unwrap();

        loop {
            let (sock, _addr) = t.accept().expect("TCP Accept failed.");

            // Create the initial session of the player. Fields are still
            // mostly uninitialized. Then add a clone of it to the session
            // list.
            let sesh = sessions::create_player_session(sock);
            context.sessions.lock().unwrap().add_session(sesh.clone());

            let tctx = context.clone();

            start_client_thread(sesh, tctx);
        }
    });
}

fn start_client_thread(session: PlayerSession, ctx: Arc<ServerContext>) {

    std::thread::spawn(move || {
        let mut client_session = session;
        let client_socket = client_session.player_socket.clone();
        let socket_mutex = client_socket.lock().unwrap();
        let player_socket: &TcpStream = &socket_mutex;

        let mut br = BufReader::new(player_socket);

        loop {
            let mut readbuf = vec![];
            br.read_until(b'|', &mut readbuf).unwrap();

            if readbuf.is_empty() {
                println!("Client is done. Exiting thread.");

                break;
            }

            match handle_user_packet(&readbuf, &mut client_session, ctx.clone()) {
                Ok(_) => {
                    // packet handled successfully
                    // Return value is Unit so not much to do.
                },
                Err(msg) => {
                    log_error(&msg);
                }
            }

            let session_list = &ctx.sessions;
            session_list.lock().unwrap().save_session(&client_session);
        }
    });
}

fn log_error(message: &str) {
    println!("ERROR: {}", message);
}

fn handle_user_packet(data: &[u8], session: &mut PlayerSession, ctx: Arc<ServerContext>) -> Result<(), String> {
    session.increment_msg_count();

    let message_type = find_message_type(data);

    if message_type.is_none() {
        return Err("Invalid input from find_message_type".to_string());
    }

    let message_type = message_type.unwrap();

    println!("Received type {:?}", message_type);

    match message_type {
        ref x if x == HELLO_MSG_ID => {
            let msg = HelloCommand::from_client_message(&data).unwrap();

            handle_hello_message(&msg, session, ctx)?;
        },
        ref x if x == BYE_MSG_ID => {
            let msg = ByeCommand::from_client_message(&data).unwrap();

            handle_bye_message(&msg, session)?;
        },
        ref x if x == PUTOBJ_MSG_ID => {
            let msg = PutObjCommand::from_client_message(&data).unwrap();

            handle_put_obj_message(&msg, session)?;
        },
        _ => {
            return Err("Unrecognized message type".to_string());
        }
    }

    return Ok(());
}

/**
 * Read the ID part of the data buffer.Arc
 *
 * Specifically, this reads the first 8 bytes and returns a string with the
 * trailing null bytes trimmed.
 */
fn find_message_type(data: &[u8]) -> Option<String> {
    let header = &data[0..8];

    String::from_utf8(trim_vec_end(header)).ok()
}

/**
 * Processing for the Hello message. This opens the player session
 */
fn handle_hello_message(message: &HelloCommand, session: &mut PlayerSession, ctx: Arc<ServerContext>) -> Result<(), &'static str> {
    println!("Received HELLO message {:?}", message);

    match session.state {
        SessionState::Closed => {
            session.state = SessionState::Active;

            ctx.sessions.lock().unwrap().add_session(session.clone());

            return Ok(());
        },
        SessionState::Active => {
            return Err("Session already open");
        }
    }
}

fn handle_bye_message(message: &ByeCommand, session: &mut PlayerSession) -> Result<(), &'static str> {
    println!("Received BYE message {:?}", message);

    match session.state {
        SessionState::Closed => {
            return Err("Session is not open");
        },
        SessionState::Active => {
            session.state = SessionState::Active;

            return Ok(());
        }
    }
}

fn handle_put_obj_message(message: &PutObjCommand, session: &mut PlayerSession) -> Result<(), String> {
    println!("Received PUTOBJ message {:?}", message);

    Err("Not implemented".to_string())
}

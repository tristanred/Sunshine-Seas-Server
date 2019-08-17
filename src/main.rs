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

    let context = Arc::new(Mutex::new(create_server_context()));

    // Server Accept thread
    std::thread::spawn(move || {
        let t = std::net::TcpListener::bind("127.0.0.1:5555").unwrap();

        loop {
            let loop_ctx = context.clone();
            let (sock, _addr) = t.accept().expect("TCP Accept failed.");

            // Create the initial session of the player. Fields are still
            // mostly uninitialized. Then add a clone of it to the session
            // list.
            let sesh = sessions::create_player_session(sock);

            {
                let ctx = loop_ctx.lock().unwrap();
                let mut sessions = ctx.sessions.lock().unwrap();

                sessions.add_session(sesh.clone());
            }

            let session = Arc::new(Mutex::new(sesh));

            start_client_thread(session, loop_ctx);
        }
    });
}

fn start_client_thread(session: Arc<Mutex<PlayerSession>>, ctx: Arc<Mutex<ServerContext>>) {

    std::thread::spawn(move || {

        /*
         * Intricate method to grab a TcpStream instance from inside the session
         * while leaving the session instance unlocked after the copy.
         *
         * This is needed because we need a TcpStream instance to pass to the
         * BufReader and it is found inside the PlayerSession instance. We
         * don't want to lock the PlayerSession struct up here because that
         * would lock it indefinitely and the server thread could never
         * update the information.
         *
         * So we reach in the structure and try_clone it and return it outside
         * the artificial scope. This releases the resources and the mutex locks
         * so they remain unlocked until later in the thread when a message
         * was received and we need to call handle_user_packet.
         *
         * This respects the "lock as late as possible, release as early as
         * possible" principle.
         */
        let socket: TcpStream = {
            let mut session_lock = session.lock().unwrap();
            let session: &mut PlayerSession = &mut session_lock;

            let socket_check = session.player_socket.clone().unwrap();
            let socket_lock = socket_check.lock().unwrap();
            let socket: &TcpStream = &socket_lock;

            socket.try_clone().unwrap()
        };

        // A bufreader is created using the TcpStream copy
        let mut br = BufReader::new(socket);

        loop {
            let mut readbuf = vec![];
            br.read_until(b'|', &mut readbuf).unwrap();

            if readbuf.is_empty() {
                println!("Client is done. Exiting thread.");

                break;
            }

            // Lock the structures as close as possible to their callsites
            // Lock, call handle_user_packet(...) on it, save session and
            // the locks will be unlocked when the loop scope ends.
            let mut session_lock = session.lock().unwrap();
            let mut session: &mut PlayerSession = &mut session_lock;

            let ctx_lock = ctx.lock().unwrap();
            let ctx: &ServerContext = &ctx_lock;

            match handle_user_packet(&readbuf, &mut session, ctx) {
                Ok(_) => {
                    // packet handled successfully
                    // Return value is Unit so not much to do.
                },
                Err(msg) => {
                    log_error(&msg);
                }
            }

            let session_list = &ctx.sessions;
            session_list.lock().unwrap().save_session(session);
        }
    });
}

fn log_error(message: &str) {
    println!("ERROR: {}", message);
}

fn handle_user_packet(data: &[u8], session: &mut PlayerSession, ctx: &ServerContext) -> Result<(), String> {
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

            handle_bye_message(&msg, session, ctx)?;
        },
        ref x if x == PUTOBJ_MSG_ID => {
            let msg = PutObjCommand::from_client_message(&data).unwrap();

            handle_put_obj_message(&msg, session, ctx)?;
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
fn handle_hello_message(message: &HelloCommand, session: &mut PlayerSession, ctx: &ServerContext) -> Result<(), &'static str> {
    println!("Received HELLO message {:?}", message);

    match session.state {
        SessionState::Closed => {
            session.state = SessionState::Active;

            let mut manager = ctx.sessions.lock().unwrap();
            manager.add_session(session.clone());

            return Ok(());
        },
        SessionState::Active => {
            return Err("Session already open");
        }
    }
}

fn handle_bye_message(message: &ByeCommand, session: &mut PlayerSession, _ctx: &ServerContext) -> Result<(), &'static str> {
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

fn handle_put_obj_message(message: &PutObjCommand, _session: &mut PlayerSession, _ctx: &ServerContext) -> Result<(), String> {
    println!("Received PUTOBJ message {:?}", message);

    Err("Not implemented".to_string())
}

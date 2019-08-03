#![allow(unused_imports)]

use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::string::*;
use std::borrow::*;
use std::rc::*;
use std::cell::*;
use std::sync::*;
use std::ops::Deref;
use std::iter::*;

mod messageblocks;
mod sessions;
mod commands;
mod testclients;
use commands::*;
use sessions::*;
use messageblocks::*;

fn main() {

    start_server_thread();

    // Wait 1 sec before starting the clients
    std::thread::sleep(std::time::Duration::from_secs(1));

    testclients::start_client_start_stop();

    std::thread::sleep(std::time::Duration::from_secs(5));
}

struct SessionList
{
    sessions: std::vec::Vec<PlayerSession>
}

impl SessionList {

    /**
     * Create a new session list.
     */
    pub fn new() -> SessionList {
        SessionList {
            sessions: vec![]
        }
    }

    /**
     * Test method to try borrowing out of the list.
     */
    pub fn get_first_session(&mut self) -> &PlayerSession {
        return self.sessions.first().unwrap();
    }

    /**
     * Add a session to the list.
     */
    pub fn add_session(&mut self, new_session: PlayerSession) {
        self.sessions.push(new_session);
    }

    pub fn save_session(&mut self, session: PlayerSession) {
        if session.player_name.is_none() {
            return;
        }

        let session_discriminant = session.player_name.clone().unwrap();

        let sessionlist = &self.sessions;

        let res: Option<(usize, &sessions::PlayerSession)> =
                    sessionlist.iter()
                     .enumerate()
                     .find(|(_, item)| {
                         return is_session_match(item, &session_discriminant);
                     });

        match res {
            Some((i, _)) => {
                let mut dat = &self.sessions[i];
                std::mem::replace(&mut dat, &session);

                println!("Saved session {} with new info {:?}", i, &self.sessions[i]);
            },
            _ => {
                println!("Session was not saved because it was not found.");
            }
        }
    }
}

fn is_session_match(session: &sessions::PlayerSession, playername: &str) -> bool{
    match &session.player_name {
        Some(name) => {
            return name == playername;
        },
        None => {
            return false;
        }
    }
}

fn start_server_thread() {

    let sessions_list = Arc::new(Mutex::new(SessionList::new()));

    // Server Accept thread
    std::thread::spawn(move || {
        let t = std::net::TcpListener::bind("127.0.0.1:5555").unwrap();

        loop {
            let (sock, _addr) = t.accept().expect("TCP Accept failed.");

            // Create the initial session of the player. Fields are still
            // mostly uninitialized. Then add a clone of it to the session
            // list.
            let sesh = sessions::create_player_session(sock);
            sessions_list.lock().unwrap().add_session(sesh.clone());

            let tsession_list = sessions_list.clone();

            start_client_thread(sesh, tsession_list);
        }
    });
}

fn start_client_thread(session: PlayerSession, session_list: Arc<Mutex<SessionList>>) {

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

            match handle_user_packet(&readbuf, &mut client_session) {
                Ok(_) => {
                    // packet handled successfully
                    // Return value is Unit so not much to do.
                },
                Err(msg) => {
                    log_error(msg);
                }
            }

            session_list.lock().unwrap().save_session(client_session.clone());
        }
    });
}

fn log_error(message: &str) {
    println!("ERROR: {}", message);
}

fn handle_user_packet(data: &[u8], session: &mut PlayerSession) -> Result<(), &'static str> {
    session.increment_msg_count();

    let message_type = find_message_type(data);

    println!("{:?}", message_type);

    match message_type {
        Some("HELO") => {
            let msg = HelloCommand::from_client_message(&data).unwrap();

            handle_hello_message(msg, session)?;
        },
        Some("BYYE") => {
            let msg = ByeCommand::from_client_message(&data).unwrap();

            handle_bye_message(msg, session)?;
        },
        _ => {
            return Err("Unrecognized message type");
        }
    }

    return Ok(());
}

fn find_message_type(data: &[u8]) -> Option<&str> {
    let header = &data[0..4];

    std::str::from_utf8(header).ok()
    //std::str::from_utf8(header).map(|st: &str| st.replace(" ", "") ).ok()
}

/**
 * Processing for the Hello message. This opens the player session
 */
fn handle_hello_message(message: HelloCommand, session: &mut PlayerSession) -> Result<(), &'static str> {
    println!("Received HELLO message {:?}", message);

    match session.state {
        SessionState::Closed => {
            session.state = SessionState::Active;

            return Ok(());
        },
        SessionState::Active => {
            return Err("Session already open");
        }
    }
}

fn handle_bye_message(message: ByeCommand, session: &mut PlayerSession) -> Result<(), &'static str> {
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

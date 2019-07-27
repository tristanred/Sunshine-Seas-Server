use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::string::*;
use std::borrow::*;
use std::rc::*;
use std::cell::*;
use std::sync::*;
use std::ops::Deref;

mod sessions;
mod commands;
mod testclients;
use commands::*;
use sessions::*;

fn main() {

    start_server_thread();

    let mut tc = std::net::TcpStream::connect("127.0.0.1:5555").unwrap();
    tc.write_all(b"aaaaa\n").unwrap();

    let mut sesh = sessions::create_player_session(tc);
    //sesh.player_socket.write_all(b"bbbbb").unwrap();

    // let mut tc2 = (&tc).clone();
    // tc2.write_all(b"bbbbbbb\n").unwrap();

    // let mut tc3 = (&tc).clone();
    // tc3.write_all(b"ccccccc\n").unwrap();

    // let joiner = std::thread::spawn(move || {
    //     tc.write_all(b"dddddd").unwrap();
    // });

    // joiner.join().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));
}

struct SessionList
{
    sessions: std::vec::Vec<sessions::PlayerSession>
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
    pub fn get_first_session(&mut self) -> PlayerSession {
        return self.sessions.pop().unwrap();
    }

    /**
     * Add a session to the list.
     */
    pub fn add_session(&mut self, new_session: PlayerSession) {
        self.sessions.push(new_session);
    }
}

fn start_server_thread() {

    let mut sessions_list = SessionList::new();

    std::thread::spawn(move || {
        let t = std::net::TcpListener::bind("127.0.0.1:5555").unwrap();

        loop {
            let (sock, _addr) = t.accept().expect("TCP Accept failed.");

            let mut sesh = sessions::create_player_session(sock);

            // TODO : Does not work, can't update the session in the list
            sessions_list.add_session(sesh.clone());

            std::thread::spawn(move || {
                let socks: &TcpStream = &sesh.get_stream();

                let mut br = BufReader::new(socks);

                loop {
                    let mut readbuf = vec![];
                    br.read_until(b'|', &mut readbuf).unwrap();

                    handle_user_packet(&readbuf, &mut sesh.clone()).expect("Unable to make shit work.");
                }
            });
        }
    });
}

fn handle_user_packet(data: &[u8], session: &mut PlayerSession) -> Result<(), &'static str> {
    session.increment_msg_count();

    let message_type = find_message_type(data);

    match message_type {
        Some("HELO") => {
            let msg = HelloCommand::from_client_message(&data).unwrap();

            handle_hello_message(msg, session)?;
        },
        _ => {
            return Err("Unrecognized message type");
        }
    }

    return Ok(());
}

fn find_message_type(data: &[u8]) -> Option<&str> {
    let header = &data[0..5];

    std::str::from_utf8(header).ok()
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
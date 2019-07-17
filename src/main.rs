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
    pub fn get_first_session(&mut self) -> sessions::PlayerSession {
        return self.sessions.pop().unwrap();
    }

    /**
     * Add a session to the list.
     */
    pub fn add_session(&mut self, new_session: sessions::PlayerSession) {
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

            sessions_list.add_session(sesh.clone());

            std::thread::spawn(move || {

                //let mt: &mut TcpStream = &mut sesh.get_stream();

                //let socks: &TcpStream = &sesh.player_socket.lock().unwrap();
                let socks: &TcpStream = &sesh.get_stream();

                let mut br = BufReader::new(socks);

                loop {
                    let mut readbuf = vec![];
                    br.read_until(b'|', &mut readbuf).unwrap();

                    handle_user_packet(&readbuf, &mut sesh.clone());
                }
            });
        }
    });
}

fn handle_user_packet(data: &Vec<u8>, session: &mut sessions::PlayerSession)
{
    let command = commands::HelloCommand::from_client_message(data);

    session.increment_msg_count();
}
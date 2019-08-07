use std::net::{TcpStream};
use std::io::{Write};
use crate::utils::*;

/**
 * Public functions in this file are used to start client threads
 * that acts in a particular way. Helpful for testing server implementation.
 */

/**
 * Start a client thread that connects and keeps the connection open
 * forever by sending an HELO message each 5 seconds.
 */
pub fn start_client_idle() {
    std::thread::spawn(move || {
        let mut conn = connect_local();

        loop {
            conn.write_all(b"HELO_OYYY").unwrap();

            std::thread::sleep(std::time::Duration::from_millis(5000));
        }
    });
}

/**
 * Start a client thread that connects to the server and does nothing. This will
 * trigger the timeout eventually.
 */
pub fn start_client_timeout() {
    std::thread::spawn(move || {
        let _conn = connect_local();

        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    });
}

pub fn start_client_bad_hello_sequence() {
    std::thread::spawn(move || {
        let mut conn = connect_local();

        conn.write_all(b"HELO_NBONE|").unwrap();
        conn.write_all(b"HELO_NBTWO|").unwrap();
    });
}

pub fn start_client_start_stop() {
    std::thread::spawn(move || {
        let mut conn = connect_local();

        let mut cmd: Vec<u8> = Vec::new();

        let mut one = pad_string(b"HELO", 8);
        let mut two = pad_string(b"TestUsername", 32);
        let mut three = pad_string(b"Super Message", 64);

        cmd.append(&mut one);
        cmd.append(&mut two);
        cmd.append(&mut three);

        conn.write_all(&cmd).unwrap();
        conn.write_all(b"BYYE|").unwrap();
    });
}

/**
 * Private utility function to create a connection to the server.
 */
fn connect_local() -> std::net::TcpStream {
    return TcpStream::connect("localhost:5555").unwrap();
}
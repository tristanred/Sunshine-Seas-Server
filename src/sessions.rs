use chrono::prelude::*;
use std::net::{TcpStream};
use std::sync::*;
use std::borrow::*;
use std::rc::*;
use std::cell::*;

#[derive(PartialEq, Copy, Clone)]
pub enum SessionState {
    Closed,
    Active
}

#[derive(Clone)]
pub struct PlayerSession {
    pub state: SessionState,
    pub last_comm_time: chrono::DateTime<UTC>,
    pub player_socket: Arc<Mutex<TcpStream>>,
    pub messages_count: u32,
    pub player_name: Option<String>
}

impl PlayerSession {
    pub fn increment_msg_count(&mut self) {
        self.messages_count += 1;
    }

    pub fn get_stream(&self) -> std::sync::MutexGuard<'_, std::net::TcpStream> {
        self.player_socket.lock().unwrap()
    }

    pub fn set_username(&mut self, username: String) {
        self.player_name = Some(username);
    }
}

pub fn create_player_session(client_socket: TcpStream) -> PlayerSession {
    PlayerSession {
        state: SessionState::Closed,
        last_comm_time: chrono::UTC::now(),
        player_socket: Arc::new(Mutex::new(client_socket)),
        messages_count: 0,
        player_name: None
    }
}

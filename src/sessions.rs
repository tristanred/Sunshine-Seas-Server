use chrono::prelude::*;
use std::net::{TcpStream};
use std::sync::*;
use std::borrow::*;
use std::rc::*;
use std::cell::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum SessionState {
    Closed,
    Active
}

#[derive(Clone, Debug)]
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

pub struct SessionManager {
    sessions: std::vec::Vec<PlayerSession>
}

impl SessionManager {
    /**
     * Create a new session list.
     */
    pub fn new() -> SessionManager {
        SessionManager {
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

        let res: Option<(usize, &PlayerSession)> =
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

fn is_session_match(session: &PlayerSession, playername: &str) -> bool{
    match &session.player_name {
        Some(name) => {
            return name == playername;
        },
        None => {
            return false;
        }
    }
}

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

// Warning, this struct gets copied a lot !!
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

impl PartialEq for PlayerSession {
    fn eq(&self, other: &Self) -> bool {
        if self.player_name.is_none() || other.player_name.is_none() {
            return false;
        }

        // TODO : Clone() used. Find better way ?
        let left = self.player_name.clone().unwrap();
        let right = other.player_name.clone().unwrap();

        return left == right;
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

    pub fn save_session(&mut self, session: &PlayerSession) {
        if session.player_name.is_none() {
            return;
        }

        let session_discriminant = session.player_name.clone().unwrap();
        let filter_func = |item: &&PlayerSession| is_session_match(item, &session_discriminant);

        let res = self.find_session(filter_func);

        match res {
            Some((i, _)) => {
                let mut dat = &self.sessions[i];
                std::mem::replace(&mut dat, session);

                println!("Saved session {} with new info {:?}", i, &self.sessions[i]);
            },
            _ => {
                println!("Session was not saved because it was not found.");
            }
        }
    }

    pub fn remove_session(&mut self, session: &PlayerSession) {
        if session.player_name.is_none() {
            return;
        }

        let session_discriminant = session.player_name.clone().unwrap();
        let filter_func = |item: &&PlayerSession| is_session_match(item, &session_discriminant);

        let res = self.find_session(filter_func);

        match res {
            Some((i, _)) => {
                self.sessions.remove(i);

                println!("Removed session {} with name {}", i, session.player_name.clone().unwrap());
            },
            _ => {
                println!("Session was not removed because it was not found.");
            }
        }
    }

    /**
     * Find a session given a predicate. Returns the player session and its
     * index in the list.
     *
     * TODO : Might be a good idea to find a way to return a reference instead
     * of a clone of PlayerSession, that structures gets copied all over.
     */
    fn find_session<F>(&self, predicate: F) -> Option<(usize, PlayerSession)>
        where F: Fn(&&PlayerSession) -> bool
    {
        let sessionlist = &self.sessions;

        let res = sessionlist.iter()
                             .enumerate()
                             .find(|(_, item)| predicate(item));

        res.map(|(i, item)| (i, item.clone()))
    }
}

/**
 * Checks if a session matches for a particular username.
 *
 * This is a shorthand method to avoid dealing with the optional username.
 */
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

mod tests {
    use super::*;

    // #[test]
    // fn test_session_match() {

    // }

    // fn create_test_session() -> PlayerSession {



    // }
}
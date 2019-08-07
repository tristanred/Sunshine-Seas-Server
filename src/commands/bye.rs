use std::iter::*;
use std::io::{BufReader, Read};
use crate::utils::*;

// BYE
#[derive(Debug)]
pub struct ByeCommand {
    pub id: String,
}

impl ByeCommand {
    pub fn from_client_message(_data: &[u8]) -> Result<ByeCommand, String> {
        let created = ByeCommand {
            id: String::from("BYYE")
        };
        return Ok(created);
    }
}

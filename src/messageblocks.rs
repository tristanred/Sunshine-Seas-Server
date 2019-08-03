use chrono::prelude::*;

use std::io::*;


/**
 * Data about misc data for a request.
 */
#[derive(Debug)]
pub struct Commondata {
    /**
     * Static identifier of the message type. Must be assigned to the correct
     * value by the creation function.
     */
    pub id: &'static str,

    /**
     * Date the request was made.
     */
    pub request_date: chrono::DateTime<UTC>
}

/**
 * Data about a particular user of a request.
 */
#[derive(Debug)]
pub struct Userdata {
    /**
     * Identifier for the player.
     */
    pub user_id: std::string::String
}

impl Userdata {

    pub fn parse(data: &[u8]) -> std::result::Result<Userdata, &'static str> {
        if validate_no_message_marker(data) == false {
            return Err("Parse error.");
        }

        let mut reader = BufReader::new(data);

        // Read 8 bytes for the block identifier
        let mut ident = Vec::with_capacity(8);
        reader.read_exact(&mut ident).unwrap();

        messageblock_ident_matches(&ident, "ID")?;

        // Read 32 bytes for the username data
        let mut read_user_id = Vec::with_capacity(32);
        reader.read_exact(&mut read_user_id).unwrap();

        let result = Userdata {
            user_id: String::from_utf8(read_user_id).unwrap()
        };

        return Ok(result);
    }
}

/**
 * Checks if the characters in `data` up to the first null character matches
 * the expected string.
 */
fn messageblock_ident_matches(data: &[u8], expected: &str) -> std::result::Result<bool, &'static str> {
    let mut reader = BufReader::new(data);

    let mut result = vec![];

    reader.read_until(0, &mut result).map_err(|_v| "Failed to read from message ident.")?;

    return Ok(std::str::from_utf8(&result).unwrap() == expected);
}

fn validate_no_message_marker(to_verify: &[u8]) -> bool {
    let block_delimiter = b'&'; // TODO : Static value ?

    to_verify.contains(&block_delimiter)
}



/**
 * Create default structures that are usually included in every request types.
 *
 * Returned as a tuple with a copy of each type of structs. Can be extended if
 * more structs are created.
 *
 * The structs are created with a bunch of default values. Normally each
 * structures would be created separately with proper data.
 */
pub fn generate_default_structs() -> (Userdata, Commondata) {
    let default_user = Userdata {
        user_id: std::string::String::from("Default")
    };

    let default_misc = Commondata {
        id: "DEF",
        request_date: chrono::UTC::now()
    };

    return (default_user, default_misc);
}

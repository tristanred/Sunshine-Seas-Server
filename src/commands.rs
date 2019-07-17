use chrono::prelude::*;

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

/**
 * Create default structures that are usually included in every request types.
 *
 * Returned as a tuple with a copy of each type of structs. Can be extended if
 * more structs are created.
 *
 * The structs are created with a bunch of default values. Normally each
 * structures would be created separately with proper data.
 */
fn generate_default_structs() -> (Userdata, Commondata) {
    let default_user = Userdata {
        user_id: std::string::String::from("Default")
    };

    let default_misc = Commondata {
        id: "DEF",
        request_date: chrono::UTC::now()
    };

    return (default_user, default_misc);
}

// HELLO
#[derive(Debug)]
pub struct HelloCommand {
    pub commondat: Commondata,
    pub userdat: Userdata,
    pub msg: std::string::String
}

impl HelloCommand {
    // pub fn new() -> HelloCommand {
    //     let (user, mut misc) = generate_default_structs();
    //     misc.id = "HELLO";

    //     return HelloCommand {
    //         commondat: misc,
    //         userdat: user,
    //         msg: std::string::String::from("TEST_MESSAGE")
    //     }
    // }

    pub fn from_client_message(data: &[u8]) -> Result<HelloCommand, String> {
        let (user, mut misc) = generate_default_structs();
        misc.id = "HELLO";

        let msg_ident = get_string_from_msgdata(data)?;

        let created = HelloCommand {
            userdat: user,
            commondat: misc,
            msg: msg_ident
        };

        return Ok(created);
    }
}

fn get_string_from_msgdata(slice: &[u8]) -> Result<String, String> {
    std::string::String::from_utf8(slice.to_vec()).map_err(|err| err.to_string())
}
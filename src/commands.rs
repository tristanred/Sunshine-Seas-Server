use std::iter::*;
use crate::messageblocks::*;

/**
 * MSG Structure
 *           +--------+
 * MSG ID    |        |   8 bytes
 *           +--------+
 * MSG blks  |        |
 *           |        |
 *              ...
 *           |        |
 *           +--------+
 *
 * Start of block marker = '&'. The ampersand should be aligned on a 8 byte
 * boundary.
 *
 * Block types and sizes
 *
 *
 *
 */

// HELLO
#[derive(Debug)]
pub struct HelloCommand {
    pub commondat: Commondata,
    pub userdat: Userdata,
    pub msg: std::string::String
}

impl HelloCommand {

    pub fn from_client_message(data: &[u8]) -> Result<HelloCommand, &'static str> {
        let blocks = get_messageblocks_groups(data);

        /**
         * Each block is formatted as such
         * First 8 bytes is the block identifier
         *
         */




        return Err("");

    }
}


// BYE
#[derive(Debug)]
pub struct ByeCommand {
    pub commondat: Commondata,
    pub userdat: Userdata,
}

impl ByeCommand {
    pub fn from_client_message(_data: &[u8]) -> Result<ByeCommand, String> {
        let (user, mut misc) = generate_default_structs();
        misc.id = "BYYE";

        let created = ByeCommand {
            userdat: user,
            commondat: misc
        };

        return Ok(created);
    }
}

/**
 * Separate the buffer into groups of bytes separated by ampersand '&'
 * characters. The separators are not included in each group.
 *
 */
pub fn get_messageblocks_groups(data: &[u8]) -> std::result::Result<Vec<Vec<u8>>, &'static str> {

    let mut result_vector = vec![];

    let mut current_index = 0;
    let mut current_vec = vec![];
    loop {
        if current_index >= data.len() {
            break;
        }

        let current = data[current_index];

        if current == b'&' {
            result_vector.push(current_vec);
            current_vec = vec![];

        } else {
            current_vec.push(current);
        }

        current_index += 1;
    }

    return Ok(result_vector);
}

fn get_string_from_msgdata(slice: &[u8]) -> Result<String, String> {
    std::string::String::from_utf8(slice.to_vec()).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messageblock_grouping() {

        let test_vec = b"TEST&aaaaaaaa&bbbbbb&ccc&xxx";

        let t_one = get_messageblocks_groups(test_vec).unwrap();

        let mapped: Vec<String> = t_one.iter().map(|v| String::from_utf8(v.to_vec()).unwrap() ).collect();

        assert_eq!(&mapped[0], "TEST");
        assert_eq!(&mapped[1], "aaaaaaaa");
        assert_eq!(&mapped[2], "bbbbbb");
        assert_eq!(&mapped[3], "ccc");
    }
}
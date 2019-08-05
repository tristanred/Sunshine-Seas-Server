use std::iter::*;
use crate::messageblocks::*;
use std::io::{BufReader, Read};

// HELLO
#[derive(Debug)]
pub struct HelloCommand {
    // Command contents
    pub id: String,
    pub user: String,
    pub msg: String
}

impl HelloCommand {

    pub fn deserialize(data: &[u8]) -> Result<HelloCommand, &'static str> {
        let mut reader = BufReader::new(data);

        let mut id_bytes = [0; 8];
        reader.read_exact(&mut id_bytes);

        let mut user_bytes = [0; 32];
        reader.read_exact(&mut user_bytes);

        let mut msg_bytes = vec![];
        reader.read_to_end(&mut msg_bytes);

        let res = HelloCommand {
            id: String::from_utf8(trim_vec_end(&id_bytes)).unwrap(),
            user: String::from_utf8(trim_vec_end(&user_bytes)).unwrap(),
            msg: String::from_utf8(trim_vec_end(&msg_bytes)).unwrap()
        };

        return Ok(res);
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<Vec<u8>> = vec![];

        result.push(self.id.as_bytes().to_vec());
        result.push(self.user.as_bytes().to_vec());
        result.push(self.msg.as_bytes().to_vec());

        let flat: Vec<u8> = result.into_iter().flatten().collect();

        return flat;
    }

    pub fn from_client_message(data: &[u8]) -> Result<HelloCommand, &'static str> {
        let blocks = get_messageblocks_groups(data);





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

/**
 * Pad a buffer to a specific length of bytes.
 */
fn pad_string(buf: &[u8], padlen: usize) -> Vec<u8> {
    let mut result = vec![];

    for i in 0..padlen {
        if i < buf.len() {
            result.push(buf[i]);
        } else {
            result.push(0);
        }
    }

    return result;
}

fn trim_vec_end(buf: &[u8]) -> Vec<u8> {
    let mut res = vec![];

    let mut data_started = false;
    for i in (0..buf.len()).rev() {
        if data_started == true {
            res.push(buf[i]);
        } else if buf[i] != 0 {
            res.push(buf[i]);
            data_started = true;
        }
    }

    res.reverse();

    return res;
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

    #[test]
    fn test_hello_deserialize() {
        let mut cmd: Vec<u8> = Vec::new();

        let mut one = pad_string(b"HELLO", 8);
        let mut two = pad_string(b"TestUsername", 32);
        let mut three = pad_string(b"Super Message", 64);

        cmd.append(&mut one);
        cmd.append(&mut two);
        cmd.append(&mut three);

        // let test = HelloCommand {
        //     id: String::from_utf8(one).unwrap(),
        //     user: String::from_utf8(two).unwrap(),
        //     msg: String::from_utf8(three).unwrap()
        // };

        let test = HelloCommand::deserialize(&cmd).unwrap();

        assert_eq!(test.id, "HELLO");
        assert_eq!(test.user, "TestUsername");
        assert_eq!(test.msg, "Super Message");
    }

    #[test]
    fn test_hello_serialize() {

    }

    #[test]
    fn test_hello_serde() {

    }

    #[test]
    fn test_pad_string() {
        let test_string = pad_string(b"qwerty\0", 7);

        assert_eq!(test_string, b"qwerty\0");
    }

    #[test]
    fn test_trim_string() {
        let test_string = trim_vec_end(b"qwerty\0\0\0");

        assert_eq!(test_string, b"qwerty");
    }
}
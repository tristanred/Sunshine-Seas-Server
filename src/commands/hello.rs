use std::iter::*;
use std::io::{BufReader, Read};
use crate::utils::*;

// HELLO
#[derive(Debug, PartialEq)]
pub struct HelloCommand {
    pub id: String,
    pub user: String,
    pub msg: String
}

impl HelloCommand {
    /**
     * Creates a HelloCommand from a buffer.
     *
     * The data in the buffer must be formatted correctly using the message
     * structure of the server ABI.
     *
     * This basically calls the HelloCommand::deserialize function and returns
     * the result if it was able to deserialize correctly.
     */
    pub fn from_client_message(data: &[u8]) -> Result<HelloCommand, String> {
        let result = HelloCommand::deserialize(data)?;

        Ok(result)
    }

    /**
     * Create a HelloCommand with the information provided.
     */
    pub fn from_info(from_user: &str, with_message: &str) -> HelloCommand {
        HelloCommand {
            id: String::from("HELLO"),
            user: String::from(from_user),
            msg: String::from(with_message)
        }
    }

    /**
     * Deserialize a buffer into a HelloCommand instance.
     *
     * The buffer must be properly formatted with the protocol ABI or the
     * call will return an error.
     *
     * Right now, the data is not validated. As long as there are enough bytes
     * in the buffer it will return Ok even if the data is garbage. TODO.
     */
    pub fn deserialize(data: &[u8]) -> Result<HelloCommand, String> {
        let mut reader = BufReader::new(data);

        let mut id_bytes = [0; 8];
        reader.read_exact(&mut id_bytes).map_err(|_| "Unable to read message id from buffer.")?;

        let mut user_bytes = [0; 32];
        reader.read_exact(&mut user_bytes).map_err(|_| "Unable to read message user from buffer.")?;

        let mut msg_bytes = vec![];
        reader.read_to_end(&mut msg_bytes).map_err(|_| "Unable to read message content from buffer.")?;

        let res = HelloCommand {
            id: String::from_utf8(trim_vec_end(&id_bytes)).unwrap(),
            user: String::from_utf8(trim_vec_end(&user_bytes)).unwrap(),
            msg: String::from_utf8(trim_vec_end(&msg_bytes)).unwrap()
        };

        validate_command(&res)?;

        return Ok(res);
    }

    /**
     * Serialize the HelloCommand instance.
     *
     * The resulting buffer will be correctly formatted with the ABI rules
     * of the server protocol.
     */
    pub fn serialize(&self) -> Vec<u8> {
        let mut result: Vec<Vec<u8>> = vec![];

        result.push(pad_string(self.id.as_bytes(), 8));
        result.push(pad_string(self.user.as_bytes(), 32));
        result.push(self.msg.as_bytes().to_vec());

        let flat: Vec<u8> = result.into_iter().flatten().collect();

        return flat;
    }
}

fn validate_command(command: &HelloCommand) -> Result<(), String>
{
    if command.id != String::from("HELO") {
        let error = format!("HelloCommand has invalid ID [{}]", command.id);
        
        return Err(error);
    }

    if command.user.is_ascii() == false {
        let error = format!("HelloCommand has a username that's not entirely in ASCII [{}]", command.user);

        return Err(error);
    }

    if command.msg.is_ascii() == false {
        let error = format!("HelloCommand has a message that's not entirely in ASCII [{}]", command.msg);

        return Err(error);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Tests that deserializing a structure generates the correct fields 
     * without padding or extra bytes.
     */
    #[test]
    fn test_hello_deserialize() {
        let mut cmd: Vec<u8> = Vec::new();

        let mut one = pad_string(b"HELO", 8);
        let mut two = pad_string(b"TestUsername", 32);
        let mut three = pad_string(b"Super Message", 64);

        cmd.append(&mut one);
        cmd.append(&mut two);
        cmd.append(&mut three);

        let test = HelloCommand::deserialize(&cmd).unwrap();

        assert_eq!(test.id, "HELO");
        assert_eq!(test.user, "TestUsername");
        assert_eq!(test.msg, "Super Message");
    }

    /**
     * Tests that serializing the structure generates the correct data with the
     * appropriate padding and field sizes.
     */
    #[test]
    fn test_hello_serialize() {
        let test = HelloCommand {
            id: String::from("HELO"),
            user: String::from("TestUsername"),
            msg: String::from("Super Message")
        };

        let result = test.serialize();

        let one = pad_string(b"HELO", 8);
        let two = pad_string(b"TestUsername", 32);
        let three = b"Super Message".to_vec();

        let result_one: &[u8] = &result[0..8];
        assert_eq!(result_one.to_vec(), one);

        let result_two: &[u8] = &result[8..40];
        assert_eq!(result_two.to_vec(), two);

        let result_three: &[u8] = &result[40..];
        assert_eq!(result_three.to_vec(), three);
    }

    /**
     * Tests that building from existing information generates the correct
     * structure fields.
     */
    #[test]
    fn test_build_hello_command() {
        let test = HelloCommand::from_info("SuperUser", "SuperMessage");

        assert_eq!(test.user, String::from("SuperUser"));
        assert_eq!(test.msg, String::from("SuperMessage"));
    }

    /**
     * Tests that two commands with the same username and message compare as
     * equals.
     */
    #[test]
    fn test_hello_command_equality() {
        let cmd_one = HelloCommand::from_info("SuperUser", "SuperMessage");
        let cmd_two = HelloCommand::from_info("SuperUser", "SuperMessage");

        assert_eq!(cmd_one, cmd_two);
    }

    /**
     * Tests that the `validate_command` generates errors when presented
     * with invalid input.
     */
    #[test]
    fn test_validate_command() {
        let mut test = HelloCommand::from_info("HELO", "svcsdgdrfrg");

        // Test for wrong ID
        test.id = String::from("AAA");
        validate_command(&test).expect_err("ID is invalid");

        test.id = String::from("HELO");

        // Test for non-ascii username
        test.user = String::from("Gordon Freeman \u{039B}");

        validate_command(&test).expect_err("Username contains non ASCII characters");

        test.user = String::from("Gordon Freeman");

        // Test for non-ascii message
        test.msg = String::from("HIDDEN MESSAGE\u{03A8}");
        validate_command(&test).expect_err("Message contains non ASCII characters.");
    }
}
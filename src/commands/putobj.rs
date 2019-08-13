use std::iter::*;
use std::io::{BufReader, Read};
use crate::utils::*;

extern crate byteorder;
use byteorder::*;
use std::convert::*;

pub static PUTOBJ_MSG_ID: &'static str = "PUTOBJ";

// API Call : PUTOBJ
//
// Uploads a game object and a set of properties to the server. The caller
// can set several options when calling this method such as deleting an object
// and updating its properties.

#[derive(Debug)]
pub enum PutOperation {
    Add,
    Update,
    Delete
}

fn u64_to_put_operation(value: u64) -> Result<PutOperation, String> {
    match value {
        0 => Ok(PutOperation::Add),
        1 => Ok(PutOperation::Delete),
        2 => Ok(PutOperation::Update),
        _ => Err(String::from("Unknown value"))
    }
}

#[derive(Debug)]
pub struct PutObjCommand {
    pub id: String,
    pub operation: PutOperation, // Encoded as 8 bytes
    pub properties: Vec<ObjProperties>
}

impl PutObjCommand {
    pub fn from_client_message(data: &[u8]) -> Result<PutObjCommand, String> {
        Ok(PutObjCommand::from(data))
    }
}

impl From::<&[u8]> for PutObjCommand {
    fn from(buffer: &[u8]) -> Self {
        let mut reader = BufReader::new(buffer);

        let mut id_bytes = [0; 8];
        reader.read_exact(&mut id_bytes).unwrap();

        let mut oper_bytes = [0; 8];
        reader.read_exact(&mut oper_bytes).unwrap();

        let mut prop_len_bytes = [0; 8];
        reader.read_exact(&mut prop_len_bytes).unwrap();

        let prop_len: usize = buf_to_u64(prop_len_bytes).try_into().unwrap();

        let mut properties_list = vec![];
        for _i in 0..prop_len {
            let read_prop = ObjProperties::try_from(&mut reader).unwrap();

            properties_list.push(read_prop);
        }

        let res = PutObjCommand {
            id: String::from("PUTOBJ"),
            operation: u64_to_put_operation(buf_to_u64(oper_bytes)).unwrap(),
            properties: properties_list
        };

        return res;
    }
}

#[derive(Debug, PartialEq)]
pub struct ObjProperties {
    pub name: String,
    pub length: u32,
    pub data: Vec<u8>
}

impl ObjProperties {

    /**
     * Serialize the structure to a binary vector.
     */
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<Vec<u8>> = vec![];

        result.push(pad_string(self.name.as_bytes(), 8));
        result.push(u32_to_buf(usize_to_u32(self.data.len())));
        result.push(self.data.to_vec());

        return result.into_iter().flatten().collect();
    }
}

impl From::<&[u8]> for ObjProperties {
    fn from(buffer: &[u8]) -> Self {
        let mut reader = BufReader::new(buffer);

        ObjProperties::try_from(&mut reader).unwrap()
    }
}

impl TryFrom::<&mut BufReader<&[u8]>> for ObjProperties {
    type Error = String;

    fn try_from(reader: &mut BufReader<&[u8]>) -> Result<Self, Self::Error> {
        let mut name_buf = [0; 8];
        reader.read_exact(&mut name_buf).map_err(|_| "Unable to read name from buffer.")?;

        let mut length_buf = [0; 4];
        reader.read_exact(&mut length_buf).map_err(|_| "Unable to read length from buffer.")?;

        let data_buf_len = buf_to_u32(length_buf);
        let mut data_buf = vec![0; u32_to_usize(data_buf_len)];
        reader.read_exact(&mut data_buf).map_err(|_| "Unable to read data from buffer.")?;

        let result = ObjProperties {
            name: vec_to_trimmed_string(&name_buf).expect("Object property is not UTF8."),
            length: data_buf_len,
            data: data_buf
        };

        Ok(result)
    }
}

/*
 * Struct binary format :
 *
 * [id bytes]
 *
 * [name:8 bytes]
 * [length:4 bytes]
 * [data:length bytes]
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8_test() {
        let input = ObjProperties {
            name: String::from("TestProp"),
            length: 15,
            data: b"{ x: 5, y: 14 }".to_vec()
        };
        let bytes = input.to_bytes();

        let output = ObjProperties::from(&bytes[..]);

        assert_eq!(input, output);

        // Test the automatic blanket implementation given by the From trait.
        let output: &[u8] = &bytes;
        let tester: ObjProperties = output.into();
        assert_eq!(input, tester);
    }

    #[test]
    fn test_serialize() {
        // Name is exactly 8 bytes
        let props1 = ObjProperties {
            name: String::from("TestProp"),
            length: 15,
            data: b"{ x: 5, y: 14 }".to_vec()
        };

        // Name is 4 bytes, padded to 8 bytes
        let props2 = ObjProperties {
            name: String::from("Size"),
            length: 16,
            data: b"{ w: 50, h: 50 }".to_vec()
        };

        let serialized1 = props1.to_bytes();
        let serialized2 = props2.to_bytes();

        let mut prop1_buf = vec![];
        prop1_buf.append(&mut pad_string(b"TestProp", 8));
        prop1_buf.append(&mut u32_to_buf(15));
        prop1_buf.append(&mut b"{ x: 5, y: 14 }".to_vec());

        assert_eq!(serialized1, prop1_buf);

        let mut prop2_buf = vec![];
        prop2_buf.append(&mut pad_string(b"Size", 8));
        prop2_buf.append(&mut u32_to_buf(16));
        prop2_buf.append(&mut b"{ w: 50, h: 50 }".to_vec());

        assert_eq!(serialized2, prop2_buf);
    }

    #[test]
    fn test_deserialize_properties() {
        // Create 3 test structures with different properties
        let prop1 = ObjProperties {
            name: String::from("Position"),
            length: 64,
            data: vec![1; 64]
        };

        let prop2 = ObjProperties {
            name: String::from("Size"),
            length: 32,
            data: vec![2; 32]
        };

        let prop3 = ObjProperties {
            name: String::from("Texture"),
            length: 8,
            data: vec![3; 8]
        };

        // Serialize them
        let mut data_bytes = vec![];
        data_bytes.append(&mut prop1.to_bytes());
        data_bytes.append(&mut prop2.to_bytes());
        data_bytes.append(&mut prop3.to_bytes());

        // Recreate each object using a shared reader
        let mut reader = BufReader::new(data_bytes.as_slice());

        let obj1 = ObjProperties::try_from(&mut reader).unwrap();
        let obj2 = ObjProperties::try_from(&mut reader).unwrap();
        let obj3 = ObjProperties::try_from(&mut reader).unwrap();

        // Deserialized objects should be equal to the original structures
        assert_eq!(prop1, obj1);
        assert_eq!(prop2, obj2);
        assert_eq!(prop3, obj3);
    }
}
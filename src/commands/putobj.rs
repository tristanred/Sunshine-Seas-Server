use std::iter::*;
use std::io::{BufReader, Read};
use crate::utils::*;

extern crate byteorder;
use byteorder::*;
use std::convert::TryInto;

pub static PUTOBJ_MSG_ID: &'static str = "PUTOBJ";

// API Call : PUTOBJ
//
// Uploads a game object and a set of properties to the server. The caller
// can set several options when calling this method such as deleting an object
// and updating its properties.

pub enum PutOperation {
    Add,
    Update,
    Delete
}

pub struct PutObjCommand {
    pub id: String,
    pub properties: Vec<ObjProperties>,
    pub operation: PutOperation
}

#[derive(Debug, PartialEq)]
pub struct ObjProperties {
    pub name: String,
    pub length: u32,
    pub data: Vec<u8>
}

impl ObjProperties {
    pub fn from_bytes(data: &[u8]) -> Result<ObjProperties, String> {
        let mut reader = BufReader::new(data);

        ObjProperties::from_reader(&mut reader)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<Vec<u8>> = vec![];

        result.push(pad_string(self.name.as_bytes(), 8));
        result.push(u32_to_buf(usize_to_u32(self.data.len()))); // TODO : 0 length for now
        result.push(self.data.to_vec());

        return result.into_iter().flatten().collect();
    }

    pub fn from_reader(reader: &mut BufReader<&[u8]>) -> Result<ObjProperties, String> {

        let mut name_buf = [0; 8];
        reader.read_exact(&mut name_buf).map_err(|_| "Unable to read name from buffer.")?;

        let mut length_buf = [0; 4];
        reader.read_exact(&mut length_buf).map_err(|_| "Unable to read length from buffer.")?;

        // Technically should only read `length` bytes instead of reading
        // to the end.
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
 *
 * Struct binary format :
 *
 * [id bytes]
 *
 * [name:8B]
 * [length:4B]
 * [data:NB]
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_properties() {

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

        let mut data_bytes = vec![];
        data_bytes.append(&mut prop1.to_bytes());
        data_bytes.append(&mut prop2.to_bytes());
        data_bytes.append(&mut prop3.to_bytes());

        let mut reader = BufReader::new(data_bytes.as_slice());

        let obj1 = ObjProperties::from_reader(&mut reader).unwrap();
        let obj2 = ObjProperties::from_reader(&mut reader).unwrap();
        let obj3 = ObjProperties::from_reader(&mut reader).unwrap();

        assert_eq!(prop1, obj1);
        assert_eq!(prop2, obj2);
        assert_eq!(prop3, obj3);
    }
}
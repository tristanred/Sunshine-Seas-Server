extern crate byteorder;
use byteorder::*;
use std::io::{BufReader, Cursor};
use std::convert::TryInto;

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

/**
 * Pad a buffer to a specific length of bytes.
 */
pub fn pad_string(buf: &[u8], padlen: usize) -> Vec<u8> {
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

pub fn trim_vec_end(buf: &[u8]) -> Vec<u8> {
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

pub fn vec_to_trimmed_string(buf: &[u8]) -> Result<String, std::string::FromUtf8Error> {
    String::from_utf8(trim_vec_end(buf))
}

pub fn buf_to_u32(buf: [u8; 4]) -> u32 {
    let mut rdr = Cursor::new(buf);

    let x = rdr.read_u32::<LittleEndian>().unwrap();

    return x;
}

pub fn u32_to_buf(nb: u32) -> Vec<u8> {
    let mut res = vec![];

    res.write_u32::<LittleEndian>(nb).unwrap();

    return res;
}

pub fn u32_to_usize(nb: u32) -> usize {
    return nb.try_into().unwrap();
}

pub fn usize_to_u32(size: usize) -> u32 {
    return size.try_into().unwrap();
}

/**
 * Checks a condition and return an Err(String) if it is false. Returns Ok()
 * otherwise.
 */
pub fn result_from_condition(condition: bool, errorstring: String) -> Result<(), String> {
    if condition == false {
        return Err(errorstring);
    }

    Ok(())
}

/**
 * Checks a predicate and return an Err(String) if it is false. Returns Ok()
 * otherwise.
 */
pub fn result_from_predicate<F>(predicate: F, errorstring: String) -> Result<(), String>
    where F: Fn() -> bool
{
    if predicate() == false {
        return Err(errorstring);
    }

    Ok(())
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
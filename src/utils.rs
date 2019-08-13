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

pub fn buf_to_u64(buf: [u8; 8]) -> u64 {
    let mut rdr = Cursor::new(buf);

    let x = rdr.read_u64::<LittleEndian>().unwrap();

    return x;
}

pub fn u64_to_buf(nb: u64) -> Vec<u8> {
    let mut res = vec![];

    res.write_u64::<LittleEndian>(nb).unwrap();

    return res;
}

pub fn u32_to_usize(nb: u32) -> usize {
    return nb.try_into().unwrap();
}

pub fn usize_to_u32(size: usize) -> u32 {
    return size.try_into().unwrap();
}

/**
 * Checks a condition and return an Err(T) if it is false. Returns Ok()
 * otherwise.
 */
pub fn result_from_condition<T>(condition: bool, errordata: T) -> Result<(), T> {
    if condition == false {
        return Err(errordata);
    }

    Ok(())
}

/**
 * Checks a predicate and return an Err(T) if it is false. Returns Ok()
 * otherwise.
 */
pub fn result_from_predicate<F, T>(predicate: F, errordata: T) -> Result<(), T>
    where F: Fn() -> bool
{
    if predicate() == false {
        return Err(errordata);
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

        let test_string = trim_vec_end(b"qwerty");
        assert_eq!(test_string, b"qwerty");
    }

    #[test]
    fn test_vec_trim_tostring() {
        let test_string = vec_to_trimmed_string(b"ICanHazPadding\0\0\0").unwrap();
        assert_eq!(test_string, "ICanHazPadding");

        let test_string = vec_to_trimmed_string(b"NoPadding").unwrap();
        assert_eq!(test_string, "NoPadding");
    }

    #[test]
    fn test_buf_to_u32() {
        // Transformation function is using LSB
        let nb_buf: [u8; 4] = [15, 0, 0, 0];
        let res = buf_to_u32(nb_buf);
        assert_eq!(res, 15);

        let nb_buf: [u8; 4] = [15, 255, 0, 0];
        let res = buf_to_u32(nb_buf);
        assert_eq!(res, 0xFF0F);

        let nb_buf: [u8; 4] = [0xAA, 0xAA, 0xAA, 0xAA];
        let res = buf_to_u32(nb_buf);
        assert_eq!(res, 0xAAAAAAAA);
    }

    #[test]
    fn test_u32_to_buf() {
        let buf = u32_to_buf(55);
        assert_eq!(buf, [55, 0, 0, 0]);

        let buf = u32_to_buf(0xAABBCCDD);
        assert_eq!(buf, [0xDD, 0xCC, 0xBB, 0xAA]);
    }

    #[test]
    fn test_u32_to_usize() {
        // The test is using the same method to compare the usize as u32
        // so given the current implementation this test is fairly redundant.
        // But the u32_to_usize is meant as a shorthand so this checks that the
        // implementation stays valid.
        let nb: u32 = 55;
        let test: usize = u32_to_usize(nb);
        assert_eq!(nb, test.try_into().unwrap());
    }

    #[test]
    fn test_usize_to_u32() {
        // Running this test from the VSCode IDE also tests the below code
        // but without the [should_panic] attribute. Run with `cargo test`
        // instead.

        let nb: usize = 500;
        let test: u32 = usize_to_u32(nb);

        assert_eq!(nb, test.try_into().unwrap());
    }

    #[test]
    #[should_panic]
    fn test_usize_to_u32_failures() {
        // Since this test fails, the VSCode IDE does not show the little
        // handle to start the test inline. Run with `cargo test` instead.
        let nb: usize = 0x8000000000000000;
        let test: u32 = usize_to_u32(nb);
        assert_eq!(nb, test.try_into().unwrap());
    }

    #[test]
    fn test_result_from_condition() {
        let error_test = result_from_condition(false, "GOT ERROR");
        assert_eq!(error_test.is_err(), true);
        assert_eq!(error_test.err().unwrap(), "GOT ERROR");

        let ok_test = result_from_condition(true, "GOT ERROR");
        assert_eq!(ok_test.is_ok(), true);
    }

    #[test]
    fn test_result_from_predicate() {
        let error_test = result_from_predicate(|| false, "GOT ERROR");
        assert_eq!(error_test.is_err(), true);
        assert_eq!(error_test.err().unwrap(), "GOT ERROR");

        let ok_test = result_from_predicate(|| true, "GOT ERROR");
        assert_eq!(ok_test.is_ok(), true);
    }
}
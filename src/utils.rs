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

pub fn get_string_from_msgdata(slice: &[u8]) -> Result<String, String> {
    std::string::String::from_utf8(slice.to_vec()).map_err(|err| err.to_string())
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
/// Checks whether its argument is a valid UUID, per
/// https://webbluetoothcg.github.io/web-bluetooth/#dfn-valid-uuid
pub fn valid_uuid(uuid: &str) -> bool {
    if uuid.len() != 36 {
        return false;
    }
    for (index, char) in uuid.chars().enumerate() {
        match index {
            8 | 13 | 18 | 23 => {
                if char != '-' {
                    return false;
                }
            }
            _ => {
                match char {
                    '0' ... '9' | 'a' ... 'f' => (),
                    _ => return false,
                }
            }
        }
    }
    return true;
}

/// Checks whether its argument is a blacklist file that's usable in the algorithm at
/// https://webbluetoothcg.github.io/web-bluetooth/#dfn-parsing-the-blacklist
pub fn validate_blacklist(blacklist: &str) -> Option<String> {
    for (index, line) in blacklist.lines().enumerate() {
        let line_num = index + 1;
        if line.is_empty() || line.starts_with("#") {
            // Comment or blank line.
            continue;
        }
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens.len() {
            0 => unreachable!("split(' ') never returns an empty array"),
            1 | 2 => {
                let uuid = tokens[0];
                if !valid_uuid(uuid) {
                    return Some(format!("line {}: '{}' is not a valid UUID", line_num, uuid));
                }
                if tokens.len() == 2 {
                    let exclusion = tokens[1];
                    match exclusion {
                        "exclude-reads" | "exclude-writes" => (),
                        _ => return Some(format!(
			    "line {}: '{}' should be 'exclude-reads', or 'exclude-writes'",
			    line_num, exclusion)),
                    }
                }
            },
            _ => return Some(format!("line {}: Too many tokens", line_num)),
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_valid_uuid() {
	assert!( valid_uuid("00000000-0000-0000-0000-000000000000"));
	assert!( valid_uuid("01234567-89ab-cdef-0123-456789abcdef"));
	assert!(!valid_uuid("01234567-89AB-CDEF-0123-456789ABCDEF"));
	assert!(!valid_uuid("g1234567-89ab-cdef-0123-456789abcdef"));
	assert!(!valid_uuid("01234567-89ab-cdef-0123-456789abcdef0"));
	assert!(!valid_uuid("0123456789abcdef0123456789abcdef"));
	assert!(!valid_uuid("01234567089ab0cdef001230456789abcdef"));
    }

    #[test]
    fn test_validate_blacklist() {
        assert_eq!(None, validate_blacklist(""));
        // Lines are terminated by \n, not \r\n.
        assert_eq!(Some("line 1: '\r' is not a valid UUID".to_string()),
                   validate_blacklist("\r\n"));
        assert_eq!(None, validate_blacklist("# comment"));
        assert_eq!(None, validate_blacklist("# comment
00001812-0000-1000-8000-00805f9b34fb"));
        // No extraneous spaces.
        assert_eq!(Some("line 1: Too many tokens".to_string()),
		   validate_blacklist("  # comment
  00001812-0000-1000-8000-00805f9b34fb"));
        assert_eq!(
	    Some("line 1: Too many tokens".to_string()),
	    validate_blacklist("00001812-0000-1000-8000-00805f9b34fb # not a comment"));
        assert_eq!(
            Some("line 1: 'X0001812-0000-1000-8000-00805f9b34fb' is not a valid UUID".to_string()),
            validate_blacklist("X0001812-0000-1000-8000-00805f9b34fb"));
        assert_eq!(None,
                   validate_blacklist("00001812-0000-1000-8000-00805f9b34fb exclude-reads"));
        assert_eq!(None,
                   validate_blacklist("00001812-0000-1000-8000-00805f9b34fb exclude-writes"));
        assert_eq!(
            Some("line 1: '00001812-0000-1000-8000-00805f9b34fb\u{A0}exclude-reads' is not a valid UUID".to_string()),
            validate_blacklist("00001812-0000-1000-8000-00805f9b34fb\u{A0}exclude-reads"));
        assert_eq!(
            Some("line 1: 'X0001812-0000-1000-8000-00805f9b34fb' is not a valid UUID".to_string()),
            validate_blacklist("X0001812-0000-1000-8000-00805f9b34fb exclude-reads"));
        assert_eq!(
            Some("line 1: 'exclude' should be 'exclude-reads', or 'exclude-writes'".to_string()),
            validate_blacklist("00001812-0000-1000-8000-00805f9b34fb exclude"));
        assert_eq!(Some("line 1: Too many tokens".to_string()),
                   validate_blacklist("00001812-0000-1000-8000-00805f9b34fb token token"));
    }

    #[test]
    fn validate_gatt_blacklist() {
	let filename = "gatt_blacklist.txt";
        let content = File::open(filename).and_then(|mut file| {
            let mut result = String::new();
            try!(file.read_to_string(&mut result));
            Ok(result)
        }).unwrap_or_else(|e| { panic!("Error reading {}: {}", filename, e) });

        if let Some(error) = validate_blacklist(&content) {
	    panic!("{} is invalid: {}", filename, error);
	}
    }
}

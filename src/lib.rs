use std::collections::HashMap;

/// Checks whether its argument is a valid UUID, per
/// https://webbluetoothcg.github.io/web-bluetooth/#valid-uuid
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

/// Checks whether its argument is a blocklist file that's usable in the algorithm at
/// https://webbluetoothcg.github.io/web-bluetooth/#parsing-the-blocklist
pub fn validate_blocklist(blocklist: &str) -> Option<String> {
    let mut result = HashMap::<&str, &str>::new();
    for (index, line) in blocklist.split('\n').enumerate() {
        let line_num = index + 1;
        if line.is_empty() || line.starts_with("#") {
            // Comment or blank line.
            continue;
        }
        if line.starts_with(" ") {
            return Some(format!("line {}: Starts with extraneous space", line_num));
        }
        if line.ends_with(" ") {
            return Some(format!("line {}: Ends with extraneous space", line_num));
        }
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens.len() {
            0 => unreachable!("split(' ') never returns an empty array"),
            1 | 2 => {
                let uuid = tokens[0];
                if !valid_uuid(uuid) {
                    return Some(format!("line {}: '{}' is not a valid UUID", line_num, uuid));
                }
                let mut exclusion = "exclude";
                if tokens.len() == 2 {
                    exclusion = tokens[1];
                    match exclusion {
                        "exclude-reads" | "exclude-writes" => (),
                        _ => return Some(format!(
			    "line {}: '{}' should be 'exclude-reads' or 'exclude-writes'",
			    line_num, exclusion)),
                    }
                }
                if let Some(_) = result.insert(uuid, exclusion) {
                    return Some(format!(
                        "line {}: '{}' appears multiple times",
                        line_num, uuid));
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
    fn test_validate_blocklist() {
        assert_eq!(None, validate_blocklist(""));
        // Lines are terminated by \n, not \r\n.
        assert_eq!(Some("line 1: '\r' is not a valid UUID".to_string()),
                   validate_blocklist("\r\n"));
        assert_eq!(None, validate_blocklist("# comment"));
        assert_eq!(None, validate_blocklist("# comment\n\
                                             00001812-0000-1000-8000-00805f9b34fb"));

        // No extraneous spaces:
        assert_eq!(Some("line 1: Starts with extraneous space".to_string()),
		   validate_blocklist("  # comment"));
        assert_eq!(Some("line 1: Starts with extraneous space".to_string()),
		   validate_blocklist(" 00001812-0000-1000-8000-00805f9b34fb"));
        assert_eq!(Some("line 1: Ends with extraneous space".to_string()),
		   validate_blocklist("00001812-0000-1000-8000-00805f9b34fb "));

        assert_eq!(
	    Some("line 1: Too many tokens".to_string()),
	    validate_blocklist("00001812-0000-1000-8000-00805f9b34fb # not a comment"));
        assert_eq!(
            Some("line 1: 'X0001812-0000-1000-8000-00805f9b34fb' is not a valid UUID".to_string()),
            validate_blocklist("X0001812-0000-1000-8000-00805f9b34fb"));
        assert_eq!(None,
                   validate_blocklist("00001812-0000-1000-8000-00805f9b34fb exclude-reads"));
        assert_eq!(None,
                   validate_blocklist("00001812-0000-1000-8000-00805f9b34fb exclude-writes"));
        assert_eq!(
            Some("line 1: '00001812-0000-1000-8000-00805f9b34fb\u{A0}exclude-reads' is not a valid UUID".to_string()),
            validate_blocklist("00001812-0000-1000-8000-00805f9b34fb\u{A0}exclude-reads"));
        assert_eq!(
            Some("line 1: 'X0001812-0000-1000-8000-00805f9b34fb' is not a valid UUID".to_string()),
            validate_blocklist("X0001812-0000-1000-8000-00805f9b34fb exclude-reads"));
        assert_eq!(
            Some("line 1: 'exclude' should be 'exclude-reads' or 'exclude-writes'".to_string()),
            validate_blocklist("00001812-0000-1000-8000-00805f9b34fb exclude"));
        assert_eq!(Some("line 1: Too many tokens".to_string()),
                   validate_blocklist("00001812-0000-1000-8000-00805f9b34fb token token"));

        // Check all variants of repeated UUIDs.
        assert_eq!(
            Some("line 3: '00001812-0000-1000-8000-00805f9b34fb' appears multiple times".to_string()),
            validate_blocklist("00001812-0000-1000-8000-00805f9b34fb\n\
                                00001810-0000-1000-8000-00805f9b34fb\n\
                                00001812-0000-1000-8000-00805f9b34fb\n"));
        assert_eq!(
            Some("line 3: '00001812-0000-1000-8000-00805f9b34fb' appears multiple times".to_string()),
            validate_blocklist("00001812-0000-1000-8000-00805f9b34fb\n\
                                00001810-0000-1000-8000-00805f9b34fb\n\
                                00001812-0000-1000-8000-00805f9b34fb exclude-reads\n"));
        assert_eq!(
            Some("line 3: '00001812-0000-1000-8000-00805f9b34fb' appears multiple times".to_string()),
            validate_blocklist("00001812-0000-1000-8000-00805f9b34fb exclude-reads\n\
                                00001810-0000-1000-8000-00805f9b34fb\n\
                                00001812-0000-1000-8000-00805f9b34fb\n"));
        assert_eq!(
            Some("line 3: '00001812-0000-1000-8000-00805f9b34fb' appears multiple times".to_string()),
            validate_blocklist("00001812-0000-1000-8000-00805f9b34fb exclude-reads\n\
                                00001810-0000-1000-8000-00805f9b34fb\n\
                                00001812-0000-1000-8000-00805f9b34fb exclude-reads\n"));
    }

    #[test]
    fn validate_gatt_blocklist() {
	let filename = "gatt_blocklist.txt";
        let content = File::open(filename).and_then(|mut file| {
            let mut result = String::new();
            try!(file.read_to_string(&mut result));
            Ok(result)
        }).unwrap_or_else(|e| { panic!("Error reading {}: {}", filename, e) });

        if let Some(error) = validate_blocklist(&content) {
	    panic!("{} is invalid: {}", filename, error);
	}
    }
}

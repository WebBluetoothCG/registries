use std::collections::HashMap;

pub fn valid_hex_digit(digit: char) -> bool {
    return digit.is_ascii_digit() || (digit.is_ascii_lowercase() && digit.is_ascii_hexdigit());
}

pub fn valid_hex_string(hex_string: &str) -> bool {
    let mut chars = hex_string.chars();

    while let Some(char) = chars.next() {
        if !valid_hex_digit(char) {
            return false;
        }
    }

    return true;
}

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
                if !valid_hex_digit(char) {
                    return false;
                }
            }
        }
    }
    return true;
}

/// Checks whether its argument is a valid company identifer, per
/// https://webbluetoothcg.github.io/web-bluetooth/#valid-company-identifier
pub fn valid_company_identifier(company_identifier: &str) -> bool {
    if company_identifier.len() == 0 || company_identifier.len() > 4 {
        return false;
    }
    if !valid_hex_string(company_identifier) {
        return false;
    }
    return true;
}

/// Checks whether its argument is a valid advertise data prefix, per
/// https://webbluetoothcg.github.io/web-bluetooth/#valid-advertise-data-prefix
pub fn valid_advertise_data_prefix(data_prefix: &str) -> bool {
    let tokens: Vec<&str> = data_prefix.split('-').collect();
    if tokens.len() != 2 {
        return false;
    }
    if tokens[0] != "advdata" {
        return false;
    }
    let pair: Vec<&str> = tokens[1].split('/').collect();
    if pair.len() != 2 || pair[0].len() != pair[1].len() {
        return false;
    }
    if !valid_hex_string(pair[0]) || !valid_hex_string(pair[1]) {
        return false;
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
            3 => {
                if tokens[0] != "manufacturer" {
                    return Some(format!(
                        "line {}: Invalid token '{}' for manufacturer data", line_num, tokens[0]));
                }
                if !valid_company_identifier(tokens[1]) {
                    return Some(format!(
                        "line {}: Invalid company identifier '{}'", line_num, tokens[1]));
                }
                if !valid_advertise_data_prefix(tokens[2]) {
                    return Some(format!(
                        "line {}: Invalid advertise data prefix '{}'", line_num, tokens[2]));
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
    fn test_valid_company_identifier() {
        assert!(valid_company_identifier("4c"));
        assert!(valid_company_identifier("04c"));
        assert!(valid_company_identifier("004c"));
        assert!(!valid_company_identifier("4C"));
        assert!(!valid_company_identifier("0004C"));
        assert!(!valid_company_identifier("h"));
        assert!(!valid_company_identifier(""));
    }

    #[test]
    fn test_valid_advertise_data_prefix() {
        assert!(valid_advertise_data_prefix("advdata-02/ff"));
        assert!(valid_advertise_data_prefix("advdata-0277/ff1c"));
        assert!(!valid_advertise_data_prefix("ad-02/ff"));
        assert!(!valid_advertise_data_prefix("advdata-02-ff"));
        assert!(!valid_advertise_data_prefix("advdata-027/ff"));
        assert!(!valid_advertise_data_prefix("advdata-027"));
        assert!(!valid_advertise_data_prefix("advdata-027/f"));
        assert!(!valid_advertise_data_prefix("advdata-027/FF"));
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
        assert_eq!(Some("line 1: Invalid token '00001812-0000-1000-8000-00805f9b34fb' for manufacturer data".to_string()),
                   validate_blocklist("00001812-0000-1000-8000-00805f9b34fb token token"));
        assert_eq!(
            Some("line 1: Invalid company identifier '0004c'".to_string()),
            validate_blocklist("manufacturer 0004c advdata-02/ff"));
        assert_eq!(
            Some("line 1: Invalid advertise data prefix 'advdata-02/fff'".to_string()),
            validate_blocklist("manufacturer 4c advdata-02/fff"));

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
            file.read_to_string(&mut result)?;
            Ok(result)
        }).unwrap_or_else(|e| { panic!("Error reading {}: {}", filename, e) });

        if let Some(error) = validate_blocklist(&content) {
	    panic!("{} is invalid: {}", filename, error);
	}
    }

    #[test]
    fn validate_manufacturer_data_blocklist() {
	let filename = "manufacturer_data_blocklist.txt";
        let content = File::open(filename).and_then(|mut file| {
            let mut result = String::new();
            file.read_to_string(&mut result)?;
            Ok(result)
        }).unwrap_or_else(|e| { panic!("Error reading {}: {}", filename, e) });

        if let Some(error) = validate_blocklist(&content) {
	    panic!("{} is invalid: {}", filename, error);
	}
    }
}

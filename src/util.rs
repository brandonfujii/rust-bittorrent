use bencode::{Bencode};
use std::collections::BTreeMap;
use bencode::util::ByteString;
use regex::Regex;

#[derive(Debug)]
pub enum Error {
    DictMatchErr,
    FieldNotFound
}

/// Takes a string which is denoted within quotation marks and returns that string, or the original
/// string if no match is found
///
/// # Example
///
/// ```
/// let s = parse_string("s\"pieces\"")?;
/// assert_eq!(s, "pieces");
/// ```
pub fn parse_string(s: &str) -> String {
    let re = Regex::new("\"([0-9a-zA-Z.:/]+)\"").unwrap();

    if re.is_match(s) {
        let cap = re.captures(s).unwrap();
        return (&cap[1]).to_string();
    }

    s.to_string()
}

/// Finds a value in the BTreeMap corresponding to a given key and returns a Result containing
///     1) a Vec<u8> of the data, if it exists
///     2) a FieldNotFound error otherwise
pub fn decode_field_as_bytes(map: &BTreeMap<ByteString, Bencode>, field: &str) -> Result<Vec<u8>, Error> {
    match map.get(&ByteString::from_str(field)) {
        Some(contents) => Ok(contents.to_bytes().unwrap()),
        None => Err(Error::FieldNotFound)
    }
}

/// Finds a value in the BTreeMap corresponding to a given key and returns a Result containing
///     1) a String of the data, if it exists
///     2) a FieldNotFound error otherwise
pub fn decode_field_as_string(map: &BTreeMap<ByteString, Bencode>, field: &str) -> Result<String, Error> {
    match map.get(&ByteString::from_str(field)) {
        Some(contents) => {
            Ok(parse_string(&contents.to_string()))
        }
        None => Err(Error::FieldNotFound)
    }
}

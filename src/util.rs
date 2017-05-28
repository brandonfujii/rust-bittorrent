use bencode::{Bencode};
use std::collections::BTreeMap;
use bencode::util::ByteString;
use regex::Regex;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use rand::{thread_rng, Rng};

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
    let re = Regex::new("\"(.*)\"").unwrap();

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

pub fn decode_field_as_content_bytes(map: &BTreeMap<ByteString, Bencode>, field: &str) -> Result<Vec<u8>, Error> {
    if let Ok(mut contents) = decode_field_as_bytes(&map, field) {
        if let Some(i) = contents.iter().position(|&b| b == 58) {
            return Ok(contents.split_off(i).split_off(1));
        }
    }

    Err(Error::FieldNotFound)
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

pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    let mut buf = Cursor::new(&bytes);
    buf.read_u32::<BigEndian>().unwrap()
}

pub fn u32_to_bytes(integer: u32) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.write_u32::<BigEndian>(integer).unwrap();
    bytes
}

// Azureus-style peer_id formatting
pub fn create_peer_id() -> String {
    let random_chars: String = thread_rng().gen_ascii_chars().take(12).collect();
    format!("{}{}", "-AZ2060-", random_chars)
}

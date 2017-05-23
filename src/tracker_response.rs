use bencode;
use bencode::{Bencode, FromBencode};
use bencode::util::ByteString;
use std::collections::BTreeMap;
use regex::Regex;

/// Finds a value in the BTreeMap corresponding to a given key and returns a Result containing
///     1) a Vec<u8> of the data, if it exists
///     2) a FieldNotFound error otherwise
fn decode_field_as_bytes(map: &BTreeMap<ByteString, Bencode>, field: &str) -> Result<Vec<u8>, Error> {
    match map.get(&ByteString::from_str(field)) {
        Some(contents) => Ok(contents.to_bytes().unwrap()),
        None => Err(Error::FieldNotFound)
    }
}

/// Finds a value in the BTreeMap corresponding to a given key and returns a Result containing
///     1) a String of the data, if it exists
///     2) a FieldNotFound error otherwise
fn decode_field_as_string(map: &BTreeMap<ByteString, Bencode>, field: &str) -> Result<String, Error> {
    match map.get(&ByteString::from_str(field)) {
        Some(contents) => {
            Ok(parse_string(&contents.to_string()))
        }
        None => Err(Error::FieldNotFound)
    }
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
fn parse_string(s: &str) -> String {
    let re = Regex::new("\"([0-9a-zA-Z.:/]+)\"").unwrap();

    if re.is_match(s) {
        let cap = re.captures(s).unwrap();
        return (&cap[1]).to_string();
    }

    s.to_string()
}

#[derive(Debug)]
pub enum Error {
    DictMatchErr,
    FieldNotFound
}

#[derive(Debug)]
pub struct TrackerResponse {
    // pub failure_reason: String,
    // pub warning_message: String,
    pub interval: u32,
    // pub min_interval: u32,
    // pub tracker_id: String,
    pub complete: u32,
    pub incomplete: u32,
    pub peers: Vec<u8>,
}

impl FromBencode for TrackerResponse {
    type Err = Error;

    /// Attempts to construct a TrackerResponse object from a Bencode object. Returns a Result
    /// containing either
    ///     1) a TrackerResponse object, if a proper Bencode object was passed in
    ///     2) DictMatchErr otherwise
    fn from_bencode(bn: &bencode::Bencode) -> Result<TrackerResponse, Error> {
        match bn {
            &Bencode::Dict(ref m) => {
                let interval = decode_field_as_string(m, "interval")?;
                let complete = decode_field_as_string(m, "complete")?;
                let incomplete = decode_field_as_string(m, "incomplete")?;
                let peers = decode_field_as_bytes(m, "peers")?;

                let tracker_response = TrackerResponse {
                    interval: interval.parse::<u32>().unwrap(),
                    complete: complete.parse::<u32>().unwrap(),
                    incomplete: incomplete.parse::<u32>().unwrap(),
                    peers: peers
                };

                Ok(tracker_response)
            }
            _ => Err(Error::DictMatchErr)
        }
    }
}

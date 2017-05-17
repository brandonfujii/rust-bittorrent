use bencode;
use bencode::{Bencode, FromBencode};
use bencode::util::ByteString;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use hash;

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
pub struct MetaInfo {
    pub announce: String,
    pub created_by: String,
    pub info: Info,
    pub info_hash: String,
}

impl FromBencode for MetaInfo {
    type Err = Error;

    /// Attempts to construct a MetaInfo object from a Bencode object. Returns a Result containing either
    ///     1) a MetaInfo object, if a proper Bencode object was passed in
    ///     2) DictMatchErr otherwise
    fn from_bencode(bn: &bencode::Bencode) -> Result<MetaInfo, Error> {
        match bn {
            &Bencode::Dict(ref m) => {
                let b = decode_field_as_bytes(m, "info")?;
                let info: Bencode = bencode::from_vec(b.to_owned()).unwrap();
                let decoded: Result<Info, Error> = FromBencode::from_bencode(&info);
                let info_hash = hash::sha(&b);

                let announce = decode_field_as_string(m, "announce")?;
                let created_by;
                match decode_field_as_string(m, "created by") {
                    Ok(s) => created_by = s,
                    Err(_) => created_by = String::from("")
                }

                let metainfo = MetaInfo {
                    announce: announce,
                    created_by: created_by,
                    info: decoded?,
                    info_hash: info_hash,
                };

                Ok(metainfo)
            }
            _ => Err(Error::DictMatchErr)
        }
    }
}

#[derive(Debug)]
pub struct Info {
    pub piece_length: u32,
    pub pieces: Vec<u8>,
    pub num_pieces: u32,
    pub name: String,
    pub length: u64,
}

impl FromBencode for Info {
    type Err = Error;

    /// Attempts to construct an Info object from a Bencode object. Returns a Result containing
    /// either:
    ///     1) an Info object, if a proper Bencode object was passed in
    ///     2) DictMatchErr otherwise
    fn from_bencode(bencode: &bencode::Bencode) -> Result<Info, Error> {
        match bencode {
            &Bencode::Dict(ref m) => {
                let pieces = decode_field_as_bytes(m, "pieces")?;
                let num_pieces = pieces.len() as u32;
                let length = decode_field_as_string(m, "length")?;
                let piece_length = decode_field_as_string(m, "piece length")?;

                let info = Info {
                    piece_length: piece_length.parse::<u32>().unwrap(),
                    pieces: pieces,
                    num_pieces: num_pieces,
                    name: decode_field_as_string(m, "name")?,
                    length: length.parse::<u64>().unwrap(),
                };
                Ok(info)
            }
            _ => Err(Error::DictMatchErr)
        }
    }
}

/// Attempts to construct a MetaInfo object from a torrent file located at the specified path.
/// Returns a Result containing either:
///     1) a MetaInfo object, if one can successfully be created
///     2) an Error otherwise
///
/// # Example
///
/// ```
/// # use metainfo;
/// let m = metainfo::from_file("data/flagfromserver.torrent");
/// ```
pub fn from_file(filename: &String) -> Result<MetaInfo, Error> {
    let mut f = File::open(filename).unwrap();
    let mut s = Vec::new();
    f.read_to_end(&mut s).unwrap();

    let torrent: Bencode = bencode::from_vec(s).unwrap();
    FromBencode::from_bencode(&torrent)
}

#[cfg(test)]
mod from_bencode_tests {
    use bencode;
    use std::io::prelude::*;
    use std::fs::File;
    use super::{MetaInfo, FromBencode, Error};

    #[test]
    fn flagfromserver_torrent_test() {
        let mut f = File::open("data/flagfromserver.torrent").unwrap();
        let mut s = Vec::new();
        f.read_to_end(&mut s).unwrap();

        let torrent: bencode::Bencode = bencode::from_vec(s).unwrap();
        let decoded: Result<MetaInfo, Error> = FromBencode::from_bencode(&torrent);

        match decoded {
            Ok(metainfo) => {
                assert_eq!(metainfo.announce, "http://thomasballinger.com:6969/announce");
                assert_eq!(metainfo.info.name, "flag.jpg");
            }
            _ => panic!("Decoded bencode incorrectly")
        }
    }
}

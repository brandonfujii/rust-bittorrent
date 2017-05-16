use bencode;
use bencode::{Bencode, FromBencode};
use bencode::util::ByteString;
use std::collections::BTreeMap;

use std::io::prelude::*;
use std::fs::File;

fn decode_field_as_bytes(map: &BTreeMap<ByteString, Bencode>, field: &str) -> Result<Vec<u8>, Error> {
    match map.get(&ByteString::from_str(field)) {
        Some(contents) => Ok(contents.to_bytes().unwrap()),
        None => Err(Error::FieldNotFound)
    }
}

fn decode_field_as_string(map: &BTreeMap<ByteString, Bencode>, field: &str) -> Result<String, Error> {
    match map.get(&ByteString::from_str(field)) {
        Some(contents) => Ok(contents.to_string()),
        None => Err(Error::FieldNotFound)
    }
}

#[derive(Debug)]
pub enum Error {
    DictMatchErr,
    FieldNotFound
}

#[derive(PartialEq, Debug)]
pub struct MetaInfo {
    pub announce: String,
    pub created_by: String,
    pub info: Info,
}

impl FromBencode for MetaInfo {
    type Err = Error;

    fn from_bencode(bn: &bencode::Bencode) -> Result<MetaInfo, Error> {
        match bn {
            &Bencode::Dict(ref m) => {
                let b = decode_field_as_bytes(m, "info")?;
                let info: Bencode = bencode::from_vec(b).unwrap();
                let decoded: Result<Info, Error> = FromBencode::from_bencode(&info);

                let announce = decode_field_as_string(m, "announce")?;
                let created_by = decode_field_as_string(m, "created by")?;

                let metainfo = MetaInfo {
                    announce: announce,
                    created_by: created_by,
                    info: decoded?
                };

                Ok(metainfo)
            }
            _ => Err(Error::DictMatchErr)
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Info {
    pub piece_length: u32,
    pub pieces: Vec<u8>,
    pub num_pieces: u32,
    pub name: String,
    pub length: u64,
}

impl FromBencode for Info {
    type Err = Error;

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

pub fn from_file(filename: &String) -> Result<MetaInfo, Error> {
    let mut f = File::open(filename).unwrap();
    let mut s = Vec::new();
    f.read_to_end(&mut s).unwrap();

    let torrent: Bencode = bencode::from_vec(s).unwrap();
    FromBencode::from_bencode(&torrent)
}

#[test]
fn bencode_test() {
    let mut f = File::open("flagfromserver.torrent").unwrap();
    let mut s = Vec::new();
    f.read_to_end(&mut s).unwrap();

    let torrent: Bencode = bencode::from_vec(s).unwrap();
    let _: Result<MetaInfo, Error> = FromBencode::from_bencode(&torrent);
}

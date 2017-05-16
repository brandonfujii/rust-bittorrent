use bencode;
use bencode::{Bencode, FromBencode};
use bencode::util::ByteString;
use std::collections::BTreeMap;

use std::fmt;
use std::io::prelude::*;
use std::fs::File;

fn decode_field_as_bytes(map: &BTreeMap<ByteString, Bencode>, field: &str) -> Vec<u8> {
    match map.get(&ByteString::from_str(field)) {
        Some(contents) => contents.to_bytes().unwrap(),
        None => panic!("Does not contain expected field")
    }
}

fn get_field(map: &BTreeMap<ByteString, Bencode>, field: &str) -> Result<String, Error> {
    match map.get(&ByteString::from_str(field)) {
        Some(contents) => Ok(contents.to_string()),
        None => panic!("Field not found!")
    }
}

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
                let b = decode_field_as_bytes(m, "info");
                let info: Bencode = bencode::from_vec(b).unwrap();
                let decoded: Result<Info, Error> = FromBencode::from_bencode(&info);

                let announce = get_field(m, "announce")?;
                let created_by = get_field(m, "created by")?;

                let metainfo = MetaInfo {
                    announce: announce,
                    created_by: created_by,
                    info: decoded?
                };

                Ok(metainfo)
            }
            _ => panic!("Dict match error")
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
                let pieces: Vec<u8> = decode_field_as_bytes(m, "pieces");
                let num_pieces = pieces.len() as u32;
                let length = get_field(m, "length")?;
                let piece_length = get_field(m, "piece length")?;

                let info = Info {
                    piece_length: piece_length.parse::<u32>().unwrap(),
                    pieces: pieces,
                    num_pieces: num_pieces,
                    name: get_field(m, "name")?,
                    length: length.parse::<u64>().unwrap(),
                };
                Ok(info)
            }
            _ => Err(Error::DictMatchErr)
        }
    }
}

#[test]
fn bencode_test() {
    let mut f = File::open("flagfromserver.torrent").unwrap();
    let mut s = Vec::new();
    f.read_to_end(&mut s).unwrap();

    let torrent: Bencode = bencode::from_vec(s).unwrap();
    let decoded: Result<MetaInfo, Error> = FromBencode::from_bencode(&torrent);

    match decoded {
        Ok(metainfo) => {println!("{:?}", metainfo)}
        Err(e) => panic!("Metainfo error!")
    }
}

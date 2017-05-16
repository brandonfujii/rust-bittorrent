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
}

impl FromBencode for MetaInfo {
    type Err = Error;

    fn from_bencode(bn: &bencode::Bencode) -> Result<MetaInfo, Error> {
        match bn {
            &Bencode::Dict(ref m) => {
                // let info = decode_field_as_bytes(m, "info");
                let announce = get_field(m, "announce")?;
                let created_by = get_field(m, "created by")?;

                let metainfo = MetaInfo {
                    announce: announce,
                    created_by: created_by,
                };

                Ok(metainfo)
            }
            _ => panic!("Dict match error")
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

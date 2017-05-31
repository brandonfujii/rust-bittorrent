use bencode;
use bencode::{Bencode, FromBencode};
use std::io::prelude::*;
use std::fs::File;
use hash;
use util::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MetaInfo {
    pub announce: String,
    pub created_by: String,
    pub info: Info,
    pub info_hash: Vec<u8>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Info {
    pub piece_length: u32,
    // an array of sha1 hashes which we will use to verify that the downloads were not corrupted
    pub pieces: Vec<Vec<u8>>,
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
                let pieces_bytes = decode_field_as_content_bytes(m, "pieces")?;
                let pieces: Vec<Vec<u8>> = pieces_bytes.chunks(20).map(|v| v.to_owned()).collect();
                let num_pieces = pieces.len() as u32;
                let length = decode_field_as_string(m, "length")?;
                let piece_length = decode_field_as_string(m, "piece length")?;
                let name = decode_field_as_string(m, "name")?;

                let info = Info {
                    piece_length: piece_length.parse::<u32>().unwrap(),
                    pieces: pieces,
                    num_pieces: num_pieces,
                    name: name,
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
mod metainfo_tests {
    use bencode;
    use std::io::prelude::*;
    use std::fs::File;
    use super::{MetaInfo, FromBencode};
    use util::*;

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

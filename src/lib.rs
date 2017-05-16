extern crate rustc_serialize;
extern crate bencode;

use rustc_serialize::{Encodable, Decodable};

use bencode::{encode, Decoder};

use std::fmt;
use std::io::prelude::*;
use std::fs::File;

mod metainfo;

type Buf = Vec<u8>;

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
struct TorrentFile {
    length: usize,
    path: Vec<Buf>
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
struct TorrentInfo {
    files: Vec<TorrentFile>,
    name: Buf,
    piecelength: usize,
    pieces: Buf
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
struct Torrent {
    announce: Buf,
    announcelist: Vec<Vec<Buf>>,
    createdby: Buf,
    creationdate: usize,
    info: TorrentInfo
}

impl fmt::Display for Torrent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "test")
    }
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
struct MyStruct {
    a: i32,
    b: String,
    c: Vec<u8>,
}

pub fn tracker() {
    let mut f = File::open("e.torrent").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);

    let s = s.trim();

    let b: Vec<u8> = s.as_bytes().to_vec();

    let torrent: bencode::Bencode = bencode::from_vec(b).unwrap();
    let mut decoded_torrent = Decoder::new(&torrent);
    let result: Torrent;

    match Decodable::decode(&mut decoded_torrent) {
        Ok(torrent) => {
            result = torrent;
            println!("{:?}", result);
        }
        Err(e) => {
            println!("Decoder error: {:?}", e);
        }
    }
}

#[test]
fn basic_test() {
	assert!(true);
    // tracker();
}

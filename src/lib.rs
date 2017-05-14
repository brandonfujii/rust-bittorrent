extern crate rustc_serialize;
extern crate bencode;

use rustc_serialize::Decodable;

use bencode::Decoder;

use std::fmt;
use std::io::prelude::*;
use std::fs::File;

type Buf = Vec<u8>;

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct TorrentFile {
    length: usize,
    path: Vec<Buf>
}

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct TorrentInfo {
    files: Vec<TorrentFile>,
    name: Buf,
    piece_length: usize,
    pieces: Buf
}

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct Torrent {
    announce: Buf,
    announce_list: Vec<Vec<Buf>>,
    created_by: Buf,
    creation_date: usize,
    info: TorrentInfo
}

impl fmt::Display for Torrent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "test")
    }
}

pub fn tracker() {
    let mut f = File::open("t.torrent").unwrap();
    let mut b = Vec::new();
    let _ = f.read_to_end(&mut b);

    let torrent: bencode::Bencode = bencode::from_vec(b).unwrap();
    let mut decoded_torrent = Decoder::new(&torrent);
    let result: Torrent;

    match Decodable::decode(&mut decoded_torrent) {
        Ok(torrent) => {
            result = torrent;
            println!("{}", result);
        }
        Err(e) => {
            println!("Decoder error: {:?}", e);
        }
    }
}

#[test]
fn basic_test() {
    tracker();
}

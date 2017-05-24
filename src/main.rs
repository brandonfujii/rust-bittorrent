extern crate bencode;
extern crate hyper;
extern crate regex;
extern crate urlencoding;
extern crate url;

use std::env;

mod metainfo;
mod tracker;
mod tracker_response;
mod hash;
mod util;
mod peer;
mod block;
mod piece;
mod torrent;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let m = metainfo::from_file(filename).unwrap();

    let peers = tracker::retrieve_peers(&m, "tovatovatovatovatova", "8080").unwrap();
    // println!("{:?}", peers);
    let torrent = torrent::Torrent::new(m);
    println!("{:?}", torrent);
}

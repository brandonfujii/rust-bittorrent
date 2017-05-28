extern crate bencode;
extern crate hyper;
extern crate regex;
extern crate urlencoding;
extern crate url;
extern crate byteorder;
extern crate rand;

use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

mod metainfo;
mod tracker;
mod tracker_response;
mod hash;
mod util;
mod peer;
mod block;
mod piece;
mod torrent;
mod connection;
mod message;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let m = metainfo::from_file(filename).unwrap();

    let peers = tracker::retrieve_peers(&m, "tovatovatovatovatova", "8080").unwrap();
    let torrent = torrent::Torrent::new(m);
    let mut torrent_arc = Arc::new(Mutex::new(torrent));
    let client = peer::Peer::from_bytes(&[127, 0, 0, 1, 31, 144]);

    let _ = peers.into_iter().map(|peer| {
    	let mut p = &mut peer;
    	let _ = thread::spawn(move || {
    		let _ = connection::Connection::connect(client, p, torrent_arc);
	    });
    });
}

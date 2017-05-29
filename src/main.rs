extern crate bencode;
extern crate hyper;
extern crate regex;
extern crate urlencoding;
extern crate url;
extern crate byteorder;
extern crate rand;

use std::{env, thread};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

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
    let peer_id: String = util::create_peer_id();

    let peers = tracker::retrieve_peers(&m, &peer_id, "8080").unwrap();
    let torrent = torrent::Torrent::new(peer_id, m);
    let torrent_mutex = Arc::new(Mutex::new(torrent));
    let client_mutex = Arc::new(Mutex::new(peer::Peer::from_bytes(&[127, 0, 0, 1, 31, 144])));

    let threads: Vec<JoinHandle<()>> = peers.into_iter().map(|peer| {
        let torrent_mutex = torrent_mutex.clone();
        let client_mutex = client_mutex.clone();
        thread::spawn(move || {
            let _ = connection::Connection::connect(client_mutex, peer, torrent_mutex);
        })
    }).collect();

    // wait for peers to complete
    for t in threads {
        t.join().unwrap();
    }
}

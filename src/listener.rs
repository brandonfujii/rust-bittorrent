use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::net::IpAddr;
use util;

use connection::Connection;
use torrent::Torrent;
use peer::Peer;
use message::Message;

pub fn start(host: &str, port: u16, client_mutex: Arc<Mutex<Peer>>, torrent_mutex: Arc<Mutex<Torrent>>) {
	let listener = TcpListener::bind((host, port)).unwrap();
	thread::spawn(move || {
		for stream in listener.incoming() {
			match stream {
				Ok(s) => {
					let peer_addr = s.peer_addr().expect("Could not retrieve peer address");
					let ip = peer_addr.ip();

					match ip {
						IpAddr::V4(ipv4_addr) => {
							let mut bytes = vec![];
							bytes.extend(ipv4_addr.octets().iter());
							bytes.extend(util::u32_to_bytes(peer_addr.port() as u32));
							let peer = Peer::from_bytes(bytes.as_slice());
							let mut c = Connection::new(client_mutex.clone(), peer, s, torrent_mutex.clone());
							let _ = c.send_message(Message::Choke);

						}
						_ => println!("Unsupported IP format") 
					}
				}	
				Err(e) => println!("{:?}", e)
			}
		}
	});
}


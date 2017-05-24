use peer::Peer;
use torrent::Torrent;
use std::net::TcpStream;
use util::Error;

#[derive(Debug)]
pub struct Connection {
	stream: TcpStream,
	peer: Peer,
	torrent: Torrent,
}

impl Connection {
	#[allow(dead_code)]
	pub fn new(peer: Peer, stream: TcpStream, t: Torrent) -> Result<(), Error> {

        let _ = Connection {
			stream: stream,
			peer: peer,
			torrent: t,
		};

        // let _ = connection.initiate_handshake();

        Ok(())		
	}

	#[allow(dead_code)]
	pub fn connect(peer: Peer, t: Torrent) {
		println!("Connecting to {}:{}...", peer.ip, peer.port);
		let stream = TcpStream::connect((peer.ip, peer.port))
			.expect("Couldn't connect to the peer...");

		println!("Connected...");
		let _ = Connection::new(peer, stream, t);
	}

	#[allow(dead_code)]
	pub fn initiate_handshake(&mut self) {
        // Create Message object
	}
}
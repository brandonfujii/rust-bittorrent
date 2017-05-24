use peer::Peer;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Connection {
	stream: TcpStream,
	peer: Peer,
}

impl Connection {
	pub fn new(peer: Peer, stream: TcpStream) -> Self {
		Connection {
			stream: stream,
			peer: peer,
		}
	}

	pub fn connect(peer: Peer) {
		println!("Connecting to {}:{}...", peer.ip, peer.port);
		let stream = TcpStream::connect(&format!("{}:{}", peer.ip, peer.port))
			.expect("Couldn't connect to the peer...");
		let _ = Connection::new(peer, stream);
	}
}
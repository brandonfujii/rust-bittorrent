use peer::Peer;
use torrent::Torrent;
use std::net::TcpStream;
use std::io::{Read, Write, Error, ErrorKind};
use util::{bytes_to_u32};
use message::Message;

const PROTOCOL: &'static str = "BitTorrent protocol";
static BLOCK_SIZE: u32 = 16384; // 2^14

#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
    peer: Peer,
    torrent: Torrent,
    have: Vec<bool>,
    client_choked: bool,
    client_interested: bool,
    peer_choked: bool,
    peer_interested: bool,
}

impl Connection {
    pub fn new(peer: Peer, stream: TcpStream, t: Torrent) -> Result<(), Error> {
        let num_pieces = t.pieces.len();
        let mut c = Connection {
            stream: stream,
            peer: peer,
            torrent: t,
            client_choked: false,
            client_interested: false,
            peer_choked: false,
            peer_interested: false,
            have: vec![false; num_pieces as usize],
        };

        let _ = c.initiate_handshake();
        println!("Sent handshake");
        let _ = c.receive_handshake();
        println!("Received handshake");

        let mut done = false;
        while !done {
            let message = try!(c.receive_message());
            println!("Received: {:?}", message);
            done = try!(c.process(message));
        }

        Ok(())
    }

    pub fn connect(peer: Peer, t: Torrent) {
        println!("Connecting to {}:{}...", peer.ip, peer.port);
        let stream = TcpStream::connect((peer.ip, peer.port)).unwrap();
        let _ = Connection::new(peer, stream, t);
    }

    fn initiate_handshake(&mut self) -> Result<(), Error> {
        let mut message = vec![];
        message.push(PROTOCOL.len() as u8);
        message.extend(PROTOCOL.bytes());
        message.extend(vec![0;8].into_iter());
        message.extend(self.torrent.metainfo.info_hash.iter().cloned());
        message.extend(self.torrent.peer_id.bytes());
        try!(self.stream.write_all(&message));
        Ok(())
    }

    fn receive_handshake(&mut self) -> Result<(), Error> {
        let pstrlen = try!(self.read_n(1));
        let _pstr = try!(self.read_n(pstrlen[0] as u32));
        let _reserved = try!(self.read_n(8));
        let _info_hash = try!(self.read_n(20));
        let _peer_id = try!(self.read_n(20));
        Ok(())
    }

    fn receive_message(&mut self) -> Result<Message, Error> {
        let length = bytes_to_u32(&try!(self.read_n(4)));
        if length > 0 {
            let message = try!(self.read_n(length));
            Ok(Message::new(&message[0], &message[1..]))
        } else {
            Ok(Message::KeepAlive)
        }
    }

    fn read_n(&mut self, bytes_to_read: u32) -> Result<Vec<u8>, Error> {
        let mut buf = vec![];
        let bytes_read = (&mut self.stream).take(bytes_to_read as u64).read_to_end(&mut buf);
        match bytes_read {
            Ok(n) => {
                if (n as u32) == bytes_to_read {
                    Ok(buf)
                } else {
                    Err(Error::new(ErrorKind::Other, "Not enough bytes"))
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    fn process(&mut self, message: Message) -> Result<bool, Error>{
        match message {
            Message::KeepAlive => {},
            Message::Bitfield(bytes) => {
                for i in 0..self.have.len() {
                    let bytes_index = i / 8;
                    let index_into_byte = i % 8;
                    let byte = bytes[bytes_index];
                    let value = (byte & (1 << (7 - index_into_byte))) != 0;
                    self.have[i] = value;
                };
                try!(self.send_interested());
            },
            Message::Have(have_index) => {
                self.have[have_index as usize] = true;
                try!(self.send_interested());
            },
            Message::Unchoke => {
                if self.client_choked {
                    self.client_choked = false;
                }
                try!(self.request_next_block());
            }
            Message::Piece(piece_index, offset, data) => {
                let block_index = offset / BLOCK_SIZE;
                let is_complete = try!(self.torrent.store(piece_index, block_index, data));
                if is_complete {
                    return Ok(true)
                } else {
                    try!(self.request_next_block());
                }
            }
            _ => panic!("Need to process message: {:?}", message)
        };
        Ok(false)
    }

    fn send_interested(&mut self) -> Result<(), Error> {
        if self.client_interested == false {
            self.client_interested = true;
            try!(self.send_message(Message::Interested));
        }
        Ok(())
    }

    fn request_next_block(&mut self) -> Result<(), Error> {
        match self.torrent.next_block_to_request(&self.have) {
            Some((piece_index, block_index, block_length)) => {
                let offset = block_index * BLOCK_SIZE;
                self.send_message(Message::Request(piece_index, offset, block_length))
            },
            None => {
                println!("We've downloaded all the pieces we can from this peer.");
                Ok(())
            }
        }
    }

    fn send_message(&mut self, message: Message) -> Result<(), Error> {
        println!("Sending: {:?}", message);
        try!(self.stream.write_all(&message.serialize()));
        Ok(())
    }
}

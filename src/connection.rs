use peer::Peer;
use torrent::Torrent;
use std::net::{TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
use std::io::{Read, Write, Error, ErrorKind};
use util::{bytes_to_u32};
use message::Message;
use ipc::IpcMessage;
use std::sync::mpsc::{channel, Receiver};

const PROTOCOL: &'static str = "BitTorrent protocol";
const BLOCK_SIZE: u32 = 16384; // 2^14

#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
    client: Arc<Mutex<Peer>>,
    peer: Peer,
    torrent: Arc<Mutex<Torrent>>,
    channel: Receiver<IpcMessage>,
}

impl Connection {
    pub fn new(client_mutex: Arc<Mutex<Peer>>, mut peer: Peer, stream: TcpStream, torrent_mutex: Arc<Mutex<Torrent>>) -> Self {
        let num_pieces = {
            let t = torrent_mutex.lock().unwrap();
            t.pieces.len()
        };

        {
            let mut client = client_mutex.lock().unwrap();
            client.register(num_pieces);
            peer.register(num_pieces);
        }

        let (tx, rx) = channel::<IpcMessage>();
        {
            let mut torrent = torrent_mutex.lock().unwrap();
            torrent.register_peer(tx);
        }

        Connection {
            stream: stream,
            client: client_mutex,
            peer: peer,
            torrent: torrent_mutex,
            channel: rx
        }
    }

    pub fn connect(client_mutex: Arc<Mutex<Peer>>, peer: Peer, torrent_mutex: Arc<Mutex<Torrent>>) {
        println!("Connecting to {}:{}...", peer.ip, peer.port);
        let addr = SocketAddr::new(peer.ip, peer.port);
        match TcpStream::connect(&addr) {
            Ok(stream) => {
                println!("Connected successfully to {}:{}", peer.ip, peer.port);
                let mut c = Connection::new(client_mutex, peer, stream, torrent_mutex);
                let _ = c.initiate_handshake();
                println!("Sent handshake");
                let _ = c.receive_handshake();
                println!("Received handshake");

                let mut done = false;
                while !done {
                    let _ = c.check_messages();
                    let message = c.receive_message().unwrap();
                    println!("Received: {:?}", message);
                    done = c.handle_message(message).unwrap();
                }
            }
            _ => println!("Failed to connect")
        }
    }

    fn initiate_handshake(&mut self) -> Result<(), Error> {
        let mut message = vec![];

        {
            let t = self.torrent.lock().unwrap();

            message.push(PROTOCOL.len() as u8);
            message.extend(PROTOCOL.bytes());
            message.extend(vec![0;8].into_iter());
            message.extend(t.metainfo.info_hash.iter().cloned());
            message.extend(t.peer_id.bytes());
            try!(self.stream.write_all(&message));
            Ok(())
        }
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
        let stream = &mut self.stream;
        let mut take = stream.take(bytes_to_read as u64);
        let bytes_read = take.read_to_end(&mut buf);
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

    fn handle_message(&mut self, message: Message) -> Result<bool, Error>{
        match message {
            Message::KeepAlive => {},
            Message::Bitfield(bytes) => {
                {
                    let client = self.client.lock().unwrap().clone();
                    let num_pieces = client.have.unwrap().len();
                    let mut peer_have = self.peer.have.take().unwrap();
                    for i in 0..num_pieces {
                        let bytes_index = i / 8;
                        let index_into_byte = i % 8;
                        let byte = bytes[bytes_index];
                        let value = (byte & (1 << (7 - index_into_byte))) != 0;
                        peer_have[i] = value;
                    };
                    self.peer.have = Some(peer_have);
                }
                try!(self.send_interested());
            },
            Message::Have(have_index) => {
                {
                    let client = self.client.lock().unwrap().clone();
                    client.have.unwrap()[have_index as usize] = true;
                }
                try!(self.send_interested());
            },
            Message::Unchoke => {
                {
                    let mut client = self.client.lock().unwrap();
                    client.choked = Some(false);
                }
                try!(self.request_next_block());
            },
            Message::Piece(piece_index, offset, data) => {
                let is_complete = {
                    let mut t = self.torrent.lock().unwrap();
                    let block_index = offset / BLOCK_SIZE;
                    try!(t.store(piece_index, block_index, data))
                };

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
        let flag;
        {
            let client = self.client.lock().unwrap();
            flag = client.interested.unwrap();
        }
        if !flag {
            {
                let mut client = self.client.lock().unwrap();
                client.interested = Some(true);
            }
            try!(self.send_message(Message::Interested));
        }
        Ok(())
    }

    fn request_next_block(&mut self) -> Result<(), Error> {
        let next_block = {
            let t = self.torrent.lock().unwrap();
            t.next_block_to_request(&self.peer.clone().have.unwrap())
        };

        match next_block {
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

    fn check_messages(&mut self) -> Result<(), Error> {
        loop {
            match self.channel.try_recv() {
                Ok(message) => return self.handle_ipc(message),
                Err(_) => break
            }
        }
        Ok(())
    }

    fn handle_ipc(&mut self, message: IpcMessage) -> Result<(), Error> {
        match message {
            IpcMessage::CancelRequest(piece_index, block_index) => {
                return Ok(())
            }
        }
    }
}

#[cfg(test)]
mod connection_tests {
    #[test]
    fn create_connection_test() {
        use metainfo;
        use tracker;
        use peer::Peer;
        use torrent::Torrent;
        use std::net::SocketAddr;
        use std::sync::{Arc, Mutex};
        use util::create_peer_id;

        let f: String = String::from("data/flagfromserver.torrent");
        let m = metainfo::from_file(&f).unwrap();
        let peer_id: String = create_peer_id();

        let peers = tracker::retrieve_peers(&m, &peer_id, "8080").unwrap();
        let ref peer = peers[0];
        let torrent = Torrent::new(peer_id, m);
        let _ = Arc::new(Mutex::new(torrent));
        let _ = Arc::new(Mutex::new(Peer::from_bytes(&[127, 0, 0, 1, 31, 144])));
        let _ = SocketAddr::new(peer.ip, peer.port);
    }
}

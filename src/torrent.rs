use metainfo::MetaInfo;
use ipc::IpcMessage;
use piece::Piece;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::Error;
use std::sync::mpsc::{Sender};

#[derive(Debug)]
pub struct Torrent {
    pub metainfo: MetaInfo,
    pub peer_id: String,
    file: File,
    pub pieces: Vec<Piece>,
    peer_channels: Vec<Sender<IpcMessage>>,
}

/// Represents the entire torrent, including metainfo derived from the `.torrent` file as well as
/// the client's id, the file to be downloaded and the pieces of the file
impl Torrent {
    pub fn new(peer_id: String, metainfo: MetaInfo) -> Self {
        let filename = metainfo.clone().info.name;
        let piece_length = metainfo.info.piece_length;
        let num_pieces = metainfo.info.num_pieces;
        let path = Path::new(&filename);

        if !path.exists() {
            let f = File::create(path).unwrap();
            let _ = f.set_len(metainfo.info.length);
        }

        let file = OpenOptions::new().write(true).open(path).unwrap();
        let mut pieces: Vec<Piece> = vec![];
        let n = metainfo.info.pieces.len();

        for i in 0..n {
            let length = {
                if i == n - 1 {
                    num_pieces % piece_length
                } else {
                    piece_length
                }
            };

            let piece = Piece::new(length, i as u32, piece_length, metainfo.info.pieces[i as usize].clone());
            pieces.push(piece);
        }

        Torrent {
            metainfo: metainfo,
            peer_id: peer_id,
            file: file,
            pieces: pieces,
            peer_channels: vec![]
        }
    }

    /// Given a piece index, block index, a vector of bytes for a block, we store
    /// the new block at its position within the piece and return whether or not 
    /// the piece is complete to determine if we should keep requesting blocks
    pub fn store(&mut self, piece_index: u32, block_index: u32, data: Vec<u8>) -> Result<bool, Error> {
        {
            let piece = &mut self.pieces[piece_index as usize];
            try!(piece.store(&mut self.file, block_index, data));
        }

        for channel in self.peer_channels.iter() {
            let _ = channel.send(IpcMessage::CancelRequest(piece_index, block_index));
        }

        Ok(self.is_complete())
    }


    /// Loops through pieces and checks if peer has requested piece 
    /// If so, it returns the next block's information in a triple of
    /// the piece length, the block index, and the block length
    pub fn next_block_to_request(&self, peer_has_pieces: &[bool]) -> Option<(u32, u32, u32)> {
        for piece in self.pieces.iter() {
            if peer_has_pieces[piece.index as usize] {
                match piece.next_block_to_request() {
                    Some(block) => {
                        return Some((piece.index, block.index, block.length))
                    },
                    None => {}
                }
            }
        }
        None
    }

    /// Returns a boolean that represents whether all the pieces for the 
    /// torrent has been retrieved 
    fn is_complete(&self) -> bool {
        for piece in self.pieces.iter() {
            if !piece.is_complete {
                return false
            }
        }
        println!("Torrent is complete");
        true
    }

    pub fn register_peer(&mut self, channel: Sender<IpcMessage>) {
        self.peer_channels.push(channel);
    }
}

impl PartialEq for Torrent {
    fn eq(&self, other: &Torrent) -> bool {
        self.metainfo == other.metainfo &&
        self.peer_id == other.peer_id &&
        self.pieces == other.pieces
    }

    fn ne(&self, other: &Torrent) -> bool {
        !(self == other)
    }
}

#[cfg(test)]
mod torrent_tests {
    use super::Torrent;
    use piece::Piece;
    use block::Block;
    use metainfo::{MetaInfo, Info};
    use std::fs::File;
    use std::path::Path;
    use std::fs;
    use util::create_peer_id;

    #[test]
    fn make_torrent_test() {
        let filename = String::from("info.txt");
        let i = Info {
            piece_length: 12,
            pieces: vec![vec![1, 2, 3]],
            num_pieces: 3,
            name: filename.clone(),
            length: 12
        };

        let m = MetaInfo {
            announce: String::from("https://google.com/announce"),
            created_by: String::from("tov"),
            info: i,
            info_hash: vec![2, 3, 4]
        };

        let path = Path::new(&filename);
        let _ = File::create(path);
        let f = File::open(path).unwrap();
        let peer_id = create_peer_id();

        let t = Torrent::new(peer_id.clone(), m.clone());
        assert_eq!(t, Torrent {
            metainfo: m,
            peer_id: peer_id,
            file: f,
            pieces: vec![Piece {
                length: 3,
                index: 0,
                piece_length: 12,
                blocks: vec![Block::new(0, 3)],
                hash: vec![1, 2, 3],
                is_complete: false,
            }],
            peer_channels: vec![]
        });

        let _ = fs::remove_file(path);
    }
}

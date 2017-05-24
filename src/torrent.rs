use metainfo::MetaInfo;
use piece::Piece;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub struct Torrent {
    pub metainfo: MetaInfo,
    pub peer_id: String,
    file: File,
    pieces: Vec<Piece>,
}

/// Represents the entire torrent, including metainfo derived from the `.torrent` file as well as
/// the client's id, the file to be downloaded and the pieces of the file
impl Torrent {
    pub fn new(metainfo: MetaInfo) -> Self {
        let filename = metainfo.clone().info.name;
        let piece_length = metainfo.info.piece_length;
        let num_pieces = metainfo.info.num_pieces;
        let path = Path::new(&filename);

        if !path.exists() {
            let _ = File::create(path);
        }

        let file = File::open(path).unwrap();
        let mut pieces: Vec<Piece> = vec![];
        let n = ((num_pieces as f64)/20.).ceil() as usize;

        for i in 0..n {
            let offset = i as u32 * piece_length;
            let length;

            if i == n - 1 {
                length = num_pieces % piece_length;
            } else {
                length = piece_length;
            }

            let piece = Piece::new(length, offset, metainfo.info.pieces[i as usize].clone());
            pieces.push(piece);
        }

        Torrent {
            metainfo: metainfo,
            peer_id: String::from("tovatovatovatovatova"),
            file: file,
            pieces: pieces,
        }
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

        let t = Torrent::new(m.clone());
        assert_eq!(t, Torrent {
            metainfo: m,
            peer_id: String::from("tovatovatovatovatova"),
            file: f,
            pieces: vec![Piece {
                length: 3,
                offset: 0,
                blocks: vec![Block::new(0, 3)],
                hash: vec![1, 2, 3]
            }]
        });

        let _ = fs::remove_file(path);
    }
}

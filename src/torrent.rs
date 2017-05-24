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
        let file_length = metainfo.info.length;
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
            peer_id: "tovatovatovatovatova".to_string(),
            file: file,
            pieces: pieces,
        }
    }
}

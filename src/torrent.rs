use metainfo::MetaInfo;
use piece::Piece;
use std::fs::File;
use std::path::Path;

pub struct Torrent {
	pub metainfo: MetaInfo,
	pub peer_id: String,
	file: File, 
	pieces: Vec<Piece>,
}

impl Torrent {
	pub fn new(metainfo: MetaInfo) -> Self {

		let filename = metainfo.clone().info.name;
		let file_length = metainfo.info.length;
		let piece_length = metainfo.info.piece_length;
		let num_pieces = metainfo.info.num_pieces;
		let path = Path::new(&filename);
		let file; 

		if path.exists() {
			file = File::open(path).unwrap();
		} else {
			panic!("File does not exist");
		}

		let mut pieces: Vec<Piece> = vec![];

		for i in 0..num_pieces {
			let offset = i as u64 * piece_length as u64;
			let length;

			if i < num_pieces - 1 {
                length = piece_length
            } else {
                length = (file_length - offset) as u32
            }

            let piece = Piece::new(length, offset, metainfo.clone().info_hash);
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
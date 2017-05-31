use block::Block;
use std::fs::File;
use std::io;
use std::io::{Error, Write, Seek};
use hash;

static BLOCK_SIZE: u32 = 16384; // 2^14

#[derive(Debug, PartialEq)]
pub struct Piece {
    pub length: u32,
    pub piece_length: u32,
    pub index: u32,
    pub blocks: Vec<Block>,
    pub hash: Vec<u8>,
    pub is_complete: bool,
}

/// Represents a portion of the data to be downloaded which is described in the metainfo file and
/// can be verified by a SHA1 hash. A piece is made up of blocks
impl Piece {
    pub fn new(length: u32, index: u32, piece_length: u32, hash: Vec<u8>) -> Self {
        let mut blocks: Vec<Block> = vec![];
        let num_blocks = ((length as f64) / (BLOCK_SIZE as f64)).ceil() as u32;

        for i in 0..num_blocks {
            let block_length;

            if i < num_blocks - 1 {
                block_length = BLOCK_SIZE;
            } else {
                block_length = length - (BLOCK_SIZE * (num_blocks - 1));
            }

            let block = Block::new(i, block_length);
            blocks.push(block);
        }

        Piece {
            length: length,
            piece_length: piece_length,
            index: index,
            hash: hash,
            blocks: blocks,
            is_complete: false
        }
    }

    pub fn store(&mut self, file: &mut File, block_index: u32, data: Vec<u8>) -> Result<(), Error> {
        {
            let block = &mut self.blocks[block_index as usize];
            block.data = Some(data);
        }

        if self.have_all_blocks() {
            // concatenate data from blocks together
            let mut data = vec![];
            for block in self.blocks.iter() {
                data.extend(block.data.clone().unwrap());
            }

            // validate that piece data matches SHA1 hash
            if self.hash == hash::sha(&data) {
                println!("Piece {} is complete and correct, writing to the file.", self.index);
                let offset = self.index as u64 * self.piece_length as u64;
                try!(file.seek(io::SeekFrom::Start(offset)));
                try!(file.write_all(&data));
                self.clear_block_data();
                self.is_complete = true;
            } else {
                println!("Piece is corrupt, deleting downloaded piece data!");
                println!("Expected {:?}", self.hash);
                println!("Got {:?}", hash::sha(&data));
                self.clear_block_data();
            }
        }
        Ok(())
    }

    pub fn next_block_to_request(&self) -> Option<&Block> {
        if self.is_complete {
            return None
        }

        for block in self.blocks.iter() {
            if block.data.is_none() {
                return Some(block)
            }
        }

        None
    }

    pub fn have_all_blocks(&self) -> bool {
        for block in self.blocks.iter() {
            if block.data.is_none() {
                return false
            }
        }
        true
    }

    pub fn clear_block_data(&mut self) {
        for block in self.blocks.iter_mut() {
            block.data = None;
        }
    }
}

#[cfg(test)]
mod piece_tests {
    use super::Piece;
    use block::Block;

    #[test]
    fn make_piece_test() {
        let p = Piece::new(256, 4, 4, vec![1, 2, 3]);
        assert_eq!(p, Piece {
            length: 256,
            piece_length: 4,
            index: 4,
            blocks: vec![Block::new(0, 256)],
            hash: vec![1, 2, 3],
            is_complete: false,
        });
    }

    #[test]
    fn next_block_test() {
        let mut p = Piece::new(256, 4, 4, vec![1, 2, 3]);
        assert_eq!(p.next_block_to_request(), Some(&Block {
            index: 0,
            length: 256,
            data: None
        }));

        p.is_complete = true;
        assert_eq!(p.next_block_to_request(), None);
    }

    #[test]
    fn have_all_blocks_test() {
        let mut p = Piece::new(256, 4, 4, vec![1, 2, 3]);
        assert_eq!(p.have_all_blocks(), false);

        p = Piece {
            length: 256,
            piece_length: 4,
            index: 4,
            blocks: vec![Block {
                index: 12,
                length: 12,
                data: Some(vec![])
            }],
            hash: vec![1, 2, 3],
            is_complete: false,
        };
        assert_eq!(p.have_all_blocks(), true);
    }

    #[test]
    fn clear_block_data_test() {
        let mut p = Piece {
            length: 256,
            piece_length: 4,
            index: 4,
            blocks: vec![Block {
                index: 12,
                length: 12,
                data: Some(vec![])
            }],
            hash: vec![1, 2, 3],
            is_complete: false,
        };

        p.clear_block_data();
        assert_eq!(p.blocks, vec![Block {
            index: 12,
            length: 12,
            data: None
        }]);
    }
}

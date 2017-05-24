use block::Block;

static BLOCK_SIZE: u32 = 16384; // 2^14

#[derive(Debug, PartialEq)]
pub struct Piece {
    pub length: u32,
    pub offset: u32,
    pub blocks: Vec<Block>,
    pub hash: Vec<u8>,
}

/// Represents a portion of the data to be downloaded which is described in the metainfo file and
/// can be verified by a SHA1 hash. A piece is made up of blocks
impl Piece {
    pub fn new(length: u32, offset: u32, hash: Vec<u8>) -> Self {

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
            offset: offset,
            hash: hash,
            blocks: blocks,
        }
    }
}

#[cfg(test)]
mod piece_tests {
    use super::Piece;
    use block::Block;

    #[test]
    fn make_piece_test() {
        let p = Piece::new(256, 4, vec![1, 2, 3]);
        assert_eq!(p, Piece {
            length: 256,
            offset: 4,
            blocks: vec![Block::new(0, 256)],
            hash: vec![1, 2, 3]
        });
    }
}

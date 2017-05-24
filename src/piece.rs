use block::Block;


static BLOCK_SIZE: u32 = 16384; // 2^14

pub struct Piece {
	length: u32,
	offset: u64,
	block_size: u32,
	blocks: Vec<Block>,
	hash: Vec<u8>,
}

impl Piece {
	pub fn new(length: u32, offset: u64, hash: Vec<u8>) -> Self {

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
			block_size: BLOCK_SIZE,
			blocks: blocks,
		}
	}
}

pub struct Block {
	index: u32,
	length: u32,
}

impl Block {
	pub fn new(index: u32, length: u32) -> Self {
		Block {
			index: index,
			length: length,
		}
	}
}
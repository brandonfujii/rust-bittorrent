#[derive(Debug, PartialEq)]
pub struct Block {
    pub index: u32,
    pub length: u32,
}

/// Represents a portion of data that a client may request from a peer.
impl Block {
    pub fn new(index: u32, length: u32) -> Self {
        Block {
            index: index,
            length: length,
        }
    }
}

#[cfg(test)]
mod block_tests {
    use super::Block;

    #[test]
    fn make_block_test() {
        let block: Block = Block::new(0, 12);
        assert_eq!(block, Block{index: 0, length: 12});
    }
}

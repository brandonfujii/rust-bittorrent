#[derive(Debug)]
pub struct Block {
    index: u32,
    length: u32,
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

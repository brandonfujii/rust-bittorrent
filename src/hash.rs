extern crate sha1;

use self::sha1::Sha1;

pub fn sha(b: &[u8]) -> Vec<u8> {
    let mut m = Sha1::new();
    m.update(b);
    m.digest().bytes().to_vec()
}

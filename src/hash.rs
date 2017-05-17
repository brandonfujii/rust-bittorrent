extern crate sha1;

use self::sha1::Sha1;

pub fn sha(b: &[u8]) -> String {
    let mut m = Sha1::new();
    m.update(b);
    m.digest().to_string()
}

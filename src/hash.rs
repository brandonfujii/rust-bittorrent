extern crate sha1;

use self::sha1::Sha1;

pub fn sha(b: &[u8]) -> Vec<u8> {
    let mut m = Sha1::new();
    m.update(b);
    m.digest().bytes().to_vec()
}

#[cfg(test)]
mod sha_tests {
    use super::sha;

    #[test]
    fn make_sha_test() {
        let hash: Vec<u8> = sha(&[0]);
        assert_eq!(
            hash,
            vec![91, 169, 60, 157, 176, 207, 249, 63, 82, 181, 33, 215, 66, 14, 67, 246, 237, 162, 120, 79]
        );
    }
}

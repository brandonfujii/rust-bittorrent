extern crate bencode;
use std::env;

mod metainfo;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("{:?}", metainfo::from_file(filename));
}

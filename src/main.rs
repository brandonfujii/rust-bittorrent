extern crate bencode;
extern crate hyper;
extern crate regex;

use std::env;

mod metainfo;
mod tracker;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("{:?}", metainfo::from_file(filename));
}

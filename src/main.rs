extern crate bencode;
extern crate hyper;
extern crate regex;
extern crate urlencoding;
extern crate url;

use std::env;

mod metainfo;
mod tracker;
mod hash;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let m = metainfo::from_file(filename).unwrap();
    println!("{:?}", m);

    let res = tracker::retrieve_peers(&m, "tovatovatovatovatova", "8080");
}

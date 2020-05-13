extern crate walkdir;
extern crate crypto;
extern crate rustc_serialize;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::hex::ToHex;

use std::{env};

fn main() {
    let mut paths: Vec<String> = vec![];
    for arg in env::args().skip(1) {
        for entry in walkdir::WalkDir::new(arg)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            paths.push(String::from(entry.path().to_string_lossy()))
        }
    }
    // let mut hashcat = String::from("");
    let mut hashcat_vec: Vec<u8> = Vec::new();
    for path in paths {
        let mut f = File::open(path).unwrap();
        let mut buffer = Vec::new();
        f.read(&mut buffer).unwrap();

        let mut digest = Md5::new();
        digest.input(&buffer);
        let mut output = [0; 16]; // md5 is 16 bytes long
        digest.result(&mut output);

        hashcat_vec.append(&mut output.to_vec());
        // hashcat += &output.to_hex();
    }
    let mut digest = Md5::new();
    // digest.input(&hashcat.as_bytes());
    digest.input(&hashcat_vec);
    let mut output = [0; 16];
    digest.result(&mut output);
    print!("{}\n", output.to_hex());
}
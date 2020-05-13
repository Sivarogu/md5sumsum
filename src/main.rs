extern crate walkdir;
extern crate crypto;
extern crate rustc_serialize;
extern crate rayon;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::hex::ToHex;
use rayon::prelude::*;

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

    // Use rayon to hash in parallel
    paths.par_iter().for_each(|path| {
        // Open and read the file
        let mut f = File::open(path).unwrap();
        let mut buffer = Vec::new();
        f.read(&mut buffer).unwrap();

        // Hash using MD5
        let mut digest = Md5::new();
        digest.input(&buffer);
        let mut output = [0; 16]; // md5 is 16 bytes long
        digest.result(&mut output);

        // Format result to keep Vec<String> type
        output.to_hex();
    });

    // Hash joined hashes and print result
    let mut digest = Md5::new();
    digest.input(&paths.join("").as_bytes());
    let mut output = [0; 16];
    digest.result(&mut output);
    print!("{}\n", output.to_hex());
}
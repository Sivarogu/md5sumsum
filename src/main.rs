extern crate walkdir;
extern crate crypto;
extern crate rustc_serialize;
extern crate rayon;

use std::{env, fs};
use crypto::md5::Md5;
use crypto::digest::Digest;
use rustc_serialize::hex::ToHex;
use rayon::prelude::*;

fn main() {
    // Get file paths
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

    // Get file hashes using parallel iterations
    let hashes: Vec<_> = paths.par_iter().map(|path| {
        let mut digest = Md5::new();
        digest.input(&fs::read(path).unwrap());
        let mut output = [0; 16]; // md5 is 16 bytes long
        digest.result(&mut output);
        output
    }).collect();

    // Hash concatenated hashes and print result
    let mut digest = Md5::new();
    let concat_hashes = hashes.concat();
    digest.input(&concat_hashes);
    let mut output = [0; 16];
    digest.result(&mut output);
    print!("{}\n", output.to_hex());
}
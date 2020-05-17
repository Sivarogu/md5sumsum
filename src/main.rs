extern crate walkdir;
extern crate crypto;
extern crate rustc_serialize;
extern crate rayon;

use walkdir::DirEntry;
use std::{env, fs};
use crypto::md5::Md5;
use crypto::digest::Digest;
use rustc_serialize::hex::ToHex;
use rayon::prelude::*;

fn main() {
    // The final md5 hashing instance is created
    let mut digest = Md5::new();
    for arg in env::args().skip(1) {
        // The concatenated lists of file hashes is given as input
        digest.input(
            // This part is unchanged from original commit
            // We iterate on arg directory and filter out non-file items
            &walkdir::WalkDir::new(arg)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| !e.file_type().is_dir())
                .collect::<Vec<DirEntry>>()
                // Then threads are used to iterate over files and hash them
                .par_iter()
                .map(|entry| {
                    let mut digest = Md5::new();
                    digest.input(&fs::read(String::from(entry.path().to_string_lossy())).unwrap());
                    let mut output = [0; 16]; // md5 is 16 bytes long
                    digest.result(&mut output);
                    output
                }).collect::<Vec<[u8;16]>>()
                // Finally all hashes are concatenated
                .concat()
        );
    }
    let mut output = [0; 16];
    digest.result(&mut output);
    print!("{}\n", output.to_hex());
}
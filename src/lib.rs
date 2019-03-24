use std::io::prelude::*;

extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2;

struct Hasher {
    name: String,
    hasher: Box<Digest>,
}

pub struct Options<T1: Read, T2: Write> {
    pub binary: bool,
    pub input: T1,
    pub output: T2,
}

fn hash<T1: Read, T2: Write> (options: &mut Options<T1, T2>, hashes: &mut Vec<Hasher>) -> std::io::Result<()> {
    const BLOCK_SIZE: usize = 1024;
    let mut buf: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];
    let input = &mut options.input;

    loop {
        match input.read(&mut buf) {
            Ok(0) => break,
            Ok(v) => {
                for hash in hashes.iter_mut() {
                    hash.hasher.input(&buf[0..v]);
                }
            },
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn print<T1: Read, T2: Write>(mut options: Options<T1, T2>, mut hashes: Vec<Hasher>) -> std::io::Result<()> {
    match options.binary {
        false => {
            let mut result = String::new();
            for hash in hashes.iter_mut() {
                result.push_str(&format!("{}: {}\n", hash.name, hash.hasher.result_str()));
            }
            options.output.write(result.as_bytes())?;
        },
        true => {
            let mut result = Vec::new();
            for hash in hashes.iter_mut() {
                let size = hash.hasher.output_bytes();
                let mut ary: [u8; 1000] = [0; 1000];
                hash.hasher.result(&mut ary[0..size]);
                result.append(&mut ary[0..size].to_vec());
            }
            options.output.write(&result)?;
        },
    }
    Ok(())
}

pub fn multihash<T1: Read, T2: Write>(mut options: Options<T1, T2>) -> std::io::Result<()> {
    let mut hashes = vec![
        Hasher {
            name: String::from("Sha256"),
            hasher: Box::new(sha2::Sha256::new()),
        },
        Hasher {
            name: String::from("Sha512"),
            hasher: Box::new(sha2::Sha512::new()),
        },
    ];
    hash(&mut options, &mut hashes)?;
    print(options, hashes)?;
    Ok(())
}

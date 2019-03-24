use std::fs::File;
use std::io::Read;
use std::io::BufReader;

extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

fn main() -> std::io::Result<()> {
    let mut buf_reader = BufReader::new(std::io::stdin());
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut hasher = Sha256::new();
    hasher.input_str(&contents);
    let hex = hasher.result_str();
    println!("sha256: {}", hex);
    Ok(())
}

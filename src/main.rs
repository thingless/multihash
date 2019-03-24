use std::io::Read;
use std::io::BufReader;

extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

fn main() -> std::io::Result<()> {
    let mut buf_reader = BufReader::new(std::io::stdin());
    let mut bytes: [u8; 10] = [0; 10];
    let mut hasher = Sha256::new();
    loop {
        let res = buf_reader.read_exact(&mut bytes);
        hasher.input(&bytes);
        println!("err?: {}", res.is_err());
        if res.is_err() {break;}
    }

    let hex = hasher.result_str();
    println!("sha256: {}", hex);
    Ok(())
}

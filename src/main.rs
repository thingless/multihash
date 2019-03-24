use std::io::Read;

extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use self::crypto::sha2::Sha224;

struct Hasher {
    name: String,
    hasher: Box<Digest>,
}

fn main() -> std::io::Result<()> {
    let mut stdin = std::io::stdin();
    let mut hashes : [Hasher; 2] = [
        Hasher {
            name: String::from("Sha256"),
            hasher: Box::new(Sha256::new()),
        },
        Hasher {
            name: String::from("Sha224"),
            hasher: Box::new(Sha224::new()),
        },
    ];
    let mut buf: [u8; 1024] = [0; 1024];

    loop {
        match stdin.read(&mut buf) {
            Ok(0) => break,
            Err(e) => panic!(e),
            Ok(v) => {
                for hash in hashes.iter_mut() {
                    hash.hasher.input(&buf[0..v]);
                }
            },
        }
    }

    for hash in hashes.iter_mut() {
        println!("{}: {}", hash.name, hash.hasher.result_str());
    }
    Ok(())
}

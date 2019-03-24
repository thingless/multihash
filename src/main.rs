use std::io::Read;

extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use self::crypto::sha2::Sha224;

fn main() -> std::io::Result<()> {
    let mut stdin = std::io::stdin();
    let mut hashers : [Box<Digest>; 2] = [
        Box::new(Sha256::new()), 
        Box::new(Sha224::new())
    ];
    let mut buf: [u8; 1024] = [0; 1024];

    loop {
        match stdin.read(&mut buf) {
            Ok(0) => break,
            Err(e) => panic!(e),
            Ok(v) => {
                for hasher in hashers.iter_mut() {
                    hasher.input(&buf[0..v]);
                }
            },
        }
    }

    for hasher in hashers.iter_mut() {
        let hex = hasher.result_str();
        println!("some hash: {}", hex);
    }
    Ok(())
}

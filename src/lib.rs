use std::io::prelude::*;

extern crate crypto;
//extern crate byteorder;
extern crate integer_encoding;
use integer_encoding::VarInt;

use self::crypto::digest::Digest;
use self::crypto::sha2;
//use byteorder::{BigEndian, WriteBytesExt};

// This is the file format
// 37 magic bytes are used to identify a multihash file. These will never change.
// First, the byte 0xFF so nothing tries to read this. Then, the following ascii GUID (generated at random): "6245aa00-19d7-4690-badd-e9fb9f2829a5"
const MAGIC_FILE_BYTES: [u8; 37] = [ 255, 54, 50, 52, 53, 97, 97, 48, 48, 45, 49, 57, 100, 55, 45, 52, 54, 57, 48, 45, 98, 97, 100, 100, 45, 101, 57, 102, 98, 57, 102, 50, 56, 50, 57, 97, 53];
// Two bytes for the version (big-endian)
const VERSION_NUMBER: u16 = 1;
// Two bytes for the length of the multihash itself
const EXPECTED_BODY_LENGTH: u16 = 541;


struct Hasher {
    name: String,
    protocol_name: Vec<u8>,
    hasher: Box<Digest>,
}

pub enum InputFile {
    Filename(String),
    Stdin
}

pub enum OutputFile {
    Filename(String),
    Stdout
}

pub struct Options {
    pub binary: bool,
    pub input: InputFile,
    pub output: OutputFile,
}

fn hash<T: Read> (options: &mut Options, input: &mut T, hashes: &mut Vec<Hasher>) -> std::io::Result<()> {
    const BLOCK_SIZE: usize = 1024;
    let mut buf: [u8; BLOCK_SIZE] = [0; BLOCK_SIZE];

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

fn print<T: Write>(mut options: Options, output: &mut T, mut hashes: Vec<Hasher>) -> std::io::Result<()> {
    match options.binary {
        false => {
            let mut result = String::new();
            for hash in hashes.iter_mut() {
                result.push_str(&format!("{}: {}\n", hash.name, hash.hasher.result_str()));
            }
            output.write(result.as_bytes())?;
        },
        true => {
            let mut all = vec![];
            all.extend_from_slice(&MAGIC_FILE_BYTES);
            all.extend(VERSION_NUMBER.encode_var_vec());

            let mut body = Vec::new();
            for hash in hashes.iter_mut() {
                body.extend_from_slice(&hash.protocol_name);
                body.push(b':');

                let size = hash.hasher.output_bytes();
                let mut ary: [u8; 1000] = [0; 1000];
                hash.hasher.result(&mut ary[0..size]);
                body.extend(size.encode_var_vec());
                body.append(&mut ary[0..size].to_vec());
            }
            all.extend(body.len().encode_var_vec());
            all.append(&mut body);

            output.write(&all)?;
        },
    }
    Ok(())
}

fn open_input(input_option: &InputFile) -> std::io::Result<Box<Read>> {
    Ok(match input_option {
        InputFile::Stdin => Box::new(std::io::stdin()),
        InputFile::Filename(f) => Box::new(std::fs::File::open(f)?),
    })
}

fn open_output(output_option: &OutputFile) -> std::io::Result<Box<Write>> {
    Ok(match output_option {
        OutputFile::Stdout => Box::new(std::io::stdout()),
        OutputFile::Filename(f) => Box::new(std::fs::File::create(f)?),
    })
}

pub fn run(mut options: Options) -> std::io::Result<()> {
    let mut hashes = vec![
//4       sum32
//4       crc-32          #
//8       crc-64          #
//32      BLAKE2s         # librsync 1.0+ strong checksum
//64      BLAKE2b         # maybe IPFS uses this
//16      MD4             # librsync 0.9- strong checksum
//16      MD5             # md5sum
//20      ripemd160       # bitcoin uses it
//20      SHA-1           # shasum
        Hasher {
            name: String::from("Sha256"),
            protocol_name: Vec::from("sha256"),
            hasher: Box::new(sha2::Sha256::new()),
        },
        Hasher {
            name: String::from("Sha512"),
            protocol_name: Vec::from("sha512"),
            hasher: Box::new(sha2::Sha512::new()),
        },
//32      SHA-3-256       # no one uses SHA-3
//64      SHA-3-512       # no one uses SHA-3
//64      whirlpool       #
//4       XXH32           # 
//8       XXH64           # no one really uses it but it's fast to calculate
//4       <custom>        rsync's rolling checksum [https://github.com/librsync/librsync/blob/master/src/rollsum.h]
//20      <custom>        git's blob storage SHA
//20      <custom>        first chunk of torrent file (zero-padded if needed) - 256K
//20      <custom>        first chunk of torrent file (zero-padded if needed) - 512K
//20      <custom>        first chunk of torrent file (zero-padded if needed) - 1M
    ];
    let mut input = open_input(&options.input)?;
    let mut output = open_output(&options.output)?;
    hash(&mut options, &mut input, &mut hashes)?;
    print(options, &mut output, hashes)?;
    Ok(())
}

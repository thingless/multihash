use std::fs::File;
use std::io::Read;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    let mut buf_reader = BufReader::new(input);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    print!("{:}", contents);
    Ok(())
}

extern crate multihash;

fn main() -> std::io::Result<()> {
    let options = multihash::Options {
        binary: false,
        input: std::io::stdin(),
        output: std::io::stdout(),
    };
    multihash::multihash(options)?;
    Ok(())
}

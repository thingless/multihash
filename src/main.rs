extern crate multihash;

fn main() -> std::io::Result<()> {
    let options = multihash::Options {
        binary: false,
        input: multihash::InputFile::Stdin,
        output: multihash::OutputFile::Stdout,
    };
    multihash::run(options)?;
    Ok(())
}

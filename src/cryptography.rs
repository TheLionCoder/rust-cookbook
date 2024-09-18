use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Error, Read, Write};

pub fn calculate_the_sha256_digest_of_a_file() -> Result<(), Error> {
    let path: &str = "./assets/file.txt";
    write_a_file(path)?;
    let input = File::open(&path)?;
    let reader: BufReader<File> = BufReader::new(input);
    let digest: Digest = compute_sha256_digest(reader)?;

    print!(
        "SHA-256 digest of the file is: {}",
        HEXUPPER.encode(digest.as_ref())
    );
    Ok(())
}

fn compute_sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Error> {
    let mut context: Context = Context::new(&SHA256);
    let mut buffer: [u8; 1024] = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok(context.finish())
}

fn write_a_file(path: &str) -> Result<(), Error> {
    let mut output_file: File = File::create(&path)?;

    write!(output_file, "We will generate a digest of this text")
}

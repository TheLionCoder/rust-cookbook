use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, hmac, pbkdf2, rand};
use std::fs::File;
use std::io::{BufReader, Error, Read, Write};
use std::num::NonZeroU32;

pub fn calculate_the_sha256_digest_of_a_file() -> Result<(), Error> {
    let path: &str = "./assets/file.txt";
    write_a_file(path)?;
    let input = File::open(&path)?;
    let reader: BufReader<File> = BufReader::new(input);
    let digest: Digest = compute_sha256_digest(reader)?;

    print!(
        "SHA-256 digest of the file is: {} \n",
        HEXUPPER.encode(digest.as_ref())
    );
    Ok(())
}

pub fn sign_and_verify_with_hmac() -> Result<(), Unspecified> {
    let mut key_value: [u8; 48] = [0u8; 48];
    let rng: rand::SystemRandom = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    let key: hmac::Key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message: &str = "Legitimate and important message";
    let signature: hmac::Tag = hmac::sign(&key, message.as_bytes());

    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;

    Ok(())
}

pub fn hash_a_password() -> Result<(), Unspecified> {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let algorithm: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA512;
    let n_iter: NonZeroU32 = NonZeroU32::new(100_000).unwrap();
    let rng: rand::SystemRandom = rand::SystemRandom::new();

    let mut salt: [u8; CREDENTIAL_LEN] = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;

    let password: &str = "Guess Me if You Can";
    let mut pbkdf2_hash: [u8; CREDENTIAL_LEN] = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        algorithm,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

    let should_succeed: Result<(), Unspecified> =
        pbkdf2::verify(algorithm, n_iter, &salt, password.as_bytes(), &pbkdf2_hash);

    let wrong_password: &str = "Definitely not the correct password";
    let should_fail: Result<(), Unspecified> = pbkdf2::verify(
        algorithm,
        n_iter,
        &salt,
        wrong_password.as_bytes(),
        &pbkdf2_hash,
    );

    assert!(should_succeed.is_ok());
    assert!(!should_fail.is_ok());

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

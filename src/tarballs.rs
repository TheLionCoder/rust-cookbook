use std::fs::File;
use std::io::Error;
use std::path::PathBuf;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use tar::Archive;

pub fn decompress_tarball() -> Result<(), Error> {
    let path: String = String::from("./assets/archive.tar.gz");

    let tar_gz = File::open(path)?;
    let tar= GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())

}

pub fn compress_tarball() -> Result<(), Error> {
    let tar_gz = File::create("./assets/archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/logs", "/var/log")?;
    Ok(())
}

pub fn decompress_tarball_remove_prefix() -> Result<(), Error> {
    let file = File::open("./assets/archive.tar.gz")?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = "bundle/logs";
    
    println!("Extracted the following files");
    archive
        .entries()?
        .filter_map(|entry| entry.ok())
        .map(|mut entry| -> Result<PathBuf, Error> {
            let path = entry.path()?
                .strip_prefix(prefix)
                .map_err(|e | Error::new(std::io::ErrorKind::Other, e))?
                .to_owned();
            entry.unpack(&path)?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|p| println!("> {:?}", p.display()));
    
    Ok(())
}
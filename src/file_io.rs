use std::fs::File;
use std::io::{BufReader, BufRead, Error, Write, ErrorKind};
use std::path::Path;

use memmap::Mmap;
use same_file::Handle;

pub fn write_file() -> Result<(), Error> {
    let path: &str = "./assets/content.txt";

    let mut output: File = File::create(path)?;
    write!(output, "My hovercraft is full of eels!")?;

    Ok(())
}

pub fn read_file() -> Result<(), Error> {
    let path: &str = "./assets/lines.txt";

    let input: File = File::open(path)?;
    let buffered: BufReader<File> = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

pub fn avoid_concurrent_file_access() -> Result<(), Error>{
    let path_to_read: &Path = Path::new("./assets/file.txt");

    let stdout_handle: Handle = Handle::stdout()?;
    let handle: Handle = Handle::from_path(path_to_read)?;

    if stdout_handle == handle {
        return Err(Error::new(
            ErrorKind::Other,
            "You are reading and writing to the same file"
        ));
    } else {
        let file: File = File::open(&path_to_read)?;
        let file: BufReader<File> = BufReader::new(file);
        for (num, line) in file.lines().enumerate() {
            println!("{} : {}", num, line?.to_uppercase());
        }
    }
    Ok(())
}


pub fn random_access_file_with_mmap() -> Result<(), Error> {
    let file: File = File::open("./assets/content.txt")?;
    let map = unsafe {Mmap::map(&file)?};

    let random_indexes: [u8;8] = [0, 1, 2, 19, 22, 10, 11, 29];
    assert_eq!(&map[3..13], b"hovercraft");
    let random_bytes: Vec<u8> = random_indexes.iter()
        .map(|&idx| map[idx as usize])
        .collect();

    assert_eq!(&random_bytes[..], b"My loaf!");

    Ok(())

}
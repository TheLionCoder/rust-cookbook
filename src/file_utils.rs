use std::collections::HashMap;
use std::error::Error;
use std::path::{PathBuf};
use std::{env, fs};
use std::borrow::Cow;
use std::time::SystemTime;

use glob::{glob, glob_with, MatchOptions};
use walkdir::{DirEntry, WalkDir};


pub fn find_modified_file_names() -> Result<(), Box<dyn Error>> {
    let current_dir: PathBuf = env::current_dir()?;

    println!("Entries modified in the last 24 hours in {:?}", current_dir);

    for entry in fs::read_dir(current_dir)? {
        let entry: fs::DirEntry = entry?;
        let path: PathBuf = entry.path();

        let metadata: fs::Metadata = fs::metadata(&path)?;
        let last_modified: u64 = metadata.modified()?.elapsed().unwrap().as_secs();

        if last_modified < 24 * 3600 && metadata.is_file() {
            println!(
                "Last modified: {:?} seconds, in read only: {:?}, size: {:?} bytes, file_name: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No file name")
            )
        }
    }
    Ok(())
}

pub fn find_duplicate_file_name () {
    let mut filenames: HashMap<_, _> = HashMap::new();

    for entry in WalkDir::new("./src")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()) {
        let f_name: String = String::from(entry.file_name().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            println!("{}", f_name);
        }
    }

}

pub fn find_files_with_predicate () -> Result<(), Box<dyn Error>> {
    for entry in WalkDir::new("./src")
        .follow_links(true)
        .into_iter()
        .filter_map(|entry| entry.ok()) {
        let f_name: Cow<str> = entry.file_name().to_string_lossy();
        let sec: SystemTime = entry.metadata()?.modified()?;

        if f_name.ends_with(".rs") && sec.elapsed().unwrap().as_secs() < 86400 {
            println!("{}", f_name);
        }
    }
    Ok(())
}

pub fn traverse_dir_skip_dotfiles() {
    WalkDir::new("./src")
        .into_iter()
        .filter_entry(|entry| is_not_hidden(entry))
        .filter_map(|value| value.ok())
        .for_each(|entry| println!("{}", entry.path().display()));
}

pub fn calculate_file_size_by_depth () {
    let total_size: u64 = WalkDir::new(".")
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metada| metada.is_file())
        .fold(0, |acc, m| acc + m.len());

    println!("Total size: {} bytes.", total_size);
}

pub fn find_txt_files() -> Result<(), Box<dyn Error>> {
    for entry in glob("**/*.txt")? {
        match entry {
            Ok(path) => println!("{}", path.display()),
            Err(e) => eprintln!("Error: {:?}", e)
        }
    }
    Ok(())
}

pub fn find_all_files_ignore_case() -> Result<(), Box<dyn Error>> {
    let options = MatchOptions{
        case_sensitive: false,
        ..Default::default() //".." set the rest of the fields to default
    };

    for entry in glob_with("./assets/images/*", options)? {
        match entry {
            Ok(path) => println!("{}?", path.display()),
            Err(e) => eprintln!("Error: {:?}", e)
        };
    }
    Ok(())
}

// Auxiliary function
fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|str| entry.depth() == 0 || !str.starts_with("."))
        .unwrap_or(false)
}
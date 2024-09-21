use std::fs::File;
use std::io::Read;

use error_chain::error_chain;

error_chain!(
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        ParseIntError(std::num::ParseIntError);
    }
    errors {RandomRespondeError(t: String)}
);

pub fn handle_errors() {
    match read_uptime() {
        Ok(uptime) => println!("Uptime: {} seconds", uptime),
        Err(err) => eprintln!("Error: {}", err)
    }
}

pub fn check_and_handle_errors() {
    if let Err(error) = run() {
        match *error.kind() {
            ErrorKind::Io(_) => println!("Standard IO error: {:?}", error),
            ErrorKind::Reqwest(_) => println!("Reqwest error: {:?}", error),
            ErrorKind::ParseIntError(_) => println!("Standard parse int error: {:?}", error),
            ErrorKind::RandomRespondeError(_) => println!("User defined error: {:?}", error),
            _ => println!("Other error: {:?}", error),

        }
    }
}

// Auxiliary functions
fn read_uptime() -> Result<u64> {
    let mut uptime: String = String::new();
    File::open("proc/uptime")?.read_to_string(&mut uptime)?;

    Ok(uptime
        .split(".")
        .next()
        .ok_or("Cannot parse uptime data")?
        .parse()?
    )
}

fn parse_response(response: reqwest::blocking::Response) -> Result<u32> {
    let mut body: String = response.text()?;
    body.pop(); //remove the last character
    body.parse::<u32>()
        .chain_err(|| ErrorKind::RandomRespondeError(body))
}

fn run() -> Result<()> {
    let url: String =
        "https://rust-lang-nursery.github.io/rust-cookbook/errors/handle.html#handle-errors-correctly-in-main".to_string();
    let response = reqwest::blocking::get(&url)?;
    let random_value: u32 = parse_response(response)?;
    println!("a random number between 0 and 10: {}", random_value);
    Ok(())
}


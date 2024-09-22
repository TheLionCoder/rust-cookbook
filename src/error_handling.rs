use std::fmt;
use std::fs::File;
use std::io::Read;

use error_chain::error_chain;
use serde::Deserialize;

error_chain!(
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        ParseIntError(std::num::ParseIntError);
        Reader(csv::Error);
    }
    errors {RandomRespondeError(t: String)}
);

#[derive(Debug, Deserialize)]
struct Rgb {
    red: u8,
    blue: u8,
    green: u8
}

impl Rgb {
    fn from_reader(csv_data: &[u8]) -> Result<Rgb> {
        let color: Rgb = csv::Reader::from_reader(csv_data)
            .deserialize()
            .nth(0)
            .ok_or("Cannot deserialize the first csv records")?
            .chain_err(|| "Cannot deserialize Rgb color")?;
    Ok(color)
    }
}

impl fmt::UpperHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hexa: u32 = u32::from(self.red) << 16 | u32::from(self.blue) << 8 | u32::from(self.green);
        write!(f, "{:X}", hexa)
    }
}

pub fn handle_errors() {
    match read_uptime() {
        Ok(uptime) => println!("Uptime: {} seconds", uptime),
        Err(err) => eprintln!("Error: {}", err)
    }
}

pub fn check_and_handle_errors() {
    if let Err(error) = check_response() {
        match *error.kind() {
            ErrorKind::Io(_) => println!("Standard IO error: {:?}", error),
            ErrorKind::Reqwest(_) => println!("Reqwest error: {:?}", error),
            ErrorKind::ParseIntError(_) => println!("Standard parse int error: {:?}", error),
            ErrorKind::RandomRespondeError(_) => println!("User defined error: {:?}", error),
            _ => println!("Other error: {:?}", error),

        }
    }
}

pub fn get_backtrace(){
    if let Err(ref errors) = run_rgb_check() {
        eprintln!("Error level - description");
        errors
            .iter()
            .enumerate()
            .for_each(|(index, error)| eprintln!("└> {} - {}", index, error));

        if let Some(backtrace) = errors.backtrace() {
            eprintln!("{:?}", backtrace);
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

fn check_response() -> Result<()> {
    let url: String =
        "https://rust-lang-nursery.github.io/rust-cookbook/errors/handle.html#handle-errors-correctly-in-main".to_string();
    let response = reqwest::blocking::get(&url)?;
    let random_value: u32 = parse_response(response)?;
    println!("a random number between 0 and 10: {}", random_value);
    Ok(())
}

fn run_rgb_check() -> Result<()> {
    let csv_data: &str = "red,blue,green\n\
    102,256,204";

    let rgb: Rgb = Rgb::from_reader(csv_data.as_bytes()).chain_err(|| "Cannot read csv data")?;
    println!("{:?} to hexadecimal #{:X}", rgb, rgb);

    Ok(())

}
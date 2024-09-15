use clap::{Arg, ArgMatches, Command};
use std::string:: String;

pub fn parse_cli() {
    let matches: ArgMatches = Command::new("My test program")
        .version("0.1.0")
        .author("TheLionCoder")
        .about("Learn argument parsing with clap")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .help("A file to process"))
        .arg(Arg::new("num")
            .short('n')
            .long("number")
            .help("Five less than your favorite number")
        ).get_matches();

    let my_file: Option<&String> = matches.get_one::<String>("file");
    println!("The file passed is {:?}", my_file);

    let num_str: Option<&String> = matches.get_one::<String>("num");
    match num_str {
        None => println!("No idea what you favorite number is"),
        Some(n) => {
            match n.parse::<i8>() {
                Ok(n) => println!("Your favorite number must be {}", n + 5),
                Err(_) => println!("That's not a number! {}", n)
            }
        }
    }

}
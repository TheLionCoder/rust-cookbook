use std::process::{Command, Output};

use std::error::Error;
use regex::Regex;

#[derive(Clone, Default, Debug, PartialEq)]
struct Commit {
    hash: String,
    message: String,
}

pub fn run_external_command() -> Result<(), Box<dyn Error>> {
    let output: Output = Command::new("git").arg("log").arg("--oneline").output()?;

    if !output.status.success() {
        return Err("Command execute with failing error code".into());
    }

    let pattern: Regex = Regex::new(
        r"(?x)
    ([0-9a-fA-f]+) # commit hash
        (.*) # The commit message",
    )?;

    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|capture| {
            Commit {
                hash: capture[1].to_string(),
                message: capture[2].to_string()
            }
        })
        .take(5)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}

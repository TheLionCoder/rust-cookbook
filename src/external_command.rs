use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command, Output, Stdio};

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
        .map(|capture| Commit {
            hash: capture[1].to_string(),
            message: capture[2].to_string(),
        })
        .take(5)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}

pub fn run_external_python_command() -> Result<(), Box<dyn Error>> {
    let mut child: Child = Command::new("python3")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .ok_or("Child process stdin has not been captured!")?
        .write_all(b"import this; copyright(); credits(); exit()")?;

    let output: Output = child.wait_with_output()?;

    if output.status.success() {
        let raw_output: String = String::from_utf8(output.stdout)?;
        let words: HashSet<String> = raw_output
            .split_whitespace()
            .map(|str| str.to_lowercase())
            .collect::<HashSet<_>>();
        println!("Found {} unique words:", words.len());
        println!("{:#?}", words);
        Ok(())
    } else {
        let error_message: String = String::from_utf8(output.stderr)?;
        Err(error_message.into())
    }
}

pub fn run_piped_external_command() -> Result<(), Box<dyn Error>> {
    let directory: PathBuf = env::current_dir()?;
    let mut du_output_child: Child = Command::new("du")
        .arg("-ah")
        .arg(&directory)
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(du_output) = du_output_child.stdout.take() {
        let mut sort_output_child: Child = Command::new("sort")
            .arg("-hr")
            .stdin(du_output)
            .stdout(Stdio::piped())
            .spawn()?;

        du_output_child.wait()?;

        if let Some(sort_output) = sort_output_child.stdout.take() {
            let head_output_child: Child = Command::new("head")
                .args(&["-n", "10"])
                .stdin(sort_output)
                .stdout(Stdio::piped())
                .spawn()?;

            let head_stdout: Output = head_output_child.wait_with_output()?;

            sort_output_child.wait()?;

            println!(
                "Top 10 biggest files and directories in '{}': \n{}",
                directory.display(),
                String::from_utf8(head_stdout.stdout)?
            );
        }
    }
    Ok(())
}

pub fn redirect_stdout_and_stderr_to_file() -> Result<(), Box<dyn Error>> {
    let outputs: File = File::create("./assets/out.txt")?;
    let errors: File = outputs.try_clone()?;

    Command::new("ls")
        .args(&[".", "oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;

    Ok(())
}

pub fn read_env_var() -> Result<(), Box<dyn Error>> {
    let config_path: String = env::var("CONFIG").unwrap_or("/etc/myapp/config".to_string());

    let config: String = fs::read_to_string(config_path)?;
    println!("Config: {}", config);
    Ok(())
}

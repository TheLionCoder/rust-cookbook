use std::fs::File;
use std::io::{copy, Read};
use std::path::PathBuf;
use std::str::FromStr;

use error_chain::error_chain;
use reqwest::blocking::Client;
use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
use reqwest::Response;
use reqwest::StatusCode;
use tempfile::{Builder, TempDir};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequestError(reqwest::Error);
        HeaderError(reqwest::header::ToStrError);
    }
}

struct PartialRangeIter {
    start: u64,
    end: u64,
    buffer_size: u32,
}

impl PartialRangeIter {
    pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self> {
        if buffer_size == 0 {
            Err("invalid buffer_size, give a value greater than zero")?;
        }
        Ok(PartialRangeIter {
            start,
            end,
            buffer_size,
        })
    }
}

impl Iterator for PartialRangeIter {
    type Item = HeaderValue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let prev_start: u64 = self.start;
            self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
            Some(
                HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1))
                    .expect("string provided by format"),
            )
        }
    }
}

#[tokio::main]
pub async fn download_file_to_tmp_dir() -> Result<()> {
    let tmp_dir: TempDir = Builder::new().prefix("example").tempdir()?;
    let target: &str = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response: Response = reqwest::get(target).await?;

    let mut dest = {
        let fname: &str = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}", fname);
        let fname: PathBuf = tmp_dir.path().join(fname);
        println!("Will be located under: {:?}", fname);
        File::create(fname)?
    };

    let content: String = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}

#[tokio::main]
pub async fn post_file_to_paste_rs() -> Result<()> {
    let paste_api: &str = "https://paste.rs";
    let mut file: File = File::open("./assets/lines.txt")?;

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    let client = reqwest::Client::new();
    let response = client.post(paste_api).body(contents).send().await?;

    let response_text: String = response.text().await?;
    println!("Your paste is located at: {}", response_text);
    Ok(())
}

pub fn make_a_partial_download_with_http() -> Result<()> {
    let url: &str = "https://httpbin.org/range/102400?duration=2";
    const CHUNK_SIZE: u32 = 10240;

    let client: Client = Client::new();
    let response: reqwest::blocking::Response = client.head(url).send()?;
    let content_lenght: &HeaderValue = response
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or("response doesn´t include the content length")?;

    let length: u64 =
        u64::from_str(content_lenght.to_str()?).map_err(|_| "invalid Content-Length header")?;

    let mut output_file: File = File::create("assets/download.bin")?;

    println!("Stating download...");
    for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)? {
        println!("range {:?}", range);
        let mut response = client.get(url).header(RANGE, range).send()?;

        let status = response.status();
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
            error_chain::bail!("Unexpected server response: {}", status);
        }
        std::io::copy(&mut response, &mut output_file)?;
    }

    let content: String = response.text()?;
    std::io::copy(&mut content.as_bytes(), &mut output_file)?;

    println!("Finished with success!");
    Ok(())
}

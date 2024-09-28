use std::io::Read;

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequestError(reqwest::Error);
    }
}

pub fn make_get_request(url: &str) -> Result<()> {
    let mut res = reqwest::blocking::get(url)?;
    let mut body: String = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n {:#?}", res.headers());
    println!("Body: \n{}", body);
    Ok(())
}


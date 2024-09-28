use error_chain::error_chain;
use url::{Host, Origin, ParseError, Position, Url};

error_chain! {
    foreign_links {
        UrlParse(ParseError);
    }
    errors {
        CannotBeABase
    }
}

pub fn parse_url_from_string(url: &str) -> Result<()> {
    let parsed: Url = Url::parse(url)?;
    println!("The path part of the URL is: {} \n", parsed.path());

    Ok(())
}

pub fn create_base_url_by_remove_path(url: &str) -> Result<()> {
    let url: Url = Url::parse(url)?;
    let base: Url = base_url(url)?;

    assert_eq!(base.as_str(), "https://github.com/");
    println!("The base of the URL is: {} \n", base);
    Ok(())
}

pub fn create_github_url() -> Result<()> {
    let path: &str = "/rust-lang-cargo";

    let gh: Url = build_github_url(path)?;

    assert_eq!(gh.as_str(), "https://github.com/rust-lang-cargo");
    println!("The joined URL is: {}", gh);
    Ok(())
}

pub fn extract_url_origin() -> Result<()> {
    let str: &str = "ftp:/rust-lang.org/examples";
    let url: Url = Url::parse(str)?;

    let expected_scheme: String = "ftp".to_owned();
    let expected_host: Host = Host::Domain("rust-lang.org").to_owned();
    let expected_port = 21_u16;
    let expected: Origin = Origin::Tuple(expected_scheme, expected_host, expected_port);

    let origin: Origin = url.origin();
    assert_eq!(expected, origin);
    println!("The origin is as expected");

    Ok(())
}

pub fn remove_fragment_from_url(url: &str) -> Result<()> {
    let parsed: Url = Url::parse(url)?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("Cleaned URL: {}", cleaned);
    Ok(())
}

// Auxiliary functions
fn base_url(mut url: Url) -> Result<Url> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err(Error::from_kind(ErrorKind::CannotBeABase));
        }
    }
    url.set_query(None);
    Ok(url)
}

fn build_github_url(path: &str) -> Result<Url> {
    const GITHUB: &str = "https://github.com";

    let base: Url = Url::parse(GITHUB).expect("hardcoded URL is known to be valid");
    let joined: Url = base.join(path)?;
    Ok(joined)
}

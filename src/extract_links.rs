use std::borrow::Cow;
use std::collections::HashSet;

use error_chain::error_chain;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{Response, StatusCode};
use select::document::Document;
use select::predicate::Name;
use url::{Position, Url};

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
        UrlParseError(url::ParseError);
        JoinError(tokio::task::JoinError);
    }
}

#[tokio::main]
pub async fn extract_all_links(url: &str) -> Result<()> {
    let res: String = reqwest::get(url).await?.text().await?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|name| name.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}

#[tokio::main]
pub async fn check_webpage_links(url: &str) -> Result<()> {
    let url: Url = Url::parse(url)?;
    let res: String = reqwest::get(url.as_ref()).await?.text().await?;
    let doc: Document = Document::from(res.as_str());
    let base_url: Url = get_base_url(&url, &doc).await?;
    let base_parser = Url::options().base_url(Some(&base_url));
    let links: HashSet<Url> = doc
        .find(Name("a"))
        .filter_map(|name| name.attr("href"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();
    let mut tasks: Vec<_> = vec![];

    for link in links {
        let link_clone: Url = link.clone();
        tasks.push(tokio::spawn(async move {
            if check_link(&link).await.unwrap() {
                println!("{} is Ok", link_clone)
            } else {
                println!("{} is broken", link_clone)
            }
        }));
    }

    for task in tasks {
        task.await?
    }

    Ok(())
}

#[tokio::main]
pub async fn extract_unique_wiki_links() -> Result<()> {
    let content: String = reqwest::get(
        "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",
    )
    .await?
    .text()
    .await?;

    println!("{:?}", extract_links(content.as_str()));

    Ok(())
}

// Aux functions
async fn get_base_url(url: &Url, doc: &Document) -> Result<Url> {
    let base_tag_href: Option<&str> = doc
        .find(Name("base"))
        .filter_map(|name| name.attr("href"))
        .next();
    let base_url: Url =
        base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
    Ok(base_url)
}

async fn check_link(url: &Url) -> Result<bool> {
    let res: Response = reqwest::get(url.as_ref()).await?;
    Ok(res.status() != StatusCode::NOT_FOUND)
}

fn extract_links(content: &str) -> HashSet<Cow<str>> {
    lazy_static! {
        static ref WIKI_REGEX: Regex = Regex::new(
            r"(?x)
            \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\] # internal links
            |
            (url=|URL\||\[)(?P<external>http.*?)[ \|}] # external links
            "
        )
        .unwrap();
    }

    let links: HashSet<_> = WIKI_REGEX
        .captures_iter(content)
        .map(|cap| match (cap.name("internal"), cap.name("external")) {
            (Some(val), None) => Cow::from(val.as_str().to_lowercase()),
            (None, Some(val)) => Cow::from(val.as_str()),
            _ => unreachable!(),
        })
        .collect();

    links
}

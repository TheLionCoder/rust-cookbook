use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

#[tokio::main]
pub async fn extract_all_links() -> Result<()> {
    let res: String = reqwest::get("https://www.rust-lang.org/en-US")
        .await?
        .text()
        .await?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|name| name.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}

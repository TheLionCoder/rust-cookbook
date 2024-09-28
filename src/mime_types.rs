use std::str::FromStr;

use error_chain::error_chain;
use mime::{Mime, APPLICATION_OCTET_STREAM};
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::Response;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        Header(reqwest::header::ToStrError);
        Mime(mime::FromStrError);
    }
}

pub fn get_mime_type_from_string() {
    let invalid_mime_type: &str = "i n v a l i d";
    let default_mime = invalid_mime_type
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);

    println!(
        "MIME for {:?} used default value {:?}",
        invalid_mime_type, default_mime
    );

    let valid_mime_type: &str = "TXT/PLAIN";
    let parsed_mime = valid_mime_type
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);

    println!(
        "MIME for {:?} was parsed as {:?}",
        valid_mime_type, parsed_mime
    )
}

pub fn get_mime_type_from_filename(file_name: &str) -> Mime {
    let parts: Vec<&str> = file_name.split('.').collect();

    let res = match parts.last() {
        Some(v) => match *v {
            "png" => mime::IMAGE_PNG,
            "jpg" => mime::IMAGE_JPEG,
            "json" => mime::TEXT_PLAIN,
            &_ => mime::TEXT_PLAIN,
        },
        None => mime::TEXT_PLAIN,
    };
    res
}

#[tokio::main]
pub async fn parse_http_mime_type_response(url: &str) -> Result<()> {
    let response: Response = reqwest::get(url).await?;
    let headers: &HeaderMap = response.headers();

    match headers.get(CONTENT_TYPE) {
        None => {
            println!("The response doesnot contain a Content-Type header");
        }
        Some(content_type) => {
            let content_type = Mime::from_str(content_type.to_str()?)?;
            let media_type = match (content_type.type_(), content_type.subtype()) {
                (mime::TEXT, mime::HTML) => "a HTML document",
                (mime::TEXT, _) => "a text document",
                (mime::IMAGE, mime::PNG) => "a PNG image",
                (mime::IMAGE, _) => "an image",
                _ => "neither text nor image",
            };
            println!("The response contains {}.", media_type);
        }
    };

    Ok(())
}

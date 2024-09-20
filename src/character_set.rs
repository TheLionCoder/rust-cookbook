use std::borrow::Cow;
use percent_encoding::{percent_decode, utf8_percent_encode, AsciiSet, PercentDecode, PercentEncode, CONTROLS};
use std::str::Utf8Error;
use url::form_urlencoded::{byte_serialize, parse};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn encode_string() -> Result<(), Utf8Error> {
    let input: &str = "confident, productive systems programming";

    let iter: PercentEncode = utf8_percent_encode(input, FRAGMENT);
    let encoded: String = iter.collect();
    assert_eq!(encoded, "confident,%20productive%20systems%20programming");

    let iter: PercentDecode = percent_decode(encoded.as_bytes());
    let decoded :Cow<str>= iter.decode_utf8()?;
    assert_eq!(decoded.as_ref(), input);

    Ok(())
}

pub fn encode_url() {
    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");
    println!("urlencoded: '{}'", urlencoded);

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    assert_eq!(decoded, "What is ❤?");
    println!("decoded: '{}'", decoded);
}
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;

struct PhoneNumber<'a> {
    area: &'a str,
    exchange: &'a str,
    subscriber: &'a str,
}

impl<'a> fmt::Display for PhoneNumber<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "1 ({}) {}-{}", self.area, self.exchange, self.subscriber)
    }
}

pub fn extract_phone_number() -> Result<(), Box<dyn Error>> {
    let phone_text: &str = "\
    +1 505 881 9292 (v) +1 505 778 2212 (c) +1 505 881 9297 (f)
    (202) 991 9534
    Alex 5553920011
    1 (800) 233-2010
    1.299.339.1020
    ";

    let re: Regex = Regex::new(
        r#"(?x)
        (?:\+?1)? # Country code
        [\s.]*
        (([2-9]\d{2})|\(([2-9]\d{2})\)) # Area code
        [\s.-]?
        ([2-9]\d{2}) # Exchange code
        [\s.-]?
        (\d{4}) # Subscriber Number
        "#,
    )?;
    let phone_numbers = re.captures_iter(phone_text).filter_map(|capture| {
        let groups = (
            capture.get(2).or(capture.get(3)),
            capture.get(4),
            capture.get(5),
        );
        match groups {
            (Some(area), Some(ext), Some(sub)) => Some(PhoneNumber {
                area: area.as_str(),
                exchange: ext.as_str(),
                subscriber: sub.as_str(),
            }),
            _ => None,
        }
    });
    assert_eq!(
        phone_numbers.map(|m| m.to_string()).collect::<Vec<_>>(),
        vec![
            "1 (505) 881-9292",
            "1 (505) 778-2212",
            "1 (505) 881-9297",
            "1 (202) 991-9534",
            "1 (555) 392-0011",
            "1 (800) 233-2010",
            "1 (299) 339-1020"
        ]
    );
    Ok(())
}

pub fn validate_login_from_email() {
    assert_eq!(extract_login(r"I❤email@example.com"), Some(r"I❤email"));
    assert_eq!(
        extract_login(r"sdf+sdsfsd.as.sds@jhkk.r.rl"),
        Some(r"sdf+sdsfsd.as.sds")
    );
    assert_eq!(extract_login(r"More@Than@one@at.com"), None);
    assert_eq!(extract_login(r"Not an email@email"), None);
}

pub fn extract_list_unique_hashtags() {
    let tweet: &str = "Hey #world, I just got my new #dog, say hello to Till.\
   #dog #forever #2 #_";
    let tags: HashSet<&str> = extract_hashtags(tweet);
    assert!(tags.contains("#dog") && tags.contains("#forever") && tags.contains("#world"));
    assert_eq!(tags.len(), 3);
}

pub fn replace_all_text_occurrences() {
    let before: &str = "2012-03-14, 2013-01-15 and 2014-07-05";
    let after: Cow<str> = reformat_dates(before);
    assert_eq!(after, "03/14/2012, 01/15/2013 and 07/05/2014");
}

// Auxiliary functions
fn extract_login(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
        ^(?P<login>[^@\s]+)@
        ([[:word:]]+\.)*
        [[:word:]]+$"
        )
        .unwrap();
    }
    RE.captures(input)
        .and_then(|cap| cap.name("login").map(|login| login.as_str()))
}

fn extract_hashtags(text: &str) -> HashSet<&str> {
    lazy_static! {
        static ref HASHTAG_REGEX: Regex = Regex::new(r"\#[a-zA-Z][0-9a-zA-Z_]*").unwrap();
    }
    HASHTAG_REGEX
        .find_iter(text)
        .map(|mat| mat.as_str())
        .collect()
}

fn reformat_dates(before: &str) -> Cow<str> {
    lazy_static! {
        static ref ISO8601_DATE_REGEX: Regex =
            Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
    }
    ISO8601_DATE_REGEX.replace_all(before, "$m/$d/$y")
}

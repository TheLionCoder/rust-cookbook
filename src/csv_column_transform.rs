use csv::{Reader, Writer};
use serde::{de, Deserialize, Deserializer};
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct HexColor {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Deserialize)]
struct Row {
    color_name: String,
    color: HexColor,
}

#[derive(Debug)]
enum HexColorError {
    InvalidLength,
    ParseIntError(ParseIntError),
}

impl fmt::Display for HexColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HexColorError::InvalidLength => write!(f, "Invalid length of hex string"),
            HexColorError::ParseIntError(e) => write!(f, "{}", e),
        }
    }
}

impl From<ParseIntError> for HexColorError {
    fn from(err: ParseIntError) -> HexColorError {
        HexColorError::ParseIntError(err)
    }
}

impl FromStr for HexColor {
    type Err = HexColorError;

    fn from_str(hex_color: &str) -> Result<Self, Self::Err> {
        let trimmed: &str = hex_color.trim_matches('#');
        if trimmed.len() != 6 {
            Err(HexColorError::InvalidLength)
        } else {
            Ok(HexColor {
                red: u8::from_str_radix(&trimmed[..2], 16)?,
                green: u8::from_str_radix(&trimmed[2..4], 16)?,
                blue: u8::from_str_radix(&trimmed[4..6], 16)?,
            })
        }
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string: String = String::deserialize(deserializer)?;
        FromStr::from_str(&string).map_err(de::Error::custom)
    }
}

pub fn transform_column() -> Result<(), Box<dyn Error>> {
    let data: String = "color_name,color\n\
    red,#ff0000\n\
    green,#00ff00\n\
    blue,#0000FF\n\
    periwinkle,#ccccff\n\
    magenta,#ff00ff"
        .to_owned();

    let mut out: Writer<Vec<u8>> = Writer::from_writer(vec![]);
    let mut reader: Reader<&[u8]> = Reader::from_reader(data.as_bytes());
    for result in reader.deserialize::<Row>() {
        match result {
            Ok(res) => {
                out.serialize((
                    res.color_name,
                    res.color.red,
                    res.color.green,
                    res.color.blue,
                ))?;
            }
            Err(e) => {
                eprintln!("Error deserializing row: {}", e);
                return Err(Box::new(e));
            }
        }
    }

    let written: String = String::from_utf8(out.into_inner()?)?;
    assert_eq!(Some("magenta,255,0,255"), written.lines().last());
    println!("{}", written);
    Ok(())
}

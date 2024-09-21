use std::io;

use csv::{Error, Writer};
use serde::Serialize;

#[derive(Serialize)]
struct Record<'a> {
    name: &'a str,
    place: &'a str,
    id: u64
}

pub fn serialize_records() -> Result<(), Error> {
    let mut writer: Writer<io::Stdout> = Writer::from_writer(io::stdout());

    let rec1: Record = Record {name: "Mark", place: "Melbourne", id: 46};
    let rec2: Record = Record {name: "Ashley", place: "Sidney", id: 64};
    let rec3: Record = Record {name: "Akshat", place: "Delhi", id: 92};

    writer.serialize(rec1)?;
    writer.serialize(rec2)?;
    writer.serialize(rec3)?;

    writer.flush()?;

    Ok(())
}
use std::io;

use csv::{Error, Writer};

pub fn serialize_records() -> Result<(), Error> {
    let mut writer: Writer<io::Stdout> = Writer::from_writer(io::stdout());

    writer.write_record(&["Name", "Place", "ID"])?;

    writer.serialize(("Mark", "Sidney", 87))?;
    writer.serialize(("Ashely", "Dublin", 32))?;
    writer.serialize(("Akshat", "Delhi", 11))
}
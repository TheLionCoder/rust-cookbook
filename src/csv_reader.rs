use csv::{Error, Reader, ReaderBuilder};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

pub fn read_csv() -> Result<(), Error> {
    let data: &str = "name\tplace\tid\n
    Mark\tMelbourne\t46\n
    Ashley\tSydney\t92";

    let mut reader: Reader<&[u8]> = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(data.as_bytes());
    for result in reader.deserialize::<Record>() {
        println!("{:?}", result?)
    }

    Ok(())
}

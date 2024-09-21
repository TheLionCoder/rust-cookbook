use csv::{Reader, Error};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>
}

pub fn handle_invalid_csv_data() -> Result<(), Error> {
    let data: &str = "name,place,id\n\
    mark,Sidney,46.5\n\
    ashley,zurich,92\n\
    akshat,delhi,37\n\
    alisha,colombo,xyz";

    let mut reader: Reader<&[u8]> = Reader::from_reader(data.as_bytes());
    for result in reader.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
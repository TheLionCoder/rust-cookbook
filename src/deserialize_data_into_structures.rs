use csv::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String
}

pub fn deserialize_data_into_structures() -> Result<(), Error> {
    let csv: &str = "year,make,model,description\n\
    1948,Porsche,356,Luxury sports car\n\
    1967,Ford,Mustang fastback 1967,American car";

    let mut reader: csv::Reader<&[u8]> = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.deserialize() {
        let record: Record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}",
            record.year,
            record.make,
            record.model,
            record.description
        )
    }

    Ok(())
}

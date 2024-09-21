use std::io;
use std::io::Stdout;
use csv::{Reader, ReaderBuilder, Error, Writer, StringRecord};

pub fn filter_csv() -> Result<(), Error> {
    let query: &str = "CA";
    let data: &str = "\
    City,State,Population,Latitude,Longitude\n
    Kenai,AK,7610,60.5544444,-151.2583333\n
    Oakman,AL,,33.7133333,-87.3886111\n
    Sandfort,AL,,32.3380556,-85.2233333\n
    West Hollywood,CA,370031,34.0900000,-118.3608333";

    let mut reader: Reader<&[u8]> = ReaderBuilder::new().from_reader(data.as_bytes());
    let mut writer: Writer<Stdout> = Writer::from_writer(io::stdout());

    writer.write_record(reader.headers()?)?;

    for result in reader.records() {
        let record: StringRecord = result?;
        if record.iter().any(|field| field == query) {
            writer.write_record(&record)?
        }
    }
    writer.flush()?;

    Ok(())
}

use csv::{Error, Reader};

pub fn read_std_csv() -> Result<(), Error> {
    let csv: &str = "year,make,model,description\n
    1948,Posche,356,Luxury sports car\n
    1967,Ford,Mustang fastback 1967,American car";

    let mut reader: Reader<&[u8]> = Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record: csv::StringRecord = record?;
        println!(
            "In {}, {} built the {} model. It is a {}",
            &record[0], &record[1], &record[2], &record[3]
        )
    };
    Ok(())
}

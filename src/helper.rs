use bincode::serialize_into;
use csv::{Reader, WriterBuilder};
use csv_read::read::get_temp_stations;
use data::{CsvRecord, TemperaturePoint};
use std::error::Error;
use std::fs::{read_dir, remove_file, File};
use std::path::Path;

pub fn _reader_test(_file_count: usize) {
    remove_file("Data.csv").expect("Failed to remove data file");

    let buffer = File::create("Data.csv").expect("Couldnt create file");
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(buffer);
    wtr.write_record(&[
        "YEAR",
        "MONTH",
        "DAY",
        "LONGITUDE",
        "LATITUDE",
        "ELEVATION",
        "TAVG",
    ]).expect("Failed to write headers");
    for (i, dir) in read_dir("F:/Uni/Grand Challenges/Data/gsom-latest")
        .unwrap()
        .enumerate()
    {
        if i % 1000 == 1 {
            println!("{}", i);
        }
        let dir = dir.unwrap();
        if let Ok(test) = get_temp_stations(dir.path()) {
            for value in test {
                wtr.serialize(value).expect("Failed");
            }
        } else {
            continue;
        }
    }
    wtr.flush().expect("Flush failed");
}

pub fn csv_to_bin(path: impl AsRef<Path>) -> Result<(), Box<Error>> {
    let mut reader = Reader::from_path(path)?;
    remove_file("Data.b");
    let mut file = File::create("Data.b")?;

    let mut values = Vec::new();
    for (i, result) in reader.deserialize().enumerate() {
        if i % 1000000 == 0 {
            println!("{}", i);
        }
        let record: CsvRecord = result?;
        let point = match TemperaturePoint::from(record) {
            Some(point) => point,
            None => continue,
        };
        values.push(point);
    }
    serialize_into(&mut file, &values);
    Ok(())
}

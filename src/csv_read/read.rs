use csv::{Reader, StringRecord, StringRecordsIter};
use csv_read::errors::{ColumnMissing, IncorrectColumnsErr};
use csv_read::{FromStringRecord, Header, HeaderContainer};
use std::error::Error;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct TempStation {
    longitude: Option<f32>,
    latitude: Option<f32>,
    elevation: Option<f32>,
    avg_temp: Option<f32>,
}

impl FromStringRecord for TempStation {
    fn field_count() -> usize {
        4
    }
    fn from_string_records<T: Read>(
        headers: &HeaderContainer,
        records: &mut StringRecordsIter<T>,
    ) -> Result<Vec<Self>, Box<Error>> {
        if Self::correct_num_headers(headers) {
            let long_col;
            let lat_col;
            let ele_col;
            let temp_col;
            if let Some(column) = headers.column_with_name("LONGITUDE") {
                long_col = column;
            } else {
                return Err(Box::new(ColumnMissing::new(String::from("LONGITUDE"))));
            }
            if let Some(column) = headers.column_with_name("LATITUDE") {
                lat_col = column;
            } else {
                return Err(Box::new(ColumnMissing::new(String::from("LATITUDE"))));
            }
            if let Some(column) = headers.column_with_name("ELEVATION") {
                ele_col = column;
            } else {
                return Err(Box::new(ColumnMissing::new(String::from("ELEVATION"))));
            }
            if let Some(column) = headers.column_with_name("TAVG") {
                temp_col = column;
            } else {
                return Err(Box::new(ColumnMissing::new(String::from("TAVG"))));
            }
            let mut values = Vec::new();
            for record in records {
                let record = record?;
                let mut long = None;
                let mut lat = None;
                let mut ele = None;
                let mut temp = None;
                if let Ok(num) = record[long_col].parse::<f32>() {
                    long = Some(num);
                }
                if let Ok(num) = record[lat_col].parse::<f32>() {
                    lat = Some(num);
                }
                if let Ok(num) = record[ele_col].parse::<f32>() {
                    ele = Some(num);
                }
                if let Ok(num) = record[temp_col].parse::<f32>() {
                    temp = Some(num);
                    values.push(Self {
                        longitude: long,
                        latitude: lat,
                        elevation: ele,
                        avg_temp: temp,
                    });
                }
                if temp != None {
                }
            }
            return Ok(values);
        }
        Err(Box::new(IncorrectColumnsErr::new(headers.clone())))
    }
}

pub fn get_temp_stations<P: AsRef<Path>>(path: P) -> Result<Vec<TempStation>, Box<Error>> {
    let mut reader = Reader::from_path(path)?;
    let headers;
    {
        let header_record = reader.headers()?;
        headers = HeaderContainer::from_record(
            header_record,
            &["LONGITUDE", "LATITUDE", "ELEVATION", "TAVG"],
            &mut vec![],
        )?;
    }
    let values = TempStation::from_string_records(&headers, &mut reader.records())?;
    Ok(values)
}

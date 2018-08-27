use csv::{Reader, StringRecordsIter};
use csv_read::errors::{IncorrectColumnsErr};
use csv_read::{FromStringRecord, HeaderContainer};
use std::error::Error;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use std::num::ParseIntError;


#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Date {
    pub year: Option<u32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
}

impl FromStr for Date {
    type Err = ParseIntError;
    
    // Has to be in form year-month-day
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let year = split.next().and_then(|x| x.parse().ok());
        let month = split.next().and_then(|x| x.parse().ok());
        let day = split.next().and_then(|x| x.parse().ok());

        return Ok(Self {
            year,
            month,
            day
        });
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct TempStation {
    date: Option<Date>,
    longitude: Option<f32>,
    latitude: Option<f32>,
    elevation: Option<f32>,
    avg_temp: Option<f32>,
}

impl FromStringRecord for TempStation {
    fn field_count() -> usize {
        5
    }
    fn from_string_records<T: Read>(
        headers: &HeaderContainer,
        records: &mut StringRecordsIter<T>,
    ) -> Result<Vec<Self>, Box<Error>> {
        if Self::correct_num_headers(headers) {
            let long_col = headers.column_with_name("LONGITUDE")?;
            let lat_col = headers.column_with_name("LATITUDE")?;
            let ele_col = headers.column_with_name("ELEVATION")?;
            let temp_col = headers.column_with_name("TAVG")?;
            let date_col = headers.column_with_name("DATE")?;
            

            let mut values = Vec::new();
            for record in records {
                let record = record?;
                let mut long = None;
                let mut lat = None;
                let mut ele = None;
                let mut temp = None;
                let mut date = None;
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
                }
                if let Ok(num) = record[date_col].parse::<Date>() {
                    date = Some(num);
                }
                if temp != None {
                    values.push(Self {
                        date,
                        longitude: long,
                        latitude: lat,
                        elevation: ele,
                        avg_temp: temp,
                    });
                }
            }
            return Ok(values);
        }
        Err(Box::new(IncorrectColumnsErr::new(headers.clone())))
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct WindStation {
    pub date: Option<Date>,
    pub longitude: Option<f32>,
    pub latitude: Option<f32>,
    pub avg_wind: Option<f32>,
}

impl FromStringRecord for WindStation {
    fn field_count() -> usize {
        4
    }

    fn from_string_records<T: Read>(
        headers: &HeaderContainer,
        records: &mut StringRecordsIter<T>,
    ) -> Result<Vec<Self>, Box<Error>> {
        if Self::correct_num_headers(headers) {
            let long_col = headers.column_with_name("LONGITUDE")?;
            let lat_col = headers.column_with_name("LATITUDE")?;
            let date_col = headers.column_with_name("DATE")?;
            let avg_wind_col = headers.column_with_name("AWND")?;
            

            let mut values = Vec::new();
            for record in records {
                let record = record?;
                let mut long = record[long_col].parse().ok();
                let mut lat = record[lat_col].parse().ok();
                let mut date = record[date_col].parse().ok();
                let mut avg_wind = record[avg_wind_col].parse().ok();
                if avg_wind != None {
                    values.push(Self {
                        date,
                        longitude: long,
                        latitude: lat,
                        avg_wind
                    });
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
            &["DATE", "LONGITUDE", "LATITUDE", "ELEVATION", "TAVG"],
            &mut vec![],
        )?;
    }
    let values = TempStation::from_string_records(&headers, &mut reader.records())?;
    Ok(values)
}

pub fn get_wind_stations<P: AsRef<Path>>(path: P) -> Result<Vec<WindStation>, Box<Error>> {
    let mut reader = Reader::from_path(path)?;
    let headers;
    {
        let header_record = reader.headers()?;
        headers = HeaderContainer::from_record(
            header_record,
            &["DATE", "LONGITUDE", "LATITUDE", "AWND"],
            &mut vec![],
        )?;
    }
    let values = WindStation::from_string_records(&headers, &mut reader.records())?;
    Ok(values)
}
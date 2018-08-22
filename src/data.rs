use math::Point;
use std::ops::{Add, AddAssign, Div};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct DataPoint<T: Copy> {
    pub position: Point<f32>,
    pub data: T,
}

impl<T: Copy> DataPoint<T> {
    pub fn new(position: Point<f32>, data: T) -> Self {
        Self { position, data }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CSum<T: Copy + Add<T, Output = T> + AddAssign + Div<f32, Output = T>> {
    count: i64,
    data: Option<T>,
}

impl<T: Copy + Add<T, Output = T> + AddAssign + Div<f32, Output = T>> CSum<T> {
    pub fn new() -> Self {
        Self {
            count: 0,
            data: None,
        }
    }
    pub fn average(&self) -> Option<T> {
        if let Some(sum) = self.data {
            Some(sum / self.count as f32)
        } else {
            None
        }
    }
    pub fn add(&mut self, other: T) {
        if let Some(ref mut sum) = self.data {
            *sum += other;
        } else {
            self.data = Some(other);
        }
        self.count += 1;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct YearlyData<T: Copy + Add<T, Output = T> + AddAssign + Div<f32, Output = T>> {
    pub montly_data: [CSum<T>; 12],
}

impl YearlyData<f32> {
    pub fn new() -> Self {
        Self {
            montly_data: [CSum::new(); 12],
        }
    }

    // Month has to be between 1 - 12 inclusive
    pub fn add_to(&mut self, data: f32, month: usize) {
        self.montly_data[month - 1].add(data);
    }

    pub fn yearly_average(&self) -> Option<f32> {
        let mut none_count = 0;
        let mut average = None;

        for month in self.montly_data.iter() {
            match month.average() {
                Some(number) => match average {
                    Some(ref mut value) => *value += number,
                    None => average = Some(number),
                },
                None => none_count += 1,
            }
        }
        if none_count == 0 {
            match average {
                Some(value) => {
                    let avg = value / (12 - none_count) as f32;
                    return Some(avg);
                }
                None => return None,
            }
        }
        None
    }

    pub fn variance(&self) -> Option<f32> {
        match self.yearly_average() {
            Some(avg) => {
                let mut sum = 0.0;
                for &month in &self.montly_data {
                    match month.average() {
                        Some(month_avg) => sum += (month_avg - avg).powi(2),
                        None => return None,
                    }
                }
                Some(sum / 12.0)
            }
            None => None,
        }
    }

    pub fn standard_dev(&self) -> Option<f32> {
        match self.variance() {
            Some(value) => Some(value.sqrt()),
            None => None
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CsvRecord {
    #[serde(rename = "YEAR")]
    year: Option<u32>,
    #[serde(rename = "MONTH")]
    month: Option<u32>,
    #[serde(rename = "DAY")]
    day: Option<u32>,
    #[serde(rename = "LONGITUDE")]
    longitude: Option<f32>,
    #[serde(rename = "LATITUDE")]
    latitude: Option<f32>,
    #[serde(rename = "ELEVATION")]
    elevation: Option<f32>,
    #[serde(rename = "TAVG")]
    avg_temp: f32,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct TemperaturePoint {
    pub month: usize,
    pub data: DataPoint<f32>,
}

impl TemperaturePoint {
    pub fn new(month: usize, data: DataPoint<f32>) -> Self {
        Self { month, data }
    }
    pub fn from(record: CsvRecord) -> Option<Self> {
        let temp_avg = record.avg_temp;
        Some(Self {
            month: record.month? as usize,
            data: DataPoint::new(Point::new(record.longitude?, record.latitude?), temp_avg),
        })
    }
}

use csv::Reader;
use data::{DataPoint, YearlyData};
use grid::{Grid, HeatMap};
use math::{Point, RangeBox};
use std::error::Error;
use std::path::Path;
use glium::VertexBuffer;
use glium::backend::glutin::Display;
use render::Vertex;

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

#[derive(Copy, Clone, Debug)]
pub struct TemperaturePoint {
    pub month: usize,
    pub data: DataPoint<f32>,
}

impl TemperaturePoint {
    pub fn new(month: usize, data: DataPoint<f32>) -> Self {
        Self { month, data }
    }
    pub fn from(record: CsvRecord) -> Option<Self> {
        let month;
        let longitude;
        let latitude;
        match record.month {
            Some(mon) => month = mon as usize,
            None => return None,
        }
        match record.latitude {
            Some(lat) => latitude = lat,
            None => return None,
        }
        match record.longitude {
            Some(long) => longitude = long,
            None => return None,
        }
        let temp_avg = record.avg_temp;
        Some(Self {
            month,
            data: DataPoint::new(Point::new(longitude, latitude), temp_avg),
        })
    }
}

pub type TemperatureGrid = HeatMap<YearlyData<f32>>;

impl TemperatureGrid {
    pub fn new_temperature_grid(dimensions: (usize, usize), range: RangeBox<f32>) -> Self {
        let grid = Grid::new(dimensions.0, dimensions.1, YearlyData::new());
        HeatMap::new(grid, range)
    }

    pub fn add_temperature_point(&mut self, point: &TemperaturePoint) {
        self.add_data(&point.data, |grid, index, data| {
            grid[index].add_to(*data, point.month);
        });
    }
    
    pub fn average_temp_grid(&self) -> Grid<Option<f32>> {
        let mut average_temps = Vec::with_capacity(self.grid.values.len());
        for yearly_temp in self.grid.values_ref() {
            match yearly_temp.yearly_average() {
                Some(temp) => average_temps.push(Some(temp)),
                None => average_temps.push(None),
            }
        }
        Grid::new_from_values(self.grid.horizontal, self.grid.vertical, average_temps)
    }
}

pub fn temp_heat_map_from_data(dimensions: (usize, usize), range: RangeBox<f32>, path: impl AsRef<Path>) -> Result<TemperatureGrid, Box<Error>> {
    let mut reader = Reader::from_path(path)?;
    let mut temp_grid = TemperatureGrid::new_temperature_grid(dimensions, range);
    for (i, result) in reader.deserialize().enumerate() {
        if i % 1000000 == 0 {
            println!("{}", i);
        }
        let record: CsvRecord = result?;
        let point = match TemperaturePoint::from(record) {
            Some(point) => point,
            None => continue,
        };
        temp_grid.add_temperature_point(&point);
    }
    Ok(temp_grid)
}

pub fn point_buffer(display: &Display, path: impl AsRef<Path>) -> Result<VertexBuffer<Vertex>, Box<Error>> {
    let mut reader = Reader::from_path(path)?;
    let mut points = Vec::new();
    for (i, result) in reader.deserialize().enumerate() {
        if i % 100000 == 0 {
            println!("{}", i);
        }
        let record: CsvRecord = result?;
        if let Some(num) = record.latitude {
            if let Some(num2) = record.longitude {
                let point = Vertex {position: [num, num2], tex_coord: [record.avg_temp, 0.0]};
                points.push(point);
            }
        }
        
    }
    Ok(VertexBuffer::new(display, &points).unwrap())
}

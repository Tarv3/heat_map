use bincode::deserialize_from;
use csv::Reader;
use data::{CSum, CsvRecord, DataPoint, TemperaturePoint, YearlyData};
use grid::*;
use math::{Dimensions, Point, RangeBox};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub type TempMap = HeatMap<YearlyData<f32>>;

pub struct HeatMap<T: Copy> {
    pub grid: Grid<T>,
    pub range: RangeBox<f32>,
}

impl<T: Copy> HeatMap<T> {
    pub fn new(grid: Grid<T>, range: RangeBox<f32>) -> Self {
        Self { grid, range }
    }

    pub fn unit_dims(&self) -> Dimensions<f32> {
        let dims = self.range.dims();
        Dimensions::new(
            dims.x / self.grid.horizontal as f32,
            dims.y / self.grid.vertical as f32,
        )
    }

    pub fn point_in_map(&self, point: Point<f32>) -> bool {
        self.range.contains(point)
    }

    fn point_to_grid_index(&self, point: Point<f32>) -> Option<[usize; 2]> {
        if self.point_in_map(point) {
            let unit_dims = self.unit_dims();
            let x_offset = point.x - self.range.horizontal.from;
            let y_offset = point.y - self.range.vertical.from;
            let index = [
                (x_offset / unit_dims.x).round() as usize,
                (y_offset / unit_dims.y).round() as usize,
            ];
            if index[0] + index[1] * self.grid.horizontal >= self.grid.values.len() {
                return None;
            }
            return Some(index);
        }
        None
    }

    pub fn add_data<S: Copy, U>(&mut self, datapoint: &DataPoint<S>, add_func: U)
    where
        U: Fn(&mut Grid<T>, [usize; 2], &S),
    {
        if let Some(index) = self.point_to_grid_index(datapoint.position) {
            add_func(&mut self.grid, index, &datapoint.data);
        }
    }

    pub fn add_data_points<S: Copy, U>(&mut self, datapoints: &[DataPoint<S>], add_func: U)
    where
        U: Fn(&mut Grid<T>, [usize; 2], &S),
    {
        for point in datapoints {
            self.add_data(point, &add_func);
        }
    }
}

impl<T: Copy + PartialEq + PartialOrd> HeatMap<T> {
    pub fn value_range(&self) -> [Option<T>; 2] {
        let min = self.grid.min();
        let max = self.grid.max();
        [min, max]
    }
}

impl HeatMap<YearlyData<f32>> {
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
        self.into_grid_with(|yearly_temp| yearly_temp.yearly_average())
    }

    pub fn variance_grid(&self) -> Grid<Option<f32>> {
        self.into_grid_with(|yearly_temp| yearly_temp.variance())
    }

    pub fn standard_dev_grid(&self) -> Grid<Option<f32>> {
        self.into_grid_with(|yearly_temp| yearly_temp.standard_dev())
    }

    pub fn range_grid(&self) -> Grid<Option<f32>> {
        self.into_grid_with(|yearly_temp| yearly_temp.range())
    }

    pub fn into_option_grid(&self) -> Grid<Option<YearlyData<f32>>> {
        self.grid.into_grid_with(|yearly_data| {
            if yearly_data.none_count() == 0 {
                return Some(*yearly_data);
            }
            None
        })
    }

    pub fn temp_heat_map_from_csv(
        dimensions: (usize, usize),
        range: RangeBox<f32>,
        path: impl AsRef<Path>,
    ) -> Result<Self, Box<Error>> {
        let mut reader = Reader::from_path(path)?;
        let mut temp_grid = Self::new_temperature_grid(dimensions, range);

        for (i, result) in reader.deserialize().enumerate() {
            if i % 1000000 == 0 {
                println!("{}", i);
            }
            let record: CsvRecord = result?;
            let point = match TemperaturePoint::from(record) {
                Some(point) => point,
                None => {
                    continue;
                }
            };
            temp_grid.add_temperature_point(&point);
        }
        Ok(temp_grid)
    }

    pub fn temp_heat_map_from_bin(
        dimensions: (usize, usize),
        range: RangeBox<f32>,
        path: impl AsRef<Path>,
    ) -> Result<Self, Box<Error>> {
        let file = BufReader::new(File::open(path)?);
        let values: Vec<TemperaturePoint> = deserialize_from(file)?;
        let mut temp_grid = Self::new_temperature_grid(dimensions, range);

        for result in values {
            temp_grid.add_temperature_point(&result);
        }
        Ok(temp_grid)
    }

    pub fn into_grid_with<U: Fn(&YearlyData<f32>) -> Option<f32>>(
        &self,
        func: U,
    ) -> Grid<Option<f32>> {
        self.grid.into_grid_with(func)
    }
}

impl HeatMap<CSum<f32>> {
    pub fn elevation_map_from_bin(
        dimensions: (usize, usize),
        range: RangeBox<f32>,
        path: impl AsRef<Path>,
    ) -> Result<Self, Box<Error>> {
        let file = BufReader::new(File::open(path)?);
        let values: Vec<DataPoint<f32>> = deserialize_from(file)?;
        let grid = Grid::new(dimensions.0, dimensions.1, CSum::new());
        let mut grid = HeatMap::new(grid, range);

        grid.add_data_points(&values, |grid, index, &value| {
            grid[index].add(value);
        });
        Ok(grid)
    }

    pub fn elevation_grid(&self) -> Grid<Option<f32>> {
        self.grid.into_grid_with(|&sum| sum.average())
    }
}

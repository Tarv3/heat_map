use bincode::deserialize_from;
use csv::Reader;
use data::{CsvRecord, DataPoint, TemperaturePoint, YearlyData};
use glium::backend::glutin::Display;
use glium::texture::{texture2d::Texture2d, RawImage2d};
use math::{Dimensions, Point, Range, RangeBox, RectIter};
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::ops::{Index, IndexMut};
use std::path::Path;

// Top left value will be stored first and the data will be separated by each horizontal layer
// Eg. [1 2 3 4 5] = [1 2 3 4 5 1 2 3 4 5]
//     [1 2 3 4 5]
#[derive(Clone)]
pub struct Grid<T: Copy> {
    // Ratio in terms of vertical / horizontal
    pub horizontal: usize,
    pub vertical: usize,
    pub values: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn new(horizontal: usize, vertical: usize, default_value: T) -> Self {
        Self {
            horizontal,
            vertical,
            values: vec![default_value; (vertical) * (horizontal)],
        }
    }

    pub fn new_from_values(horizontal: usize, vertical: usize, values: Vec<T>) -> Self {
        Self {
            horizontal,
            vertical,
            values,
        }
    }

    pub fn values_ref(&self) -> &[T] {
        &self.values
    }

    pub fn index_to_position(&self, index: usize) -> [usize; 2] {
        [index % self.horizontal, index / self.horizontal]
    }

    pub fn position_to_index(&self, position: [usize; 2]) -> usize {
        position[0] + position[1] * self.horizontal
    }

    pub fn checked_index(&self, position: [i32; 2]) -> Option<T> {
        if position[0] < self.horizontal as i32
            && position[0] >= 0
            && position[1] < self.vertical as i32
            && position[1] >= 0
        {
            return Some(self[[position[0] as usize, position[1] as usize]]);
        } else {
            return None;
        }
    }
}

impl<T: Copy + PartialEq + PartialOrd> Grid<T> {
    pub fn max(&self) -> Option<T> {
        if self.values.is_empty() {
            return None;
        }
        let mut max = None;
        for &item in &self.values {
            let current_max = max;
            match current_max {
                Some(value) => {
                    if value < item {
                        max = Some(item);
                    }
                }
                None => max = Some(item),
            }
        }
        max
    }
    pub fn min(&self) -> Option<T> {
        if self.values.is_empty() {
            return None;
        }
        let mut min = None;
        for &item in &self.values {
            let current_min = min;
            match current_min {
                Some(value) => {
                    if value > item {
                        min = Some(item);
                    }
                }
                None => min = Some(item),
            }
        }
        min
    }
}

impl Grid<Option<f32>> {
    pub fn max_option(&self) -> Option<f32> {
        let mut max = None;
        for &temp in &self.values {
            let current_max = max;
            if let Some(value) = temp {
                match current_max {
                    Some(current) => if current < value {
                        max = Some(value)
                    },
                    None => max = Some(value),
                }
            }
        }
        max
    }

    pub fn min_option(&self) -> Option<f32> {
        let mut min = None;
        for &temp in &self.values {
            if let Some(value) = temp {
                let current_min = min;
                match current_min {
                    Some(current) => if current > value {
                        min = Some(value)
                    },
                    None => min = Some(value),
                }
            }
        }
        min
    }

    pub fn print(&self) {
        for item in &self.values {
            println!("{:?}", item);
        }
    }

    pub fn into_texture(
        &self,
        display: &Display,
        range: Option<Range<f32>>,
    ) -> (Texture2d, Range<f32>) {
        let max;
        let min;
        match range {
            Some(rg) => {
                max = rg.to;
                min = rg.from;
            }
            None => {
                max = self.max_option().expect("No maximum value");
                min = self.min_option().expect("No minimum value");
            }
        }
        let mut rgb = Vec::with_capacity(self.values.len() * 3);
        for y in 0..self.vertical {
            for x in 0..self.horizontal {
                let to_push = match self[[x, y]] {
                    Some(temp) => ((temp - min) / (max - min) + 1.0) * 0.5,
                    None => 0.0,
                };
                rgb.push(to_push);
                rgb.push(to_push);
                rgb.push(to_push);
            }
        }
        (
            Texture2d::new(
                display,
                RawImage2d::from_raw_rgb(rgb, ((self.horizontal) as u32, (self.vertical) as u32)),
            ).expect("Failed to create texture"),
            Range::new(min, max),
        )
    }

    pub fn into_texture_with_function<U>(&self, display: &Display, func: U) -> Texture2d
    where
        U: Fn(&Grid<Option<f32>>, [usize; 2]) -> f32,
    {
        let mut rgb = Vec::with_capacity(self.values.len() * 3);
        for y in 0..self.vertical {
            for x in 0..self.horizontal {
                let to_push = func(&self, [x, y]);
                rgb.push(to_push);
                rgb.push(to_push);
                rgb.push(to_push);
            }
        }
        Texture2d::new(
            display,
            RawImage2d::from_raw_rgb(rgb, ((self.horizontal) as u32, (self.vertical) as u32)),
        ).expect("Failed to create texture")
    }

    pub fn find_closest_to(&self, index: usize, max_rad: i32) -> Option<f32> {
        let mut value = None;
        let mut smallest_dist = None;
        let mut found = false;

        let position = self.index_to_position(index);
        let position = [position[0] as i32, position[1] as i32];

        for i in 1..max_rad {
            for point in RectIter::new(position, i) {
                let option = match self.checked_index([point[0] as i32, point[1] as i32]) {
                    Some(value) => value,
                    None => continue,
                };

                if let Some(num) = option {
                    found = true;
                    let smallest = smallest_dist;
                    let current_dist = point[0] * point[0] + point[1] * point[1];

                    match smallest {
                        Some(dist) => {
                            if dist > current_dist {
                                smallest_dist = Some(current_dist);
                                value = Some(num);
                            }
                        }
                        None => {
                            smallest_dist = Some(current_dist);
                            value = Some(num);
                        }
                    }
                }
            }
            if found {
                break;
            }
        }

        return value;
    }

    pub fn fill_values_nearest(mut self) -> Grid<Option<f32>> {
        let mut vec = Vec::with_capacity(self.values.len());
        for (index, value) in self.values.iter().enumerate() {
            vec.push((index, *value));
        }

        vec.par_iter_mut()
            .for_each(|(index, value)| match self.values[*index] {
                Some(_) => (),
                None => {
                    *value = self.find_closest_to(*index, 100);
                }
            });

        let mut grid_vec = Vec::with_capacity(self.values.len());
        for (_, value) in vec {
            grid_vec.push(value);
        }
        self.values = grid_vec;
        self
    }

    pub fn compare_to(&self, other: &Grid<Option<f32>>) -> Vec<(f32, f32)> {
        let x_ratio = other.horizontal as f32 / self.horizontal as f32;
        let y_ratio = other.vertical as f32 / self.vertical as f32;
        let mut values = Vec::new();

        for (i, &value) in self.values.iter().enumerate() {
            if let Some(inner) = value {
                let position = self.index_to_position(i);
                let x_index = (position[0] as f32 * x_ratio).round() as i32;
                let y_index = (position[1] as f32 * y_ratio).round() as i32;

                if let Some(Some(inner2)) = other.checked_index([x_index, y_index]) {
                    values.push((inner, inner2));
                }
            }
        }
        return values;
    }
}

// Index is [x, y]
impl<T: Copy> Index<[usize; 2]> for Grid<T> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &T {
        let index = index[0] + index[1] * self.horizontal;
        return &self.values[index];
    }
}

impl<T: Copy> IndexMut<[usize; 2]> for Grid<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut T {
        let actual_index = index[0] + index[1] * self.horizontal;
        return &mut self.values[actual_index];
    }
}

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

    pub fn into_grid_with<U: Fn(&T) -> Option<f32>>(&self, func: U) -> Grid<Option<f32>> {
        let mut grid_values = Vec::with_capacity(self.grid.values.len());
        for value in self.grid.values_ref() {
            grid_values.push(func(value));
        }
        Grid::new_from_values(self.grid.horizontal, self.grid.vertical, grid_values)
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
        self.into_grid_with(|yearly_temp| {
            yearly_temp.yearly_average()
        })
    }

    pub fn variance_grid(&self) -> Grid<Option<f32>> {
        self.into_grid_with(|yearly_temp| {
            yearly_temp.variance()
        })
    }

    pub fn standard_dev_grid(&self) -> Grid<Option<f32>> {
        self.into_grid_with(|yearly_temp| {
            yearly_temp.standard_dev()
        })
    }

    pub fn range_grid(&self) -> Grid<Option<f32>> {
        self.into_grid_with(|yearly_temp| {
            yearly_temp.range()
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

        for (i, result) in values.iter().enumerate() {
            if i % 1000000 == 0 {
                println!("{}", i);
            }
            temp_grid.add_temperature_point(&result);
        }
        Ok(temp_grid)
    }
}

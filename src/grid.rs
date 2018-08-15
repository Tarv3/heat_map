use data::DataPoint;
use glium::backend::glutin::Display;
use glium::texture::{texture2d::Texture2d, RawImage2d};
use math::{Dimensions, Point, RangeBox};
use std::ops::{Index, IndexMut};

// Top left value will be stored first and the data will be separated by each horizontal layer
// Eg. [1 2 3 4 5] = [1 2 3 4 5 1 2 3 4 5]
//     [1 2 3 4 5]
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
            values: vec![default_value; (vertical + 1) * (horizontal + 1)],
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
    pub fn into_texture(&self, display: &Display) -> Texture2d {
        let max = self.max_option().unwrap();
        let min = self.min_option().unwrap();
        let mut rgb = Vec::with_capacity(self.values.len() * 3);
        for y in 0..self.vertical + 1 {
            for x in 0..self.horizontal + 1 {
                let to_push = match self[[x, y]] {
                    Some(temp) => ((temp - min) / (max - min) + 1.0) * 0.5 ,
                    None => 0.0,
                };
                rgb.push(to_push);
                rgb.push(to_push);
                rgb.push(to_push);
            }
        }
        Texture2d::new(
            display,
            RawImage2d::from_raw_rgb(
                rgb,
                ((self.horizontal + 1) as u32, (self.vertical + 1) as u32),
            ),
        ).expect("Failed to create texture")
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

use bincode::{deserialize_from, serialize_into};
use data::CSum;
use glium::backend::glutin::Display;
use glium::texture::{texture2d::Texture2d, RawImage2d};
use math::{Range, RectIter};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::ops::{Index, IndexMut};
use std::path::Path;

// Top left value will be stored first and the data will be separated by each horizontal layer
// Eg. [1 2 3 4 5] = [1 2 3 4 5 1 2 3 4 5]
//     [1 2 3 4 5]
#[derive(Clone, Debug, Serialize, Deserialize)]
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

    pub fn into_grid_with<V: Copy, U: Fn(&T) -> Option<V>>(&self, func: U) -> Grid<Option<V>> {
        let mut grid_values = Vec::with_capacity(self.values.len());
        for value in self.values_ref() {
            grid_values.push(func(value));
        }
        Grid::new_from_values(self.horizontal, self.vertical, grid_values)
    }
}

impl<T: Copy + Serialize> Grid<T>
where
    for<'de> T: Deserialize<'de>,
{
    pub fn save_to_bin(&self, path: impl AsRef<Path>) -> Result<(), Box<Error>> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serialize_into(writer, &self)?;
        Ok(())
    }

    pub fn load_from_bin(path: impl AsRef<Path>) -> Result<Self, Box<Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let grid = deserialize_from(reader)?;
        Ok(grid)
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

impl<T: Copy + Send + Sync> Grid<Option<T>> {
    pub fn find_closest_to(&self, index: usize, max_rad: i32) -> Option<T> {
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

    pub fn fill_values_nearest(mut self) -> Grid<Option<T>> {
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

    pub fn find_all_within(&self, index: usize, max_rad: i32) -> Vec<f32> {
        let position = self.index_to_position(index);
        let position = [position[0] as i32, position[1] as i32];
        let mut closest_points = Vec::new();
        for i in 1..max_rad {
            for point in RectIter::new(position, i) {
                match self.checked_index([point[0] as i32, point[1] as i32]) {
                    Some(value) => if let Some(value) = value {
                        closest_points.push(value);
                    },
                    None => continue,
                };
            }
        }

        return closest_points;
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

    pub fn into_range_grid(&self, radius: i32) -> Grid<Option<f32>> {
        let mut vec = Vec::with_capacity(self.values.len());
        for i in 0..self.values.len() {
            vec.push((i, None));
        }

        vec.par_iter_mut().for_each(|(index, range)| {
            let mut min = self.values[*index];
            let mut max = self.values[*index];
            let position = self.index_to_position(*index);
            let position = [position[0] as i32, position[1] as i32];
            for rad in 1..radius {
                for value in RectIter::new(position, rad) {
                    if let Some(op) = self.checked_index([value[0] as i32, value[1] as i32]) {
                        if let Some(num) = op {
                            let current_min = min;
                            let current_max = max;
                            match current_min {
                                Some(val) => if val > num {
                                    min = Some(num);
                                },
                                None => min = Some(num),
                            }
                            match current_max {
                                Some(val) => if val < num {
                                    max = Some(num);
                                },
                                None => max = Some(num),
                            }
                        }
                    }
                }
            }
            if let Some(num1) = min {
                if let Some(num2) = max {
                    *range = Some(num2 - num1);
                }
            }
        });

        let mut grid_vec = Vec::with_capacity(self.values.len());
        for index in 0..vec.len() {
            grid_vec.push(vec[index].1);
        }
        Grid::new_from_values(self.horizontal, self.vertical, grid_vec)
    }

    pub fn range_grid_from_closest(&self, radius: i32) -> Grid<Option<f32>> {
        let mut vec = Vec::with_capacity(self.values.len());
        for i in 0..self.values.len() {
            vec.push((i, None));
        }

        vec.par_iter_mut().for_each(|(index, range)| {
            let current = self.values[*index];

            if current == None {
                return;
            }

            let closest_values = self.find_all_within(*index, radius);
            let mut csum = CSum::new();

            for &value in &closest_values {
                csum.add(value);
            }

            let average = match csum.average() {
                Some(avg) => avg,
                None => return,
            };

            let mut variance: Option<f32> = None;

            for value in closest_values {
                let current = variance;
                variance = match current {
                    Some(num) => Some(num + (value - average).powi(2) / csum.count as f32),
                    None => Some((value - average).powi(2) / csum.count as f32),
                }
            }

            if let Some(num) = variance {
                *range = Some(num.sqrt());
            }
        });

        let mut grid_vec = Vec::with_capacity(self.values.len());
        for index in 0..vec.len() {
            grid_vec.push(vec[index].1);
        }
        Grid::new_from_values(self.horizontal, self.vertical, grid_vec)
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

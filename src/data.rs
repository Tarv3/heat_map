use math::Point;
use std::ops::{Add, AddAssign, Div};

#[derive(Copy, Clone, Debug)]
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

impl<T: Copy + Add<T, Output = T> + AddAssign + Div<f32, Output = T>> YearlyData<T> {
    pub fn new() -> Self {
        Self {
            montly_data: [CSum::new(); 12],
        }
    }

    // Month has to be between 1 - 12 inclusive
    pub fn add_to(&mut self, data: T, month: usize) {
        self.montly_data[month - 1].add(data);
    }
    
    pub fn yearly_average(&self) -> Option<T> {
        let mut none_count = 0;
        let mut average = None;
        for month in  self.montly_data.iter() {
            match month.average() {
                Some(number) => match average {
                    Some(ref mut value) => *value += number,
                    None => average = Some(number)
                },
                None => none_count += 1
            }
        }
        match average {
            Some(value) => Some(value / (12 - none_count) as f32),
            None => None
        }
    }
}

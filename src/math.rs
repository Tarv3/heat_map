use std::iter::Iterator;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::fmt::Debug;

pub trait Num:
    Sized
    + Copy
    + PartialEq
    + PartialOrd
    + Div<Self, Output = Self>
    + DivAssign
    + Mul<Self, Output = Self>
    + MulAssign
    + Add<Self, Output = Self>
    + AddAssign
    + Sub<Self, Output = Self>
    + SubAssign
    + Debug
{
}
impl<T> Num for T
where
    T: Sized
        + Copy
        + PartialEq
        + PartialOrd
        + Div<Self, Output = Self>
        + DivAssign
        + Mul<Self, Output = Self>
        + MulAssign
        + Add<Self, Output = Self>
        + AddAssign
        + Sub<Self, Output = Self>
        + SubAssign
        + Debug
{
}

#[derive(Debug, Clone, Copy)]
pub struct Range<T: Num> {
    pub from: T,
    pub to: T,
}

impl<T: Num> Range<T> {
    pub fn new(from: T, to: T) -> Self {
        Self { from, to }
    }
    pub fn length(&self) -> T {
        self.to - self.from
    }
    pub fn contains(&self, test: T) -> bool {
        self.from < test && self.to > test
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Point<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub struct RangeBox<T: Num> {
    pub horizontal: Range<T>,
    pub vertical: Range<T>,
}

impl<T: Num> RangeBox<T> {
    pub fn new(horizontal: Range<T>, vertical: Range<T>) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
    pub fn dims(&self) -> Dimensions<T> {
        Dimensions::new(self.horizontal.length(), self.vertical.length())
    }
    pub fn contains(&self, point: Point<T>) -> bool {
        self.horizontal.contains(point.x) && self.vertical.contains(point.y)
    }
}

pub struct Dimensions<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Dimensions<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub struct RectIter {
    bottom_left: [i32; 2],
    radius: i32,
    index: i32,
}

impl RectIter {
    pub fn new(centre: [i32; 2], radius: i32) -> Self {
        Self {
            bottom_left: [centre[0] - radius, centre[1] - radius],
            radius,
            index: 0,
        }
    }
}

impl Iterator for RectIter {
    type Item = [i32; 2];

    fn next(&mut self) -> Option<[i32; 2]> {
        if self.index >= 8 * self.radius {
            return None;
        }

        let index = self.index;
        self.index += 1;
        let bottom = index - (self.radius * 2 + 1);

        if bottom < 0 {
            return Some([self.bottom_left[0] + index, self.bottom_left[1]]);
        } else {
            let top = index - (6 * self.radius - 1);
            if top >= 0 {
                return Some([
                    self.bottom_left[0] + top,
                    self.bottom_left[1] + self.radius * 2,
                ]);
            } else {
                return Some([
                    self.bottom_left[0] + 2 * (bottom % 2) * self.radius,
                    self.bottom_left[1] + bottom / 2 + 1,
                ]);
            }
        }
    }
}

pub fn clamp(input: f32, min: f32, max: f32) -> f32 {
    if input < min {
        min
    }
    else if input > max {
        max
    }
    else {
        input
    }
}
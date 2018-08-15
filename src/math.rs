use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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
        + SubAssign,
{
}

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

#[derive(Copy, Clone, Debug)]
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

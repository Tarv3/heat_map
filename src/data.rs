use math::Point;

pub struct DataPoint<T> {
    pub position: Point<f32>,
    pub data: T,
}

impl<T> DataPoint<T> {
    pub fn new(position: Point<f32>, data: T) -> Self {
        Self { position, data }
    }
}

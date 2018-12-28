use std::ops::{Sub, Add, Mul};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Vector2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2d<T> {
    pub fn new(x: T, y: T) -> Vector2d<T> {
        Vector2d { x, y }
    }
}

impl<T: Add<Output=T>> Add for Vector2d<T> {

    type Output = Vector2d<T>;

    fn add(self, other: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output=T>> Sub for Vector2d<T> {

    type Output = Vector2d<T>;

    fn sub(self, other: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Mul<usize, Output=T>> Mul<usize> for Vector2d<T> {

    type Output = Vector2d<T>;
    
    fn mul(self, rhs: usize) -> Vector2d<T> {
        Vector2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}


macro_rules! assert_eq_delta (
    ($lhs:expr, $rhs:expr, $delta:expr) => {
        let diff = $lhs - $rhs;
        if diff.abs() > $delta {
            panic!("{:?} and {:?} not equal (delta {:?})", $lhs, $rhs, $delta);
        }
    }
);


#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn add_int_vector() {
        let v1: Vector2d<i32> = Vector2d::new(5, 1); 
        let v2: Vector2d<i32> = Vector2d::new(-3, 4);

        let added = v1 + v2;
        assert_eq!(added.x, 2);
        assert_eq!(added.y, 5);
    }

    #[test]
    fn add_float_vector() {
        let v1: Vector2d<f32> = Vector2d::new(5.2, 1.0); 
        let v2: Vector2d<f32> = Vector2d::new(-3.0, 4.1);

        let added = v1 + v2;
        assert_eq_delta!(added.x, 2.2, 0.01);
        assert_eq_delta!(added.y, 5.1, 0.01);
    }

    #[test]
    fn mul_int_vector() {
        let v = Vector2d::new(2, 4);
        let mult = v * 2;
        assert_eq!(mult.x, 4);
        assert_eq!(mult.y, 8);
    }
}

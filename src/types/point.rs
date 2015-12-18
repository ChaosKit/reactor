use std::fmt;
use std::io::Write;
use std::ops::Mul;
use types::Number;
use rand::{Rand, Rng};
use rand::distributions::{IndependentSample, Range};

#[derive(Copy, Clone)]
pub struct Point {
    pub x: Number,
    pub y: Number
}

impl Point {
    pub fn new() -> Point {
        Point {x: 0.0, y: 0.0}
    }

    pub fn from_xy(x: Number, y: Number) -> Point {
        Point {x: x, y: y}
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Mul<Number> for Point {
    type Output = Point;

    fn mul(self, rhs: Number) -> Point {
        Point { x: self.x * rhs, y: self.y * rhs}
    }
}

impl Mul<(Number, Number)> for Point {
    type Output = Point;

    fn mul(self, rhs: (Number, Number)) -> Point {
        let (wx, wy) = rhs;
        Point { x: self.x * wx, y: self.y * wy }
    }
}

impl Rand for Point {
    fn rand<R: Rng>(rng: &mut R) -> Point {
        let range: Range<Number> = Range::new(-1.0, 1.0);
        Point::from_xy(range.ind_sample(rng), range.ind_sample(rng))
    }
}

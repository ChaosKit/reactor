use std::fmt;
use std::io::Write;
use std::ops::Mul;
use types::Number;
use rand::{Rand, Rng};
use rand::distributions::{IndependentSample, Range};
use byteorder::{ByteOrder, BigEndian};

pub struct Point {
    pub x: Number,
    pub y: Number,
    pub z: Number
}

impl Point {
    pub fn new() -> Point {
        Point {x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn from_xy(x: Number, y: Number) -> Point {
        Point {x: x, y: y, z: 0.0}
    }

    pub fn bytes(&self) -> [u8; 8*3] {
        let mut buffer: [u8; 8*3] = [0; 8 * 3];

        BigEndian::write_f64(&mut buffer[8*0..8*1], self.x as f64);
        BigEndian::write_f64(&mut buffer[8*1..8*2], self.y as f64);
        BigEndian::write_f64(&mut buffer[8*2..8*3], self.z as f64);

        buffer
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Mul<Number> for Point {
    type Output = Point;

    fn mul(self, rhs: Number) -> Point {
        Point { x: self.x * rhs, y: self.y * rhs, z: self.z }
    }
}

impl Rand for Point {
    fn rand<R: Rng>(rng: &mut R) -> Point {
        let range: Range<Number> = Range::new(-1.0, 1.0);
        Point::from_xy(range.ind_sample(rng), range.ind_sample(rng))
    }
}

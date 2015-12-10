use std::fmt;
use types::{Number, Point};
use byteorder::{ByteOrder, BigEndian};

pub struct Particle {
    pub point: Point,
    pub color: Number
}

impl Particle {
    pub fn new() -> Particle {
        Particle {point: Point::new(), color: 0.0}
    }

    pub fn from_point(point: Point) -> Particle {
        Particle {point: point, color: 0.0}
    }

    pub fn bytes(&self) -> [u8; 8*3] {
        let mut buffer: [u8; 8*3] = [0; 8 * 3];

        BigEndian::write_f64(&mut buffer[8*0..8*1], self.point.x as f64);
        BigEndian::write_f64(&mut buffer[8*1..8*2], self.point.y as f64);
        BigEndian::write_f64(&mut buffer[8*2..8*3], self.color as f64);

        buffer
    }
}

impl fmt::Display for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Particle {}, color: {}", self.point, self.color)
    }
}

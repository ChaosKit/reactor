use std::fmt;
use types::{Number, Point};
use byteorder::{ByteOrder, BigEndian};

#[derive(Copy, Clone)]
pub struct Particle {
    pub point: Point,
    pub color: Number,
    pub ttl: i32
}

impl Particle {
    pub fn new() -> Particle {
        Particle {
            point: Point::new(),
            color: 0.5,
            ttl: 1
        }
    }

    pub fn bytes(&self) -> [u8; 8*3] {
        let mut buffer: [u8; 8*3] = [0; 8 * 3];

        BigEndian::write_f64(&mut buffer[8*0..8*1], self.point.x as f64);
        BigEndian::write_f64(&mut buffer[8*1..8*2], self.point.y as f64);
        BigEndian::write_f64(&mut buffer[8*2..8*3], self.color as f64);

        buffer
    }

    pub fn age(&mut self) -> &mut Particle {
        self.ttl -= 1;
        self
    }
}

impl fmt::Debug for Particle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Particle {:?}, color: {:?}, ttl: {:?}", self.point, self.color, self.ttl)
    }
}

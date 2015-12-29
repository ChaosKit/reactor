use types::{Number, Particle, Point};

pub trait ColoringMethod: Sync + Send + ::std::fmt::Debug {
    fn color(&self, old: &Particle, new: &Point) -> Number;
}

#[derive(Debug)]
pub struct Noop;

impl ColoringMethod for Noop {
    fn color(&self, old: &Particle, _new: &Point) -> Number {
        old.color
    }
}

#[derive(Debug)]
pub struct Distance;

impl ColoringMethod for Distance {
    fn color(&self, old: &Particle, new: &Point) -> Number {
        let dx = new.x - old.point.x;
        let dy = new.y - old.point.y;

        (dx * dx + dy * dy).sqrt().max(0.0).min(1.0)
    }
}

#[derive(Debug)]
pub struct SingleColor {
    color: Number
}

impl SingleColor {
    pub fn new(color: Number) -> SingleColor {
        SingleColor { color: color }
    }
}

impl ColoringMethod for SingleColor {
    fn color(&self, old: &Particle, _new: &Point) -> Number {
        (self.color + old.color) / 2.0
    }
}

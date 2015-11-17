use std::boxed::Box;
use types::{Number, Point, Applicable};

pub struct Variation {
    pub weight: Number,
    pub applicable: Box<Applicable>
}

impl Applicable for Variation {
    fn apply(&self, point: &Point) -> Point {
        self.applicable.apply(point) * self.weight
    }
}

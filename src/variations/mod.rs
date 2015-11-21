use types::{Point, Number, Variation, AffineTransformation};

pub struct Linear;

impl Variation for Linear {
    fn apply(&self, point: &Point, _: &AffineTransformation) -> Point {
        Point::from_xy(point.x, point.y)
    }
}

pub struct DeJong(pub Number, pub Number, pub Number, pub Number);

impl Variation for DeJong {
    fn apply(&self, point: &Point, _: &AffineTransformation) -> Point {
        Point::from_xy(
            (self.0 * point.x).sin() - (self.1 * point.y).cos(),
            (self.2 * point.y).sin() - (self.3 * point.x).cos()
        )
    }
}

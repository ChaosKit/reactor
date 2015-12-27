use types::{Point, Number, Variation, AffineTransformation};

pub fn make_variation(name: &str, params: &[Number]) -> Result<Box<Variation>, &'static str> {
    match name {
        "Linear" => Ok(Box::new(Linear)),
        "DeJong" => {
            if params.len() < 4 { Err("Not enough parameters") }
            else { Ok(Box::new(DeJong(params[0], params[1], params[2], params[3]))) }
        },
        _ => Err("Invalid variation")
    }
}

#[derive(Debug)]
pub struct Linear;

impl Variation for Linear {
    fn apply(&self, point: &Point, _: &AffineTransformation) -> Point {
        Point::from_xy(point.x, point.y)
    }
}

#[derive(Debug)]
pub struct DeJong(pub Number, pub Number, pub Number, pub Number);

impl Variation for DeJong {
    fn apply(&self, point: &Point, _: &AffineTransformation) -> Point {
        Point::from_xy(
            (self.0 * point.x).sin() - (self.1 * point.y).cos(),
            (self.2 * point.y).sin() - (self.3 * point.x).cos()
        )
    }
}

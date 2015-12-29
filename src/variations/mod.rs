use types::{Point, Number, Variation, AffineTransformation};

pub fn make_variation(name: &str, params: &[Number]) -> Result<Box<Variation>, &'static str> {
    match name {
        "Linear" => Ok(Box::new(Linear)),
        "DeJong" => {
            if params.len() < 4 { Err("Not enough parameters") }
            else { Ok(Box::new(DeJong(params[0], params[1], params[2], params[3]))) }
        },
        "Trigonometric" => {
            if params.len() < 8 { Err("Not enough parameters") }
            else { Ok(Box::new(Trigonometric(params[0], params[1], params[2], params[3], params[4], params[5], params[6], params[7]))) }
        },
        "Unnamed" => {
            if params.len() < 4 { Err("Not enough parameters") }
            else { Ok(Box::new(Unnamed(params[0], params[1], params[2], params[3]))) }
        }
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
            (self.0 * point.y).sin() - (self.1 * point.x).cos(),
            (self.2 * point.x).sin() - (self.3 * point.y).cos()
        )
    }
}

#[derive(Debug)]
pub struct Trigonometric(pub Number, pub Number, pub Number, pub Number, pub Number, pub Number, pub Number, pub Number);

impl Variation for Trigonometric {
    fn apply(&self, point: &Point, _: &AffineTransformation) -> Point {
        Point::from_xy(
            self.0 * (self.1 * point.y).sin() + self.2 * (self.3 * point.x).cos(),
            self.4 * (self.5 * point.x).sin() + self.6 * (self.7 * point.y).cos()
        )
    }
}

#[derive(Debug)]
pub struct Unnamed(pub Number, pub Number, pub Number, pub Number);

impl Variation for Unnamed {
    fn apply(&self, point: &Point, _: &AffineTransformation) -> Point {
        Point::from_xy(
            point.y + self.0 * point.x.signum() * (self.1 * point.x - self.2).abs().sqrt(),
            self.3 - point.x
        )
    }
}

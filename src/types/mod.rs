pub use self::point::Point;
pub use self::particle::Particle;
pub use self::system::System;
pub use self::affine_transformation::AffineTransformation;
pub use self::transform::Transform;

pub type Number = f64;

pub trait Applicable {
    fn apply(&self, point: &Point) -> Point;
}

pub trait Variation: Send + Sync {
    fn apply(&self, point: &Point, transformation: &AffineTransformation) -> Point;
}

pub enum Message {
    Start(System),
    Stop
}

mod point;
mod particle;
pub mod system;
pub mod affine_transformation;
pub mod transform;

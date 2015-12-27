use types::{Number, Point, Applicable};

#[derive(Copy, Clone, Debug)]
pub struct AffineTransformation(Number, Number, Number, Number, Number, Number);

impl AffineTransformation {
    pub fn identity() -> AffineTransformation {
        AffineTransformationBuilder::new().finalize()
    }

    pub fn rotation(angle: Number) -> AffineTransformation {
        AffineTransformationBuilder::new().rotation(angle).finalize()
    }

    pub fn scale(x: Number, y: Number) -> AffineTransformation {
        AffineTransformationBuilder::new().scale(x, y).finalize()
    }

    pub fn translation(x: Number, y: Number) -> AffineTransformation {
        AffineTransformationBuilder::new().translation(x, y).finalize()
    }
}

impl Applicable for AffineTransformation {
    fn apply(&self, point: &Point) -> Point {
        Point {
            x: self.0 * point.x + self.1 * point.y + self.2,
            y: self.3 * point.x + self.4 * point.y + self.5
        }
    }
}

pub struct AffineTransformationBuilder {
    angle: Number,
    sx: Number,
    sy: Number,
    dx: Number,
    dy: Number
}

impl AffineTransformationBuilder {
    pub fn new() -> AffineTransformationBuilder {
        AffineTransformationBuilder {
            angle: 0.0,
            sx: 1.0,
            sy: 1.0,
            dx: 0.0,
            dy: 0.0
        }
    }

    pub fn rotation(&mut self, angle: Number) -> &mut AffineTransformationBuilder {
        self.angle = angle;
        self
    }

    pub fn scale(&mut self, x: Number, y: Number) -> &mut AffineTransformationBuilder {
        self.sx = x;
        self.sy = y;
        self
    }

    pub fn translation(&mut self, x: Number, y: Number) -> &mut AffineTransformationBuilder {
        self.dx = x;
        self.dy = y;
        self
    }

    pub fn finalize(&self) -> AffineTransformation {
        let sin = self.angle.sin();
        let cos = self.angle.cos();

        AffineTransformation(
            self.sx * cos,
            self.sx * -sin,
            self.sx * (self.dx * cos - self.dy * sin),
            self.sy * sin,
            self.sy * cos,
            self.sy * (self.dx * sin + self.dy * cos)
        )
    }
}

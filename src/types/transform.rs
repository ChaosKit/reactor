use std::boxed::Box;
use types::{Number, Point, Particle, Applicable, Variation, AffineTransformation};
use types::coloring_method::{ColoringMethod, SingleColor};

type WeightedVariation = (Box<Variation>, Number, Number);

#[derive(Debug)]
pub struct Transform {
    pre: AffineTransformation,
    variations: Vec<WeightedVariation>,
    post: AffineTransformation,
    coloring_method: Box<ColoringMethod>
}

impl Transform {
    pub fn animate(&self, particle: &Particle) -> Particle {
        let point = self.apply(&particle.point);
        let color = self.coloring_method.color(&particle, &point);

        Particle {
            point: point,
            color: color,
            ttl: particle.ttl
        }
    }

    pub fn animate_mut<'a>(&'a self, particle: &'a mut Particle) -> &mut Particle {
        let point = self.apply(&particle.point);
        let color = self.coloring_method.color(&particle, &point);

        particle.point = point;
        particle.color = color;
        particle
    }
}

impl Applicable for Transform {
    fn apply(&self, point: &Point) -> Point {
        let initial = self.pre.apply(point);

        let after_variations = if self.variations.is_empty() {
            initial
        } else {
            self.variations.iter()
                .map(|&(ref variation, weight_x, weight_y)| {
                    variation.apply(&initial, &self.pre) * (weight_x, weight_y)
                })
                .fold(Point::new(), |result, p| { result + p })
        };

        self.post.apply(&after_variations)
    }
}

pub struct TransformBuilder {
    pre: AffineTransformation,
    variations: Vec<WeightedVariation>,
    post: AffineTransformation,
    coloring_method: Box<ColoringMethod>
}

impl TransformBuilder {
    pub fn new() -> TransformBuilder {
        TransformBuilder {
            pre: AffineTransformation::identity(),
            post: AffineTransformation::identity(),
            variations: Vec::new(),
            coloring_method: Box::new(SingleColor::new(0.5))
        }
    }

    pub fn pre(mut self, transformation: AffineTransformation) -> TransformBuilder {
        self.pre = transformation;
        self
    }

    pub fn post(mut self, transformation: AffineTransformation) -> TransformBuilder {
        self.post = transformation;
        self
    }

    pub fn add_boxed_variation(self, variation: Box<Variation + 'static>) -> TransformBuilder {
        self.add_boxed_2d_weighted_variation(variation, 1.0, 1.0)
    }

    pub fn add_boxed_2d_weighted_variation(mut self, variation: Box<Variation + 'static>, weight_x: Number, weight_y: Number) -> TransformBuilder {
        self.variations.push((variation, weight_x, weight_y));
        self
    }

    pub fn coloring_method(mut self, coloring_method: Box<ColoringMethod>) -> TransformBuilder {
        self.coloring_method = coloring_method;
        self
    }

    pub fn finalize(self) -> Transform {
        Transform { pre: self.pre, post: self.post, variations: self.variations, coloring_method: self.coloring_method }
    }
}

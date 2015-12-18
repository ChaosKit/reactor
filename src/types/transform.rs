use std::boxed::Box;
use types::{Number, Point, Particle, Applicable, Variation, AffineTransformation};

type WeightedVariation = (Box<Variation>, Number, Number);

pub struct Transform {
    pre: AffineTransformation,
    variations: Vec<WeightedVariation>,
    post: AffineTransformation,
    color: Number
}

impl Transform {
    pub fn from_variation<T: Variation + 'static>(variation: T) -> Transform {
        TransformBuilder::new().add_variation(variation).finalize()
    }

    pub fn color(&self) -> Number {
        return self.color;
    }

    pub fn animate(&self, particle: &Particle) -> Particle {
        Particle {
            point: self.apply(&particle.point),
            color: (self.color + particle.color) / 2.0,
            ttl: particle.ttl
        }
    }

    pub fn animate_mut<'a>(&'a self, particle: &'a mut Particle) -> &mut Particle {
        particle.point = self.apply(&particle.point);
        particle.color = (self.color + particle.color) / 2.0;
        particle
    }
}

impl Applicable for Transform {
    fn apply(&self, point: &Point) -> Point {
        let initial = self.pre.apply(point);
        let after_variations = self.variations.iter().fold(initial, |p, &(ref variation, weight_x, weight_y)| {
            variation.apply(&p, &self.pre) * (weight_x, weight_y)
        });

        self.post.apply(&after_variations)
    }
}

pub struct TransformBuilder {
    pre: AffineTransformation,
    variations: Vec<WeightedVariation>,
    post: AffineTransformation,
    color: Number
}

impl TransformBuilder {
    pub fn new() -> TransformBuilder {
        TransformBuilder {
            pre: AffineTransformation::identity(),
            post: AffineTransformation::identity(),
            variations: Vec::new(),
            color: 0.5
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

    pub fn add_variation<T: Variation + 'static>(self, variation: T) -> TransformBuilder {
        self.add_2d_weighted_variation(variation, 1.0, 1.0)
    }

    pub fn add_weighted_variation<T: Variation + 'static>(self, variation: T, weight: Number) -> TransformBuilder {
        self.add_2d_weighted_variation(variation, weight, weight)
    }

    pub fn add_2d_weighted_variation<T: Variation + 'static>(mut self, variation: T, weight_x: Number, weight_y: Number) -> TransformBuilder {
        self.variations.push((Box::new(variation), weight_x, weight_y));
        self
    }

    pub fn color(mut self, color: Number) -> TransformBuilder {
        self.color = color;
        self
    }

    pub fn finalize(self) -> Transform {
        Transform { pre: self.pre, post: self.post, variations: self.variations, color: self.color }
    }
}

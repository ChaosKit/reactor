use std::boxed::Box;
use types::{Number, Point, Applicable, AffineTransformation};

type Variation = (Box<Applicable>, Number);

pub struct Transform {
    pre: AffineTransformation,
    variations: Vec<Variation>,
    post: AffineTransformation
}

impl Transform {
    pub fn from_applicable<T: Applicable + 'static>(applicable: T) -> Transform {
        let mut builder = TransformBuilder::new();
        builder.add_variation(applicable);
        builder.finalize()
    }
}

impl Applicable for Transform {
    fn apply(&self, point: &Point) -> Point {
        let initial = self.pre.apply(point);
        let after_variations = self.variations.iter().fold(initial, |p, &(ref applicable, weight)| applicable.apply(&p) * weight);

        self.post.apply(&after_variations)
    }
}

pub struct TransformBuilder {
    pre: AffineTransformation,
    variations: Vec<Variation>,
    post: AffineTransformation
}

impl TransformBuilder {
    pub fn new() -> TransformBuilder {
        TransformBuilder {
            pre: AffineTransformation::identity(),
            post: AffineTransformation::identity(),
            variations: Vec::new()
        }
    }

    pub fn pre(&mut self, transformation: AffineTransformation) -> &mut TransformBuilder {
        self.pre = transformation;
        self
    }

    pub fn post(&mut self, transformation: AffineTransformation) -> &mut TransformBuilder {
        self.post = transformation;
        self
    }

    pub fn add_variation<T: Applicable + 'static>(&mut self, applicable: T) -> &mut TransformBuilder {
        self.add_weighted_variation(applicable, 1.0)
    }

    pub fn add_weighted_variation<T: Applicable + 'static>(&mut self, applicable: T, weight: Number) -> &mut TransformBuilder {
        self.variations.push((Box::new(applicable), weight));

        self
    }

    pub fn finalize(self) -> Transform {
        Transform { pre: self.pre, post: self.post, variations: self.variations }
    }
}

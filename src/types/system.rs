use types::{Particle, Point, Number, Transform, Applicable, AffineTransformation};
use types::transform::TransformBuilder;
use rand::{random, Rng};
use rand::distributions::{IndependentSample, Range};

#[derive(Debug)]
struct TransformWithLimit {
    transform: Transform,
    limit: f64
}

#[derive(Debug)]
pub struct System {
    transforms: Vec<TransformWithLimit>,
    max_range: f64,
    pub final_transform: Transform,
    pub ttl: i32,
    pub reset_transformation: AffineTransformation
}

impl System {
    pub fn make_particle<R: Rng>(&self, rng: &mut R) -> Particle {
        let mut particle = Particle::new();
        self.reset_particle(&mut particle, rng);

        particle
    }

    fn reset_particle<'a, R: Rng>(&'a self, particle: &'a mut Particle, rng: &mut R) -> &mut Particle {
        let ttl_range: Range<i32> = Range::new(1, self.ttl);

        particle.point = self.reset_transformation.apply(&random::<Point>());
        particle.color = random::<Number>();
        particle.ttl = ttl_range.ind_sample(rng);

        particle
    }

    fn animate_particle<R: Rng>(&self, particle: Particle, transform: &Transform, rng: &mut R) -> Particle {
        transform.animate(&if particle.ttl <= 0 { self.make_particle(rng) } else { particle }).aged()
    }

    fn animate_particle_mut<'a, R: Rng>(&'a self, particle: &'a mut Particle, transform: &'a Transform, rng: &mut R) -> &mut Particle {
        transform.animate_mut(if particle.ttl <= 0 { self.reset_particle(particle, rng) } else { particle }).age()
    }

    fn pick_transform<'a, R: Rng>(&'a self, rng: &mut R) -> &Transform {
        let total_range = Range::new(0.0, self.max_range);
        let value = total_range.ind_sample(rng);

        &(self.transforms.iter().find(|transform| value < transform.limit).unwrap().transform)
    }

    pub fn step<R: Rng>(&self, particle: &mut Particle, rng: &mut R) -> Particle {
        let transform = self.pick_transform(rng);
        self.animate_particle_mut(particle, transform, rng);
        self.final_transform.animate(particle)
    }
}

pub struct SystemBuilder {
    transforms: Vec<(Transform, f64)>,
    final_transform: Transform,
    ttl: i32,
    reset_transformation: AffineTransformation
}

impl SystemBuilder {
    pub fn new() -> SystemBuilder {
        SystemBuilder {
            transforms: Vec::new(),
            final_transform: TransformBuilder::new().finalize(),
            ttl: 30,
            reset_transformation: AffineTransformation::identity()
        }
    }

    pub fn add_transform(self, transform: Transform) -> SystemBuilder {
        self.add_weighted_transform(transform, 1.0)
    }

    pub fn add_weighted_transform(mut self, transform: Transform, weight: f64) -> SystemBuilder {
        self.transforms.push((transform, weight));
        self
    }

    pub fn final_transform(mut self, transform: Transform) -> SystemBuilder {
        self.final_transform = transform;
        self
    }

    pub fn ttl(mut self, ttl: i32) -> SystemBuilder {
        self.ttl = ttl;
        self
    }

    pub fn reset_transformation(mut self, transformation: AffineTransformation) -> SystemBuilder {
        self.reset_transformation = transformation;
        self
    }

    pub fn finalize(self) -> System {
        let starting_points: Vec<f64> = self.transforms.iter().fold(vec![0.0], |mut vec, &(_, weight)| {
            let data = vec[vec.len()-1] + weight;
            vec.push(data);
            vec
        });

        let transforms: Vec<TransformWithLimit> = starting_points[1..]
            .iter()
            .zip(self.transforms.into_iter())
            .map(|(limit, (transform, _))| TransformWithLimit { transform: transform, limit: *limit })
            .collect();

        System {
            transforms: transforms,
            final_transform: self.final_transform,
            ttl: self.ttl,
            max_range: starting_points[starting_points.len() - 1],
            reset_transformation: self.reset_transformation
        }
    }
}

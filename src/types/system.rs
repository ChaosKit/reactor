use types::{Particle, Point, Number, Transform};
use rand::{random, Rng};
use rand::distributions::{IndependentSample, Range};

pub struct System {
    pub ttl: i8
}

impl System {
    pub fn make_particle<R: Rng>(&self, rng: &mut R) -> Particle {
        let ttl_range: Range<i8> = Range::new(1, self.ttl);

        Particle {
            point: random::<Point>(),
            color: random::<Number>(),
            ttl: ttl_range.ind_sample(rng)
        }
    }

    pub fn animate_particle<R: Rng>(&self, particle: Particle, transform: &Transform, rng: &mut R) -> Particle {
        transform.animate(&if particle.ttl <= 0 { self.make_particle(rng) } else { particle }).age()
    }
}

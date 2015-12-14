use types::{Particle, Point, Number, Transform};
use rand::{random, Rng};
use rand::distributions::{IndependentSample, Range};

pub struct System {
    pub ttl: i32
}

impl System {
    pub fn make_particle<R: Rng>(&self, rng: &mut R) -> Particle {
        let mut particle = Particle::new();
        self.reset_particle(&mut particle, rng);

        particle
    }

    fn reset_particle<'a, R: Rng>(&'a self, particle: &'a mut Particle, rng: &mut R) -> &mut Particle {
        let ttl_range: Range<i32> = Range::new(1, self.ttl);

        particle.point = random::<Point>();
        particle.color = random::<Number>();
        particle.ttl = ttl_range.ind_sample(rng);

        particle
    }

    pub fn animate_particle<R: Rng>(&self, particle: Particle, transform: &Transform, rng: &mut R) -> Particle {
        transform.animate(&if particle.ttl <= 0 { self.make_particle(rng) } else { particle }).aged()
    }

    pub fn animate_particle_mut<'a, R: Rng>(&'a self, particle: &'a mut Particle, transform: &'a Transform, rng: &mut R) -> &mut Particle {
        transform.animate_mut(if particle.ttl <= 0 { self.reset_particle(particle, rng) } else { particle }).age()
    }
}

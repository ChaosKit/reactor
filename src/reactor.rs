extern crate rand;
extern crate byteorder;
extern crate num_cpus;
extern crate crossbeam;
extern crate capnp;

mod types;
mod consts;
mod variations;
mod interop;

#[allow(dead_code)]
mod chaoskit_capnp {
    include!(concat!(env!("OUT_DIR"), "/chaoskit_capnp.rs"));
}

use types::{Particle, Message};
use types::system::*;
use std::sync::mpsc;
use std::io::prelude::*;
use std::io;

// use types::transform::*;
// use types::affine_transformation::*;

const PARTICLE_COUNT: i32 = 10000;
const ITERATION_COUNT: i32 = 1000;

enum Status {
    Generated(Particle),
    Finished,
}

fn generate(system: System) {
    let mut global_rng = rand::thread_rng();

    let thread_count = num_cpus::get();
    let chunk_size = ((PARTICLE_COUNT as f32) / (thread_count as f32)).ceil() as usize;
    let mut particles: Vec<Particle> = (0..PARTICLE_COUNT).map(|_| system.make_particle(&mut global_rng)).collect();

    crossbeam::scope(|scope| {
        let (tx, rx) = mpsc::channel();

        for particle_chunk in particles.chunks_mut(chunk_size) {
            let (tx, system) = (tx.clone(), &system);

            scope.spawn(move|| {
                let mut rng = rand::thread_rng();

                for _ in 0..ITERATION_COUNT {
                    for particle in particle_chunk.iter_mut() {
                        let projected_particle = system.step(particle, &mut rng);
                        tx.send(Status::Generated(projected_particle)).unwrap();
                    }
                }

                tx.send(Status::Finished).unwrap();
            });
        }

        let mut stdout = io::stdout();
        let mut finished_threads: usize = 0;
        while finished_threads < thread_count {
            let message = rx.recv().unwrap();

            match message {
                Status::Generated(particle) => {
                    let _ = stdout.write(&particle.bytes());
                },
                Status::Finished => finished_threads += 1
            }
        }
    });
}

fn main() {
    // let variation = variations::DeJong(1.6623940085992217,-0.6880100890994072,1.4784153904765844,1.7967103328555822);
    // let transform = TransformBuilder::new()
    //     .add_boxed_variation(Box::new(variation))
    //     .color(0.5)
    //     .finalize();

    // let system = SystemBuilder::new()
    //     .add_transform(transform)
    //     .ttl(150)
    //     .finalize();

    // generate(system);

    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);

    match interop::read_message(&mut reader) {
        Ok(message) => match message {
            Message::Start(system) => {
                generate(system);
            },
            _ => {}
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

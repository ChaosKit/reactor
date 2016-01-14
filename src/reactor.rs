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
use std::env;

// use types::transform::*;
// use types::affine_transformation::*;

enum Status {
    Generated(Vec<Particle>),
    Finished,
}

fn generate(system: System) {
    let mut global_rng = rand::thread_rng();

    let particle_count = env::var("PARTICLE_COUNT").unwrap_or("10000".to_string()).parse::<u32>().unwrap();
    let iteration_count = env::var("ITERATION_COUNT").unwrap_or("1000".to_string()).parse::<u32>().unwrap();
    let particle_buffer_size = env::var("PARTICLE_BUFFER_SIZE").unwrap_or("1000".to_string()).parse::<usize>().unwrap();
    let channel_size = env::var("CHANNEL_SIZE").unwrap_or("10".to_string()).parse::<usize>().unwrap();

    let thread_count = num_cpus::get();
    let chunk_size = ((particle_count as f32) / (thread_count as f32)).ceil() as usize;
    let mut particles: Vec<Particle> = (0..particle_count).map(|_| system.make_particle(&mut global_rng)).collect();

    crossbeam::scope(|scope| {
        let (tx, rx) = mpsc::sync_channel(channel_size);

        for particle_chunk in particles.chunks_mut(chunk_size) {
            let (tx, system) = (tx.clone(), &system);

            scope.spawn(move|| {
                let mut rng = rand::thread_rng();

                let mut buffer = Vec::with_capacity(particle_buffer_size);

                for _ in 0..iteration_count {
                    for particle in particle_chunk.iter_mut() {
                        let projected_particle = system.step(particle, &mut rng);

                        if buffer.len() < particle_buffer_size {
                            buffer.push(projected_particle);
                        } else {
                            tx.send(Status::Generated(buffer.clone())).unwrap();
                            buffer.clear();
                        }
                    }
                }

                tx.send(Status::Finished).unwrap();
            });
        }

        let stdout = io::stdout();
        let mut writer = io::BufWriter::new(stdout);
        let mut finished_threads: usize = 0;
        while finished_threads < thread_count {
            let message = rx.recv().unwrap();

            match message {
                Status::Generated(buffer) => {
                    for particle in buffer.iter() {
                        let _ = writer.write(&particle.bytes());
                    }
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
    //     .reset_transformation(AffineTransformation::scale(2.0, 2.0))
    //     .ttl(150)
    //     .finalize();

    // println!("{:#?}", system);
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

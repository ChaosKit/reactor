extern crate rand;
extern crate byteorder;

use types::System;
use types::transform::*;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

const ADDRESS: &'static str = "127.0.0.1:24267";

fn handle_client(mut stream: TcpStream) {
    let variation = variations::DeJong(-1.860391774909643026, 1.100373086160729041, -1.086431197851741803, -1.426991546514589704);
    let transform = TransformBuilder::new()
        .add_variation(variation)
        .color(1.0)
        .finalize();
    let mut rng = rand::thread_rng();
    let system = System { ttl: 30 };

    let mut particle = system.make_particle(&mut rng);

    for _ in 0..10000000 {
        particle = system.animate_particle(particle, &transform, &mut rng);

        let _ = stream.write(&particle.bytes());
    }
    println!("Payload sent");
}

fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();
    println!("Reactor is listening on {}", ADDRESS);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Sending payload…");
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}

mod types;
mod consts;
mod variations;

extern crate rand;
extern crate byteorder;

use types::{Point, Applicable, Transform};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

const ADDRESS: &'static str = "127.0.0.1:24267";

fn handle_client(mut stream: TcpStream) {
    let variation = variations::DeJong(-1.860391774909643026, 1.100373086160729041, -1.086431197851741803, -1.426991546514589704);
    let transform = Transform::from_applicable(variation);

    let mut point = rand::random::<Point>();
    for _ in 0..10 {
        point = transform.apply(&point);
        let _ = stream.write(&point.bytes());
    }
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

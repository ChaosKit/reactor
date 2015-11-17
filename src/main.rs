extern crate rand;

use types::{Point, Applicable, Transform};

fn main() {
    let variation = variations::DeJong(-1.860391774909643026, 1.100373086160729041, -1.086431197851741803, -1.426991546514589704);
    let transform = Transform::from_applicable(variation);

    let mut point = rand::random::<Point>();
    for _ in 0..10 {
        point = transform.apply(&point);
        println!("{}", point);
    }
}

mod types;
mod consts;
mod variations;

extern crate algorithms;
extern crate rand;

use algorithms::percolation::{Percolation, PercolationState};
use rand::prelude::*;
use std::io;

fn main() {
    loop {
        println!("Enter size:");
        let size = read_i32();
        if size < 0 {
            break;
        }

        let mut perc: PercolationState = Percolation::new(size as usize);

        let mut rng = thread_rng();

        while !perc.percolates() {
            let row = rng.gen_range(0, size as usize);
            let col = rng.gen_range(0, size as usize);

            perc.open(row, col);
        }

        println!("{}", perc);
        let open_ratio = perc.number_of_open_sites() as f64 / (size * size) as f64;
        println!("open ratio: {}", open_ratio);
    }
}

fn read_i32() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    s.trim_right().parse::<i32>().unwrap()
}


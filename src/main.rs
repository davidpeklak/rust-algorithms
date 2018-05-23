extern crate algorithms;
extern crate rand;
extern crate stopwatch;

use algorithms::percolation::{Percolation, PercolationState};
use rand::prelude::*;
use std::io;
use stopwatch::Stopwatch;

fn main() {
    let mut size = 1;
    let mut stopwatch = Stopwatch::new();
    let mut elapsed_ms = 0i64;

    while size <= 4096 {
        stopwatch.reset();
        stopwatch.start();

        let mut perc: PercolationState = Percolation::new(size as usize);

        let mut rng = thread_rng();

        while !perc.percolates() {
            let row = rng.gen_range(0, size as usize);
            let col = rng.gen_range(0, size as usize);

            perc.open(row, col);
        }

        let new_elapsed_ms = stopwatch.elapsed_ms();

        let open_ratio = perc.number_of_open_sites() as f64 / (size * size) as f64;
        println!("Size: {}, elapsed milliseconds: {}, open ratio: {}", size, new_elapsed_ms, open_ratio);

        if elapsed_ms != 0 {
            let ratio = new_elapsed_ms as f64 / elapsed_ms as f64;
            println!("Ratio: {}", ratio);
        }

        elapsed_ms = new_elapsed_ms;

        size = size * 2;
    }
}

/*
fn read_i32() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    s.trim_right().parse::<i32>().unwrap()
}
*/

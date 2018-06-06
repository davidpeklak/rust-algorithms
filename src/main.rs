extern crate algorithms;
extern crate rand;
extern crate stopwatch;

use algorithms::percolation::PercolationState;
use algorithms::performance;

fn main() {
    let result = performance::observe::<PercolationState>(240000i64, Option::None);
    println!("{}", result);
}

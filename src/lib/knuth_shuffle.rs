extern crate rand;

use self::rand::{ThreadRng, Rng};

/// randomly shuffles a vector with the knuth shuffle algorithm
pub fn knuth_shuffle<Item>(vec: &mut Vec<Item>, rng: &mut ThreadRng) {
    let size = vec.len();

    if size > 1 {
        for i in 1..size {
            let random_j = rng.gen_range(0, i + 1);
            vec.swap(i, random_j);
        }
    }
}

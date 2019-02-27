extern crate algorithms;
extern crate rand;

use algorithms::quick_sort::quick_select;
use algorithms::performance::{PerformanceObservable, observe};
use rand::{ThreadRng, Rng};

struct Input {
    vec: Vec<u32>,
    k: usize
}

impl PerformanceObservable for Input {
    fn prepare(size: usize, rng: &mut ThreadRng) -> Input {
        let mut vec = vec![0u32; size];
        for i in 0..size {
            vec[i] = rng.gen();
        }
        let k = rng.gen_range(0, size);

        Input { vec, k }
    }

    fn run(&mut self, _size: usize, rng: &mut ThreadRng) { quick_select(&mut self.vec, self.k, rng); }
}

#[test]
fn test_performance() {
    let factor = observe::<Input>(5000i64, None);

    println!("Factor selection sort = {}", factor);

    assert!(factor > 1.8);
    assert!(factor < 2.5);
}
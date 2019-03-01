extern crate algorithms;
extern crate rand;

use algorithms::quick_sort::three_way_quick_sort;
use algorithms::performance::{PerformanceObservable, observe};
use rand::{ThreadRng, Rng};

struct VecWrap {
    vec: Vec<u32>
}

impl PerformanceObservable for VecWrap {
    fn prepare(size: usize, rng: &mut ThreadRng) -> VecWrap {
        let mut vec = vec![0u32; size];
        for i in 0..size {
            vec[i] = rng.gen();
        }
        VecWrap { vec }
    }

    fn run(&mut self, _size: usize, rng: &mut ThreadRng) {
        three_way_quick_sort(&mut self.vec, rng);
    }
}

struct DuplicateKeys {
    vec: Vec<u32>
}

impl PerformanceObservable for DuplicateKeys {
    fn prepare(size: usize, rng: &mut ThreadRng) -> DuplicateKeys {
        let mut vec = vec![0u32; size];
        for i in 0..size {
            vec[i] = rng.gen_range(0, 3); // only produces 3 (or 4??) distinct values
        }
        DuplicateKeys { vec }
    }

    fn run(&mut self, _size: usize, rng: &mut ThreadRng) {
        three_way_quick_sort(&mut self.vec, rng);
    }
}

#[test]
fn test_performance() {
    let factor = observe::<VecWrap>(5000i64, None);
    println!("Factor selection sort = {}", factor);

    assert!(factor > 2.0);
    assert!(factor < 3.5);
}

#[test]
fn test_performance_duplicate_keys() {
    let factor = observe::<DuplicateKeys>(5000i64, None);
    println!("Factor selection sort = {}", factor);

    assert!(factor > 2.0);
    assert!(factor < 3.5);
}
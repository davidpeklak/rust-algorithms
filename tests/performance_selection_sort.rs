extern crate algorithms;
extern crate rand;

use algorithms::selection_sort::selection_sort;
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

    fn run(&mut self, _size: usize, _rng: &mut ThreadRng) {
        selection_sort(&mut self.vec);
    }
}

#[test]
fn test_performance() {
    let factor = observe::<VecWrap>(5000i64, None);
    println!("Factor selection sort = {}", factor);

    assert!(factor > 3.5);
    assert!(factor < 4.8);
}
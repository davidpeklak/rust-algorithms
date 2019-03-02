extern crate algorithms;
extern crate rand;

use algorithms::max_pq::MaxPQ;
use algorithms::performance::{PerformanceObservable, observe};
use rand::{ThreadRng, Rng};

struct VecWrap {
    vec: Vec<u32>
}

impl PerformanceObservable for VecWrap {
    fn prepare(size: usize, rng: &mut ThreadRng) -> VecWrap {
        let mut vec = vec!();
        for _i in 0..size {
            vec.heap_insert(rng.gen());
        }
        VecWrap { vec }
    }

    fn run(&mut self, _size: usize, rng: &mut ThreadRng) {
        for _count in 0..5000 {
            self.vec.heap_insert(rng.gen());
            self.vec.del_max();
        }
    }
}

#[test]
fn test_performance() {
    let factor = observe::<VecWrap>(30i64, None);
    println!("Factor binary heap insert, delete = {}", factor);

    assert!(factor > 1.0);
    assert!(factor < 2.0);
}
extern crate algorithms;
extern crate rand;

use algorithms::deque::{Deque, DequeT};
use algorithms::performance::{PerformanceObservable, observe};
use rand::{ThreadRng, RngCore};

struct DequeWrap {
    deque: Deque<u32>
}

impl PerformanceObservable for DequeWrap {
    fn prepare(size: usize, rng: &mut ThreadRng) -> DequeWrap {
        let mut deque = Deque::<u32>::new();

        while deque.size() < size {
            deque.add_first(rng.next_u32());
            deque.add_last(rng.next_u32());
        }

        DequeWrap {
            deque
        }
    }

    fn run(&mut self, _size: usize, rng: &mut ThreadRng) {
        self.deque.add_first(rng.next_u32());
    }
}

#[test]
fn test_performance() {
    let factor = observe::<DequeWrap>(1000i64, Option::Some(2000000usize));
    assert!(factor < 1.2f64);
}
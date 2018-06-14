extern crate algorithms;
extern crate rand;

use algorithms::insertion_sort::insertion_sort;
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
        insertion_sort(&mut self.vec);
    }
}

#[test]
fn test_performance() {
    let factor = observe::<VecWrap>(5000i64, None);
    println!("Factor insertion sort = {}", factor);

    assert!(factor > 3.5);
    assert!(factor < 4.8);
}

struct PartiallySorted {
    vec: Vec<usize>
}

impl PerformanceObservable for PartiallySorted {
    fn prepare(size: usize, rng: &mut ThreadRng) -> Self {
        let mut vec = vec![0; size];
        for i in 0..size {
            vec[i] = i;
        }

        if size > 1 {
            for _n in 0..10 {
                let i = rng.gen_range(0, size - 1);
                vec.swap(i, i + 1);
            }
        }

        PartiallySorted { vec }
    }

    fn run(&mut self, _size: usize, _rng: &mut ThreadRng) {
        insertion_sort(&mut self.vec);
    }
}

#[test]
fn test_performance_partially_sorted() {
    let factor = observe::<PartiallySorted>(5000i64, None);
    println!("Factor insertion sort = {}", factor);

    assert!(factor > 1.8);
    assert!(factor < 2.4);
}

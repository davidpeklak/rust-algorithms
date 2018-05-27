//! Measure the growth rate of algorithms
//! # Example
//! ```
//! use algorithms::performance;
//! use algorithms::performance::example;
//! let result = performance::observe::<Vec<i32>>(1000i64);
//! ```

extern crate rand;
extern crate stopwatch;

use self::rand::{ThreadRng, thread_rng};
use self::stopwatch::Stopwatch;

/// An algorithm that can be observed in terms of execution time
pub trait PerformanceObservable {

    /// prepare a data-structure (with randomized content) of the given size
    fn prepare(size: usize, rng: &mut ThreadRng) -> Self;

    /// run an algorithm against the prepared data-structure
    fn run(&mut self, size: usize, rng: &mut ThreadRng);
}

/// observes the execution time of an algorithm, by doubling
/// the size of the data-structure (of type PO) that the algorithm works on
/// returns the ratio of the execution time of the last ran and the run
/// before that
pub fn observe<PO>(max_millis: i64) -> f64
    where PO: PerformanceObservable {
    let mut rng = thread_rng();
    let mut stopwatch = Stopwatch::new();
    let mut elapsed_ms = 0i64;
    let mut expected_ms = 0f64;
    let mut ratio = 0f64;
    let mut size = 1usize;

    while expected_ms <= max_millis as f64 {
        let mut perf_obs: PO = PerformanceObservable::prepare(size, &mut rng);

        stopwatch.reset();
        stopwatch.start();

        perf_obs.run(size, &mut rng);

        let new_elapsed_ms = stopwatch.elapsed_ms();

        println!("Size: {}, elapsed milliseconds: {}", size, new_elapsed_ms);

        if elapsed_ms != 0 {
            ratio = new_elapsed_ms as f64 / elapsed_ms as f64;
            println!("Ratio: {}", ratio);
        }

        elapsed_ms = new_elapsed_ms;

        size = size * 2;

        expected_ms = elapsed_ms as f64 * ratio;
    }

    ratio
}

pub mod example {
    use super::PerformanceObservable;
    use super::rand::{Rng, ThreadRng};

    impl PerformanceObservable for Vec<i32> {
        fn prepare(size: usize, _rng: &mut ThreadRng) -> Self {
            vec![0i32; size]
        }

        fn run(&mut self, size: usize, rng: &mut ThreadRng) {
            for i in 0..size {
                for j in i..size {
                    self[j] = rng.gen_range(0, 10000);
                }
            }
        }
    }
}

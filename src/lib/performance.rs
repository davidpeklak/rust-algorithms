//! defines an algorithm performance observation framework

extern crate rand;
extern crate stopwatch;

use self::rand::{ThreadRng, thread_rng};
use self::stopwatch::Stopwatch;


pub trait PerformanceObservable {
    fn prepare(size: usize, rng: &mut ThreadRng) -> Self;

    fn run(&mut self, size: usize, rng: &mut ThreadRng);
}

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

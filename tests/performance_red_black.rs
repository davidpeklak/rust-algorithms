extern crate algorithms;
extern crate rand;

use algorithms::performance::{PerformanceObservable, observe};
use algorithms::red_black_bst_2::Tree;
use rand::{ThreadRng, Rng};

struct TreeWrap {
    tree: Tree<i32>,
}


impl PerformanceObservable for TreeWrap {
    fn prepare(size: usize, rng: &mut ThreadRng) -> TreeWrap {
        let mut tree = Tree::<i32>::new();
        for _ in 0..size {
            tree.insert(rng.gen());
        }

        TreeWrap {
            tree
        }
    }

    fn run(&mut self, _size: usize, rng: &mut ThreadRng) {
        for _ in 0..2000 {
            self.tree.insert(rng.gen());
        }
    }
}

#[test]
fn test_performance() {
    let factor = observe::<TreeWrap>(1000i64, Some(2000000));
    println!("Factor tree insert = {}", factor);

    assert!(factor < 2.0);
}
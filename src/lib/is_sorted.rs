extern crate rand;

use self::rand::{ThreadRng, Rng};
use std::ops::Index;
use ::index_ops::Length;

pub fn is_sorted<Coll, Item>(vec: &Coll) -> bool
    where Coll: Index<usize, Output=Item> + Length,
          Item: PartialOrd {
    let len = vec.length();
    if len > 1 {
        for i in 0..len - 1 {
            if vec[i] > vec[i + 1] {
                return false;
            }
        }
    }
    true
}

pub fn sort_some<Item>(rng: &mut ThreadRng, sorting_function: fn(&mut Vec<Item>) -> ())
    where Item: Clone + PartialOrd,
          rand::distributions::Standard: rand::distributions::Distribution<Item> {
    for size in (0..20).map(|x| x * x) {
        let mut vec = vec![rng.gen(); size];
        for i in 0..size {
            vec[i] = rng.gen();
        }
        sorting_function(&mut vec);
        assert!(is_sorted(&vec));
    }
}

#[test]
fn test_is_sorted() {
    assert!(is_sorted(&vec![1, 2, 5, 8, 17, 26]));
    assert!(!is_sorted(&vec![1, 2, 5, 8, 17, 26, 3]));
}

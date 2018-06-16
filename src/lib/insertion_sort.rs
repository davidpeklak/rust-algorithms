//! implements the selection sort algorithm
//! # Example
//! ```
//! use algorithms::insertion_sort::insertion_sort;
//!
//! let mut vec = vec![3,2,4,5,1];
//! insertion_sort(&mut vec);
//! assert_eq!(vec, vec![1,2,3,4,5]);
//! ```

use std::ops::{Index, IndexMut};
use ::index_ops::{Length, Swap};

/// Sort types that implement [`Index`],  [`IndexMut`], [`Length`] and [`Swap`] with the selection sort algorithm.
///
/// [`Index`]: https://doc.rust-lang.org/std/ops/trait.Index.html
/// [`IndexMut`]: https://doc.rust-lang.org/std/ops/trait.IndexMut.html
/// [`Length`]: ../index_ops/trait.Length.html
/// [`Swap`]: ../index_ops/trait.Swap.html
pub fn insertion_sort<Coll, Item>(vec: &mut Coll)
    where Coll: Index<usize, Output=Item> + IndexMut<usize, Output=Item> + Length + Swap<Item>,
          Item: PartialOrd {
    let size = vec.length();
    if size > 1 {
        for i in 1..size {
            let mut j = i;
            while j > 0 && vec[j] < vec[j - 1] {
                vec.swap(j, j - 1);
                j = j - 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    extern crate rand;

    use self::rand::thread_rng;
    use super::insertion_sort;
    use ::is_sorted::{is_sorted, sort_some};

    #[test]
    fn small_example() {
        let mut vec = vec![8, 1, 5, 3, 4];
        insertion_sort(&mut vec);
        assert!(is_sorted(&vec));
    }

    #[test]
    fn insertion_sort_some() {
        let mut rng = thread_rng();
        sort_some::<u32>(&mut rng, insertion_sort);
    }
}
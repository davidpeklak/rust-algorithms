//! implements the selection sort algorithm
//! # Example
//! ```
//! use algorithms::selection_sort::selection_sort;
//!
//! let mut vec = vec![3,2,4,5,1];
//! selection_sort(&mut vec);
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
pub fn selection_sort<Coll, Item>(vec: &mut Coll)
    where Coll: Index<usize, Output=Item> + IndexMut<usize, Output=Item> + Length + Swap<Item>,
          Item: PartialOrd {
    let size = vec.length();
    if size > 1 {
        for i in 0..size - 1 {
            let mut min = i;
            for j in i..size {
                if vec[j] < vec[min] {
                    min = j;
                }
            }
            vec.swap(i, min);
        }
    }
}

#[cfg(test)]
mod test {
    extern crate rand;

    use self::rand::thread_rng;
    use super::selection_sort;
    use ::is_sorted::sort_some;

    #[test]
    fn vec_swap() {
        let mut vec = vec![1, 2, 3, 4];
        vec.swap(0, 2);
        assert_eq!(vec, vec![3, 2, 1, 4]);
    }

    #[test]
    fn selection_sort_some() {
        let mut rng = thread_rng();
        sort_some::<u32>(&mut rng, selection_sort);
    }
}
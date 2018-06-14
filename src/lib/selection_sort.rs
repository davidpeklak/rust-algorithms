//! implements the selection sort algorithm
//! # Example
//! ```
//! use algorithms::selection_sort::selection_sort;
//!
//! let mut vec = vec![3,2,4,5,1];
//! selection_sort(&mut vec);
//! assert_eq!(vec, vec![1,2,3,4,5]);
//! ```

/// sort a Vec with the selection sort algorithm
pub fn selection_sort<Item>(vec: &mut Vec<Item>)
    where Item: PartialOrd {
    let size = vec.len();
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
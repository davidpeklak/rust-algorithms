extern crate rand;

use self::rand::ThreadRng;
use knuth_shuffle::knuth_shuffle;

/// sorts a vector with the quick_sort algorithm
pub fn quick_sort<Item>(vec: &mut Vec<Item>, rng: &mut ThreadRng)
    where Item: PartialOrd {
    knuth_shuffle(vec, rng);

    let size = vec.len();
    sort(vec, 0, size);
}

fn sort<Item>(vec: &mut Vec<Item>, lo: usize, hi: usize)
    where Item: PartialOrd {
    if hi <= lo + 1 {
        return;
    } else {
        let k = partition(vec, lo, hi);
        sort(vec, lo, k);
        sort(vec, k + 1, hi);
    }
}

/// partitions a vector by its first element.
/// returns the index of that element after partitioning.
pub fn partition<Item>(vec: &mut Vec<Item>, lo: usize, hi: usize) -> usize
    where Item: PartialOrd {
    if hi <= lo + 1 {
        lo
    } else {
        let mut i = lo + 1;
        let mut j = hi - 1;

        while i <= j {
            while i < hi && vec[i] <= vec[lo] {
                i = i + 1;
            }

            while j > lo && vec[j] >= vec[lo] {
                j = j - 1;
            }

            if i < j {
                vec.swap(i, j);
            }
        }

        vec.swap(lo, j);
        j
    }
}

#[cfg(test)]
mod tests {
    use super::{partition, quick_sort};
    use super::rand::thread_rng;
    use ::is_sorted::sort_some;

    fn is_partitioned<Item>(vec: &Vec<Item>, k: usize) -> bool
        where Item: PartialOrd {
        let size = vec.len();

        for i in 0..k {
            if vec[i] > vec[k] {
                return false;
            }
        }

        for j in (k + 1)..size {
            if
                vec[j] < vec[k] {
                return false;
            }
        }

        true
    }

    #[test]
    fn partition_some() {
        {
            let mut vec = vec![1];
            let k = partition(&mut vec, 0, 1);

            assert!(is_partitioned(&vec, k));
        }
        {
            let mut vec = vec![1, 2];
            let k = partition(&mut vec, 0, 2);

            assert!(is_partitioned(&vec, k));
        }
        {
            let mut vec = vec![2, 1];
            let k = partition(&mut vec, 0, 2);

            assert!(is_partitioned(&vec, k));
        }
        {
            let mut vec = vec![1, 1, 2];
            let k = partition(&mut vec, 0, 3);

            assert!(is_partitioned(&vec, k));
        }
        {
            let mut vec = vec![2, 1, 2];
            let k = partition(&mut vec, 0, 3);

            assert!(is_partitioned(&vec, k));
        }
        {
            let mut vec = vec![2, 1, 1];
            let k = partition(&mut vec, 0, 3);

            assert!(is_partitioned(&vec, k));
        }
        {
            let mut vec = vec![1, 2, 2];
            let k = partition(&mut vec, 0, 3);

            assert!(is_partitioned(&vec, k));
        }
        {
            let mut vec = vec![4, 10, 2, 8, 3, 6, 8, 2, 3, 0, 3];
            let k = partition(&mut vec, 0, 11);

            assert_eq!(k, 6);
            assert_eq!(vec, vec![2, 3, 2, 0, 3, 3, 4, 8, 6, 8, 10]);

            let k = partition(&mut vec, 7, 11);
            assert_eq!(k, 8);
            assert_eq!(vec, vec![2, 3, 2, 0, 3, 3, 4, 6, 8, 8, 10]);

            let k = partition(&mut vec, 0, 6);
            assert_eq!(k, 1);
            assert_eq!(vec, vec![0, 2, 2, 3, 3, 3, 4, 6, 8, 8, 10]);
        }
    }

    #[test]
    fn quick_sort_some() {
        let mut rng = thread_rng();

        fn da_sort(vec: &mut Vec<u32>) {
            let mut rng = thread_rng();
            quick_sort(vec, &mut rng);
        }
        sort_some::<u32>(&mut rng, da_sort);
    }
}
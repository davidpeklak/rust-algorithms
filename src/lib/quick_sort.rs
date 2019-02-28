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

/// selects the kth largest element in a vector
pub fn quick_select<Item>(vec: &mut Vec<Item>, k: usize, rng: &mut ThreadRng) -> Item
    where Item: PartialOrd + Copy {
    knuth_shuffle(vec, rng);

    let mut lo = 0usize;
    let mut hi = vec.len();

    loop {
        let l = partition(vec, lo, hi);
        if l == k {
            return vec[k];
        }
        if l > k {
            hi = l;
        } else {
            lo = l + 1;
        }
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

/// partitions a vector by its first element
/// returns two indices, indicating the range of elements
/// that are equal to that element, after partitioning
pub fn three_way_partition<Item>(vec: &mut Vec<Item>, lo: usize, hi: usize) -> (usize, usize)
    where Item: PartialOrd {
    if hi <= lo + 1 {
        (lo, hi)
    } else {
        let mut lt = lo;
        let mut i = lo + 1;
        let mut gt = hi;

        while gt > i {
            if vec[i] < vec[lt] {
                vec.swap(lt, i);
                lt = lt + 1;
                i = i + 1;
            }
            else if vec[i] > vec[lt] {
                vec.swap(i, gt - 1);
                gt = gt - 1;
            }
            else {
                i = i + 1;
            }
        }
        (lt, gt)
    }
}

#[cfg(test)]
mod tests {
    use super::{partition, three_way_partition, quick_sort, quick_select};
    use super::rand::thread_rng;
    use ::is_sorted::sort_some;

    fn is_partitioned<Item>(vec: &Vec<Item>, k: usize) -> bool
        where Item: PartialOrd {
        let size = vec.len();

        if size == 0 {
            true
        } else {
            assert!(k < size);

            let v = &vec[k];

            for i in 0..k {
                if vec[i] > *v {
                    return false;
                }
            }

            for j in (k + 1)..size {
                if
                    vec[j] < *v {
                    return false;
                }
            }

            true
        }
    }

    fn is_three_way_partitioned<Item>(vec: &Vec<Item>, lt: usize, gt: usize) -> bool
        where Item: PartialOrd {
        let size = vec.len();
        if size == 0 {
            true
        } else {
            assert!(gt > lt);
            assert!(gt <= size);
            let v = &vec[lt];

            for j in 0..lt {
                if vec[j] >= *v {
                    return false;
                }
            }

            for j in lt..gt {
                if vec[j] != *v {
                    return false;
                }
            }

            for j in gt..size {
                if vec[j] <= *v {
                    return false;
                }
            }

            true
        }
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
    fn three_way_partition_some() {
        {
            let mut vec = vec![1];
            let (lt, gt) = three_way_partition(&mut vec, 0, 1);

            assert!(is_three_way_partitioned(&vec, lt, gt));
        }
        {
            let mut vec = vec![1, 2];
            let (lt, gt) = three_way_partition(&mut vec, 0, 2);

            assert!(is_three_way_partitioned(&vec, lt, gt));
        }
        {
            let mut vec = vec![2, 1];
            let (lt, gt) = three_way_partition(&mut vec, 0, 2);

            assert!(is_three_way_partitioned(&vec, lt, gt));
        }
        {
            let mut vec = vec![1, 1, 2];
            let (lt, gt) = three_way_partition(&mut vec, 0, 3);

            assert!(is_three_way_partitioned(&vec, lt, gt));
        }
        {
            let mut vec = vec![2, 1, 2];
            let (lt, gt) = three_way_partition(&mut vec, 0, 3);

            assert!(is_three_way_partitioned(&vec, lt, gt));
        }
        {
            let mut vec = vec![2, 1, 1];
            let (lt, gt) = three_way_partition(&mut vec, 0, 3);

            assert!(is_three_way_partitioned(&vec, lt, gt));
        }
        {
            let mut vec = vec![1, 2, 2];
            let (lt, gt) = three_way_partition(&mut vec, 0, 3);

            assert!(is_three_way_partitioned(&vec, lt, gt));
        }
        {
            let mut vec = vec![4, 10, 2, 8, 3, 6, 8, 2, 3, 0, 3];
            let (lt, gt) = three_way_partition(&mut vec, 0, 11);

            assert_eq!(lt, 6);
            assert_eq!(gt, 7);
            assert_eq!(vec, vec![3, 2, 0, 3, 3, 2, 4, 8, 6, 8, 10]);

            let (lt, gt) = three_way_partition(&mut vec, 7, 11);
            assert_eq!(lt, 8);
            assert_eq!(gt, 10);
            assert_eq!(vec, vec![3, 2, 0, 3, 3, 2, 4, 6, 8, 8, 10]);

            let (lt, gt) = three_way_partition(&mut vec, 0, 6);
            assert_eq!(lt, 3);
            assert_eq!(gt, 6);
            assert_eq!(vec, vec![2, 0, 2, 3, 3, 3, 4, 6, 8, 8, 10]);

            let (lt, gt) = three_way_partition(&mut vec, 0, 3);
            assert_eq!(lt, 1);
            assert_eq!(gt, 3);
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

    #[test]
    fn test_quick_select() {
        let mut rng = thread_rng();

        let mut vec = vec![1];
        let result = quick_select(&mut vec, 0, &mut rng);
        assert_eq!(result, 1);

        let mut vec = vec![3, 2, 6, 1];
        let result = quick_select(&mut vec, 2, &mut rng);
        assert_eq!(result, 3);

        let mut vec = vec![3, 2, 3, 6, 3, 7, 3];
        let result = quick_select(&mut vec, 3, &mut rng);
        assert_eq!(result, 3);

        let mut vec = vec![3, 2, 3, 6, 3, 7, 3];
        let result = quick_select(&mut vec, 5, &mut rng);
        assert_eq!(result, 6);
    }
}
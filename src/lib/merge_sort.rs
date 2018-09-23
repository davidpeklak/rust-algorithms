/// Sorts a vector with the merge-sort algorithm
pub fn merge_sort<Item>(vec: &mut Vec<Item>)
    where Item: PartialOrd + Copy {
    let mut aux_vec = vec.clone();
    let size = vec.len();
    do_merge_sort(vec, &mut aux_vec, 0, size);
}

/// recursivly merge-sort a vector, using an auxilary vector
fn do_merge_sort<Item>(vec: &mut Vec<Item>, aux_vec: &mut Vec<Item>, lo: usize, hi: usize)
    where Item: PartialOrd + Copy {
    let mid = (lo + hi) / 2;

    if mid - lo > 1 {
        do_merge_sort(vec, aux_vec, lo, mid);
    }

    if hi - mid > 1 {
        do_merge_sort(vec, aux_vec, mid, hi)
    }

    do_merge(vec, aux_vec, lo, hi, mid);
}


/// Merges two ranges of a vector. The first range is the range from the lo-th element to
/// the (mid-1)th element. The second part of the vector is the range from the mid-th element to the
/// (hi-1)th element.
pub fn merge<Item>(vec: &mut Vec<Item>, mid: usize)
    where Item: PartialOrd + Copy {
    let mut aux_vec = vec.clone();
    let size = vec.len();
    do_merge(vec, &mut aux_vec, 0, size, mid)
}

/// Merges two ranges of a vector, using an auxilary vector. The first range is the range from the lo-th element to
/// the (mid-1)th element. The second part of the vector is the range from the mid-th element to the
/// (hi-1)th element.
fn do_merge<Item>(vec: &mut Vec<Item>, aux_vec: &mut Vec<Item>, lo: usize, hi: usize, mid: usize)
    where Item: PartialOrd + Copy {
    let mut i = lo;
    let mut j = mid;

    for k in lo..hi {
        aux_vec[k] = vec[k];
    }

    for k in lo..hi {
        if i >= mid {
            vec[k] = aux_vec[j];
            j = j + 1;
        } else if j >= hi {
            vec[k] = aux_vec[i];
            i = i + 1;
        } else if aux_vec[i] <= aux_vec[j] {
            vec[k] = aux_vec[i];
            i = i + 1;
        } else {
            vec[k] = aux_vec[j];
            j = j + 1;
        }
    }
}

#[cfg(test)]
mod test {
extern crate rand;

    use super::{merge, merge_sort};
    use is_sorted::is_sorted;
    use self::rand::thread_rng;
    use ::is_sorted::sort_some;

    #[test]
    fn merge_some() {
        let mut vec = vec![1, 4, 7, 1, 2, 3];
        merge(&mut vec, 3);
        assert_eq!(vec, vec![1, 1, 2, 3, 4, 7]);
    }

    #[test]
    fn merge_sort_is_sorted() {
        let mut vec = vec![7, 3, 5, 6, 2, 5, 1, 9];
        merge_sort(&mut vec);
        assert!(is_sorted(&vec));
    }

    #[test]
    fn merge_sort_some() {
        let mut rng = thread_rng();
        sort_some::<u32>(&mut rng, merge_sort);
    }
}
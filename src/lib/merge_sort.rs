/// Merges two parts of a vector. The first part of the vector is the range from the 0th element to
/// the (mid-1)th element. The second part of the vector is the range from the mid-th element to the last
/// element. If `mid > vec.len()`, the behaviour is undefined.
pub fn merge<Item>(vec: &mut Vec<Item>, mid: usize)
    where Item: PartialOrd + Copy {
    let size = vec.len();
    let aux_vec = vec.clone();
    let mut i = 0usize;
    let mut j = mid;

    for k in 0..size {
        if i >= mid {
            vec[k] = aux_vec[j];
            j = j + 1;
        }
        else if j >= size {
            vec[k] = aux_vec[i];
            i = i + 1;
        }
        else if aux_vec[i] <= aux_vec[j] {
            vec[k] = aux_vec[i];
            i = i + 1;
        }
        else {
            vec[k] = aux_vec[j];
            j = j + 1;
        }
    }
}

#[cfg(test)]
mod test {
use super::merge;

    #[test]
    fn merge_some() {
        let mut vec = vec![1, 4, 7, 1, 2, 3];
        merge(&mut vec, 3);
        assert_eq!(vec, vec![1, 1, 2, 3, 4, 7]);
    }
}
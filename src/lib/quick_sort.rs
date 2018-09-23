/// partitions a vector by its first element.
/// returns the index of that element after partitioning.
pub fn partition<Item>(vec: &mut Vec<Item>) -> usize
    where Item: PartialOrd {
    let size = vec.len();
    if size <= 1 {
        0
    } else {
        let mut i = 1;
        let mut j = size - 1;

        while i <= j {
            while i < size && vec[i] <= vec[0] {
                i = i + 1;
            }

            while j > 0 && vec[j] >= vec[0] {
                j = j - 1;
            }

            if i < j {
                vec.swap(i, j);
            }
        }

        vec.swap(0, j);
        j
    }
}

#[cfg(test)]
mod tests {
    use super::partition;

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
            let k = partition(&mut vec);

            assert!(is_partitioned(&vec, k));
        }
        {
            let mut vec = vec![1, 2];
            let k = partition(&mut vec);

            assert!(is_partitioned(&vec, k));
        }
        {
            let mut vec = vec![2, 1];
            let k = partition(&mut vec);

            assert!(is_partitioned(&vec, k));
        }
        {
            let mut vec = vec![1, 1, 2];
            let k = partition(&mut vec);

            assert!(is_partitioned(&vec, k));
        }
        {
            let mut vec = vec![2, 1, 2];
            let k = partition(&mut vec);

            assert!(is_partitioned(&vec, k));
        }
        {
            let mut vec = vec![2, 1, 1];
            let k = partition(&mut vec);

            assert!(is_partitioned(&vec, k));
        }
        {
            let mut vec = vec![1, 2, 2];
            let k = partition(&mut vec);

            assert!(is_partitioned(&vec, k));
        }
        {
            let mut vec = vec![4, 10, 2, 8, 3, 6, 8, 2, 3, 0, 3];
            let k = partition(&mut vec);

            assert!(is_partitioned(&vec, k));
            assert_eq!(vec, vec![1, 2, 3])
        }
    }
}
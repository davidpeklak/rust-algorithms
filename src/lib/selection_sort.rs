
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

    use self::rand::{Rng, thread_rng};

    use super::selection_sort;

    fn is_sorted<Item>(vec: &Vec<Item>) -> bool
        where Item: PartialOrd {
        if vec.len() > 1 {
            for i in 0..vec.len() - 1 {
                if vec[i] > vec[i + 1] {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_is_sorted() {
        assert!(is_sorted(&vec![1, 2, 5, 8, 17, 26]));
        assert!(!is_sorted(&vec![1, 2, 5, 8, 17, 26, 3]));
    }

    #[test]
    fn vev_swap() {
        let mut vec = vec![1, 2, 3, 4];
        vec.swap(0, 2);
        assert_eq!(vec, vec![3, 2, 1, 4]);
    }

    #[test]
    fn sort_some() {
        let mut rng = thread_rng();
        for size in (0..20).map(|x| x * x) {
            let mut vec = vec![0u8; size];
            for i in 0..size {
                vec[i] = rng.gen();
            }
            selection_sort(&mut vec);
            assert!(is_sorted(&vec));
        }
    }
}
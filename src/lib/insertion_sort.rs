pub fn insertion_sort<Item>(vec: &mut Vec<Item>)
    where Item: PartialOrd {
    let size = vec.len();
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
use ::index_ops::Length;
use ::insertion_sort::insertion_sort;
use ::step::Step;

pub fn shell_sort<Item>(vec: &mut Vec<Item>)
    where Item: PartialOrd {
    let size = vec.length();
    let hs = generate_hs(size);
    for h in hs.iter().rev() {
        h_sort(vec, *h);
    }
}

fn generate_hs(size: usize) -> Vec<usize> {
    let mut vec = vec![1usize];
    loop {
        let h_size = vec.len();
        let new_h = vec[h_size - 1] * 3 + 1;
        if new_h > size {
            break;
        }
        vec.push(new_h);
    }
    vec
}

fn h_sort<Item>(vec: &mut Vec<Item>, h: usize)
    where Item: PartialOrd {
    let size = vec.length();
    let mut offset = 0;
    while offset + h < size {
        insertion_sort(&mut Step::new(vec, h, offset));
        offset = offset + 1;
    }
}

#[cfg(test)]
mod test {
    extern crate rand;

    use ::step::Step;
    use ::insertion_sort::insertion_sort;
    use ::is_sorted::{is_sorted, sort_some};
    use super::{shell_sort, h_sort, generate_hs};

    use self::rand::thread_rng;

    #[test]
    fn sorted_step() {
        let mut vec = vec![1, 6, 2, 7, 3, 8, 4, 9, 5, 10];

        {
            let step = Step::new(&mut vec, 2, 0);
            assert!(is_sorted(&step));
        }
        {
            let step = Step::new(&mut vec, 2, 1);
            assert!(is_sorted(&step));
        }
    }

    #[test]
    fn handwritten() {
        let mut vec = vec![6, 8, 10, 1, 0, 3, 7, 3, 6, 1];

        {
            let mut step = Step::new(&mut vec, 4, 0);
            insertion_sort(&mut step);
            assert!(is_sorted(&step));
        }

        {
            let mut step = Step::new(&mut vec, 4, 1);
            insertion_sort(&mut step);
            assert!(is_sorted(&step));
        }

        {
            let mut step = Step::new(&mut vec, 4, 2);
            insertion_sort(&mut step);
            assert!(is_sorted(&step));
        }

        {
            let mut step = Step::new(&mut vec, 4, 3);
            insertion_sort(&mut step);
            assert!(is_sorted(&step));
        }

        assert_eq!(vec![0, 1, 7, 1, 6, 3, 10, 3, 6, 8], vec);

        {
            let mut step = Step::new(&mut vec, 2, 0);
            insertion_sort(&mut step);
            assert!(is_sorted(&step));
        }

        {
            let mut step = Step::new(&mut vec, 2, 1);
            insertion_sort(&mut step);
            assert!(is_sorted(&step));
        }

        assert_eq!(vec![0, 1, 6, 1, 6, 3, 7, 3, 10, 8], vec);

        {
            let step = Step::new(&mut vec, 4, 0);
            assert!(is_sorted(&step));
        }

        {
            let step = Step::new(&mut vec, 4, 1);
            assert!(is_sorted(&step));
        }

        {
            let step = Step::new(&mut vec, 4, 2);
            assert!(is_sorted(&step));
        }

        {
            let step = Step::new(&mut vec, 4, 3);
            assert!(is_sorted(&step));
        }

        {
            let mut step = Step::new(&mut vec, 1, 0);
            insertion_sort(&mut step);
            assert!(is_sorted(&step));
        }

        assert!(is_sorted(&vec));
    }

    #[test]
    fn small_example() {
        let mut vec = vec![8, 1, 5, 3, 4, 7, 3, 6, 9, 7, 6, 5, 4, 2];
        shell_sort(&mut vec);
        assert!(is_sorted(&vec));
    }

    #[test]
    fn example_from_course() {
        let mut vec = vec!['S', 'O', 'R', 'T', 'E', 'X', 'A', 'M', 'P', 'L', 'E'];
        h_sort(&mut vec, 7);
        assert_eq!(vec!['M', 'O', 'L', 'E', 'E', 'X', 'A', 'S', 'P', 'R', 'T'], vec);
        h_sort(&mut vec, 3);
        assert_eq!(vec!['A', 'E', 'L', 'E', 'O', 'P', 'M', 'S', 'X', 'R', 'T'], vec);
        h_sort(&mut vec, 1);
        assert_eq!(vec!['A', 'E', 'E', 'L', 'M', 'O', 'P', 'R', 'S', 'T', 'X'], vec);
    }

    #[test]
    fn h_generation() {
        assert_eq!(generate_hs(364), vec![1, 4, 13, 40, 121, 364])
    }

    #[test]
    fn shell_sort_some() {
        let mut rng = thread_rng();
        sort_some::<u32>(&mut rng, shell_sort);
    }
}
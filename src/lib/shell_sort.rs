
#[cfg(test)]
mod test {
    use ::step::Step;
    use ::selection_sort::selection_sort;
    use ::is_sorted::is_sorted;

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
    fn h_sort_short() {
        let mut vec = vec![6, 8, 10, 1, 0, 3, 7, 3, 6, 1];
        let mut step = Step::new(&mut vec, 4, 0);
        selection_sort(&mut step);
    }
}
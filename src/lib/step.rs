//! an adapter for types that implement [`Index`] and [`IndexMut`],
//! to intex into them with a certain step size.
//!
//! [`Index`]: https://doc.rust-lang.org/std/ops/trait.Index.html
//! [`IndexMut`]: https://doc.rust-lang.org/std/ops/trait.IndexMut.html
//!
//! # Example
//! ```
//! use algorithms::step::Step;
//!
//! let mut vec = vec![1, 2, 3, 4, 5];
//! let step = Step::new(&mut vec, 3, 1);
//!
//! assert_eq!(step[0], 2);
//! assert_eq!(step[1], 5);
//! ```

use std::ops::{Index, IndexMut};

pub struct Step<'a, Underl>
    where Underl: 'a {
    underl: &'a mut Underl,
    step: usize,
    offset: usize
}

impl<'a, Underl, Output> Index<usize> for Step<'a, Underl>
    where Underl: Index<usize, Output=Output> {
    type Output = Output;

    fn index(&self, index: usize) -> &Output {
        &self.underl[self.offset + index * self.step]
    }
}

impl<'a, Underl, Output> IndexMut<usize> for Step<'a, Underl>
    where Underl: 'a + IndexMut<usize, Output=Output> {
    fn index_mut(&mut self, index: usize) -> &mut Output {
        &mut self.underl[self.offset + index * self.step]
    }
}

impl<'a, Underl> Step<'a, Underl> {
    pub fn new(underl: &'a mut Underl, step: usize, offset: usize) -> Step<'a, Underl> {
        Step { underl, step, offset }
    }
}

impl<'a, Item> Step<'a, Vec<Item>> {
    pub fn len(&self) -> usize {
        (self.underl.len() - self.offset + self.step - 1) / self.step
    }
}

#[cfg(test)]
mod test {
    use super::Step;

    #[test]
    fn index() {
        let mut vec = vec![1, 2, 3, 4, 5];
        let step = Step::new(&mut vec, 3, 0);

        assert_eq!(step[0], 1);
        assert_eq!(step[1], 4);
    }

    #[test]
    fn index_offset() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        let step = Step::new(&mut vec, 3, 2);

        assert_eq!(step[0], 3);
        assert_eq!(step[1], 6);
    }

    #[test]
    fn index_mut() {
        let mut vec = vec![1, 2, 3, 4, 5];
        {
            let mut step = Step::new(&mut vec, 3, 1);

            step[1] = 10;
        }

        assert_eq!(vec, vec![1, 2, 3, 4, 10]);
    }

    #[test]
    fn len() {
        let mut vec = vec![1, 2, 3, 4];
        assert_eq!(Step::new(&mut vec, 1, 0).len(), 4);
        assert_eq!(Step::new(&mut vec, 2, 0).len(), 2);
        assert_eq!(Step::new(&mut vec, 3, 0).len(), 2);
        assert_eq!(Step::new(&mut vec, 4, 0).len(), 1);
        assert_eq!(Step::new(&mut vec, 5, 0).len(), 1);
        assert_eq!(Step::new(&mut vec, 20, 0).len(), 1);
    }

    #[test]
    fn len_offset() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(Step::new(&mut vec, 1, 2).len(), 4);
        assert_eq!(Step::new(&mut vec, 2, 2).len(), 2);
        assert_eq!(Step::new(&mut vec, 3, 2 ).len(), 2);
        assert_eq!(Step::new(&mut vec, 4, 2).len(), 1);
        assert_eq!(Step::new(&mut vec, 5, 2).len(), 1);
        assert_eq!(Step::new(&mut vec, 20, 2).len(), 1);
    }
}


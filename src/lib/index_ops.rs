//! Traits of indexed collections, and their implementations for [`Vec`].
//!
//! [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html

use std::ops::IndexMut;

/// Types that have a length
pub trait Length {
    /// Returns the number of elements.
    fn length(&self) -> usize;
}

impl<Item> Length for Vec<Item> {
    fn length(&self) -> usize {
        self.len()
    }
}

/// Types that can swap elements
pub trait Swap<Item> : IndexMut<usize, Output=Item> {
    /// Swaps two elements in the slice.
    ///
    /// # Arguments
    ///
    /// * a - The index of the first element
    /// * b - The index of the second element
    fn swap(&mut self, a: usize, b: usize);
}

impl<Item> Swap<Item> for Vec<Item> {
    fn swap(&mut self, a: usize, b: usize) {
        use std::ops::DerefMut;
        self.deref_mut().swap(a, b);
    }
}

#[cfg(test)]
mod test {
    use super::Length;

    #[test]
    fn vec_length() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(vec.len(), vec.length());
    }
}
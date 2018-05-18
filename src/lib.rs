/// a module implementing the quick-find algorithm as presented
/// # Example
/// ```
/// use algorithms::quick_find::QuickFind;
/// let mut vec: Vec<u32> = QuickFind::new(3);
/// assert!(!vec.is_connected(0, 2));
/// vec.connect(0, 1);
/// vec.connect(1, 2);
/// assert!(vec.is_connected(0, 2));
/// ```
pub mod quick_find {

    /// a trait that exposes the methods of the quick-find algorithm
    pub trait QuickFind {
        /// the type of the identifier of elements
        type Name;

        /// create a new data-structure without any connections
        /// # Example
        /// ```
        /// use algorithms::quick_find::QuickFind;
        /// let mut vec: Vec<u32> = QuickFind::new(3);
        /// ```
        fn new(size: usize) -> Self;

        /// test if two elements are connected
        fn is_connected(&self, first: Self::Name, second: Self::Name) -> bool;

        /// connect two elements
        fn connect(&mut self, first: Self::Name, second: Self::Name);
    }

    impl QuickFind for Vec<u32> {
        type Name = usize;

        fn new(size: usize) -> Vec<u32> {
            (0..size as u32).collect()
        }

        fn is_connected(&self, first: usize, second: usize) -> bool {
            self[first] == self[second]
        }

        fn connect(&mut self, first: usize, second: usize) {
            let compound = self[first];
            let other_compund = self[second];

            for i in 0..self.len() {
                if self[i] == other_compund {
                    self[i] = compound;
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::QuickFind;

        #[test]
        fn test_3() {
            let vec: Vec<u32> = QuickFind::new(3);
            assert!(!vec.is_connected(0, 1));
            assert!(!vec.is_connected(0, 2));
            assert!(!vec.is_connected(1, 2));

            assert!(vec.is_connected(0, 0));
            assert!(vec.is_connected(1, 1));
            assert!(vec.is_connected(2, 2));
        }

        #[test]
        fn test_3_c() {
            let mut vec: Vec<u32> = QuickFind::new(3);

            vec.connect(0, 1);
            assert!(vec.is_connected(0, 1));
            assert!(!vec.is_connected(0, 2));
            assert!(!vec.is_connected(1, 2));

            vec.connect(1, 2);
            assert!(vec.is_connected(0, 2));
        }
    }
}

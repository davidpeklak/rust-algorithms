/// a trait that exposes the methods of the quick-unnion algorithm
pub trait QuickUnion {
    /// the type of the identifier of elements
    type Name;

    /// create a new data-structure without any connections
    /// # Example
    /// ```
    /// use algorithms::quick_union::QuickUnion;
    /// let mut vec: Vec<usize> = QuickUnion::new(3);
    /// ```
    fn new(size: usize) -> Self;

    /// test if two elements are connected
    fn is_connected(&self, first: Self::Name, second: Self::Name) -> bool;

    /// connect two elements
    fn connect(&mut self, first: Self::Name, second: Self::Name);
}

impl QuickUnion for Vec<usize> {
    type Name = usize;

    fn new(size: usize) -> Vec<usize> {
        (0..size as usize).collect()
    }

    fn is_connected(&self, first: usize, second: usize) -> bool {
        find_root(self, first) == find_root(self, second)
    }

    fn connect(&mut self, first: usize, second: usize) {
        let second_root = find_root(self, second);
        self[second_root] = find_root(self, first);
    }
}

fn find_root(vec: &Vec<usize>, elem: usize) -> usize {
    if vec[elem] == elem {
        elem
    } else {
        find_root(vec, vec[elem])
    }
}

#[cfg(test)]
mod tests {
    use super::QuickUnion;

    #[test]
    fn test_3() {
        let vec: Vec<usize> = QuickUnion::new(3);
        assert!(!vec.is_connected(0, 1));
        assert!(!vec.is_connected(0, 2));
        assert!(!vec.is_connected(1, 2));

        assert!(vec.is_connected(0, 0));
        assert!(vec.is_connected(1, 1));
        assert!(vec.is_connected(2, 2));
    }

    #[test]
    fn test_3_c() {
        let mut vec: Vec<usize> = QuickUnion::new(3);

        vec.connect(0, 1);
        assert!(vec.is_connected(0, 1));
        assert!(!vec.is_connected(0, 2));
        assert!(!vec.is_connected(1, 2));

        vec.connect(1, 2);
        assert!(vec.is_connected(0, 2));
    }
}

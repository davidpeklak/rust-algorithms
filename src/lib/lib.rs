/// a module implementing the quick-find algorithm as presented in the Algorithms course
/// by Robert Sedgewick and Kevin Wayne on coursera.org
/// # Example
/// ```
/// use algorithms::quick_find::QuickFind;
/// let mut vec: Vec<usize> = QuickFind::new(3);
/// assert!(!vec.is_connected(0, 2));
/// vec.connect(0, 1);
/// vec.connect(1, 2);
/// assert!(vec.is_connected(0, 2));
/// ```
pub mod quick_find;

/// a module implementing the quick-union algorithm as presented in the Algorithms course
/// by Robert Sedgewick and Kevin Wayne on coursera.org
/// # Example
/// ```
/// use algorithms::quick_union::QuickUnion;
/// let mut vec: Vec<usize> = QuickUnion::new(3);
/// assert!(!vec.is_connected(0, 2));
/// vec.connect(0, 1);
/// vec.connect(1, 2);
/// assert!(vec.is_connected(0, 2));
/// ```
pub mod quick_union;

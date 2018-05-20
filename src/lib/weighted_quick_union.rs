/// a trait that exposes the methods of the weighted-quick-unnion algorithm
pub trait WeightedQuickUnion {
    /// the type of the identifier of elements
    type Name;

    /// create a new data-structure without any connections
    /// # Example
    /// ```
    /// use algorithms::weighted_quick_union::{WeightedQuickUnion, WQU};
    /// let mut wqu: WQU = WeightedQuickUnion::new(3);
    /// ```
    fn new(size: usize) -> Self;

    /// test if two elements are connected
    fn is_connected(&self, first: Self::Name, second: Self::Name) -> bool;

    /// connect two elements
    fn connect(&mut self, first: Self::Name, second: Self::Name);
}

pub struct WQU {
    elems: Vec<usize>,
    weights: Vec<usize>
}

impl WeightedQuickUnion for WQU {
    type Name = usize;

    fn new(size: usize) -> WQU {
        WQU {
            elems: (0..size as usize).collect(),
            weights: vec![1; size]
        }
    }

    fn is_connected(&self, first: usize, second: usize) -> bool {
        find_root(&self.elems, first) == find_root(&self.elems, second)
    }

    fn connect(&mut self, first: usize, second: usize) {
        let first_root = find_root(&self.elems, first);
        let second_root = find_root(&self.elems, second);
        if first_root != second_root {
            let first_tree_weight = self.weights[first_root];
            let second_tree_weight = self.weights[second_root];

            if first_tree_weight < second_tree_weight {
                self.elems[first_root] = second_root;
                self.weights[second_root] = first_tree_weight + second_tree_weight;
            } else {
                self.elems[second_root] = first_root;
                self.weights[first_root] = first_tree_weight + second_tree_weight;
            }
        }
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
    use super::{WeightedQuickUnion, WQU};

    #[test]
    fn test_3() {
        let wqu: WQU = WeightedQuickUnion::new(3);
        assert!(!wqu.is_connected(0, 1));
        assert!(!wqu.is_connected(0, 2));
        assert!(!wqu.is_connected(1, 2));

        assert!(wqu.is_connected(0, 0));
        assert!(wqu.is_connected(1, 1));
        assert!(wqu.is_connected(2, 2));
    }

    #[test]
    fn test_3_c() {
        let mut wqu: WQU = WeightedQuickUnion::new(3);

        wqu.connect(0, 1);
        assert!(wqu.is_connected(0, 1));
        assert!(!wqu.is_connected(0, 2));
        assert!(!wqu.is_connected(1, 2));

        wqu.connect(1, 2);
        assert!(wqu.is_connected(0, 2));
    }
}

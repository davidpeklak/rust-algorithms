//! Defines a trait representing a priority queue, and an implementation for [`Vec`].
//!
//! [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html


// a trait representing a priority queue
pub trait MaxPQ {
    type Item;
    fn heap_insert(&mut self, item: Self::Item);
    fn del_max(&mut self) -> Option<Self::Item>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

// for a binary heap, implemented as a [`Vec`], as described in the course, but starting at index
// 0, this function returns the parent index of the index k.
fn parent(k: usize) -> usize {
    ((k + 1) / 2) - 1
}

// for a binary heap, implemented as a [`Vec`], as described in the course, but starting at index
// 0, this function returns the children indices of the index k, as a pair.
fn children(k: usize) -> (usize, usize) {
    (((k + 1) * 2) - 1, ((k + 1) * 2) - 1 + 1)
}

// swims the element at index k, as described in the course.
fn swim<Item>(vec: &mut Vec<Item>, k: usize)
    where Item: PartialOrd {
    let mut i = k;
    while i > 0 && vec[parent(i)] < vec[i] {
        vec.swap(parent(i), i);
        i = parent(i);
    }
}

// sinks the element at index k, as described in the course, only considerung the elements of the
// vector up to (excluding) size.
fn sink_with_size<Item>(vec: &mut Vec<Item>, k: usize, size: usize)
    where Item: PartialOrd {
    let mut i = k;
    while (i + 1) * 2 <= size {
        let (child1, child2) = children(i);
        let chosen_child = if child2 < size && vec[child1] < vec[child2] { child2 } else { child1 };
        if vec[i] >= vec[chosen_child] {
            return;
        }
        vec.swap(i, chosen_child);
        i = chosen_child;
    }
}

// sinks the element at index k, as described in the course.
fn sink<Item>(vec: &mut Vec<Item>, k: usize)
    where Item: PartialOrd {
    let size = vec.len();
    sink_with_size(vec, k, size);
}

pub fn make_heap<Item>(vec: &mut Vec<Item>)
    where Item: PartialOrd {
    let size = vec.len();
    for k in (0..((size + 1 ) / 2)).rev() {
        sink(vec, k);
    }
}

pub fn heap_sort<Item>(vec: &mut Vec<Item>)
    where Item: PartialOrd {
    let size = vec.len();
    for k in (1..size).rev() {
        vec.swap(0, k);
        sink_with_size(vec, 0, k);
    }
}

impl<Item> MaxPQ for Vec<Item>
    where Item: PartialOrd {
    type Item = Item;

    fn heap_insert(&mut self, item: Item) {
        self.push(item);
        let size = self.len();
        swim(self, size - 1);
    }

    fn del_max(&mut self) -> Option<Item> {
        let size = self.len();
        if size <= 1 {
            self.pop()
        } else {
            self.swap(0, size - 1);
            let rslt = self.pop();
            sink(self, 0);
            rslt
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn size(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{MaxPQ, parent, make_heap, sink, heap_sort};
    use ::is_sorted::is_sorted;

    pub fn is_binary_heap<Item>(vec: &Vec<Item>) -> bool
        where Item: PartialOrd {
        let size = vec.len();
        if size <= 1 {
            true
        } else {
            for k in 1..size {
                if vec[parent(k)] < vec[k] {
                    return false;
                }
            }
            true
        }
    }

    #[test]
    fn test_is_binary_heap() {
        assert_eq!(is_binary_heap(&vec!(19, 18, 17, 16, 15, 14, 13, 12, 11, 10)), true);
        assert_eq!(is_binary_heap(&vec!(10, 10, 10, 10, 10, 10, 10, 10, 10, 10)), true);
        assert_eq!(is_binary_heap(&vec!(34, 30, 29, 27, 25, 17, 16, 19, 22, 24)), true);
        assert_eq!(is_binary_heap(&vec!(30, 27, 23, 17, 16, 15, 14, 13, 18, 11)), false);
    }

    #[test]
    fn test_insert() {
        {
            let mut vec = vec!(19, 18, 17, 16, 15, 14, 13, 12, 11, 10);
            vec.heap_insert(16);
            assert!(is_binary_heap(&vec));
        }
        {
            let mut vec = vec!(19, 18, 17, 16, 15, 14, 13, 12, 11, 10);
            vec.heap_insert(12);
            assert!(is_binary_heap(&vec));
        }
        {
            let mut vec = vec!(10, 10, 10, 10, 10, 10, 10, 10, 10, 10);
            vec.heap_insert(9);
            assert!(is_binary_heap(&vec));
        }
        {
            let mut vec = vec!(10, 10, 10, 10, 10, 10, 10, 10, 10, 10);
            vec.heap_insert(12);
            assert!(is_binary_heap(&vec));
        }
        {
            let mut vec = vec!(10, 10, 10, 10, 10, 10, 10, 10, 10, 10);
            vec.heap_insert(10);
            assert!(is_binary_heap(&vec));
        }
    }

    #[test]
    fn test_del_max() {
        {
            let mut vec: Vec<i32> = vec!();
            assert_eq!(vec.del_max(), None);
            assert!(is_binary_heap(&vec));
        }
        {
            let mut vec = vec!(32);
            assert_eq!(vec.del_max(), Some(32));
            assert!(is_binary_heap(&vec));
            assert!(vec.is_empty());
        }
        {
            let mut vec = vec!(19, 18, 17, 16, 15, 14, 13, 12, 11, 10);
            assert_eq!(vec.del_max(), Some(19));
            assert!(is_binary_heap(&vec));
            assert_eq!(vec.len(), 9);
        }
        {
            let mut vec = vec!(10, 10, 10, 10, 10, 10, 10, 10, 10, 10);
            assert_eq!(vec.del_max(), Some(10));
            assert!(is_binary_heap(&vec));
            assert_eq!(vec.len(), 9);
        }
    }

    #[test]
    fn test_manual_sinking() {
        let mut vec = vec!(1, 89, 4, 78, 4, 9, 346, 9, 3, 56, 2, 56, 2, 6);
        //              1,
        //       89,          4,
        //  78,     4,    9,    346,
        // 9, 3, 56, 2, 56, 2, 6
        sink(&mut vec, 6);
        //              1,
        //       89,          4,
        //  78,     4,    9,    346,
        // 9, 3, 56, 2, 56, 2, 6
        assert_eq!(vec, vec!(1, 89, 4, 78, 4, 9, 346, 9, 3, 56, 2, 56, 2, 6));
        sink(&mut vec, 5);
        //              1,
        //       89,          4,
        //  78,     4,   56,    346,
        // 9, 3, 56, 2, 9, 2, 6
        assert_eq!(vec, vec!(1, 89, 4, 78, 4, 56, 346, 9, 3, 56, 2, 9, 2, 6));
        sink(&mut vec, 4);
        //              1,
        //       89,         4,
        //  78,    56,  56,    346,
        // 9, 3, 4, 2, 9, 2, 6
        assert_eq!(vec, vec!(1, 89, 4, 78, 56, 56, 346, 9, 3, 4, 2, 9, 2, 6));
        sink(&mut vec, 3);
        //              1,
        //       89,         4,
        //  78,   56,    56,   346,
        // 9, 3, 4, 2, 9, 2, 6
        assert_eq!(vec, vec!(1, 89, 4, 78, 56, 56, 346, 9, 3, 4, 2, 9, 2, 6));
        sink(&mut vec, 2);
        //              1,
        //       89,       346,
        //  78,    56,  56,    6,
        // 9, 3, 4, 2, 9, 2, 4
        assert_eq!(vec, vec!(1, 89, 346, 78, 56, 56, 6, 9, 3, 4, 2, 9, 2, 4));
        sink(&mut vec, 1);
        //              1,
        //       89,       346,
        //  78,    56,  56,    6,
        // 9, 3, 4, 2, 9, 2, 4
        assert_eq!(vec, vec!(1, 89, 346, 78, 56, 56, 6, 9, 3, 4, 2, 9, 2, 4));
        sink(&mut vec, 0);
        //            346,
        //       89,       56,
        //  78,    56,   9,    6,
        // 9, 3, 4, 2, 1, 2, 4
        assert_eq!(vec, vec!(346, 89, 56, 78, 56, 9, 6, 9, 3, 4, 2, 1, 2, 4));
        assert!(is_binary_heap(&vec));
    }

    #[test]
    fn test_make_heap() {
        let mut vec = vec!(1, 89, 4, 78, 4, 9, 346, 9, 3, 56, 2, 56, 2, 6);
        make_heap(&mut vec);
        assert!(is_binary_heap(&vec));
    }

    #[test]
    fn test_heap_sort() {
        let mut vec = vec!(1, 89, 4, 78, 4, 9, 346, 9, 3, 56, 2, 56, 2, 6);
        make_heap(&mut vec);
        assert!(is_binary_heap(&vec));
        heap_sort(&mut vec);
        assert!(is_sorted(&mut vec));
    }
}

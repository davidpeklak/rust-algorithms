//! provides an implementation for a double-ended queue (Deque)
//! #Example
//! ```
//! use algorithms::deque::{Deque, DequeT};
//!
//! let mut deque = Deque::<i32>::new();
//! deque.add_first(1);
//! deque.add_first(2);
//!
//! assert_eq!(deque.size(), 2);
//! assert_eq!(deque.remove_last(), Option::Some(1));
//! assert_eq!(deque.remove_last(), Option::Some(2));
//! assert_eq!(deque.size(), 0);
//! assert_eq!(deque.remove_last(), Option::None);
//! ```

use std::option::Option::*;
use std::ptr::NonNull;

pub trait DequeT {
    type Item;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
    fn add_first(&mut self, item: Self::Item);
    fn add_last(&mut self, item: Self::Item);
    fn remove_first(&mut self) -> Option<Self::Item>;
    fn remove_last(&mut self) -> Option<Self::Item>;
}

pub struct Deque<Item> {
    size: usize,
    first: Option<NonNull<Node<Item>>>,
    last: Option<NonNull<Node<Item>>>,
}

impl<Item> Deque<Item> {
    pub fn new() -> Deque<Item> {
        Deque {
            size: 0,
            first: None,
            last: None,
        }
    }

    pub fn iter<'a>(&'a self) -> DequeIter<'a, Item> {
        DequeIter {
            next: self.first,
            _deque: &self,
        }
    }
}

impl<Item> DequeT for Deque<Item> {
    type Item = Item;

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }

    fn add_first(&mut self, item: <Self as DequeT>::Item) {
        let node = Node::new(item);
        let node_ptr = box_and_get_ptr(node);
        // from here on, the Deque owns the node and must manage its livetime
        Node::connect(node_ptr, self.first);
        self.first = node_ptr;
        if self.size == 0 {
            self.last = node_ptr;
        }
        self.size = self.size + 1;
    }

    fn add_last(&mut self, item: <Self as DequeT>::Item) {
        let node = Node::new(item);
        let node_ptr = box_and_get_ptr(node);
        // from here on, the Deque owns the node and must manage its livetime
        Node::connect(self.last, node_ptr);
        self.last = node_ptr;
        if self.size == 0 {
            self.first = node_ptr;
        }
        self.size = self.size + 1;
    }

    fn remove_first(&mut self) -> Option<<Self as DequeT>::Item> {
        match self.first {
            None => None,
            Some(nnn) => {
                self.size = self.size - 1;
                if self.size == 0 {
                    self.first = None;
                    self.last = None;
                } else {
                    let node = unsafe { nnn.as_ref() };
                    self.first = node.next;
                    Node::connect(None, self.first);
                }
                Some(deallocate_and_return_item(nnn))
            }
        }
    }

    fn remove_last(&mut self) -> Option<<Self as DequeT>::Item> {
        match self.last {
            None => None,
            Some(nnn) => {
                self.size = self.size - 1;
                if self.size == 0 {
                    self.first = None;
                    self.last = None;
                } else {
                    let node = unsafe { nnn.as_ref() };
                    self.last = node.prev;
                    Node::connect(self.last, None);
                }
                Some(deallocate_and_return_item(nnn))
            }
        }
    }
}

impl<Item> Drop for Deque<Item> {
    fn drop(&mut self) {
        while self.remove_first().is_some() {}
    }
}

struct Node<Item> {
    val: Item,
    next: Option<NonNull<Node<Item>>>,
    prev: Option<NonNull<Node<Item>>>,
}

impl<Item> Node<Item> {
    fn new(item: Item) -> Node<Item> {
        Node {
            val: item,
            next: None,
            prev: None,
        }
    }

    fn connect(first: Option<NonNull<Node<Item>>>, second: Option<NonNull<Node<Item>>>) {
        match first {
            Some(mut nnn) => unsafe { nnn.as_mut().next = second },
            _ => ()
        }
        match second {
            Some(mut nnn) => unsafe { nnn.as_mut().prev = first },
            _ => ()
        }
    }
}

pub struct DequeIter<'a, Item>
    where Item: 'a {
    next: Option<NonNull<Node<Item>>>,
    _deque: &'a Deque<Item>,
}

impl<'a, Item> Iterator for DequeIter<'a, Item> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<&'a Item> {
        match self.next.clone() {
            None => None,
            Some(ref nnn) => {
                let node = unsafe { &*nnn.as_ptr() };
                self.next = node.next;
                Some(&node.val)
            }
        }
    }
}

fn box_and_get_ptr<Item>(node: Node<Item>) -> Option<NonNull<Node<Item>>> {
    let x = Box::new(node);
    let raw = Box::into_raw(x);
    NonNull::new(raw)
}

fn deallocate_and_return_item<Item>(nnn: NonNull<Node<Item>>) -> Item {
    // rebuild a Box around the raw pointer so that the Node gets deallocated when the Box gets
    // out of scope
    let x = unsafe { Box::from_raw(nnn.as_ptr()) };
    x.val
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;
    use std::mem;
    use std::ptr::NonNull;
    use std::option::Option::*;
    use super::{Deque, DequeT, Node};


    fn non_null_from_box(bx: Box<i32>) -> Option<NonNull<i32>> {
        let raw = Box::into_raw(bx);
        NonNull::new(raw)
    }

    fn box_from_non_null(onn: Option<NonNull<i32>>) -> Option<Box<i32>> {
        match onn {
            None => None,
            Some(nn) => Some(unsafe { Box::from_raw(nn.as_ptr()) })
        }
    }

    // size requirements of the deque in Programming Assignment 2 of the course
    #[test]
    fn sizes() {
        assert!(mem::size_of::<Node<i64>>() <= 48);
        assert!(mem::size_of::<Deque<i64>>() <= 192);
    }

    #[test]
    fn box_to_non_null_to_box() {
        let bx = Box::new(2i32);
        let onn = non_null_from_box(bx);
        let _bx = box_from_non_null(onn);
    }

    #[test]
    fn construct_deque_and_query() {
        let mut deque = Deque::<i32>::new();

        assert_eq!(deque.size(), 0);
        assert!(deque.is_empty());
        assert_eq!(deque.remove_first(), None);
        assert_eq!(deque.remove_last(), None);
    }

    #[test]
    fn add_first_and_query() {
        let mut deque = Deque::<i32>::new();

        deque.add_first(1);

        assert_eq!(deque.size(), 1);
        assert!(!deque.is_empty());
        assert_eq!(deque.remove_first(), Some(1));
        assert_eq!(deque.size(), 0);
        assert!(deque.is_empty());
        assert_eq!(deque.remove_first(), None);
        assert_eq!(deque.size(), 0);
        assert!(deque.is_empty());
        assert_eq!(deque.remove_last(), None);
        assert_eq!(deque.size(), 0);
        assert!(deque.is_empty());
    }

    #[test]
    fn add_last_and_query() {
        let mut deque = Deque::<i32>::new();

        deque.add_last(1);

        assert_eq!(deque.size(), 1);
        assert!(!deque.is_empty());
        assert_eq!(deque.remove_last(), Some(1));
        assert_eq!(deque.size(), 0);
        assert!(deque.is_empty());
        assert_eq!(deque.remove_last(), None);
        assert_eq!(deque.size(), 0);
        assert!(deque.is_empty());
        assert_eq!(deque.remove_first(), None);
        assert_eq!(deque.size(), 0);
        assert!(deque.is_empty());
    }

    #[test]
    fn fifo_right() {
        let mut deque = Deque::<i32>::new();
        deque.add_first(1);
        deque.add_first(2);

        assert_eq!(deque.size(), 2);
        assert_eq!(deque.remove_last(), Some(1));
        assert_eq!(deque.size(), 1);
        assert_eq!(deque.remove_last(), Some(2));
        assert_eq!(deque.size(), 0);
        assert_eq!(deque.remove_last(), None);
    }

    #[test]
    fn fifo_left() {
        let mut deque = Deque::<i32>::new();
        deque.add_last(1);
        deque.add_last(2);

        assert_eq!(deque.size(), 2);
        assert_eq!(deque.remove_first(), Some(1));
        assert_eq!(deque.size(), 1);
        assert_eq!(deque.remove_first(), Some(2));
        assert_eq!(deque.size(), 0);
        assert_eq!(deque.remove_first(), None);
    }

    #[test]
    fn filo_right() {
        let mut deque = Deque::<i32>::new();
        deque.add_first(1);
        deque.add_first(2);

        assert_eq!(deque.size(), 2);
        assert_eq!(deque.remove_first(), Some(2));
        assert_eq!(deque.size(), 1);
        assert_eq!(deque.remove_first(), Some(1));
        assert_eq!(deque.size(), 0);
        assert_eq!(deque.remove_first(), None);
    }

    #[test]
    fn filo_left() {
        let mut deque = Deque::<i32>::new();
        deque.add_last(1);
        deque.add_last(2);

        assert_eq!(deque.size(), 2);
        assert_eq!(deque.remove_last(), Some(2));
        assert_eq!(deque.size(), 1);
        assert_eq!(deque.remove_last(), Some(1));
        assert_eq!(deque.size(), 0);
        assert_eq!(deque.remove_last(), None);
    }

    #[test]
    fn iter() {
        let mut deque = Deque::<i32>::new();
        deque.add_last(1);
        deque.add_last(2);
        deque.add_first(3);

        let mut output = String::new();

        for i in deque.iter() {
            write!(&mut output, "{},", i).unwrap();
        }

        assert_eq!(output, "3,1,2,")
    }
}
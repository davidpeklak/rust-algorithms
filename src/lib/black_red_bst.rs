//! Implements a black-red-binary-search-tree as described in the course.

use std::mem;

#[derive(PartialEq)]
enum Color {
    Red,
    Black,
}

use self::Color::{Red, Black};

pub struct Link<Item> {
    to: Box<Node<Item>>,
    color: Color,
}

pub struct Node<Item> {
    value: Item,
    left: Option<Link<Item>>,
    right: Option<Link<Item>>,
}

pub fn modify<T, F>(t_box_ref_mut: &mut Box<T>, f: F)
    where F: Fn(T) -> T {
    let null_box = unsafe { Box::<T>::from_raw(std::ptr::null_mut()) };
    // temporarily put that box in place of self.to, in order to own self.to
    let t_box = mem::replace(t_box_ref_mut, null_box);
    let t = f(*t_box);
    let null_box = mem:: replace(t_box_ref_mut, Box::new(t));
    // consume the box that  holds the null pointer, so that it does not try to de-allocate
    // it later when it would be dropped
    mem::forget(null_box);
}

impl<Item> Link<Item>
    where Item: PartialOrd
    {
    pub fn new_red(node: Node<Item>) -> Link<Item> {
        Link { to: Box::new(node), color: Red }
    }

    pub fn new_black(node: Node<Item>) -> Link<Item> {
        Link { to: Box::new(node), color: Black }
    }

    pub fn value_ref(&self) -> &Item {
        &self.to.as_ref().value
    }

    pub fn value(&self) -> Item
    where Item: Copy {
        self.to.as_ref().value
    }

    pub fn rotate_left(&mut self) {
        modify(&mut self.to, |n| n.rotate_left());
    }
}

impl<Item> Node<Item>
    where Item: PartialOrd {
    pub fn new(value: Item) -> Node<Item>
    {
        Node { value, left: None, right: None }
    }

    pub fn insert_val(&mut self, value: Item) {
        let node = Node::new(value);
        self.insert_node(node);
    }

    pub fn insert_node(&mut self, node: Node<Item>) {
        match self {
            Node { value: v, left: None, right: None } if *v >= node.value =>
                self.left = Some(Link::new_red(node)),
            Node { value: _, left: None, right: None } /* if *v < boxed_node.value */ =>
                self.right = Some(Link::new_red(node)),
            _ => unimplemented!()
        }
    }

    pub fn left_value_ref(&self) -> Option<&Item> {
        self.left.as_ref().map(|l| &l.to.value)
    }

    pub fn left_value(&self) -> Option<Item>
        where Item: Copy {
        self.left.as_ref().map(|l| l.to.value)
    }

    pub fn right_value_ref(&self) -> Option<&Item> {
        self.right.as_ref().map(|l| &l.to.value)
    }

    pub fn right_value(&self) -> Option<Item>
        where Item: Copy {
        self.right.as_ref().map(|l| l.to.value)
    }

    pub fn rotate_left(self) -> Node<Item> {
        // I run into https://github.com/rust-lang/rust/issues/16223
        // so I unglily have to split up the
        // pattern matching in two:

        // first part of the pattern
        if let Node {
            value: top_value,
            left,
            right: Some(Link {
                            to: right_node,
                            color: Red
                        })
        } = self {
            // deref the box
            let right_node = *right_node;
            // second part of the pattern
            let Node {
                value: right_value,
                left: middle,
                right
            } = right_node;
            Node {
                value: right_value,
                left: Some(Link {
                    to: Box::new(Node {
                        value: top_value,
                        left,
                        right: middle,
                    }),
                    color: Red,
                }),
                right,
            }
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, Link};
    use super::Color::Red;

    #[test]
    fn test_new() {
        let node = Node::new(32);
        assert_eq!(node.value, 32);
    }

    #[test]
    fn test_left_value_ref() {
        let left_node = Node::new(20);
        let link = Link { to: Box::new(left_node), color: Red };
        let node = Node { value: 30, left: Some(link), right: None };

        assert_eq!(node.left_value_ref(), Some(&20));
    }

    #[test]
    fn test_left_value() {
        let left_node = Node::new(20);
        let link = Link { to: Box::new(left_node), color: Red };
        let node = Node { value: 30, left: Some(link), right: None };

        assert_eq!(node.left_value(), Some(20));
    }

    #[test]
    fn insert_lower_node_into_empty_node() {
        let mut node = Node::new(32);
        node.insert_node(Node::new(20));

        assert_eq!(node.left_value(), Some(20));
    }

    #[test]
    fn insert_lower_value_into_empty_node() {
        let mut node = Node::new(32);
        node.insert_val(20);

        assert_eq!(node.left_value(), Some(20));
    }

    #[test]
    fn insert_higher_value_into_empty_node() {
        let mut node = Node::new(32);
        node.insert_val(40);

        assert_eq!(node.right_value(), Some(40));
    }

    #[test]
    fn rotate_left() {
        let mut node = Node::new(32);
        node.insert_val(40);

        let mut link = Link::new_black(node);

        link.rotate_left();

        assert_eq!(link.value(), 40);
        let node = link.to.as_ref();
        let left_val = node.left.as_ref().unwrap().value();
        assert_eq!(left_val, 32);
        assert!(link.to.as_ref().right.is_none());
    }
}

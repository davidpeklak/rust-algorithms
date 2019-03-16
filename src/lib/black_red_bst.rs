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

impl<Item> Link<Item> {
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
        // get the rigth link out as an Option, if there is one. Self needs to be borrowed
        // mutably for that, so we have to close the block and continue working with the resulting
        // option
        let right_link_opt = {
            let Link { to, .. } = self;
            let to_node = to.as_mut();
            if let Node {
                right: right @ Some(Link { to: _, color: Red }),
                ..
            } = to_node {
                // replace right by a None, and get it out
                mem::replace(right, None)
            } else {
                None
            }
        };
        // here we have self back, and can work with the option.
        if let Some(Link { to: right_node, .. }) = right_link_opt {
            // link the right node into self, and get out self.to
            let mut left_node = mem::replace(&mut self.to, right_node);
            // now right_node is consumed, so I need to get it again.
            let right_node = &mut self.to;
            let right_node = right_node.as_mut();
            let middle = mem::replace(&mut right_node.left, Some(Link { to: left_node, color: Red }));
            // now the left_node is consumed, so I need to get it again.
            let left_link = &mut right_node.left;
            if let Some(Link { to: left_node, .. }) = left_link {
                // this will always be the case, because that is how we constructed it 3 lines above
                // plug middle into left_node
                left_node.right = middle;
            }
        }
    }

    pub fn rotate_left_a_little_unsafe(&mut self) {
        // create a box that contains null
        let null_node_box = unsafe { Box::<Node<Item>>::from_raw(std::ptr::null_mut()) };
        // temporarily put that box in place of self.to, in order to own self.to
        let to = mem::replace(&mut self.to, null_node_box);
        let to = *to;
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
        } = to {
            // deref the box
            let right_node = *right_node;
            // second part of the pattern
            let Node {
                value: right_value,
                left: middle,
                right
            } = right_node;
            let null_node_box = mem::replace(&mut self.to, Box::new(Node {
                value: right_value,
                left: Some(Link {
                    to: Box::new(Node {
                        value: top_value,
                        left: left,
                        right: middle,
                    }),
                    color: Red,
                }),
                right: right,
            }));
            // consume the box that  holds the null pointer, so that it does not try to de-allocate
            // it later when it would be dropped
            Box::into_raw(null_node_box);
        }
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

    #[test]
    fn rotate_left_a_little_unsafe() {
        let mut node = Node::new(32);
        node.insert_val(40);

        let mut link = Link::new_black(node);

        link.rotate_left_a_little_unsafe();

        assert_eq!(link.value(), 40);
        let node = link.to.as_ref();
        let left_val = node.left.as_ref().unwrap().value();
        assert_eq!(left_val, 32);
        assert!(link.to.as_ref().right.is_none());
    }
}

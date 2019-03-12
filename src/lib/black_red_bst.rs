//! Implements a black-red-binary-search-tree as described in the course.

enum Color {
    Red,
    Black,
}

use self::Color::{Red, Black};

struct Link<Item> {
    to: Box<Node<Item>>,
    color: Color,
}

struct Node<Item> {
    value: Item,
    left: Option<Link<Item>>,
    right: Option<Link<Item>>,
}

impl<Item> Link<Item> {
    fn new_red(node: Node<Item>) -> Link<Item> {
        Link{ to: Box::new(node), color: Red}
    }

    fn new_black(node: Node<Item>) -> Link<Item> {
        Link{ to: Box::new(node), color: Black}
    }
}

impl<Item> Node<Item>
    where Item: PartialOrd {
    fn new(value: Item) -> Node<Item>
    {
        Node { value, left: None, right: None }
    }

    fn insert_val(&mut self, value: Item) {
        let node = Node::new(value);
        self.insert_node(node);
    }

    fn insert_node(&mut self, node: Node<Item>) {
        match self {
            Node { value: v, left: None, right: None } if *v >= node.value =>
                self.left = Some(Link::new_red(node)),
            Node { value: v, left: None, right: None } /* if *v < boxed_node.value */ =>
                 self.right = Some(Link::new_red(node)),
            _ => unimplemented!()
        }
    }

    fn left_value_ref(&self) -> Option<&Item> {
        self.left.as_ref().map(|l| &l.to.value)
    }

    fn left_value(&self) -> Option<Item>
    where Item: Copy {
        self.left.as_ref().map(|l| l.to.value)
    }

    fn right_value_ref(&self) -> Option<&Item> {
        self.right.as_ref().map(|l| &l.to.value)
    }

    fn right_value(&self) -> Option<Item>
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
        let link = Link{to: Box::new(left_node), color: Red};
        let node = Node{ value: 30, left: Some(link), right: None};

        assert_eq!(node.left_value_ref(), Some(&20));
    }

    #[test]
    fn test_left_value() {
        let left_node = Node::new(20);
        let link = Link{to: Box::new(left_node), color: Red};
        let node = Node{ value: 30, left: Some(link), right: None};

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
}
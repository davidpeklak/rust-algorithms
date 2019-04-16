//! Implements a red-black-binary-search-tree as described in the course.

use std::{cmp, mem};
use std::ops::Range;

use self::Color::{Red, Black};
use self::Link::{ColoredLink, End};

#[derive(Eq, PartialEq, Debug)]
enum Color {
    Red,
    Black
}

#[derive(Eq, PartialEq, Debug)]
enum Link<Item> {
    End,
    ColoredLink {
        color: Color,
        node_box: Box<Node<Item>>,
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Node<Item> {
    value: Item,
    left: Link<Item>,
    right: Link<Item>,
}

impl<Item> Link<Item>
    where Item: PartialOrd {

    #[allow(dead_code)] // for testing
    fn value_ref(&self) -> Option<&Item> {
        match self {
            End => None,
            ColoredLink{node_box, .. } => Some(&node_box.value)
        }
    }

    #[allow(dead_code)] // for testing
    fn value(&self) -> Option<Item>
        where Item: Copy {
        match self {
            End => None,
            ColoredLink{node_box, .. } => Some(node_box.value)
        }
    }

    fn insert_val(&mut self, value: Item) {
        let node = Node::new(value);
        self.insert_node(node);
    }

    fn insert_node(&mut self, node: Node<Item>) {
        match self {
            End => {
                mem::replace(self, ColoredLink { color: Red, node_box: Box::new(node) });
            },
            ColoredLink { node_box, .. } => node_box.insert_node(node)
        }

        self.rotate_left();
        self.rotate_right();
        self.color_flip();
    }

    fn make_black(&mut self) {
        match self {
            End => (),
            ColoredLink { ref mut color, .. } => *color = Black
        }
    }

    fn modify<F>(&mut self, f: F)
    where F: Fn(Link<Item>) -> Link<Item> {
        let self_owned = mem::replace(self, End);
        let new_self = f(self_owned);
        mem::replace(self, new_self);
    }

    fn consume_node_consume<F>(self, f: F) -> Link<Item>
    where F: Fn(Node<Item>) -> Node<Item> {
        match self {
            End => self,
            ColoredLink { color, node_box } => {
                let new_node = f(*node_box);
                ColoredLink { color, node_box: Box::new(new_node) }
            }
        }
    }

    fn rotate_left(&mut self) {
        Link::modify(self, |l| l.consume_node_consume(|n| n.rotate_left()));
    }

    fn rotate_right(&mut self) {
        Link::modify(self, |l| l.consume_node_consume(|n| n.rotate_right()));
    }

    fn color_flip(&mut self) {
        if let ColoredLink { color: top_color@Black, node_box } = self {
            if let Node {
                left: ColoredLink { color: left_color@Red, .. },
                right: ColoredLink { color: right_color@Red, .. },
                ..
            } = node_box.as_mut() {
                mem::replace(top_color, Red);
                mem::replace(left_color, Black);
                mem::replace(right_color, Black);
            }
        }
    }

    /// tests if the link is the top of a binary search tree, i.e. if the left of each node is
    /// lower than the node itself, and if the rigth of each node is greater than the node itself
    #[cfg(test)]
    fn is_bst(&self) -> bool {
        match &self {
            End => true,
            ColoredLink {node_box, ..} => {
                let node = node_box.as_ref();
                (
                    match &node.left {
                        End => true,
                        ColoredLink { node_box: left_node_box, .. } => {
                            left_node_box.as_ref().value < node.value && left_node_box.is_bst()
                        }
                    }
                ) && (
                    match &node.right {
                        End => true,
                        ColoredLink { node_box: right_node_box, .. } => {
                            right_node_box.as_ref().value < node.value && right_node_box.is_bst()
                        }
                    }
                )
            }
        }
    }

    /// returns the number of black links found under the node, as a range The start of the range is the
    /// lowest depth found, the end of the range minus one is the highest depth found.
    #[cfg(test)]
    fn black_depth(&self) -> Range<usize> {
        match &self {
            End => 0..1,
            ColoredLink { color: Black, node_box} => {
                let node_depth = node_box.as_ref().black_depth();
                (node_depth.start + 1) .. (node_depth.end + 1)
            },
            ColoredLink { color: Red, node_box} => node_box.as_ref().black_depth()
        }
    }

    #[cfg(test)]
    fn total_depth(&self) -> Range<usize> {
        match &self {
            End => 0..1,
            ColoredLink { node_box,.. } => {
                let node_depth = node_box.as_ref().total_depth();
                (node_depth.start + 1) .. (node_depth.end + 1)
            }
        }
    }

    #[cfg(test)]
    fn is_black_balanced(&self) -> bool {
        let black_depth = self.black_depth();
        black_depth.end - black_depth.start <= 1
    }

    #[cfg(test)]
    fn has_right_leaning_red_links(&self) -> bool {
        match &self {
            End => false,
            ColoredLink { node_box, .. } => node_box.has_right_leaning_red_links()
        }
    }

    #[cfg(test)]
    fn has_consecutive_red_links(&self) -> bool {
        match &self {
            End => false,
            ColoredLink { node_box, color: Red } => {
                let node = node_box.as_ref();
                match node {
                    Node { right: ColoredLink { color: Red, ..}, .. } => true,
                    Node { left: ColoredLink { color: Red, .. }, .. } => true,
                    Node { left, right, ..} =>
                        left.has_consecutive_red_links() || right.has_consecutive_red_links()
                }
            },
            ColoredLink {node_box, .. }=> {
                node_box.left.has_consecutive_red_links() || node_box.right.has_consecutive_red_links()
            }
        }
    }
}

impl<Item> Node<Item>
    where Item: PartialOrd {
    fn new(value: Item) -> Node<Item>
    {
        Node { value, left: End, right: End }
    }

    fn insert_val(&mut self, value: Item) {
        let node = Node::new(value);
        self.insert_node(node);
    }

    fn insert_node(&mut self, node: Node<Item>) {
        match self {
            Node { value: v, ..} if *v == node.value =>
                (),
            Node { value: v, left: End, .. } if *v > node.value =>
                self.left = ColoredLink { color: Red, node_box: Box::new(node) },
            Node { value: v, right: End, .. } if *v < node.value =>
                self.right = ColoredLink { color: Red, node_box: Box::new(node) },
            Node { value: v, left: left@ColoredLink { .. }, .. } if *v > node.value =>
                left.insert_node(node),
            Node { value: v, right: right@ColoredLink { .. }, .. } if *v < node.value =>
                right.insert_node(node),
            _ => unreachable!()
        }
    }

    fn left_value_ref(&self) -> Option<&Item> {
        self.left.value_ref()
    }

    fn left_value(&self) -> Option<Item>
        where Item: Copy {
        self.left.value()
    }

    fn right_value_ref(&self) -> Option<&Item> {
        self.right.value_ref()
    }

    fn right_value(&self) -> Option<Item>
        where Item: Copy {
        self.right.value()
    }

    fn rotate_left(self) -> Node<Item> {
        // I run into https://github.com/rust-lang/rust/issues/16223
        // so I unglily have to split up the
        // pattern matching in two:

        // first part of the pattern
        if let Node {
            value: top_value,
            left,
            right: ColoredLink{ color: Red, node_box: right_node }
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
                left: ColoredLink {
                    color: Red,
                    node_box: Box::new(Node {
                        value: top_value,
                        left,
                        right: middle,
                    })
                },
                right,
            }
        } else {
            self
        }
    }

    fn rotate_right(self) -> Node<Item> {
        if let Node {
            value: top_value,
            left: ColoredLink{ color: Red, node_box: left_node },
            right
        } = self {
            // deref the box
            let left_node = *left_node;
            // second part of the pattern
            if let Node {
                value: left_value,
                left: left@ColoredLink {
                    color: Red,
                    ..
                },
                right: middle
            } = left_node {
                Node {
                    value: left_value,
                    left,
                    right: ColoredLink {
                        color: Red,
                        node_box: Box::new(Node {
                            value: top_value,
                            left: middle,
                            right
                        })
                    }
                }
            } else {
                // the second pattern does not match, so we need to re-construct the first one,
                // because we already de-constructed it, sigh.
                Node {
                    value: top_value,
                    left: ColoredLink{ color: Red, node_box: Box::new(left_node) },
                    right
                }
            }
        } else {
            self
        }
    }

    /// tests if the link is the top of a binary search tree, i.e. if the left of each node is
    /// lower than the node itself, and if the rigth of each node is greater than the node itself
    #[cfg(test)]
    fn is_bst(&self) -> bool {
        let result = match &self.left {
            End => true,
            ColoredLink { node_box, .. } => {
                let node = node_box.as_ref();
                node.value < self.value && node.is_bst()
            }
        } && match &self.right {
            End => true,
            ColoredLink { node_box, .. } => {
                let node = node_box.as_ref();
                node.value > self.value && node.is_bst()
            }
        };
        result
    }

    /// returns the number of black links found under the node, as a range The start of the range is the
    /// lowest depth found, the end of the range minus one is the highest depth found.
    #[cfg(test)]
    fn black_depth(&self) -> Range<usize> {
        let left_depth = self.left.black_depth();
        let right_depth = self.right.black_depth();
        cmp::min(left_depth.start, right_depth.start) .. cmp::max(left_depth.end, right_depth.end)
    }

    /// returns the number of links found under the node, as a range The start of the range is the
    /// lowest depth found, the end of the range minus one is the highest depth found.
    #[cfg(test)]
    fn total_depth(&self) -> Range<usize> {
        let left_depth = self.left.total_depth();
        let right_depth = self.right.total_depth();
        cmp::min(left_depth.start, right_depth.start) .. cmp::max(left_depth.end, right_depth.end)
    }

    #[cfg(test)]
    fn has_right_leaning_red_links(&self) -> bool {
        match &self {
            Node { right: ColoredLink { color: Red, ..}, ..} => true,
            Node { right, left, ..} => right.has_right_leaning_red_links() && left.has_right_leaning_red_links()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color::{Red, Black};
    use super::Link;
    use super::Link::{ColoredLink, End};
    use super::Node;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_new() {
        let node = Node::new(32);
        assert_eq!(node.value, 32);
    }

    #[test]
    fn test_left_value_ref() {
        let left_node = Node::new(20);
        let link = ColoredLink{ color: Red, node_box: Box::new(left_node)};
        let node = Node { value: 30, left: link, right: End };

        assert_eq!(node.left_value_ref(), Some(&20));
    }

    #[test]
    fn test_left_value() {
        let left_node = Node::new(20);
        let link = ColoredLink{color: Red, node_box: Box::new(left_node)};
        let node = Node { value: 30, left: link, right: End };

        assert_eq!(node.left_value(), Some(20));
    }

    #[test]
    fn insert_lower_value() {
        let mut link: Link<i32> = Link::End;
        link.insert_val(32);
        link.insert_val(20);

        let expected = ColoredLink {
            color: Red,
            node_box: Box::new(Node {
                value: 32,
                left: ColoredLink {
                    color: Red,
                    node_box: Box::new(Node {
                        value: 20,
                        left: End,
                        right: End,
                    }),
                },
                right: End,
            }),
        };

        assert_eq!(expected, link);
        assert!(link.is_bst());
    }

    #[test]
    fn insert_higher_value() {
        let mut link: Link<i32> = Link::End;
        link.insert_val(32);
        link.insert_val(40);

        let expected = ColoredLink {
            color: Red,
            node_box: Box::new(Node {
                value: 40,
                left: ColoredLink {
                    color: Red,
                    node_box: Box::new(Node {
                        value: 32,
                        left: End,
                        right: End,
                    }),
                },
                right: End,
            }),
        };

        assert_eq!(expected, link);
        assert!(link.is_bst());
    }

    #[test]
    fn test_rotate_left() {
        let mut link = ColoredLink {
            color: Black,
            node_box: Box::new(Node {
                value: 32,
                left: ColoredLink {
                    color: Black,
                    node_box: Box::new(Node {
                        value: 20,
                        left: End,
                        right: End,
                    }),
                },
                right: ColoredLink {
                    color: Red,
                    node_box: Box::new(Node {
                        value: 40,
                        left: End,
                        right: End,
                    }),
                },
            }),
        };

        link.rotate_left();

        let expectation = ColoredLink {
            color: Black,
            node_box: Box::new(Node {
                value: 40,
                left: ColoredLink {
                    color: Red,
                    node_box: Box::new(Node {
                        value: 32,
                        left: ColoredLink {
                            color: Black,
                            node_box: Box::new(Node {
                                value: 20,
                                left: End,
                                right: End,
                            })
                        },
                        right: End,
                    }),
                },
                right: End,
                }),
            };
        assert_eq!(expectation, link);
    }

    #[test]
    fn test_rotate_right() {
        let mut link = ColoredLink {
            color: Black,
            node_box: Box::new(Node {
                value: 40,
                left: ColoredLink {
                    color: Red,
                    node_box: Box::new(Node {
                        value: 32,
                        left: ColoredLink {
                            color: Red,
                            node_box: Box::new(Node {
                                value: 20,
                                left: End,
                                right: End,
                            })
                        },
                        right: End,
                    }),
                },
                right: End,
            }),
        };

        link.rotate_right();

        let expectation = ColoredLink {
            color: Black,
            node_box: Box::new(Node {
                value: 32,
                left: ColoredLink {
                    color: Red,
                    node_box: Box::new(Node {
                        value: 20,
                        left: End,
                        right: End,
                    }),
                },
                right: ColoredLink {
                    color: Red,
                    node_box: Box::new(Node {
                        value: 40,
                        left: End,
                        right: End,
                    }),
                },
            }),
        };
        assert_eq!(expectation, link);
    }


    #[test]
    fn color_flip() {
        let mut link = ColoredLink {
            color: Black,
            node_box: Box::new({ Node {
                value: 32,
                left: ColoredLink {
                    color: Red,
                    node_box: Box::new(Node::new(20))
                },
                right: ColoredLink {
                    color: Red,
                    node_box: Box::new(Node::new(40))
                }
            }})
        };

        link.color_flip();

        let expected = ColoredLink {
            color: Red,
            node_box: Box::new({ Node {
                value: 32,
                left: ColoredLink {
                    color: Black,
                    node_box: Box::new(Node::new(20))
                },
                right: ColoredLink {
                    color: Black,
                    node_box: Box::new(Node::new(40))
                }
            }})
        };

        assert_eq!(link, expected);
    }

    #[test]
    fn depth() {
        let link = ColoredLink {
            color: Black,
            node_box: Box::new({ Node {
                value: 32,
                left: ColoredLink {
                    color: Red,
                    node_box: Box::new(Node::new(20))
                },
                right: ColoredLink {
                    color: Black,
                    node_box: Box::new(Node::new(40))
                }
            }})
        };

        let result = link.black_depth();
        assert_eq!(result, 1..3);
        let result = link.total_depth();
        assert_eq!(result, 2..3);

        let link = ColoredLink {
            color: Black,
            node_box: Box::new({ Node {
                value: 32,
                left: ColoredLink {
                    color: Red,
                    node_box: Box::new(Node::new(20))
                },
                right: End
            }})
        };

        let result = link.black_depth();
        assert_eq!(result, 1..2);
        let result = link.total_depth();
        assert_eq!(result, 1..3);
    }

    #[test]
    fn test_is_bst() {
        let node = Node {
            value: 32,
            left: ColoredLink {
                color: Red,
                node_box: Box::new(Node::new(20))
            },
            right: ColoredLink {
                color: Black,
                node_box: Box::new(Node::new(40))
            }
        };

        let result = node.is_bst();
        assert_eq!(result, true);

        let node = Node {
            value: 32,
            left: ColoredLink {
                color: Red,
                node_box: Box::new(Node::new(40))
            },
            right: ColoredLink {
                color: Black,
                node_box: Box::new(Node::new(20))
            }
        };

        let result = node.is_bst();
        assert_eq!(result, false);
    }

    /// confirms that inserting 5 values into a link results in a binary search tree. Does not
    /// test whether the tree is black balanced, nor if it is a valid red black tree.
    #[test]
    fn multiple_insertion_results_in_bst() {
        let mut tree: Link<i32> =  End;
        let mut rng = thread_rng();

        let number_of_nodes = 20;

        for _ in 0..number_of_nodes {
            let new_value: i32 = rng.gen_range(0, 100);
            tree.insert_val(new_value);
            tree.make_black()
        }
        assert!(tree.is_bst(), "{:?}", tree);
        assert!(tree.is_black_balanced(), "{:?}", tree);
        assert!(!tree.has_right_leaning_red_links(), "{:?}", tree);
        assert!(!tree.has_consecutive_red_links(), "{:?}", tree);
    }

}


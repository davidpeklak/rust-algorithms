//! Implements a red-black-binary-search-tree as described in the course.
//! Choses an approach with only one struct for both link and node

use self::Color::{Red, Black};
use self::Link::{ColoredLink, End};
use std::ops::Range;
use std::cmp;
use std::mem;

pub struct Tree<Item> {
    top: Link<Item>
}

impl<Item> Tree<Item>
    where Item: PartialOrd {

    pub fn new() -> Tree<Item> {
        Tree {
            top: End,
        }
    }

    pub fn insert(&mut self, value: Item) {
        let top = mem::replace(&mut self.top, End);
        let top = top.insert(value);
        let top = top.make_black();
        mem::replace(&mut self.top, top);
    }

    pub fn size(&self) -> usize {
        self.top.size()
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Color {
    Red,
    Black,
}

#[derive(Eq, PartialEq, Debug)]
enum Link<Item> {
    End,
    ColoredLink {
        color: Color,
        value: Item,
        left: Box<Link<Item>>,
        right: Box<Link<Item>>,
    },
}

impl<Item> Link<Item>
    where Item: PartialOrd {

    fn insert(self, value: Item) -> Link<Item> {
        match self {
            End => ColoredLink {
                color: Red,
                value,
                left: Box::new(End),
                right: Box::new(End),
            },
            ColoredLink {
                color,
                value: self_value,
                left,
                right
            } => {
                (
                    if value > self_value {
                        ColoredLink {
                            color,
                            value: self_value,
                            left,
                            right: Box::new(right.insert(value)),
                        }
                    } else if value < self_value {
                        ColoredLink {
                            color,
                            value: self_value,
                            left: Box::new(left.insert(value)),
                            right,
                        }
                    } else {
                        ColoredLink {
                            color,
                            value: self_value,
                            left,
                            right,
                        }
                    }
                )
                    .rotate_left()
                    .rotate_right()
                    .color_flip()
            }
        }
    }

    fn size(&self) -> usize {
        match self {
            End => 0,
            ColoredLink {
                left,
                right,
                ..
            } => left.size() + right.size() + 1
        }
    }

    // make a ColoredLink black. Used for the top of the tree
    fn make_black(self) -> Link<Item> {
        match self {
            End => End,
            ColoredLink {
                value,
                left,
                right,
                ..
            } => ColoredLink {
                color: Black,
                value,
                left,
                right
            }
        }
    }

    fn rotate_left(self) -> Link<Item> {
        if let ColoredLink {
            color,
            value: top_value,
            left,
            right
        } = self {
            let right = *right;
            if let ColoredLink {
                color: Red,
                value: right_value,
                left: middle,
                right
            } = right {
                // construct the rotated link
                ColoredLink {
                    color,
                    value: right_value,
                    left: Box::new(ColoredLink {
                        color: Red,
                        value: top_value,
                        left,
                        right: middle,
                    }),
                    right,
                }
            } else {
                // the second pattern does not match, so we need to re-construct the first one,
                // because we already de-constructed it
                ColoredLink {
                    color,
                    value: top_value,
                    left,
                    right: Box::new(right),
                }
            }
        } else {
            self
        }
    }

    fn rotate_right(self) -> Link<Item> {
        if let ColoredLink {
            color,
            value: top_value,
            left,
            right
        } = self {
            let left = *left;
            if let ColoredLink {
                color: Red,
                value: left_value,
                left,
                right: middle
            } = left {
                match left.as_ref() { // we are only checking the color of the link here, so we can match on its ref and do not need to deconstruct
                    ColoredLink { color: Red, .. } => {
                        // construct the rotated link
                        ColoredLink {
                            color,
                            value: left_value,
                            left,
                            right: Box::new(ColoredLink {
                                color: Red,
                                value: top_value,
                                left: middle,
                                right,
                            }),
                        }
                    }
                    _ => {
                        // the third pattern (the red color of the link) does not match, so we need to re-construct the first one,
                        // because we already de-constructed it
                        ColoredLink {
                            color,
                            value: top_value,
                            left: Box::new(ColoredLink {
                                color: Red,
                                value: left_value,
                                left,
                                right: middle,
                            }),
                            right,
                        }
                    }
                }
            } else {
                // the second pattern does not match, so we need to re-construct the first one,
                // because we already de-constructed it
                ColoredLink {
                    color,
                    value: top_value,
                    left: Box::new(left),
                    right,
                }
            }
        } else {
            self
        }
    }

    fn color_flip(self) -> Link<Item> {
        if let ColoredLink {
            color: Black,
            value,
            left,
            right
        } = self {
            let left = *left;
            let right = *right;

            match (left, right) {
                (ColoredLink {
                    color: Red,
                    value: left_value,
                    left: left_left,
                    right: left_right,
                }, ColoredLink {
                    color: Red,
                    value: right_value,
                    left: right_left,
                    right: right_right,
                }) => {
                    ColoredLink {
                        color: Red,
                        value,
                        left: Box::new(ColoredLink {
                            color: Black,
                            value: left_value,
                            left: left_left,
                            right: left_right,
                        }),
                        right: Box::new(ColoredLink {
                            color: Black,
                            value: right_value,
                            left: right_left,
                            right: right_right,
                        }),
                    }
                }
                (l, r) => ColoredLink {
                    color: Black,
                    value,
                    left: Box::new(l),
                    right: Box::new(r),
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
        match self {
            End => true,
            ColoredLink { value, left, right, .. } => {
                (
                    match left.as_ref() {
                        End => true,
                        ColoredLink { value: left_value, .. } => *value > *left_value && left.is_bst()
                    }
                ) && (
                    match right.as_ref() {
                        End => true,
                        ColoredLink { value: right_value, .. } => *value < *right_value && left.is_bst()
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
            ColoredLink { color: Black, left, right, .. } => {
                let left_depth = left.black_depth();
                let right_depth = right.black_depth();
                cmp::min(left_depth.start, left_depth.start) + 1..cmp::max(left_depth.end, right_depth.end) + 1
            }
            ColoredLink { color: Red, left, right, .. } => {
                let left_depth = left.black_depth();
                let right_depth = right.black_depth();
                cmp::min(left_depth.start, left_depth.start)..cmp::max(left_depth.end, right_depth.end)
            }
        }
    }

    /// returns the number of links (of whatever color) found under the node, as a range The start of the range is the
    /// lowest depth found, the end of the range minus one is the highest depth found.
    #[cfg(test)]
    fn total_depth(&self) -> Range<usize> {
        match &self {
            End => 0..1,
            ColoredLink { left, right, .. } => {
                let left_depth = left.total_depth();
                let right_depth = right.total_depth();
                cmp::min(left_depth.start, left_depth.start) + 1..cmp::max(left_depth.end, right_depth.end) + 1
            }
        }
    }

    /// checks if this link has a black balanced tree under it, i.e. the range returned by black_depth()
    /// is not wider than 1
    #[cfg(test)]
    fn is_black_balanced(&self) -> bool {
        let black_depth = self.black_depth();
        black_depth.end - black_depth.start <= 1
    }

    /// checks if this link has any right leaning red links under it
    #[cfg(test)]
    fn has_right_leaning_red_links(&self) -> bool {
        match &self {
            End => false,
            ColoredLink { left, right, .. } => {
                (
                    match right.as_ref() {
                        ColoredLink { color: Red, .. } => true,
                        _ => false
                    }
                ) || left.has_right_leaning_red_links() || right.has_right_leaning_red_links()
            }
        }
    }

    /// checks if the link has any consecutive red links under it
    #[cfg(test)]
    fn has_consecutive_red_links(&self) -> bool {
        match &self {
            End => false,

            ColoredLink { color: Red, left, right, .. } =>
                (
                    match left.as_ref() {
                        ColoredLink { color: Red, .. } => true,
                        _ => false
                    }
                ) || (
                    match right.as_ref() {
                        ColoredLink { color: Red, .. } => true,
                        _ => false
                    }
                ) || left.has_consecutive_red_links() || right.has_consecutive_red_links(),

            ColoredLink { left, right, .. } =>
                left.has_consecutive_red_links() || right.has_consecutive_red_links()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Tree, Link};
    use super::Link::{ColoredLink, End};
    use super::Color::{Black, Red};
    use rand::{thread_rng, Rng};

    /// returns an exemplary tree that is used in various tests
    fn link_1() -> Link<i32> {
        ColoredLink {
            color: Black,
            value: 32,
            left: Box::new(ColoredLink {
                color: Red,
                value: 20,
                left: Box::new(End),
                right: Box::new(ColoredLink {
                    color: Black,
                    value: 25,
                    left: Box::new(End),
                    right: Box::new(End),
                }),
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End),
            }),
        }
    }

    /// returns an exemplary tree that is used in various tests
    fn link_2() -> Link<i32> {
        ColoredLink {
            color: Black,
            value: 32,
            left: Box::new(ColoredLink {
                color: Red,
                value: 20,
                left: Box::new(ColoredLink {
                    color: Black,
                    value: 18,
                    left: Box::new(End),
                    right: Box::new(End),
                }),
                right: Box::new(ColoredLink {
                    color: Black,
                    value: 25,
                    left: Box::new(End),
                    right: Box::new(End),
                }),
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End),
            }),
        }
    }

    #[test]
    fn test_is_bst() {
        let link = link_1();

        assert!(link.is_bst(), "{:?}", link);

        let link = ColoredLink {
            color: Black,
            value: 32,
            left: Box::new(ColoredLink {
                color: Red,
                value: 40,
                left: Box::new(End),
                right: Box::new(End),
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End),
            }),
        };

        assert!(!link.is_bst(), "{:?}", link);
    }

    #[test]
    fn test_black_depth() {
        let link = link_1();

        let black_depth = link.black_depth();
        assert_eq!(black_depth, 1..3);

        let link = link_2();

        let black_depth = link.black_depth();
        assert_eq!(black_depth, 2..3);
    }

    #[test]
    fn test_total_depth() {
        let link = link_1();

        let total_depth = link.total_depth();
        assert_eq!(total_depth, 2..4);

        let link = link_2();

        let total_depth = link.total_depth();
        assert_eq!(total_depth, 3..4);
    }

    #[test]
    fn test_has_right_leaning_red_links() {
        let link = link_2();
        assert!(!link.has_right_leaning_red_links());

        let link = ColoredLink {
            color: Black,
            value: 32,
            left: Box::new(ColoredLink {
                color: Red,
                value: 20,
                left: Box::new(ColoredLink {
                    color: Black,
                    value: 18,
                    left: Box::new(End),
                    right: Box::new(End),
                }),
                right: Box::new(ColoredLink {
                    color: Red,
                    value: 25,
                    left: Box::new(End),
                    right: Box::new(End),
                }),
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End),
            }),
        };

        assert!(link.has_right_leaning_red_links());
    }

    #[test]
    fn test_has_consecutive_red_links() {
        let link = link_1();
        assert!(!link.has_consecutive_red_links());

        let link = ColoredLink {
            color: Black,
            value: 32,
            left: Box::new(ColoredLink {
                color: Red,
                value: 20,
                left: Box::new(ColoredLink {
                    color: Red,
                    value: 18,
                    left: Box::new(End),
                    right: Box::new(End),
                }),
                right: Box::new(ColoredLink {
                    color: Black,
                    value: 25,
                    left: Box::new(End),
                    right: Box::new(End),
                }),
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End),
            }),
        };

        assert!(link.has_consecutive_red_links());
    }

    #[test]
    fn test_rotate_left() {
        let link = ColoredLink {
            color: Black,
            value: 32,
            left: Box::new(ColoredLink {
                color: Black,
                value: 20,
                left: Box::new(End),
                right: Box::new(End),
            }),
            right: Box::new(ColoredLink {
                color: Red,
                value: 40,
                left: Box::new(End),
                right: Box::new(End),
            }),
        };

        let link = link.rotate_left();

        let expectation = ColoredLink {
            color: Black,
            value: 40,
            left: Box::new(ColoredLink {
                color: Red,
                value: 32,
                left: Box::new(ColoredLink {
                    color: Black,
                    value: 20,
                    left: Box::new(End),
                    right: Box::new(End),
                }),
                right: Box::new(End),
            }),
            right: Box::new(End),
        };
        assert_eq!(expectation, link);
    }

    #[test]
    fn test_rotate_right() {
        let link = ColoredLink {
            color: Black,
            value: 40,
            left: Box::new(ColoredLink {
                color: Red,
                value: 32,
                left: Box::new(ColoredLink {
                    color: Red,
                    value: 20,
                    left: Box::new(End),
                    right: Box::new(End),
                }),
                right: Box::new(End),
            }),
            right: Box::new(End),
        };

        let link = link.rotate_right();

        let expectation = ColoredLink {
            color: Black,
            value: 32,
            left: Box::new(ColoredLink {
                color: Red,
                value: 20,
                left: Box::new(End),
                right: Box::new(End),
            }),
            right: Box::new(ColoredLink {
                color: Red,
                value: 40,
                left: Box::new(End),
                right: Box::new(End),
            }),
        };
        assert_eq!(expectation, link);
    }

    #[test]
    fn test_color_flip() {
        let link = ColoredLink {
            color: Black,
            value: 32,
            left: Box::new(ColoredLink {
                color: Red,
                value: 20,
                left: Box::new(End),
                right: Box::new(End)
            }),
            right: Box::new(ColoredLink {
                color: Red,
                value: 40,
                left: Box::new(End),
                right: Box::new(End)
            })
        };

        let link = link.color_flip();

        let expectation = ColoredLink {
            color: Red,
            value: 32,
            left: Box::new(ColoredLink {
                color: Black,
                value: 20,
                left: Box::new(End),
                right: Box::new(End)
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End)
            })
        };

        assert_eq!(link, expectation);

        let link = ColoredLink {
            color: Black,
            value: 32,
            left: Box::new(ColoredLink {
                color: Red,
                value: 20,
                left: Box::new(End),
                right: Box::new(End)
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End)
            })
        };

        let link = link.color_flip();

        let expectation = ColoredLink {
            color: Black,
            value: 32,
            left: Box::new(ColoredLink {
                color: Red,
                value: 20,
                left: Box::new(End),
                right: Box::new(End)
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End)
            })
        };

        assert_eq!(link, expectation);
    }

    #[test]
    fn test_insert_1() {
        let result = End
            .insert(32)
            .make_black();

        let expectation = ColoredLink {
            color: Black,
            value: 32,
            left: Box::new(End),
            right: Box::new(End)
        };

        assert_eq!(expectation, result);
    }

    #[test]
    fn test_insert_2() {
        let result = End
            .insert(32)
            .make_black()
            .insert(20)
            .make_black();

        let expectation = ColoredLink {
            color: Black,
            value: 32,
            left: Box::new(ColoredLink {
                color: Red,
                value: 20,
                left: Box::new(End),
                right: Box::new(End),
            }),
            right: Box::new(End),
        };

        assert_eq!(expectation, result);
    }

    #[test]
    fn test_insert_3() {
        let result = End
            .insert(32)
            .make_black()
            .insert(20)
            .make_black()
            .insert(40)
            .make_black();

        let expectation = ColoredLink {
            color: Black,
            value: 32,
            left: Box::new(ColoredLink {
                color: Black,
                value: 20,
                left: Box::new(End),
                right: Box::new(End),
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End),
            }),
        };

        assert_eq!(expectation, result);
    }

    #[test]
    fn multiple_insertion_results_in_bst() {
        let mut tree: Link<i32> = End;
        let mut rng = thread_rng();

        let number_of_items = 20;

        for _ in 0..number_of_items {
            let new_value: i32 = rng.gen_range(0, 100);
            tree = tree.insert(new_value);
            tree = tree.make_black();
        }

        assert!(tree.is_bst(), "{:?}", tree);
        assert!(tree.is_black_balanced(), "{:?}", tree);
        assert!(!tree.has_right_leaning_red_links(), "{:?}", tree);
        assert!(!tree.has_consecutive_red_links(), "{:?}", tree);
    }

    #[test]
    fn test_size() {
        let mut tree = Tree::<i32>::new();
        tree.insert(32);
        tree.insert(20);
        tree.insert(45);

        assert_eq!(3, tree.size());
    }
}


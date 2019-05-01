//! Implements a red-black-binary-search-tree as described in the course.
//! Choses an approach with only one struct for both link and node

use self::Color::{Red, Black};
use self::Link::{ColoredLink, End};
use std::ops::Range;
use std::cmp;

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
        value: Item,
        left: Box<Link<Item>>,
        right: Box<Link<Item>>
    }
}

impl<Item> Link<Item>
    where Item: PartialOrd {

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
                cmp::min(left_depth.start, left_depth.start) + 1 .. cmp::max(left_depth.end, right_depth.end) + 1
            },
            ColoredLink { color: Red, left, right, .. } => {
                let left_depth = left.black_depth();
                let right_depth = right.black_depth();
                cmp::min(left_depth.start, left_depth.start) .. cmp::max(left_depth.end, right_depth.end)
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
                cmp::min(left_depth.start, left_depth.start) + 1 .. cmp::max(left_depth.end, right_depth.end) + 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Link::{ColoredLink, End};
    use super::Color::{Black, Red};

    #[test]
    fn test_is_bst() {
        let link = ColoredLink {
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
                    right: Box::new(End)
                })
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End)
            })
        };

        assert!(link.is_bst(), "{:?}", link);

        let link = ColoredLink {
            color: Black,
            value: 32,
            left: Box::new(ColoredLink {
                color: Red,
                value: 40,
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

        assert!(!link.is_bst(), "{:?}", link);
    }

    #[test]
    fn test_black_depth() {
        let link = ColoredLink {
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
                    right: Box::new(End)
                })
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End)
            })
        };

        let black_depth = link.black_depth();
        assert_eq!(black_depth, 1..3);

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
                    right: Box::new(End)
                }),
                right: Box::new(ColoredLink {
                    color: Black,
                    value: 25,
                    left: Box::new(End),
                    right: Box::new(End)
                })
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End)
            })
        };

        let black_depth = link.black_depth();
        assert_eq!(black_depth, 2..3);
    }

    #[test]
    fn test_total_depth() {
        let link = ColoredLink {
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
                    right: Box::new(End)
                })
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End)
            })
        };

        let total_depth = link.total_depth();
        assert_eq!(total_depth, 2..4);

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
                    right: Box::new(End)
                }),
                right: Box::new(ColoredLink {
                    color: Black,
                    value: 25,
                    left: Box::new(End),
                    right: Box::new(End)
                })
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End)
            })
        };

        let total_depth = link.total_depth();
        assert_eq!(total_depth, 3..4);
    }

}


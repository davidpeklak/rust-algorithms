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
                    },
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
    use super::Link;
    use super::Link::{ColoredLink, End};
    use super::Color::{Black, Red};

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
                    right: Box::new(End)
                })
            }),
            right: Box::new(ColoredLink {
                color: Black,
                value: 40,
                left: Box::new(End),
                right: Box::new(End)
            })
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
                    right: Box::new(End)
                }),
                right: Box::new(ColoredLink {
                    color: Red,
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
            right: Box::new(End)
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
}


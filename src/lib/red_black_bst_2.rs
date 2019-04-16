//! Implements a red-black-binary-search-tree as described in the course.
//! Choses an approach with only one struct for both link and node

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

}


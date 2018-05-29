//! Defines a LinkedList trait and provides implementations
//! for both stack and heap allocations
//! # Example
//! ```
//! use algorithms::linked_list::{Cons, Nil, LinkedList};
//! use std::marker::PhantomData;
//!
//! fn some_linked_list() -> Box<LinkedList<Item=i32>> {
//!     let none = Nil{_p: PhantomData};
//!     let list_1 = Cons { h: 1i32, t: none };
//!     Box::new(list_1)
//! }
//!
//! let list_1 = some_linked_list();
//! let list_2 = Cons { h: 2i32, t: list_1 };
//!
//! for i in list_2.iter() {
//!     println!("{},", i);
//! }
//! ```

use std::marker::PhantomData;
use std::ops::Deref;

pub trait LinkedList {
    type Item;

    fn head<'a>(&'a self) -> Option<&'a Self::Item>;
    fn tail<'a>(&'a self) -> Option<&'a LinkedList<Item=Self::Item>>;

    fn iter<'a>(&'a self) -> LinkedListIter<'a, Self::Item>;
}

pub struct LinkedListIter<'a, T>
where T: 'a {
    l: &'a LinkedList<Item=T>
}

pub struct Cons<T, L> {
    pub h: T,
    pub t: L,
}

pub struct Nil<T> {
  pub _p: PhantomData<T>
}

impl<T, L> LinkedList for Cons<T, L>
    where L: LinkedList<Item=T> {
    type Item = T;

    fn head<'a>(&'a self) -> Option<&'a T> {
        Some(&self.h)
    }

    fn tail<'a>(&'a self) -> Option<&'a LinkedList<Item=T>> {
        Some(&self.t)
    }

    fn iter<'a>(&'a self) -> LinkedListIter<'a, T> {
        LinkedListIter { l: self }
    }
}

impl<T> LinkedList for Box<LinkedList<Item=T>> {
    type Item = T;

    fn head<'a>(&'a self) -> Option<&T> {
        self.deref().head()
    }

    fn tail<'a>(&'a self) -> Option<&LinkedList<Item=T>> {
        self.deref().tail()
    }

    fn iter<'a>(&'a self) -> LinkedListIter<'a, T> {
        LinkedListIter { l: self }
    }
}

impl<T> LinkedList for Nil<T> {
    type Item = T;

    fn head<'a>(&'a self) -> Option<&'a T> {
        Option::None
    }

    fn tail<'a>(&'a self) -> Option<&'a LinkedList<Item=T>> {
        Option::None
    }

    fn iter<'a>(&'a self) -> LinkedListIter<'a, T> {
        LinkedListIter { l: self }
    }
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let result = self.l.head();
        let tail = self.l.tail();
        match tail {
            Option::Some(t) => self.l = t,
            Option::None => {}
        }

        result
    }
}


#[cfg(test)]
mod tests {
    use super::{Cons, Nil, LinkedList};
    use std::fmt::Write;
    use std::marker::PhantomData;

    #[test]
    fn stack_construction() {
        let none = Nil{_p: PhantomData};
        assert_eq!(none.head(), Option::None);

        let list_1 = Cons { h: 1i32, t: none };
        assert_eq!(list_1.head(), Option::Some(&1));
        assert_eq!(list_1.tail().unwrap().head(), Option::None);

        let list_2 = Cons { h: 2i32, t: list_1 };
        assert_eq!(list_2.head(), Option::Some(&2));
        assert_eq!(list_2.tail().unwrap().head(), Option::Some(&1));
        assert_eq!(list_2.tail().unwrap().tail().unwrap().head(), Option::None);
    }

    fn some_linked_list() -> Box<LinkedList<Item=i32>> {
        let none = Nil{_p: PhantomData};
        let list_1 = Cons { h: 1i32, t: none };
        Box::new(list_1)
    }

    #[test]
    fn heap_construction() {
        let list_1 = some_linked_list();
        assert_eq!(list_1.head(), Option::Some(&1));
        assert_eq!(list_1.tail().unwrap().head(), Option::None);

        let list_2 = Cons { h: 2i32, t: list_1 };

        assert_eq!(list_2.head(), Option::Some(&2));
        assert_eq!(list_2.tail().unwrap().head(), Option::Some(&1));
        assert_eq!(list_2.tail().unwrap().tail().unwrap().head(), Option::None);
    }

    fn prepend_some<'a, L>(l: L) -> Box<LinkedList<Item=i32> + 'a>
        where L: LinkedList<Item=i32> + 'a {
        let list_1 = Cons { h: 4i32, t: l };
        let list_2 = Cons { h: 5i32, t: list_1 };
        Box::new(list_2)
    }

    #[test]
    fn mixed_construction() {
        let list_1 = some_linked_list();
        let list_2 = Cons { h: 2i32, t: list_1 };
        let list_3 = Cons { h: 3i32, t: list_2 };
        let list_4 = prepend_some(list_3);

        assert_eq!(list_4.head(), Option::Some(&5));
    }

    #[test]
    fn iter_stack() {
        let none = Nil::<i32>{_p: PhantomData};
        let list_1 = Cons { h: 1i32, t: none };
        let list_2 = Cons { h: 2i32, t: list_1 };

        let mut iter = list_2.iter();

        assert_eq!(iter.next(), Option::Some(&2));
        assert_eq!(iter.next(), Option::Some(&1));
        assert_eq!(iter.next(), Option::None);
    }

    #[test]
    fn iter_loop() {
        let none = Nil::<i32>{_p: PhantomData};
        let list_1 = Cons { h: 1i32, t: none };
        let list_2 = Cons { h: 2i32, t: list_1 };

        let mut output = String::new();

        for i in list_2.iter() {
            write!(&mut output, "{},", i).unwrap();
        }

        assert_eq!(output, "2,1,")
    }
}
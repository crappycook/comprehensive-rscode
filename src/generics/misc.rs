#![allow(dead_code)]

use std::fmt::Debug;
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Why T is specified twice in impl<T> Point<T> {}? Isn’t that redundant?
// This is because it is a generic implementation section for generic type. They are independently generic.
// It means these methods are defined for any T.
impl<T> Point<T> {
    fn coords(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

// Trait’s methods
// Require the types T to implement some traits like Clone + Debug
fn duplicate<T>(a: T, b: T) -> (T, T)
where
    T: Clone + Debug,
{
    (a.clone(), b.clone())
}

// 语法糖
// Syntactic sugar for:
//   fn add_42_millions<T>(x: T) -> i32 where T: Into<i32> + Clone
fn add_42_millions(x: impl Into<i32> + Clone) -> i32 {
    let y = x.clone();
    y.into() + 42_000_000
}

// For a return type,
// it means that the return type is some concrete type that implements the trait, without naming the type.
// This can be useful when you don’t want to expose the concrete type in a public API.
fn make_pair(x: i32, y: i32) -> impl Debug {
    (x + y, x - y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generics() {
        let p = Point { x: 5, y: 10 };
        assert_eq!(*p.coords().0, 5);

        let a = String::from("Rust");
        let b = String::from("Programming");
        let ds = duplicate(a, b);
        println!("{:?}", ds);

        assert_eq!(add_42_millions(5000), 42005000);

        let pair = make_pair(10, 5);
        println!("{:?}", pair);
    }
}

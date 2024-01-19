#![allow(dead_code)]
use core::hash::Hash;
use std::collections::HashMap;

/// Counter counts the number of times each value of type T has been seen.
struct Counter<T: Eq + Hash> {
    hash: HashMap<T, u64>,
}

impl<T> Counter<T>
where
    T: Eq + Hash,
{
    /// Create a new Counter.
    fn new() -> Self {
        Counter {
            hash: HashMap::new(),
        }
    }

    /// Count an occurrence of the given value.
    fn count(&mut self, value: T) {
        if self.hash.contains_key(&value) {
            *self.hash.get_mut(&value).unwrap() += 1;
        } else {
            self.hash.insert(value, 1);
        }
    }

    /// Return the number of times the given value has been seen.
    fn times_seen(&self, value: T) -> u64 {
        // if let Some(v) = self.hash.get(&value) {
        //     return *v;
        // } else {
        //     return 0;
        // };
        self.hash.get(&value).copied().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let mut ctr = Counter::new();
        ctr.count(1);
        ctr.count(2);
        ctr.count(3);
        ctr.count(1);
        ctr.count(1);
        ctr.count(2);
        assert_eq!(ctr.times_seen(1), 3);
        assert_eq!(ctr.times_seen(2), 2);
        assert_eq!(ctr.times_seen(3), 1);

        let mut strctr = Counter::new();
        strctr.count("apple");
        strctr.count("orange");
        strctr.count("apple");
        assert_eq!(strctr.times_seen("apple"), 2);
    }
}

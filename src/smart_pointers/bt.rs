#![allow(unused)]
/// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: Subtree::new(),
            right: Subtree::new(),
        }
    }
}

/// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

impl<T> Subtree<T>
where
    T: Ord,
{
    /// Create a new, empty subtree.
    fn new() -> Self {
        Subtree(None)
    }

    fn insert(&mut self, value: T) {
        let r = &mut self.0;
        match r {
            None => {
                *r = Some(Box::new(Node::new(value)))
            }
            Some(n) => {
                if value < n.value {
                    n.left.insert(value);
                } else if value > n.value {
                    n.right.insert(value);
                } else {
                    return;
                }
            }
        }
    }

    fn len(&self) -> usize {
        let r = &self.0;
        match r {
            None => 0,
            Some(n) => n.left.len() + n.right.len() + 1,
        }
    }

    fn has(&self, value: &T) -> bool {
        let r = &self.0;
        match r {
            None => false,
            Some(n) => {
                if n.value == *value {
                    true
                } else if n.value > *value {
                    n.left.has(value)
                } else {
                    n.right.has(value)
                }
            }
        }
    }
}

/// A container storing a set of values, using a binary tree.
///
/// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

// Implement `new`, `insert`, `len`, and `has`.
impl<T> BinaryTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinaryTree {
            root: Subtree(None),
        }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn len(&self) -> usize {
        self.root.len()
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn test_has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn test_unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}

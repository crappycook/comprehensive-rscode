mod bt;

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[test]
    fn test_box() {
        let br = Box::new(10);
        assert!(*br == 10);
    }

    #[test]
    fn test_rc() {
        let a = Rc::new(10);
        let b = Rc::clone(&a);
        assert_eq!(*a, *b);
        let num = Rc::strong_count(&a);
        assert_eq!(num, 2);
    }
}

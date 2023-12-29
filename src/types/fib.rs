#[allow(unused)]
fn fib(n: u32) -> u32 {
    if n <= 2 {
        // The base case.
        1
    } else {
        // The recursive case.
        fib(n - 1) + fib(n - 2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fib() {
        assert_eq!(super::fib(1), 1);
        assert_eq!(super::fib(2), 1);
        assert_eq!(super::fib(3), 2);
        assert_eq!(super::fib(4), 3);
        assert_eq!(super::fib(5), 5);
    }
}

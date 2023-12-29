/// Determine the length of the collatz sequence beginning at `n`.
#[allow(unused)]
pub fn collatz_length(mut n: i32) -> u32 {
    let mut i = 1;
    loop {
        if n == 1 {
            break;
        }
        if n % 2 == 0 {
            n = n / 2
        } else {
            n = 3 * n + 1
        }
        i += 1;
    }
    i
}

mod tests {
    #[test]
    fn test_collatz_length() {
        assert_eq!(2, super::collatz_length(2));
        assert_eq!(15, super::collatz_length(11));
    }
}

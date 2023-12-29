#[allow(unused)]
pub fn label_loop() {
    'outer: for i in 1..6 {
        println!("i = {}", i);
        if i == 3 {
            break 'outer;
        }
    }
}

mod tests {
    #[test]
    fn test_loops() {
        // while loop
        let mut x = 1;
        while x < 5 {
            x += 1;
        }
        assert_eq!(x, 5);

        // for loop
        let mut sum = 0;
        for x in 1..10 {
            sum += x;
        }
        assert_eq!(sum, 45);

        // loop
        let mut i = 0;
        loop {
            i += 1;
            if i > 5 {
                break;
            }
        }
        assert_eq!(i, 6);
    }
}

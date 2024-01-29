#![allow(unused)]

// FnMut is a subtype of FnOnce.
// Fn is a subtype of FnMut and FnOnce.
// I.e. you can use an FnMut wherever an FnOnce is called for,
// and you can use an Fn wherever an FnMut or FnOnce is called for.

// When you define a function that takes a closure,
// you should take FnOnce if you can (i.e. you call it once),
// or FnMut else, and last Fn. This allows the most flexibility for the caller.

// In contrast, when you have a closure, the most flexible you can have is Fn (it can be passed everywhere),
//  then FnMut, and lastly FnOnce.
fn function_log_wrapper(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
    println!("Calling function on {input}");
    func(input)
}

// By default, closures will capture by reference if they can. The move keyword makes them capture by value.
fn make_greeter(prefix: String) -> impl Fn(&str) {
    // data is no longer available, it is owned by the closure
    return move |name| println!("{} {}", prefix, name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_log_wrapper() {
        let double = |x| x * 2;
        let res = function_log_wrapper(double, 5);
        assert_eq!(res, 10);

        let mut v: Vec<i32> = Vec::new();
        let mut push_and_sum = |x| {
            v.push(x);
            v.iter().sum::<i32>()
        };
        let mut r = function_log_wrapper(&mut push_and_sum, 2);
        assert_eq!(r, 2);
        r = function_log_wrapper(&mut push_and_sum, 3);
        assert_eq!(r, 5);

        let multiply_sum = |x| x * v.into_iter().sum::<i32>();
        r = function_log_wrapper(multiply_sum, 2);
        assert_eq!(r, 10);

        let hi = make_greeter("Hi".to_string());
        hi("Rust");
    }
}

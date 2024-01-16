#![allow(dead_code)]
#[derive(Debug)]
enum CustomResult {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> CustomResult {
    if n & 1 == 0 {
        CustomResult::Ok(n >> 1)
    } else {
        CustomResult::Err(format!("cannot divide {n} into two equal parts"))
    }
}

// let-else for Option<T> matching
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    // Bind s to maybe_string
    let Some(s) = maybe_string else {
        return Err(String::from("got None string"));
    };

    // Bind first_byte_char to s.chars().next()
    let Some(first_byte_char) = s.chars().next() else {
        return Err(String::from("got empty string"));
    };

    // Bind digit to first_byte_char.to_digit()
    let Some(digit) = first_byte_char.to_digit(16) else {
        return Err(String::from("not a hex digit"));
    };

    Ok(digit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time::Duration};

    #[test]
    fn test_divide_in_two() {
        // Patterns can also be used to bind variables to parts of your values
        match divide_in_two(20) {
            CustomResult::Ok(n) => assert_eq!(n, 10),
            CustomResult::Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn test_time_sleep() {
        // if let func
        let d = if let Ok(dur) = Duration::try_from_secs_f32(-0.7) {
            dur
        } else {
            Duration::from_millis(500)
        };

        thread::sleep(d);
        println!("sleep for {:?}", d);
    }
}

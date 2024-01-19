use std::{fs::File, io::Read};

#[allow(unused)]
fn read_file() {
    let rs = File::open(".gitignore");
    match rs {
        Ok(mut file) => {
            let mut buf = String::new();
            if let Ok(size) = file.read_to_string(&mut buf) {
                println!("Read file: {buf} ({size} bytes)");
            } else {
                println!("Could not read the file");
            }
        }
        Err(err) => {
            println!("The file could not be opened: {err}");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*; // 导入父模块的公有项

    #[test]
    fn test_read_file() {
        read_file();
    }

    #[test]
    fn test_string_len() {
        let s = String::from("你好");
        // String::len returns the size of the String in bytes
        // which can be different from its length in characters
        assert_eq!(s.len(), 6);

        // String::chars returns an iterator over the actual characters.
        assert_eq!(s.chars().count(), 2);

        for c in s.chars() {
            println!("{c}");
        }

        for b in s.bytes() {
            println!("{b}");
        }
    }

    #[test]
    fn test_vector() {
        let mut v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.len(), 5);
        println!("cap = {}, len = {}", v.capacity(), v.len()); // cap = 5, len = 5
        v.push(6);
        println!("cap = {}, len = {}", v.capacity(), v.len()); // cap = 10, len = 6

        let mut v1: Vec<i32> = Vec::with_capacity(v.len() + 1);
        v1.extend(v.iter());
        v1.push(100);
        println!("cap = {}, len = {}", v1.capacity(), v1.len()); // cap = 7, len = 7

        let mut v2 = vec![2, 2, 3, 4, 5];
        v2.retain(|x| x % 2 == 0);
        println!("{v2:?}"); // [2, 2, 4]

        v2.dedup(); // remove duplicates
        println!("{v2:?}"); // [2, 4]
    }

    #[test]
    fn test_hashmap() {
        let player_number_map =
            HashMap::from([("Messi".to_string(), 10), ("Neymar".to_string(), 11)]);

        for entry in player_number_map.iter() {
            println!("{}, {}", entry.0, entry.1)
        }
    }
}

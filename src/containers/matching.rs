#[allow(unused)]
fn pattern_matching(input: char) {
    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        key if key.is_lowercase() => println!("{}", key),
        _ => println!("I don't know what this is."),
    }
}

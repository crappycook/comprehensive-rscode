mod async_basic;
mod channel;
mod join_select;
mod nonblock;
mod philosophers;
mod pin;
mod shared;

#[test]
fn test_threads() {
    use std::thread;
    let s = String::from("value");
    println!("{}", s);
    let h = thread::spawn(move || {
        println!("Length: {}", s.len());
    });
    h.join().unwrap();
}

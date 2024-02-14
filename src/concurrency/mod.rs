mod async_basic;
mod channel;
mod join_select;
mod philosophers;
mod shared;
mod nonblock;

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

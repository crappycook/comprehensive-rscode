#![allow(unused)]
use std::time::Duration;
use std::{sync::mpsc, thread};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_async_channels() {
        // mpsc stands for Multi-Producer, Single-Consumer.
        // Sender and SyncSender implement Clone (so you can make multiple producers) but Receiver does not
        let (tx, rx) = mpsc::channel::<i32>();
        tx.send(1).unwrap();
        tx.send(2).unwrap();
        let mut r = rx.recv().unwrap();
        assert_eq!(r, 1);
        r = rx.recv().unwrap();
        assert_eq!(r, 2);

        // Multi-Producer
        let tx2 = tx.clone();
        tx2.send(3).unwrap();
        r = rx.recv().unwrap();
        assert_eq!(r, 3);
    }

    #[test]
    fn test_sync_channels() {
        let (tx, rx) = mpsc::sync_channel::<String>(3);

        // The thread can be blocked indefinitely if there is nobody who reads from the channel
        thread::spawn(move || {
            let thread_id = thread::current().id();
            for i in 1..10 {
                let mut r = tx.send(format!("Message {i}"));
                println!("{thread_id:?}: sent Message {i} result: {r:?}");
            }
            println!("{thread_id:?}: done");
        });
        thread::sleep(Duration::from_millis(100));

        for msg in rx.iter() {
            println!("Main: got {msg}");
        }
        assert!(rx.recv().is_err());
    }
}

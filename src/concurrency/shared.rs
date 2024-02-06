#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        thread,
    };

    #[test]
    fn test_arc() {
        // Arc stands for “Atomic Reference Counted”,
        // a thread safe version of Rc that uses atomic operations
        let v = Arc::new(vec![10, 20, 30]);
        let mut handles = Vec::new();
        for _ in 0..3 {
            let v = Arc::clone(&v);
            handles.push(thread::spawn(move || {
                let tid = thread::current().id();
                println!("{tid:?}: {v:?}");
            }));
        }

        handles.into_iter().for_each(|h| h.join().unwrap());
        println!("threads all done, v: {v:?}");
    }

    #[test]
    fn test_arc_mutex() {
        // Wrapping a Mutex in an Arc is a common pattern to share mutable state between threads
        let mu = Arc::new(Mutex::new(vec![]));
        let mut handlers = Vec::new();

        for i in 0..10 {
            let muc = Arc::clone(&mu);
            let h = thread::spawn(move || {
                let mut v = muc.lock().unwrap();
                v.push(i * 10);
            });
            handlers.push(h);
        }
        handlers.into_iter().for_each(|h| h.join().unwrap());

        let res = mu.lock().unwrap();
        println!("{:?}", res);
        assert_eq!(res.iter().sum::<i32>(), 450);
    }
}

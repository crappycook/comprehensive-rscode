#![allow(unused)]
use futures::future::join_all;
use std::time::{Duration, Instant};
use tokio::time::sleep;

async fn sleep_ms(start: &Instant, id: u64, duration_ms: u64) {
    // `std::thread::sleep` Puts the current thread to sleep for at least the specified amount of time.
    // this will block the current thread.
    // std::thread::sleep(Duration::from_millis(duration_ms));

    // Switch the `std::thread::sleep` to `tokio::time::sleep`
    sleep(Duration::from_millis(duration_ms)).await;
    println!(
        "future {id} slept for {duration_ms}ms, finished after {}ms",
        start.elapsed().as_millis()
    );
}

// `current_thread` puts all tasks on a single thread
#[tokio::main(flavor = "current_thread")]
async fn run_nonblock_sleep() {
    let start = Instant::now();
    let sleep_futures = (1..=10).map(|t| sleep_ms(&start, t, t * 10));
    join_all(sleep_futures).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nonblock_sleep() {
        run_nonblock_sleep();
    }
}

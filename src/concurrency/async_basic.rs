#![allow(unused)]
use tokio::time;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("Count in task: {i}!");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

#[tokio::main]
async fn main() {
    // The spawn function creates a new, concurrent `task`.
    tokio::spawn(count_to(10));
    // .await asynchronously waits for the completion of another operation. Unlike block_on,
    // .await doesn’t block the current thread. await 不会阻塞当前的线程
    // count_to(10).await;

    for i in 1..5 {
        println!("Main task: {i}");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

use tokio::sync::mpsc::{self, Receiver};

async fn ping_handler(mut input: Receiver<()>) {
    let mut count: usize = 0;

    while let Some(_) = input.recv().await {
        count += 1;
        println!("Received {count} pings so far.");
    }

    println!("ping_handler complete");
}

#[tokio::main]
async fn channels() {
    let (sender, receiver) = mpsc::channel(3);
    // Spawns a new asynchronous task, returning a `JoinHandle` for it.
    let ping_handler_task = tokio::spawn(ping_handler(receiver));
    for i in 0..10 {
        sender.send(()).await.expect("Failed to send ping.");
        println!("Sent {} pings so far.", i + 1);
    }

    drop(sender);
    ping_handler_task
        .await
        .expect("Something went wrong in ping handler task.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_runtime() {
        main();
    }

    #[test]
    fn test_tokio_channels() {
        channels();
    }
}

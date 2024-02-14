#![allow(unused)]
use std::time::Duration;

use tokio::{
    sync::{mpsc, oneshot},
    time::{self, sleep, Instant},
};

// A work item. In this case, just sleep for the given time and respond
// with a message on the `respond_on` channel.
#[derive(Debug)]
struct Work {
    input: u32,
    respond_on: oneshot::Sender<u32>,
}

// A worker which listens for work on a queue and performs it.
async fn worker(mut work_rx: mpsc::Receiver<Work>) {
    let mut num = 0;

    // `Pin` is a wrapper around a reference.
    // An object cannot be moved from its place using a pinned pointer.
    // However, it can still be moved through an unpinned pointer.
    // The `poll` method of the Future trait uses `Pin<&mut Self>` instead of `&mut Self` to refer to the instance.
    // That’s why it can only be called on a pinned pointer.
    let mut tick = sleep(Duration::from_millis(100));
    tokio::pin!(tick);

    // or use interval
    // let mut interval = time::interval(Duration::from_millis(100));

    loop {
        tokio::select! {
            Some(work) = work_rx.recv() => {
                sleep(Duration::from_millis(10)).await; // Pretend to work.
                work.respond_on.send(work.input*10).expect("failed to send response");
                num += 1;
            }
            // report number of iterations every 100ms
            _ = &mut tick => {
                println!("this is the {num} iterations");
                tick.as_mut().reset(Instant::now() + Duration::from_millis(100));
            }
            // alternative
            // _ = interval.tick() => {
            //     println!("this is the {num} iterations");
            // }
            else => {
                break;
            }
        }
    }
}

// A requester which requests work and waits for it to complete.
async fn do_work(work_queue: &mpsc::Sender<Work>, input: u32) -> u32 {
    let (tx, rx) = oneshot::channel();
    let w = Work {
        input,
        respond_on: tx,
    };

    work_queue
        .send(w)
        .await
        .expect("failed to send on work queue");

    rx.await.expect("failed waiting for response")
}

#[tokio::main]
async fn run_actor() {
    let (tx, rx) = mpsc::channel(10);

    tokio::task::spawn(worker(rx));

    for i in 0..20 {
        let res = do_work(&tx, i).await;
        println!("work result for iteration {i}: {res}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_actor() {
        run_actor();
    }
}

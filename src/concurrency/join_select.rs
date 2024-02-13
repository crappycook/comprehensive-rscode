#![allow(unused)]
use std::{thread, time::SystemTime};
use tokio::{
    sync::mpsc::{self, Receiver},
    time::{self, sleep, Duration},
};

async fn do_work1() {
    let t1 = SystemTime::now();
    println!("doing work1");
    time::sleep(time::Duration::from_millis(100)).await;

    let t2 = SystemTime::now();
    println!(
        "done work1, time elapsed: {:?}",
        t2.duration_since(t1).unwrap()
    );
}

async fn do_work2() {
    let t1 = SystemTime::now();
    println!("doing work2");
    time::sleep(time::Duration::from_millis(200)).await;

    let t2 = SystemTime::now();
    println!(
        "done work2, time elapsed: {:?}",
        t2.duration_since(t1).unwrap()
    );
}

#[tokio::main]
async fn wait_group() {
    let t1 = SystemTime::now();

    let task1 = tokio::spawn(async {
        do_work1().await;
    });
    let task2 = tokio::spawn(async {
        do_work2().await;
    });

    /// `join!` polls both futures concurrently and therefore is more efficient.
    // futures::join!(task1, task2);

    /// By running all async expressions on the current task, the expressions are
    /// able to run **concurrently** but not in **parallel**. This means all
    /// expressions are run on the same thread and if one branch blocks the thread,
    /// all other expressions will be unable to continue. If parallelism is
    /// required, spawn each async expression using [`tokio::spawn`] and pass the
    /// join handle to `join!`.
    /// join 并没有并行去执行两个任务，两个任务其实是在同一个 thread 上执行
    tokio::join!(task1, task2);

    let t2 = SystemTime::now();
    println!("time elapsed: {:?}", t2.duration_since(t1).unwrap());
}

#[derive(Debug, PartialEq)]
enum Animal {
    Cat { name: String },
    Dog { name: String },
}

async fn first_animal_to_finish_race(
    mut cat_rcv: Receiver<String>,
    mut dog_rcv: Receiver<String>,
) -> Option<Animal> {
    // select timeout
    let time_after = time::sleep(Duration::from_millis(200));
    tokio::pin!(time_after);

    tokio::select! {
        cat_name = cat_rcv.recv() => Some(Animal::Cat { name: cat_name? }),
        dog_name = dog_rcv.recv() => Some(Animal::Dog { name: dog_name? }),
        _ = &mut time_after => None,
    }
}

#[tokio::main]
async fn run_select() {
    let (cat_sender, cat_receiver) = mpsc::channel(32);
    let (dog_sender, dog_receiver) = mpsc::channel(32);

    tokio::spawn(async move {
        // thread::sleep(std::time::Duration::from_millis(500));
        sleep(Duration::from_millis(500)).await;
        cat_sender
            .send(String::from("Felix"))
            .await
            .expect("Failed to send cat.");
    });

    tokio::spawn(async move {
        // thread::sleep(std::time::Duration::from_millis(50));
        sleep(Duration::from_millis(100)).await;
        dog_sender
            .send(String::from("Rex"))
            .await
            .expect("Failed to send dog.");
    });

    if let Some(animal) = first_animal_to_finish_race(cat_receiver, dog_receiver).await {
        println!("Winner is {animal:?}");
    } else {
        println!("Timeout! No winner.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_wait_group() {
        wait_group();
    }

    #[test]
    fn test_async_select() {
        run_select();
    }
}

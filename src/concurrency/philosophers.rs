#![allow(unused)]
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

// The dining philosophers problem is a classic problem in concurrency:
struct Philosopher {
    name: String,
    thoughts: mpsc::SyncSender<String>,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up forks...
        println!("{} is trying to eat", &self.name);
        self.left_fork.lock().unwrap();
        self.right_fork.lock().unwrap();
        // Eat...
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn dining() {
    // Create forks
    let mut forks = Vec::new();
    for _ in 0..PHILOSOPHERS.len() {
        forks.push(Arc::new(Mutex::new(Fork)));
    }

    let (tx, rx) = mpsc::sync_channel(10);
    // Create philosophers
    for i in 0..PHILOSOPHERS.len() {
        let tx = tx.clone();
        let mut left_fork = Arc::clone(&forks[i]);
        let mut right_fork = Arc::clone(&forks[(i + 1) % PHILOSOPHERS.len()]);

        // To avoid a deadlock, we have to break the symmetry
        // somewhere. This will swap the forks without deinitializing
        // either of them.
        // 这段代码中的交换操作是为了避免死锁。在哲学家就餐问题中，可能会出现死锁的情况，即所有哲学家都在等待拿到左右两边的叉子，但没有人可以成功拿到两把叉子，从而导致所有哲学家都无法就餐。
        // 为了避免死锁，这段代码在分配叉子时有意地打破了哲学家之间的对称性。
        // 具体来说，当循环到达最后一个哲学家时，它会交换左右叉子的引用。这样，最后一个哲学家会先尝试获取右边的叉子，然后再尝试获取左边的叉子，而不是像其他哲学家那样先尝试获取左边的叉子。
        // 这种交换操作可以确保至少有一个哲学家可以成功拿到两把叉子，从而避免死锁。其他哲学家可能会等待一段时间，但最终他们仍然可以成功拿到叉子并开始就餐。
        if i == forks.len() - 1 {
            std::mem::swap(&mut left_fork, &mut right_fork);
        }

        let philosopher = Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            thoughts: tx,
            left_fork,
            right_fork,
        };

        thread::spawn(move || {
            // Make each of them think and eat 10 times
            // cloned `tx` dropped within thread
            for _ in 0..10 {
                philosopher.eat();
                philosopher.think();
            }
        });
    }

    // Output their thoughts
    // Drop the last sender to stop `rx` waiting for message.
    drop(tx);
    // Sender 被释放, Receiver 不会被阻塞
    for thought in rx.iter() {
        println!("{}", thought);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dining_philosophers() {
        dining();
    }
}

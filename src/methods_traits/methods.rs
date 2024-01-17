#![allow(dead_code)]
#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    // No receiver, a static method
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            laps: Vec::new(),
        }
    }

    // Exclusive borrowed read-write access to self
    // &mut self: borrows the object from the caller using a unique and mutable reference. The object can be used again afterwards.
    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    // Shared and read-only borrowed access to self
    // borrows the object from the caller using a shared and immutable reference.
    // 翻译过来就是: 使用共享且不可变的引用从调用者那里借用对象
    fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    // Exclusive ownership of self
    // self: takes ownership of the object and moves it away from the caller.
    // The method becomes the owner of the object. The object will be dropped (deallocated) when the method returns, unless its ownership is explicitly transmitted.
    // Complete ownership does not automatically mean mutability.
    fn finish(self) -> i32 {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_racing_master() {
        let mut race = Race::new("Racing Master");
        race.add_lap(20);
        race.add_lap(22);
        race.add_lap(23);
        race.print_laps();
        assert_eq!(race.finish(), 65);
    }
}

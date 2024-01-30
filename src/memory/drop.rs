#![allow(unused)]
struct Droppable {
    name: &'static str,
}

// Executes the destructor for this type.
// However, the [`mem::drop`] function in the prelude can be
// used to call the argument's `Drop` implementation.
// std::mem::drop is just an empty function that takes any value.
// The significance is that it takes ownership of the value, so at the end of its scope it gets dropped.
impl Drop for Droppable {
    // This method is called implicitly when the value goes out of scope
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop() {
        let a = Droppable { name: "a" };
        {
            let b = Droppable { name: "b" };
            {
                let c = Droppable { name: "c" };
                let d = Droppable { name: "d" };
                println!("Exiting block B");
            }
            println!("Exiting block A");
        }
        drop(a); // 手动 drop
        println!("Exiting main");
        // Exiting block B
        // Dropping d
        // Dropping c
        // Exiting block A
        // Dropping b
        // Exiting main
        // Dropping a
    }
}

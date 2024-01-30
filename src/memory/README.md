# Review of Program Memory

Programs allocate memory in two ways:

- Stack: Continuous area of memory for local variables.
  - Values have fixed sizes known at compile time.
  - Extremely fast: just move a stack pointer.
  - Easy to manage: follows function calls.
  - Great memory locality.

- Heap: Storage of values outside of function calls.
  - Values have dynamic sizes determined at runtime.
  - Slightly slower than the stack: some book-keeping needed.
  - No guarantee of memory locality.

# Ownership

[Move Semantics](https://google.github.io/comprehensive-rust/memory-management/move.html)

When you pass a value to a function, the value is assigned to the function parameter. This transfers ownership

```rust
fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name);
}
```

# Copy

Copying and cloning are not the same thing:

- Copying refers to bitwise copies of memory regions and does not work on arbitrary objects. 复制是指内存区域的按位复制，不适用于任意对象。只有基本类型可以复制
- Copying does not allow for custom logic (unlike copy constructors in C++). Clone 更加灵活
- Cloning is a more general operation and also allows for custom behavior by implementing the Clone trait. Clone 是一种更通用的操作，还允许通过实现 Clone 特征来实现自定义行为。
- Copying does not work on types that implement the Drop trait. 复制不适用于实现 Drop 特征的类型。

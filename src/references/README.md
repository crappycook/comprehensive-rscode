## Shared Ref

1. A reference is said to “borrow” the value it refers to, and this is a good model for students not familiar with pointers: code can use the reference to access the value, but is still “owned” by the original variable.
2. References are implemented as pointers, and a key advantage is that they can be much smaller than the thing they point to.
3. A shared reference does not allow modifying the value it refers to, even if that value was mutable.

## Exclusive references

1. “Exclusive” means that only this reference can be used to access the value. No other references (shared or exclusive) can exist at the same time, and the referenced value cannot be accessed while the exclusive reference exists. Try making an &point.0 or changing point.0 while x_coord is alive

2. Be sure to note the difference between `let mut x_coord: &i32` and `let x_coord: &mut i32`. **The first one represents a shared reference which can be bound to different values, while the second represents an exclusive reference to a mutable value.**

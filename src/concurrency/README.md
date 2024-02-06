# Send and Sync

## Send

> A type `T` is `Send` if it is safe to move a `T` value to another thread.

## Sync

> A type `T` is `Sync` if it is safe to access a `T` value from multiple threads at the same time.
> `T` is Sync if and only if `&T` is `Send`

## Examples

[examples](https://google.github.io/comprehensive-rust/concurrency/send-sync/examples.html)

# Shared State

- `Arc<T>`, atomic reference counted `T`: handles sharing between threads and takes care to deallocate `T` when the last reference is dropped,
- `Mutex<T>`: ensures mutually exclusive access to the `T` value.

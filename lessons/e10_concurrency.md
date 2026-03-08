# Lesson 10: Concurrency

Concurrency is the ability of different parts of a program to execute independently. Rust makes concurrency safe by preventing data races at compile time.

## Threads

You can spawn a new thread using `std::thread::spawn`.

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Hello from a thread!");
});

handle.join().unwrap(); // Wait for the thread to finish
```

## Shared State: `Arc` and `Mutex`

To share data between threads, you often need:
- `Arc` (Atomic Reference Count): Allows multiple threads to own the same data.
- `Mutex` (Mutual Exclusion): Ensures only one thread can access the data at a time.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());
```

## Message Passing: Channels

Alternatively, you can share memory by communicating. `mpsc` stands for "multi-producer, single-consumer".

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
});

let received = rx.recv().unwrap();
println!("Got: {}", received);
```

## Further Reading

- [Rust Book: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust Book: Using Message Passing](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
- [Rust Book: Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)

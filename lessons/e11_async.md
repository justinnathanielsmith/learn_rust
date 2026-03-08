# Lesson 11: Asynchronous Programming

Asynchronous programming allows you to handle many tasks concurrently on a single thread (or a small number of threads), making it very efficient for I/O-bound tasks.

## Async / Await

In Rust, an `async` function returns a `Future`. A future is a value that will be computed later. To get the value, you must `.await` it.

```rust
async fn do_something() -> i32 {
    42
}

async fn main_async() {
    let result = do_something().await;
    println!("The result is {}", result);
}
```

## Tokio Runtime

Standard Rust doesn't include an async runtime. **Tokio** is the most popular runtime for executing async code.

```rust
#[tokio::main]
async fn main() {
    // Your async code starts here
}
```

## Concurrent Tasks: `tokio::spawn` and `JoinSet`

To run multiple futures concurrently:

- `tokio::spawn`: Spawns a new task (similar to a thread).
- `tokio::task::JoinSet`: A collection of tasks that you can wait for together.

```rust
use tokio::task::JoinSet;

let mut set = JoinSet::new();

for i in 0..10 {
    set.spawn(async move {
        i * 2
    });
}

while let Some(res) = set.join_next().await {
    println!("Task finished with: {:?}", res.unwrap());
}
```

## Further Reading

- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Documentation](https://tokio.rs/tokio/tutorial)

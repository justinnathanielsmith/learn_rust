# Lesson 06: Options and Results (Error Handling)

Rust doesn't have exceptions. Instead, it uses the `Option` and `Result` enums to handle values that might be missing or operations that might fail.

## Option<T>

Use `Option` when a value could be either "something" or "nothing".

```rust
fn find_word(text: &str, word: &str) -> Option<usize> {
    text.find(word)
}

let result = find_word("hello world", "world");
match result {
    Some(index) => println!("Found at: {}", index),
    None => println!("Not found"),
}
```

## Result<T, E>

Use `Result` for operations that can fail with an error.

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
```

## The Question Mark Operator (`?`)

The `?` operator is a shortcut for returning an error early. If the value is `Err`, it returns from the function. If it's `Ok`, it unwraps the value.

```rust
fn read_and_parse() -> Result<i32, MyError> {
    let s = read_from_file()?; // Returns Err early if read fails
    let n: i32 = s.parse()?;     // Returns Err early if parse fails
    Ok(n)
}
```

## Further Reading

- [Rust Book: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust Book: Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

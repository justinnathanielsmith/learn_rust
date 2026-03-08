# Lesson 01: Functions and Basic Types

Welcome to your first Rust lesson! Today, we'll cover the building blocks of any Rust program: functions and basic types.

## Defining Functions

In Rust, you define a function using the `fn` keyword.

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}
```

- `fn`: The keyword to start a function definition.
- `add_one`: The name of the function (use `snake_case`).
- `x: i32`: A parameter named `x` of type `i32` (32-bit integer).
- `-> i32`: The return type of the function.
- `{ ... }`: The function body.
- `x + 1`: In Rust, the last expression in a function is its return value. Notice there's no semicolon! Adding a semicolon would make it a statement, which returns `()` (unit type).

## Basic Types

- **Integers**: `i32` (default), `u32` (unsigned), `i64`, `u64`, etc.
- **Booleans**: `bool` (`true` or `false`).
- **Strings**:
    - `&str`: A string slice (often used for literal strings).
    - `String`: A growable, heap-allocated string.

## String vs &str

A common point of confusion for beginners is the difference between `String` and `&str`.

Think of `String` as the *owner* of the data, and `&str` as a *view* into that data.

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

The `format!` macro is a great way to create new `String`s by interpolating variables.

## Further Reading

- [Rust Book: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust Book: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)

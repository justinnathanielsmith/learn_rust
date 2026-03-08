# Lesson 05: Enums and Pattern Matching

Enums (enumerations) allow you to define a type by enumerating its possible variants.

## Defining an Enum

Enums in Rust can store data directly in their variants!

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

## Matching on Enums

The most common way to use an enum is with a `match` expression.

```rust
fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quitting...");
        }
        Message::Move { x, y } => {
            println!("Moving to x: {}, y: {}", x, y);
        }
        Message::Write(text) => {
            println!("Writing: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Changing color to RGB({}, {}, {})", r, g, b);
        }
    }
}
```

## Option Enum

One of the most important enums in Rust is `Option<T>`, which represents a value that might be something (`Some`) or nothing (`None`).

```rust
let some_number = Some(5);
let some_char = Some('e');
let absent_number: Option<i32> = None;
```

## Further Reading

- [Rust Book: Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust Book: The match Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html)

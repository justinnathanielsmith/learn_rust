# Lesson 02: Control Flow

Rust has common control flow structures like `if` and `else`, but it also has powerful `match` statements and flexible loops.

## If / Else

In Rust, `if` is an expression, meaning it can return a value.

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
```

Note: The types in the `if` and `else` blocks must match!

## Match

The `match` keyword allows you to compare a value against a series of patterns and then execute code based on which pattern matches.

```rust
let number = 3;
match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Anything else"), // The underscore is a catch-all pattern
}
```

Matches are exhaustive: you must cover every possible value!

## Loops

Rust has three kinds of loops: `loop`, `while`, and `for`.

- `loop`: An infinite loop (use `break` to exit).
- `while`: Runs as long as a condition is true.
- `for`: Best for iterating over a collection (like a range or an array).

```rust
// Iterating 1 to 5
for n in 1..=5 {
    println!("{}", n);
}
```

The `..=` syntax creates an inclusive range.

## Further Reading

- [Rust Book: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

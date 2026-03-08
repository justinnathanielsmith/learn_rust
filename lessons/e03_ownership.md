# Lesson 03: Ownership and Borrowing

Welcome to the heart of Rust! Ownership is what allows Rust to make memory safety guarantees without a garbage collector.

## The Three Rules of Ownership

1. Each value in Rust has a variable that’s called its **owner**.
2. There can only be **one owner** at a time.
3. When the owner goes out of scope, the value will be **dropped** (memory is freed).

## Moving

When you assign a `String` from one variable to another, the ownership is *moved*.

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is no longer valid!
```

## Borrowing (References)

To use a value without taking ownership, you can *borrow* it using a reference (`&`).

- `&s`: An immutable reference (you can't change the value).
- `&mut s`: A mutable reference (you can change the value).

**Rule of References:**
You can have either:
- One mutable reference OR
- Any number of immutable references

But you cannot have both at the same time!

## Cloning

If you actually want a deep copy of the data, you can use the `.clone()` method.

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); 
// Both s1 and s2 are valid and independent copies.
```

## Further Reading

- [Rust Book: Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust Book: References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

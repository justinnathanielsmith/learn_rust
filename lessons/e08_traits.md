# Lesson 08: Traits and Generics

Traits allow us to define shared behavior in a generic way. Generics allow us to write code that works for multiple types.

## Defining a Trait

A trait tells the Rust compiler about functionality a particular type has and can share with other types.

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

## Implementing a Trait

```rust
struct NewsArticle {
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}", self.headline)
    }
}
```

## Trait Bounds

You can write functions that accept any type that implements a specific trait.

```rust
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

## The `Display` Trait

The `std::fmt::Display` trait is used for formatting values for user-friendly display (used with `{}`).

```rust
use std::fmt;

impl fmt::Display for NewsArticle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Article: {}", self.headline)
    }
}
```

## Further Reading

- [Rust Book: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust Book: Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)

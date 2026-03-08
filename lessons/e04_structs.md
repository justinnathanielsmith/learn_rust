# Lesson 04: Structs and Implementations

Structs allow you to create custom types that group together related values.

## Defining a Struct

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
}
```

## Creating Instances

To use a struct, you create an *instance* of it.

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someuser123"),
    sign_in_count: 1,
};
```

## Implementation Blocks (`impl`)

Implementation blocks are where you define methods and associated functions for your struct.

- **Associated Functions**: Functions that are associated with the struct but don't take `self`. Often used for constructors (like `new`).
- **Methods**: Functions that take `&self`, `&mut self`, or `self` as their first parameter.

```rust
impl User {
    // Associated function
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 0,
        }
    }

    // Method
    fn say_hello(&self) {
        println!("Hello, {}!", self.username);
    }

    // Mutable method
    fn update_email(&mut self, new_email: String) {
        self.email = new_email;
    }
}
```

## Further Reading

- [Rust Book: Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust Book: Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

# Lesson 09: Iterators and Closures

Iterators allow you to perform some task on a sequence of items in turn. Closures are anonymous functions you can save in a variable or pass to other functions.

## Closures

Closures look like this: `|param1, param2| { body }`.

```rust
let add_one = |x: i32| x + 1;
let result = add_one(5);
```

Closures can capture their environment (variables from the scope where they are defined).

## Iterators

The `Iterator` trait requires you to define a `next` method, but most of the time you use *iterator adaptors*.

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

### Common Iterator Adaptors

- `map`: Transforms each element.
- `filter`: Keeps only elements that satisfy a condition.
- `sum`: Consumes the iterator and returns the total.
- `collect`: Consumes the iterator and transforms it into a collection (like a `Vec`).

```rust
let v: Vec<i32> = vec![1, 2, 3, 4];
let sum: i32 = v.iter()
    .filter(|x| *x % 2 == 0) // Keep even numbers
    .map(|x| x * 10)         // Multiply by 10
    .sum();                  // Sum them up
```

## Further Reading

- [Rust Book: Iterators and Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- [Rust Book: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Rust Book: Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)

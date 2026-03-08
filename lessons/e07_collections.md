# Lesson 07: Collections (Vectors and HashMaps)

Collections are data structures that can contain multiple values. Unlike arrays, collections are usually stored on the heap and can grow or shrink in size.

## Vectors (`Vec<T>`)

A vector is a growable array of values of the same type.

```rust
let mut v = Vec::new();
v.push(1);
v.push(2);

let v2 = vec![1, 2, 3]; // Using the vec! macro
```

Common methods:
- `push(item)`: Add an item.
- `pop()`: Remove the last item.
- `get(index)`: Returns an `Option<&T>`.
- `sort()`: Sort the vector.

## HashMaps (`HashMap<K, V>`)

A hash map stores a mapping of keys to values.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let score = scores.get("Blue"); // Returns Option<&i32>
```

### The Entry API

The `entry` method is a powerful way to update a value based on whether a key already exists.

```rust
scores.entry(String::from("Blue")).or_insert(0);
```

## Further Reading

- [Rust Book: Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Rust Book: Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [Rust Book: HashMaps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)

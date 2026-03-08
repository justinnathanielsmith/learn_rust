//! # Exercise 01: Functions
//!
//! In Rust, functions are declared with the `fn` keyword.
//! Type annotations for parameters and return types are required.
//!
//! Your task is to implement the following functions to make the tests pass.

/// This function should return the sum of two integers.
pub fn add(a: i32, b: i32) -> i32 {
    todo!("Implement add")
}

/// This function should return a greeting string: "Hello, [name]!"
pub fn greet(name: &str) -> String {
    todo!("Implement greet")
}

/// This function should return the square of a number.
pub fn square(n: i32) -> i32 {
    todo!("Implement square")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(10, 20), 30);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
        assert_eq!(greet("Bob"), "Hello, Bob!");
    }

    #[test]
    fn test_square() {
        assert_eq!(square(2), 4);
        assert_eq!(square(5), 25);
        assert_eq!(square(-3), 9);
    }
}

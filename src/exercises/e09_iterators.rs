/// Sums all even numbers from 0 up to and including the given maximum using iterators.
///
/// Use `0..=max` as a range, then `filter` and `sum`.
/// If the maximum is negative, return 0.
pub fn sum_even_numbers_iter(max: i32) -> i32 {
    todo!("Use iterators to sum even numbers up to {}", max)
}

/// Takes a vector of strings and returns a new vector where each string is capitalized.
///
/// Use `map` to transform each string using `.to_uppercase()`.
pub fn capitalize_words(words: Vec<&str>) -> Vec<String> {
    todo!("Capitalize each word in {:?}", words)
}

#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u32,
}

/// Filters a list of users, returning only those who are 18 or older.
///
/// Use `into_iter`, `filter`, and `collect`.
pub fn filter_adults(users: Vec<User>) -> Vec<User> {
    todo!("Filter users older than or equal to 18")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_even_numbers_iter() {
        assert_eq!(sum_even_numbers_iter(10), 30);
        assert_eq!(sum_even_numbers_iter(5), 6);
        assert_eq!(sum_even_numbers_iter(0), 0);
        assert_eq!(sum_even_numbers_iter(-1), 0);
    }

    #[test]
    fn test_capitalize_words() {
        let input = vec!["hello", "rust", "tdd"];
        let expected = vec!["HELLO".to_string(), "RUST".to_string(), "TDD".to_string()];
        assert_eq!(capitalize_words(input), expected);
    }

    #[test]
    fn test_filter_adults() {
        let users = vec![
            User { name: "Alice".to_string(), age: 25 },
            User { name: "Bob".to_string(), age: 17 },
            User { name: "Charlie".to_string(), age: 18 },
        ];
        let expected = vec![
            User { name: "Alice".to_string(), age: 25 },
            User { name: "Charlie".to_string(), age: 18 },
        ];
        assert_eq!(filter_adults(users), expected);
    }
}

/// Topic 6: Options and Results (Error Handling)
///
/// In Rust, error handling is done via the `Option` and `Result` enums.
/// This prevents unexpected null pointer exceptions and forces you to handle potential errors.

/// Returns `Some(a / b)` if `b != 0`, otherwise returns `None`.
///
/// # Arguments
/// * `a` - The dividend
/// * `b` - The divisor
pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    todo!("Implement safe_divide")
}

/// A mock function that "reads" a file length.
/// Returns `Ok(length)` if the path is not empty, otherwise returns `Err("Empty path")`.
///
/// # Arguments
/// * `path` - The file path to "read"
pub fn read_file_length(path: &str) -> Result<usize, String> {
    todo!("Implement read_file_length")
}

/// Parses two strings as integers and returns their product.
/// Demonstrates the use of the `?` operator (if you were to implement it that way).
///
/// # Arguments
/// * `s1` - The first numeric string
/// * `s2` - The second numeric string
pub fn parse_and_multiply(s1: &str, s2: &str) -> Result<i32, std::num::ParseIntError> {
    todo!("Implement parse_and_multiply")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Some(5));
        assert_eq!(safe_divide(10, 0), None);
    }

    #[test]
    fn test_read_file_length() {
        assert_eq!(read_file_length("config.txt"), Ok(10));
        assert_eq!(read_file_length(""), Err("Empty path".to_string()));
    }

    #[test]
    fn test_parse_and_multiply() {
        assert_eq!(parse_and_multiply("10", "2"), Ok(20));
        assert!(parse_and_multiply("abc", "2").is_err());
        assert!(parse_and_multiply("10", "xyz").is_err());
    }
}

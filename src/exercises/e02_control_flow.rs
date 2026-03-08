/// Returns true if the given number is even, false otherwise.
/// Use an if/else expression to return the result.
pub fn is_even(n: i32) -> bool {
    todo!("Implement is_even")
}

/// Returns a string based on the following rules:
/// - If the number is divisible by 3, return "Fizz".
/// - If the number is divisible by 5, return "Buzz".
/// - If the number is divisible by both 3 and 5, return "FizzBuzz".
/// - Otherwise, return the number as a string.
/// Use a match expression or an if-else chain.
pub fn fizzbuzz(n: u32) -> String {
    todo!("Implement fizzbuzz")
}

/// Sums all even numbers from 0 up to and including the given maximum.
/// Use a loop (for or while) to iterate through the range.
/// If the maximum is negative, return 0.
pub fn sum_even_numbers_up_to(max: i32) -> i32 {
    todo!("Implement sum_even_numbers_up_to")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(-4));
        assert!(!is_even(1));
        assert!(!is_even(-3));
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(2), "2");
        assert_eq!(fizzbuzz(0), "FizzBuzz"); // 0 is divisible by both 3 and 5
    }

    #[test]
    fn test_sum_even_numbers() {
        assert_eq!(sum_even_numbers_up_to(10), 30); // 0 + 2 + 4 + 6 + 8 + 10 = 30
        assert_eq!(sum_even_numbers_up_to(5), 6);   // 0 + 2 + 4 = 6
        assert_eq!(sum_even_numbers_up_to(0), 0);
        assert_eq!(sum_even_numbers_up_to(-1), 0);
    }
}

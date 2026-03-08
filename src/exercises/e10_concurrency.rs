use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

/// Topic 10: Concurrency
///
/// Exercise 1: Spawn threads to perform work.
/// Write a function that takes a vector of strings, spawns a thread for each string to calculate
/// its length, and returns the total sum of all lengths.
/// Hint: `thread::spawn`, and `join()`.
pub fn run_tasks(tasks: Vec<String>) -> usize {
    todo!("Implement run_tasks")
}

/// Exercise 2: Shared mutable state.
/// Write a function that creates a shared counter, spawns `num_threads` threads, and has each
/// thread increment the counter exactly 100 times. Return the final value of the counter.
/// Hint: Use `Arc<Mutex<i32>>`.
pub fn shared_counter(num_threads: usize) -> i32 {
    todo!("Implement shared_counter")
}

/// Exercise 3: Message passing using Channels.
/// Write a function that calculates the sum of a slice of numbers by splitting the work across
/// a given number of threads. Use `mpsc` channels to send the partial sums from the workers
/// back to the main thread, which computes the final total.
///
/// If `num_threads` is 0, return 0. If `numbers` is empty, return 0.
/// Try to divide the slice as evenly as possible among the threads.
pub fn channel_sum(numbers: &[i32], num_threads: usize) -> i32 {
    todo!("Implement channel_sum")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_tasks() {
        let tasks = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
        assert_eq!(run_tasks(tasks), 14);

        let empty: Vec<String> = vec![];
        assert_eq!(run_tasks(empty), 0);
    }

    #[test]
    fn test_shared_counter() {
        assert_eq!(shared_counter(5), 500);
        assert_eq!(shared_counter(10), 1000);
        assert_eq!(shared_counter(0), 0);
    }

    #[test]
    fn test_channel_sum() {
        let numbers: Vec<i32> = (1..=100).collect();
        // Sum of 1..=100 is 5050
        assert_eq!(channel_sum(&numbers, 4), 5050);
        assert_eq!(channel_sum(&numbers, 1), 5050);
        assert_eq!(channel_sum(&numbers, 10), 5050);
        
        let empty: Vec<i32> = vec![];
        assert_eq!(channel_sum(&empty, 4), 0);
        assert_eq!(channel_sum(&numbers, 0), 0);
    }
}

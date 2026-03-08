use std::time::Duration;
use tokio::task::JoinSet;

/// Topic 11: Asynchronous Programming (Tokio)
///
/// Exercise 1: Basic Async Function
/// Convert the concept of a regular function returning a String into an async function.
/// It should return the exact string "Hello, Async Rust!".
pub async fn async_hello() -> String {
    todo!("Implement async_hello")
}

/// Exercise 2: Simulated Asynchronous I/O
/// Write an async function that simulates fetching data from a database.
/// It should asynchronously sleep for 50 milliseconds using `tokio::time::sleep`,
/// and then return `Ok(format!("Data for {}", id))` if the id is even,
/// or `Err("Not found".to_string())` if the id is odd.
pub async fn fetch_data_simulated(id: u32) -> Result<String, String> {
    todo!("Implement fetch_data_simulated")
}

/// Exercise 3: Concurrent Task Execution
/// Given a list of IDs, execute the `fetch_data_simulated` functions concurrently.
/// Collect only the successful String results (`Ok` variants), ignoring the errors,
/// and return them as a `Vec<String>`. The order of the results in the vector does not matter.
///
/// Hint: Consider using `tokio::spawn` and `JoinSet`, or `tokio::join!` / `join_all` from
/// the `futures` crate (if available). For this exercise, `JoinSet` or iterating over a 
/// collection of spawned tasks is highly recommended as it comes with tokio.
pub async fn fetch_all_concurrently(ids: Vec<u32>) -> Vec<String> {
    todo!("Implement fetch_all_concurrently")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_async_hello() {
        assert_eq!(async_hello().await, "Hello, Async Rust!");
    }

    #[tokio::test]
    async fn test_fetch_data_simulated() {
        let start = Instant::now();
        let result_even = fetch_data_simulated(2).await;
        let duration = start.elapsed();
        
        assert_eq!(result_even, Ok("Data for 2".to_string()));
        assert!(duration >= Duration::from_millis(50), "Should sleep for at least 50ms");

        let result_odd = fetch_data_simulated(3).await;
        assert_eq!(result_odd, Err("Not found".to_string()));
    }

    #[tokio::test]
    async fn test_fetch_all_concurrently() {
        let ids = vec![1, 2, 3, 4, 5, 6];
        let start = Instant::now();
        
        let mut results = fetch_all_concurrently(ids).await;
        let duration = start.elapsed();
        
        // Ensure it runs concurrently (should take ~50ms total, not 6 * 50ms = 300ms)
        assert!(duration < Duration::from_millis(150), "Tasks act sequentially! Use concurrent execution.");
        
        results.sort(); // Sort since task completion order is not guaranteed
        let expected = vec![
            "Data for 2".to_string(),
            "Data for 4".to_string(),
            "Data for 6".to_string(),
        ];
        
        assert_eq!(results, expected);
    }
}

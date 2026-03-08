# Rust TDD Guide: Exercise Design Document

## 1. Goal and Vision
This document outlines the workflow and curriculum structure for the Rust Test-Driven Development (TDD) guide. The goal is to provide a smooth, isolated testing experience for users and a clear blueprint for future AI agents to implement new exercises.

## 2. User Testing Workflow

To prevent users from being overwhelmed by the test output of the entire workspace, exercises should be run in **isolation**.

### Recommended Workflow for Users
1. **Open the Exercise:**
   Navigate to `src/exercises/e<number>_<topic>.rs` (e.g., `e01_functions.rs`).
2. **Run Tests in Isolation:**
   Use Cargo's built-in test filtering so it only runs tests inside the current exercise module:
   ```bash
   cargo test e01
   ```
   This command only runs tests matching `e01` (i.e. all tests inside the `e01_functions` module).
3. **Implement the Stubs:**
   Read the doc-comments for the function. Replace the `todo!()` or `unimplemented!()` stubs with code you believe satisfies the requirements.
4. **Iterate:**
   Repeatedly run `cargo test e01` until all tests pass (you reach the "Green" state).
5. **Move to Next Exercise:**
   Once finished, proceed to `e02` and run `cargo test e02`.

*(Future Extension)*: An overarching `cargo watch` script or custom cargo runner could be provided to automatically detect modifications and run the appropriate tests, but for now, the isolated `cargo test <module>` is robust and uses standard tooling.

---

## 3. Curriculum: Rust Topics & Exercises

Here is the planned sequential list of Rust topics to cover, along with proposed exercises for each.

### Topic 1: Functions and Basic Types
- **Module:** `e01_functions`
- **Exercises:**
  - `add(a: i32, b: i32) -> i32`
  - `greet(name: &str) -> String`
  - `square(n: i32) -> i32`

### Topic 2: Control Flow (If/Else, Loops)
- **Module:** `e02_control_flow`
- **Exercises:**
  - `is_even(n: i32) -> bool` (if/else mechanism)
  - `fizzbuzz(n: u32) -> String` (match / if-else chain)
  - `sum_even_numbers_up_to(max: i32) -> i32` (using a `for` or `while` loop)

### Topic 3: Ownership and Borrowing
- **Module:** `e03_ownership`
- **Exercises:**
  - `get_length(s: &String) -> usize` (taking an immutable reference)
  - `append_world(s: &mut String)` (taking a mutable reference)
  - `clone_and_modify(s: &String) -> String` (cloning a string to modify it independently)

### Topic 4: Structs and Implementations
- **Module:** `e04_structs`
- **Exercises:**
  - Define a `User` struct with standard fields (name, email, age).
  - Implement a `new` associated function (constructor) for `User`.
  - Implement a `change_email(&mut self, new_email: &str)` method.
  - Define a `Rectangle` struct and its `area(&self) -> u32` method.

### Topic 5: Enums and Pattern Matching
- **Module:** `e05_enums`
- **Exercises:**
  - Define a `Message` enum (e.g., `Quit`, `Move { x, y }`, `Write(String)`, `ChangeColor(i32, i32, i32)`).
  - Implement a `process_message(msg: Message) -> String` function returning string representations.
  - Define a custom configuration enum and implement a safe unwrap function with a fallback value.

### Topic 6: Options and Results (Error Handling)
- **Module:** `e06_error_handling`
- **Exercises:**
  - `safe_divide(a: i32, b: i32) -> Option<i32>` (Return `None` if `b == 0`).
  - `read_file_length(path: &str) -> Result<usize, String>` (Return a generic error string).
  - `parse_and_multiply(s1: &str, s2: &str) -> Result<i32, std::num::ParseIntError>` (Demonstrate the `?` operator).

### Topic 7: Collections (Vectors and HashMaps)
- **Module:** `e07_collections`
- **Exercises:**
  - `filter_even_and_sort(numbers: &[i32]) -> Vec<i32>`
  - `word_frequency(text: &str) -> HashMap<String, u32>`
  - `find_mode(numbers: &[i32]) -> Option<i32>` (Utilize HashMap and Vec together)

### Topic 8: Traits and Generics
- **Module:** `e08_traits`
- **Exercises:**
  - Define a `Summary` trait with a `summarize(&self) -> String` method.
  - Implement `Summary` for `Article` and `Tweet` structs.
  - Write a generic function `get_summary<T: Summary>(item: &T) -> String`.
  - Implement the `std::fmt::Display` trait for a custom struct.

### Topic 9: Iterators and Closures
- **Module:** `e09_iterators`
- **Exercises:**
  - Re-implement `sum_even_numbers_up_to` using iterators (`filter`, `sum`).
  - `capitalize_words(words: Vec<&str>) -> Vec<String>` using `map`.
  - Filter a vector of structures applying a closure for an arbitrary condition (e.g., users older than 18).

### Topic 10: Concurrency
- **Module:** `e10_concurrency`
- **Exercises:**
  - `run_tasks(tasks: Vec<String>) -> usize` (Spawn threads to count characters)
  - `shared_counter(threads: usize) -> i32` (Use `Arc<Mutex<i32>>` state)
  - `channel_sum(numbers: &[i32], num_threads: usize) -> i32` (Use `mpsc` channels)

### Topic 11: Asynchronous Programming (Tokio)
- **Module:** `e11_async`
- **Exercises:**
  - `async_hello() -> String` (Basic async fn)
  - `fetch_data_simulated(id: u32) -> Result<String, String>` (Simulated I/O with `tokio::time::sleep`)
  - `fetch_all_concurrently(ids: Vec<u32>) -> Vec<String>` (Use `tokio::spawn` and `JoinSet`)

### Topic 12: Terminal UIs with Ratatui
- **Module:** `e12_ratatui_basics`
- **Exercises:**
  - `welcome_paragraph() -> Paragraph<'static>` (Basic text widget)
  - `create_app_layout(area: Rect) -> Rc<[Rect]>` (Layout engine)
  - `styled_block() -> Block<'static>` (Borders and titles)

---

## 4. Implementation Guidelines for AI Agents

When instructed to add a new exercise module, agents should follow these rules strictly:

### A. Test-Driven "Red" State Default
All newly added exercise functions must initially **panic** using `todo!("Implement this function")` or `unimplemented!()`. Users exist to learn, and must start in a "Red" (failing) state so they write code to achieve the "Green" (passing) state.

### B. Clear Instructions
Use extensive doc-comments (`///`) attached to every exercise function. Clearly explain *exactly* what the function should do, taking care not to spoil the exact Rust syntax required for the solution.

### C. Comprehensive Testing
Inside the internal test module (`#[cfg(test)] mod tests { ... }`), provide well-named unit tests covering:
- Standard "happy path" behavior
- Common boundary conditions
- Handled edge cases / error situations

### D. Module Registration
To ensure the Rust compiler tracks the new tests:
- Add a new `pub mod e<number>_<topic>;` declaration into `src/exercises/mod.rs`.
- Verify the main crate exposes the parent module (e.g., `pub mod exercises;` should exist in `src/lib.rs`).

### E. Isolated Module Structure
Every topic should have its own separate `.rs` file module encapsulating all implementations and tests for that specific lesson. Example layout for the test module:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Write your tests here...
}
```

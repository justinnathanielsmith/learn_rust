---
description: Create a new Rust TDD exercise module
---

# Creating a New Rust TDD Exercise

When the user asks you to create a new exercise (e.g., `/create_exercise e02_control_flow`), follow these exact steps to ensure the exercise fits the curriculum and testing workflow.

## 1. Understand the Request
Identify the exercise number and topic (e.g., `e02_control_flow`). Read `EXERCISE_DESIGN.md` in the workspace root to understand which specific functions/concepts are required for this topic.

## 2. Create the Module File
Create a new file in the `src/exercises/` directory (e.g., `src/exercises/e02_control_flow.rs`).

## 3. Implement the Exercise Structure
Inside the new module file, write the exercise following these rules:
- **Doc-Comments**: Every function must start with a `///` doc-comment explaining exactly what the function should do. Do not write the solution in the comments.
- **Red State Default**: All function bodies must initially just be `todo!("Implement this function")` or `unimplemented!()`. The user's goal is to replace this!
- **Test Module**: At the bottom of the file, create a test module:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;
      // Add multiple #[test] functions here covering happy paths and edge cases.
  }
  ```

## 4. Register the Module
Open `src/exercises/mod.rs`. Add the new module as a public module so the compiler and test runner can find it:
```rust
pub mod e01_functions;
// Add the new one:
pub mod e02_control_flow; 
```

## 5. Verify the "Red" State (DO NOT FIX IT)
Run the tests in isolation to ensure they fail as expected:
```bash
cargo test e<number>
```
For example, `cargo test e02`. **DO NOT** fix the failing tests. The tests *must* fail so the user can complete the exercise.

## 6. Notify the User
Inform the user that the exercise is ready and tell them to run `cargo test e<number>` to start working on it!

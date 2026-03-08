# Rust TDD Guide: Master Rust Through Testing

Welcome to the Rust TDD Guide! This project is designed to help you learn Rust by writing code that makes tests pass. We follow the **Test-Driven Development (TDD)** philosophy: **Red, Green, Refactor.**

---

## 🚀 The TDD Workflow

To learn effectively, follow these three steps for every exercise:

1.  **🔴 RED**: Run the tests for your current exercise. They will fail because the implementation uses `todo!()`.
2.  **🟢 GREEN**: Implement just enough code to satisfy the requirements and make the tests pass.
3.  **🔵 REFACTOR**: Once the tests are green, take a moment to clean up your code, improve naming, or optimize logic while keeping the tests passing.

---

## 🛠️ How to Work Through Exercises

### 1. Identify Your Current Exercise
Exercises are located in `src/exercises/`. They are numbered sequentially (e.g., `e01_functions.rs`, `e02_control_flow.rs`).

### 2. Run Tests in Isolation (Recommended)
Instead of running all tests in the project, focus on the specific module you are working on. This keeps the output clean and manageable.

Use the following command, replacing `e01` with the exercise number:
```bash
cargo test e01
```

### 3. Implement the Solution
Read the doc-comments (`///`) above each function in the exercise file. They explain the requirements. Replace `todo!()` with your code.

### 4. Iterate
Keep running `cargo test e<number>` until you see the glorious green output:
`test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; ...`

---

## 📝 Tracking Progress with Jujutsu (JJ)

We highly recommend using Jujutsu (`jj`) to track your progress through the exercises. Its unique version control model makes moving between exercises and fixing mistakes trivial!

### Setup
Ensure you've run `jj git init --colocate` (or cloned with `jj git clone`) in the repository root. This gives you a `jj` workspace alongside `git`. We also recommend creating a bookmark at the initial "all todo" state:
```bash
jj bookmark create start
```

### The Workflow
1. **Solve** an exercise (e.g., `e01_functions.rs`).
2. **Test** until green (`cargo test e01`).
3. **Commit** your progress to start fresh for the next one:
   ```bash
   jj commit -m "Completed e01"
   ```
This finalizes your progress for `e01` into a commit, and yields a new empty working copy for you to start `e02` on!

### Fixing Mistakes
- **Reset current exercise**: `jj restore` (clears uncommitted changes in your current working copy).
- **Restart an old exercise entirely**: `jj restore -c start src/exercises/e01_functions.rs` (brings back the `todo!()`s for just that file from the beginning).
- **Start the whole guide over**: `jj new start` (abandons your current timeline and starts fresh).

### 5. Consult the Lessons
If you are new to Rust or need a refresher, check out the beginner-friendly lessons in the `lessons/` directory. Each lesson corresponds to an exercise and explains the concepts needed to solve it.

---

## 📚 Curriculum Overview

Here is the path you will take through the Rust landscape. Click the **Lesson** link for an explanation of the concepts, or the **Module** link to start coding!

| ID | Topic | Lesson | Module File | Status |
|:---|:---|:---|:---|:---|
| **e01** | Functions & Basic Types | [Lesson](lessons/e01_functions.md) | `e01_functions.rs` | ✅ Ready |
| **e02** | Control Flow | [Lesson](lessons/e02_control_flow.md) | `e02_control_flow.rs` | ✅ Ready |
| **e03** | Ownership & Borrowing | [Lesson](lessons/e03_ownership.md) | `e03_ownership.rs` | ✅ Ready |
| **e04** | Structs & Implementations | [Lesson](lessons/e04_structs.md) | `e04_structs.rs` | ✅ Ready |
| **e05** | Enums & Pattern Matching | [Lesson](lessons/e05_enums.md) | `e05_enums.rs` | ✅ Ready |
| **e06** | Error Handling | [Lesson](lessons/e06_error_handling.md) | `e06_error_handling.rs` | ✅ Ready |
| **e07** | Collections | [Lesson](lessons/e07_collections.md) | `e07_collections.rs` | ✅ Ready |
| **e08** | Traits & Generics | [Lesson](lessons/e08_traits.md) | `e08_traits.rs` | ✅ Ready |
| **e09** | Iterators & Closures | [Lesson](lessons/e09_iterators.md) | `e09_iterators.rs` | ✅ Ready |
| **e10** | Concurrency | [Lesson](lessons/e10_concurrency.md) | `e10_concurrency.rs` | ✅ Ready |
| **e11** | Async Programming | [Lesson](lessons/e11_async.md) | `e11_async.rs` | ✅ Ready |
| **e12** | Ratatui Basics | [Lesson](lessons/e12_ratatui_basics.md) | `e12_ratatui_basics.rs` | ✅ Ready |
---

## 💡 Success Tips

-   **Trust the Compiler**: Rust's error messages are world-class. If it doesn't compile, read the message carefully—it often tells you exactly how to fix it.
-   **One Test at a Time**: If an exercise has multiple tests, focus on making the first one pass before moving to the next.
-   **Don't overcomplicate**: The goal is to learn the syntax and concepts. "Green" code is the first priority; "Beautiful" code comes during the Refactor stage.

Happy Coding! 🦀

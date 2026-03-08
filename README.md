# Rust TDD Journey 🦀

Master the Rust programming language through **Test-Driven Development (TDD)**.

This project is a curated path of exercises designed to take you from Rust beginner to confident developer by following the **Red-Green-Refactor** workflow.

## 🏁 Getting Started

1.  **Clone the Repository:**
    ```bash
    git clone <repository-url>
    cd rust-tdd-guide
    # Optional but recommended: initialize Jujutsu for tracking progress
    jj git init --colocate
    ```

2.  **Read the Guide:**
    Check out [GUIDE.md](./GUIDE.md) for detailed instructions on the TDD workflow and how to run tests in isolation.

3.  **Start Your First Exercise:**
    Open `src/exercises/e01_functions.rs` and run:
    ```bash
    cargo test e01
    ```

## 🗺️ Curriculum Path

The curriculum is designed to be followed in order:

- **e01**: Functions & Basic Types
- **e02**: Control Flow (If/Else, Loops)
- **e03**: Ownership & Borrowing
- **e04**: Structs & Implementations
- **e05**: Enums & Pattern Matching
- **e06**: Error Handling
- **e07**: Collections
- **e08**: Traits & Generics
- **e09**: Iterators & Closures
- **e10**: Concurrency
- **e11**: Async Programming
- **e12**: Ratatui Basics

For the full roadmap and architecture details, see [EXERCISE_DESIGN.md](./EXERCISE_DESIGN.md).

## 🛠️ Prerequisites

- **Rust & Cargo**: [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already.
- **Jujutsu (jj)**: *(Optional but recommended)* [Install jj](https://github.com/martinvonz/jj) for the best exercise tracking experience.

---
*Happy learning, and may your compiler errors be helpful!*

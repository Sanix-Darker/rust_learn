# Rust Learning Map

## LEARN MAP

![map](./rust.svg)

[CHECK THE INTERACTIVE MAP](https://sanix-darker.github.io/rust_learn/rust.html)

## CONCEPTS
- What is Rust?
  - Systems programming language
  - Memory safety without garbage collection
  - Concurrency
- History of Rust
- Installation
  - Rustup
  - Cargo

## Basic Concepts
- Syntax and Semantics
  - Hello World
  - Comments
  - Variables
    - Immutable by default
    - Mutable variables
  - Data Types
    - Scalar types (integer, floating-point, boolean, character)
    - Compound types (tuples, arrays)
  - Functions
    - Defining functions
    - Function parameters
    - Return values
  - Control Flow
    - If-else
    - Loops (loop, while, for)
    - Match

## Ownership and Borrowing
- Ownership
  - Ownership rules
  - Variable scope
  - Memory management
- Borrowing
  - References
  - Mutable references
  - Slices

## Structs and Enums
- Structs
  - Defining structs
  - Tuple structs
  - Unit-like structs
  - Methods
- Enums
  - Defining enums
  - Enum variants
  - Pattern matching with enums

## Error Handling
- Panic
  - Unrecoverable errors
  - Using panic! macro
- Result
  - Recoverable errors
  - Using Result enum
  - Error propagation with ?
- Option
  - Representing optional values
  - Using Option enum

## Collections
- Vectors
  - Creating vectors
  - Modifying vectors
  - Iterating over vectors
- Strings
  - String vs &str
  - Creating strings
  - Modifying strings
- Hash Maps
  - Creating hash maps
  - Modifying hash maps
  - Iterating over hash maps

## Lifetimes
- Lifetime Annotations
  - Function lifetimes
  - Struct lifetimes
  - Lifetime elision
- Lifetime Bounds
  - Generic type parameters
  - Lifetime bounds

## Generics
- Defining Generics
  - Generic functions
  - Generic structs
  - Generic enums
- Traits
  - Defining traits
  - Implementing traits
  - Trait bounds

## Concurrency
- Threads
  - Spawning threads
  - Thread handles
  - Using join
- Channels
  - Message passing
  - Creating channels
  - Sending and receiving messages
- Mutex
  - Shared state concurrency
  - Mutex locks

## Advanced Topics
- Macros
  - Declarative macros
  - Procedural macros
  - Attribute-like macros
  - Function-like macros
- Unsafe Rust
  - Unsafe keyword
  - Raw pointers
  - Calling unsafe functions
- Crates and Modules
  - Creating a crate
  - Defining modules
  - Re-exporting names with pub use
- FFI (Foreign Function Interface)
  - Calling C code from Rust
  - Creating bindings

## Tools and Ecosystem
- Cargo
  - Creating a project
  - Building and running
  - Dependencies
  - Cargo.toml
- Rustfmt
  - Code formatting
  - Using rustfmt
- Clippy
  - Linting
  - Using clippy
- Rustdoc
  - Generating documentation
  - Writing doc comments
- Testing
  - Unit tests
  - Integration tests
  - Documentation tests

## Best Practices
- Writing idiomatic Rust
  - Borrowing and ownership
  - Error handling
  - Code organization
- Performance optimization
  - Profiling tools
  - Efficient data structures
  - Avoiding unnecessary allocations
- Security
  - Safe Rust
  - Auditing unsafe code

## Resources
- Official Documentation
  - The Rust Programming Language (The Book)
  - Rust by Example
  - Rust Reference
- Tutorials and Courses
  - Rustlings
  - Exercism.io
  - Codecademy
  - Udemy
- Books
  - "Programming Rust" by Jim Blandy and Jason Orendorff
  - "Rust in Action" by Tim McNamara
  - "The Rust Programming Language" by Steve Klabnik and Carol Nichols
- Online Communities
  - Rust Users Forum
  - Rust subreddit (r/rust)
  - Rust Discord
  - Stack Overflow
- Practice Projects
  - Building a CLI tool
  - Developing a web application with Rocket
  - Creating a game with Amethyst

## Conclusion
- Keeping up-to-date
  - Rust Blog
  - This Week in Rust
  - Rust release notes
  - Community events (RustConf, RustFest)

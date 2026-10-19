# 01-rust-course

A sequence of small, standalone Rust projects. Each lesson is its own Cargo
package; run it from its folder with `cargo run`. Run a lesson's tests with
`cargo test`.

## Lessons

| Lesson | Topics |
| --- | --- |
| [01 - Modules](./rust-01-modules/) | Modules and organizing code |
| [02 - Control structures](./rust-02-control-structures/) | Conditions, loops, and `match` |
| [03 - Data types](./rust-03-data-types/) | Scalar and compound types, strings, conversions |
| [04 - Functions](./rust-04-functions/) | Parameters, return values, scope, higher-order functions |
| [05 - Ownership and borrowing](./rust-05-ownership-and-borrowing/) | Moves, clones, references, mutable borrowing |
| [06 - References and slices](./rust-06-references-and-slices/) | Shared/mutable references, array and string slices |
| [07 - Structs and enums](./rust-07-structs-and-enums/) | Struct data, methods, associated functions, enum variants |
| [08 - Pattern matching](./rust-08-pattern-matching/) | Exhaustive matches, destructuring, `if let`, guards |
| [09 - Collections](./rust-09-collections/) | Vectors, UTF-8 strings, hash maps |
| [10 - Error handling](./rust-10-error-handling/) | `Option`, `Result`, `?`, custom errors |
| [11 - Generics and traits](./rust-11-generics-and-traits/) | Generic types, trait implementations, bounds, associated types |
| [12 - Lifetimes](./rust-12-lifetimes/) | Lifetime annotations, borrowed structs, elision |
| [13 - Iterators and closures](./rust-13-iterators-and-closures/) | Closures, iterator consumers/adaptors, custom iterators |
| [14 - Modules and crates](./rust-14-modules-and-crates/) | Module paths, visibility, library and binary crates |
| [15 - Testing](./rust-15-testing/) | Unit, integration, documentation, and expected-panic tests |
| [16 - File I/O](./rust-16-file-io/) | Paths, metadata, reading and writing files |
| [17 - Smart pointers](./rust-17-smart-pointers/) | `Box`, `Rc`, `Weak`, `RefCell`, `Drop` |
| [18 - Concurrency](./rust-18-concurrency/) | Threads, scoped threads, channels, shared state |
| [19 - Asynchronous programming](./rust-19-asynchronous-programming/) | Futures, `async`/`await`, executor polling |
| [20 - Project development](./rust-20-project-development/) | A tested, modular task-tracker application |

Each project uses the Rust 2024 edition and the standard library only. Lesson
19 includes a small executor to make a simple `async` workflow runnable without
an external runtime; real network or timer applications typically use a
runtime such as Tokio or async-std.

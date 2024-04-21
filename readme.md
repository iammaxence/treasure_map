# Installation

You need to install Rust and Cargo

`curl https://sh.rustup.rs -sSf | sh`

For coverage:

`cargo +stable install cargo-llvm-cov --locked`

Official documentation:

- https://doc.rust-lang.org/cargo/getting-started/installation.html
- https://lib.rs/crates/cargo-llvm-cov#readme-installation

# Run app

It takes the file in files/exercice.txt to execute de program.
The result can be found in files/result.txt.

## Run command

`cargo run`

# Testing

Unit tests can be seen in the same file as the code. It's the way rust do:

**You can write your test code in a separate file if you'd like, but Rust projects tend to keep the test modules in the same file as program code.**

## Run test command

`cargo test`

## coverage

`cargo llvm-cov --html`

note: A known problem exists when running this command with Windows.. I don't know how to fix it yet. I will do it when i have some time.

# Brainfork
Brainfork is a compiler that compiles Brainfuck into Rust for better performance. That Rust is then compiled into LLVM IR, and finally Assembly. This ensures maximum performance for our programs.
## 🚀 Features
- High performance Brainfuck because it compiles to Rust.
- Very fast compile times.
- Safe, because it is written in Rust.
- Easy to use, just ``brainfork program.bf`` to compile it into a binary.
## 🧰 Installation
Just install Brainfork with ``cargo install --locked brainfork``. This will work on every platform. Otherwise you can compile from source by cloning this repository, and running ``cargo build --release``.
## Requirements
Brainfork requires a Rust compiler to be used, as it compiles the Brainfuck into Rust. Ensure you have a Rust compiler installed, otherwise get one from [here](https://rustup.rs/).

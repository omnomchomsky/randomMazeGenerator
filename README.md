# Random Maze Generator

A simple Rust-based random maze generator. This program generates mazes of size `X` by `Y` using a customizable grid.

## Building the Project

To build the project in debug mode:

```bash
cargo build
```

To build the project in release mode (optimized for performance):


```bash
cargo build --release
```

# Running The Program

To run the program with cargo (debug mode):

```bash
cargo run X Y
```
where X and Y are positive integers.

To run the binary directly (debug):
```bash
target/debug/randomMazeGenerator X Y
```

To run the binary directly (release mode):
```bash
target/release/randomMazeGenerator X Y
```

# Example

To generate a 20 by 20 maze in release mode:
```bash
cargo build --release
target/release/randomMazeGenerator 20 20
```

# Prerequisites
- [Rust](https://rustup.rs/) and cargo installed

# License
- MIT
# rust-book

This repository contains my notes and exercises from reading ["The Rust Programming Language"](https://rust-book.cs.brown.edu) book, commonly referred to as "The Rust Book". Moreover, it includes [rustlings](https://rustlings.rust-lang.org/) exercises to help reinforce my understanding of Rust concepts.


## Getting Started

To get started with this project, you'll need to have [Rust](https://www.rust-lang.org/tools/install) installed on your system. You can install it using `rustup`, the Rust toolchain installer.


1.  **Clone the repository:**
    ```bash
    git clone https://github.com/leidix/rust-book.git
    cd rust-book
    ```


2.  **Build all projects:**

    This project is a Cargo workspace, which means it contains multiple packages. To build all the main projects, run the following command from the root of the repository:
    ```bash
    cargo build
    ```
    This command builds the `default-members` of the workspace, which are the projects located in the `src/projects` directory. The `rustlings` exercises are intentionally excluded from the default build because they are expected to have failing code until they are solved.


3.  **Run a specific project:**

    To run a specific project within the workspace, use the `-p` (or `--package`) flag with `cargo run`. For example, to run the `hello_world` project:
    ```bash
    cargo run -p hello_world
    ```


## Rustlings Exercises

This repository also includes the `rustlings` exercises. You can work on them directly from this workspace without needing to install `rustlings` separately.

To start the exercises, run the following convenient command from the root of your project:

```bash
cargo rustlings
```

This command is a Cargo alias defined in `.cargo/config.toml` that will build and run the `rustlings` program in `watch` mode, providing an interactive experience.

Your progress on the exercises will be saved in the `src/rustlings/exercises` directory. By committing these changes to your repository, you ensure that anyone who clones it will have your exact progress.

## Directory Structure

* `src/projects`: Contains your personal Rust projects.
* `src/rustlings`: Contains the complete `rustlings` project, including the exercises and the runner program.
* `Cargo.toml`: The top-level manifest file that defines the Cargo workspace.

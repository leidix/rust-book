# rust-book

This repository contains my notes and exercises from reading ["The Rust Programming Language"](https://rust-book.cs.brown.edu) book, commonly referred to as "The Rust Book". Moreover, it includes [rustlings](https://rustlings.rust-lang.org/) exercises to help reinforce my understanding of Rust concepts.

## Reproducing this Repository

To reproduce this repository, you will need to have [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed. You will also need a linker such as `gcc` or `clang` installed on your system.

To install rust and Cargo, you can use the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To install a linker, you can use your system's package manager. For example, on Ubuntu, you can use:

```bash
sudo apt-get install build-essential
```

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/leidix/rust-book.git
    ```
2.  **Navigate to the project directory:**
    ```bash
    cd rust-book
    ```
3. **Install dependencies:**
    ```bash
    cargo fetch
    ```
3.  **Build all projects:**
    ```bash
    cargo build
    ```
4.  **Run a specific project:**
    ```bash
    cargo run --bin <project_name>
    ```
    For example, to run the `hello_world` project:
    ```bash
    cargo run --bin hello_world
    ```

### Directory Structure

*   `src/projects`: Contains the source code for the individual projects.
*   `rustlings`: Contains the rustlings exercises.
*   `get-dependencies`: Contains a project to get dependencies for the projects.
*   `Cargo.toml`: The Cargo manifest file, which defines the projects and their dependencies.

### Working on Projects

Each project is located in the `src/projects` directory. You can navigate to each project folder and work on the code as needed. To run a specific project, use the `cargo run --bin <project_name>` command as shown above. To build all projects, simply use `cargo build` (it will build all binaries defined in the `Cargo.toml` file; make sure to add new binaries there if you create new projects).

### Rustlings Exercises

The `rustlings` directory contains exercises to help you learn Rust. To work on these exercises, navigate to the `rustlings` directory and enter the following command:

```bash
rustlings
```

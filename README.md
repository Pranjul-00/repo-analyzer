# Repo-Analyzer

`repo-analyzer` is a high-performance command-line interface (CLI) tool built in Rust. It provides a quick and efficient way to fetch and analyze GitHub repository statistics directly from your terminal.

Designed for developers who want to evaluate a project's health and popularity without leaving their workflow, it leverages asynchronous programming to deliver near-instant results.

---

## Features

-   **Repository Health at a Glance**: Instantly view stars, open issues, and project descriptions.
-   **Language Detection**: Identifies the primary programming language of the repository.
-   **Asynchronous & Fast**: Powered by `tokio` and `reqwest` for non-blocking I/O performance.
-   **Safe Data Handling**: Uses Rust's robust type system and `serde` for reliable JSON parsing.
-   **User-Friendly CLI**: Intuitive command-line arguments powered by `clap`.

---

## Technical Stack

-   **Language**: Rust (2024 Edition)
-   **Runtime**: [Tokio](https://tokio.rs/) (Asynchronous ecosystem)
-   **HTTP Client**: [Reqwest](https://docs.rs/reqwest/latest/reqwest/) (With JSON support)
-   **Parsing**: [Serde](https://serde.rs/) (Serialization/Deserialization)
-   **CLI Framework**: [Clap v4](https://docs.rs/clap/latest/clap/) (Command Line Argument Parser)

---

## Getting Started

### Prerequisites

You must have the Rust toolchain installed. If you haven't installed it yet, use [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation

1.  **Clone the Repository**:
    ```bash
    git clone https://github.com/Pranjul-00/repo-analyzer.git
    cd repo-analyzer
    ```

2.  **Build the Project**:
    ```bash
    cargo build --release
    ```
    The compiled binary will be available at `./target/release/repo-analyzer`.

3.  **Install Locally (Optional)**:
    To use the command from anywhere in your terminal:
    ```bash
    cargo install --path .
    ```

---

## Usage

The tool requires a GitHub repository path in the format `owner/repository`.

### Basic Command

Using `cargo run`:
```bash
cargo run -- --repo <owner/repo>
```

Using the compiled binary:
```bash
./target/release/repo-analyzer --repo rust-lang/rust
```

### Examples

**Analyze the Rust compiler repository:**
```bash
repo-analyzer --repo rust-lang/rust
```

**Output Example:**
```text
Scanning repository: rust-lang/rust...

--- Repository Status ---
Name:      rust
Stars:     102450
Issues:    8420
Language:  Rust
Desc:      Empowering everyone to build reliable and efficient software.
```

---

## Command Line Options

| Flag | Long Flag | Description | Required |
| :--- | :--- | :--- | :--- |
| `-r` | `--repo` | The GitHub repository to scan (e.g., `facebook/react`) | **Yes** |
| `-h` | `--help` | Prints help information | No |
| `-V` | `--version` | Prints version information | No |

---

## Error Handling

`repo-analyzer` gracefully handles common errors:
-   **Invalid Repositories**: Notifies you if a repository doesn't exist.
-   **Network Issues**: Provides clear feedback if there are connectivity problems.
-   **GitHub API Restrictions**: Correctly identifies when it's blocked by API rate limits or missing headers.

---

## Contributing

Contributions are welcome! Feel free to:
1. Fork the project.
2. Create your feature branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a Pull Request.

---

*Made with Rust.*

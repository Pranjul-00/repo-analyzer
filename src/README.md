# Repo Analyzer CLI

A fast and clean Rust-based command-line tool to quickly analyze any GitHub repository. Get stars, forks, issues, and more directly in your terminal with a beautifully styled output.

---

## Features

- **Interactive Mode**: Prompts you for the repository name if you forget to provide it.
- **Positional Arguments**: Fast usage with just `cargo run -- <repo>`.
- **Rich Output**: Clean, colored, and text-only data visualization.
- **Detailed Stats**: View stars, forks, watchers, open issues, repository size, language, and license.

---

## Installation

### 1. Prerequisites
Make sure you have [Rust and Cargo](https://rustup.rs/) installed on your system.

### 2. Clone the Repository
```bash
git clone https://github.com/your-username/repo-analyzer.git
cd repo-analyzer
```

### 3. Build the Project
```bash
cargo build --release
```

---

## How to Use

There are two ways to use the Repo Analyzer:

### Option A: Direct Command (Fastest)
Provide the `username/reponame` as an argument after `--`:
```bash
cargo run -- tokio-rs/tokio
```

### Option B: Interactive Mode
Simply run the program, and it will ask you for the repository name:
```bash
cargo run
```

---

## Advanced Usage (System-wide)

If you want to use the analyzer from anywhere on your computer without typing `cargo run`:

1. Build the release binary:
   ```bash
   cargo build --release
   ```
2. Move the binary to your local bin folder:
   ```bash
   # On Linux/macOS
   cp target/release/repo-analyzer /usr/local/bin/analyze
   ```
3. Now you can just type:
   ```bash
   analyze tokio-rs/tokio
   ```

---

## Rate Limits

By default, the GitHub API allows **60 requests per hour** for unauthenticated users. If you hit this limit, you will see an error message. Wait an hour or contribute to the project to help us add Token support!

---

## License
This project is open-source. Check the repository's license for more details.

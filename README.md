# Repo Analyzer CLI

A fast and clean Rust-based command-line tool to quickly analyze any GitHub repository. Get stars, forks, issues, and more directly in your terminal with a beautifully styled output.

---

## Features

- **Interactive Mode**: Prompts you for the repository name if you forget to provide it.
- **Positional Arguments**: Fast usage with just `cargo run -- <repo>`.
- **Professional UI**: Clean, colored table-based data visualization with a loading spinner.
- **Detailed Stats**: View stars, forks, watchers, open issues, repository size, language, and license.
- **High Rate Limits**: Support for GitHub Personal Access Tokens (PAT).

---

## Installation

### 1. Prerequisites
Make sure you have [Rust and Cargo](https://rustup.rs/) installed on your system.

### 2. Clone the Repository
```bash
git clone https://github.com/Pranjul-00/repo-analyzer.git
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

## Authentication (Optional)

By default, the GitHub API allows **60 requests per hour** for unauthenticated users. To increase this to **5,000 requests per hour**, you can use a Personal Access Token (PAT).

### 1. Generate a Token
1. Go to your GitHub **Settings** -> **Developer Settings** -> **Personal Access Tokens**.
2. Generate a new token (Fine-grained tokens are recommended with **Public Repositories (read-only)** access).
3. Copy your token.

### 2. Configure the Tool
1. In the project root, rename `.env.example` to `.env`:
   ```bash
   cp .env.example .env
   ```
2. Open `.env` and replace `your_token_here` with your actual GitHub token:
   ```text
   GITHUB_TOKEN=ghp_your_secret_token_here
   ```

**Note**: Your `.env` file is automatically ignored by git and will never be pushed to your repository.

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

## License
This project is open-source. Check the repository's license for more details.

---

*Made with Rust.*

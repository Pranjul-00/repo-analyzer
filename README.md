# Repo Analyzer CLI

A fast and clean Rust-based command-line tool to quickly analyze any GitHub repository. Get stars, forks, issues, and more directly in your terminal with a beautifully styled output.

---

## Features

- **Interactive Mode**: Prompts you for repository names if you forget to provide them.
- **Dynamic Comparison**: Compare two or more repositories side-by-side.
- **JSON Export**: Output all repository data in a machine-readable JSON format.
- **Top Contributors**: View the top 5 contributors and their total contributions.
- **Rate Limit Tracking**: Monitor your GitHub API usage and reset times in real-time.
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

### Option A: Direct Command (Fastest)
Provide one or more `username/reponame` as arguments:
```bash
# Analyze a single repository
cargo run -- tokio-rs/tokio

# Compare multiple repositories side-by-side
cargo run -- tokio-rs/tokio actix/actix-web facebook/react
```

### Option B: Interactive Mode
Simply run the program, and it will ask you for a repository name:
```bash
cargo run
```

### Option C: JSON Export
Use the `-j` or `--json` flag to get raw data for scripting:
```bash
cargo run -- tokio-rs/tokio --json
```

---

## Authentication (Recommended)

By default, the GitHub API allows **60 requests per hour** for unauthenticated users. To increase this to **5,000 requests per hour**, you can use a Personal Access Token (PAT).

1. Go to your GitHub **Settings** -> **Developer Settings** -> **Personal Access Tokens**.
2. Generate a new token (**Public Repositories (read-only)** access).
3. In the project root, rename `.env.example` to `.env` and add your token:
   ```text
   GITHUB_TOKEN=ghp_your_secret_token_here
   ```

---

## License
This project is open-source. Check the repository's license for more details.

---

*Made with Rust.*

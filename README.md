# repo-analyzer

A command-line interface (CLI) tool written in Rust that fetches and displays statistics for GitHub repositories.

## Installation

1. Clone the repository:
   git clone https://github.com/Pranjul-00/repo-analyzer.git

2. Navigate to the directory:
   cd repo-analyzer

3. Build the compiled release:
   cargo build --release

## Usage

Run the tool by providing a target repository using the `--repo` flag in the `owner/repo` format:

./target/release/repo-analyzer --repo torvalds/linux


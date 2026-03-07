#!/usr/bin/env bash

set -e

REPO_URL="https://github.com/Pranjul-00/repo-analyzer.git"
BIN_NAME="repo-analyzer"

echo "============================================"
echo "Installing $BIN_NAME..."
echo "============================================"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: 'cargo' is not installed."
    echo ""
    echo "This tool is built with Rust. Please install Rust first by running:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    echo "After installing Rust, restart your terminal and run this script again."
    exit 1
fi

echo "Rust toolchain found."
echo "Fetching and compiling from $REPO_URL..."
echo "This might take a minute depending on your internet connection and CPU."

# Run cargo install from the git repository
cargo install --git "$REPO_URL"

echo ""
echo "============================================"
echo "Successfully installed $BIN_NAME!"
echo "Make sure your cargo bin directory (usually ~/.cargo/bin) is in your PATH."
echo "You can now run it by typing: $BIN_NAME"
echo "============================================"

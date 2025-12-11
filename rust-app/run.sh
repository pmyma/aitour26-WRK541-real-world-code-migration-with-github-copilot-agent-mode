#!/bin/bash
set -e

# Build the project
echo "Building rust-app..."
cargo build

echo "Running rust-app..."
cargo run

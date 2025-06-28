#!/bin/bash

echo "Building Quizlr Web Application..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack is not installed. Please install it from https://rustwasm.github.io/wasm-pack/installer/"
    exit 1
fi

# Check if trunk is installed
if ! command -v trunk &> /dev/null; then
    echo "trunk is not installed. Please install it with: cargo install trunk"
    exit 1
fi

# Build the web application
cd quizlr-web
trunk build --release

echo "Build complete! Output is in quizlr-web/dist/"
#!/usr/bin/env just --justfile

# Default recipe to display help
default:
    @just --list

# Development
dev: setup
    @echo "Starting development server..."
    cd quizlr-web && trunk serve --open

# Run all tests
test:
    @echo "Running tests..."
    cargo test --workspace

# Run tests in watch mode
test-watch:
    @echo "Running tests in watch mode..."
    cargo watch -x "test --workspace"

# Run linters
lint:
    @echo "Running clippy..."
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    @echo "Checking formatting..."
    cargo fmt --all -- --check

# Format code
fmt:
    @echo "Formatting code..."
    cargo fmt --all

# Building
build: build-core build-web

# Build core library
build-core:
    @echo "Building core library..."
    cargo build --package quizlr-core --release

# Build web frontend
build-web: setup-web
    @echo "Building web frontend..."
    cd quizlr-web && trunk build --release

# Build documentation
build-docs:
    @echo "Building documentation..."
    cd docs && mdbook build

# Documentation
docs-serve: setup-docs
    @echo "Serving documentation at http://localhost:3000"
    cd docs && mdbook serve

# Build and open documentation
docs-open: setup-docs
    @echo "Building and opening documentation..."
    cd docs && mdbook build --open

# Deploy docs to GitHub Pages
docs-deploy: build-docs
    @echo "Deploying documentation to GitHub Pages..."
    @echo "TODO: Add gh-pages deployment"

# Release recipes
release-patch: test lint
    @echo "Bumping patch version..."
    cargo set-version --workspace --bump patch
    git add Cargo.toml */Cargo.toml
    git commit -m "chore: bump patch version"
    git tag -a v$(cargo metadata --no-deps --format-version 1 | jq -r '.workspace_members[0]' | cut -d' ' -f2) -m "Patch release"

release-minor: test lint
    @echo "Bumping minor version..."
    cargo set-version --workspace --bump minor
    git add Cargo.toml */Cargo.toml
    git commit -m "chore: bump minor version"
    git tag -a v$(cargo metadata --no-deps --format-version 1 | jq -r '.workspace_members[0]' | cut -d' ' -f2) -m "Minor release"

release-major: test lint
    @echo "Bumping major version..."
    cargo set-version --workspace --bump major
    git add Cargo.toml */Cargo.toml
    git commit -m "chore: bump major version"
    git tag -a v$(cargo metadata --no-deps --format-version 1 | jq -r '.workspace_members[0]' | cut -d' ' -f2) -m "Major release"

# Utilities
clean:
    @echo "Cleaning build artifacts..."
    cargo clean
    rm -rf quizlr-web/dist
    rm -rf docs/book

# Setup development environment
setup: setup-rust setup-web setup-docs
    @echo "Development environment ready!"

# Install Rust toolchain and tools
setup-rust:
    @echo "Setting up Rust toolchain..."
    rustup target add wasm32-unknown-unknown
    @echo "Installing cargo tools..."
    cargo install cargo-watch || true
    cargo install cargo-edit || true

# Install web development tools
setup-web:
    @echo "Installing web development tools..."
    cargo install trunk || true
    cargo install wasm-pack || true

# Install documentation tools
setup-docs:
    @echo "Installing mdBook..."
    cargo install mdbook || true

# Check for outdated dependencies
check-deps:
    @echo "Checking for outdated dependencies..."
    cargo outdated --workspace

# Update dependencies
update-deps:
    @echo "Updating dependencies..."
    cargo update --workspace

# Run cargo check
check:
    @echo "Running cargo check..."
    cargo check --workspace --all-targets

# Run cargo doc
doc:
    @echo "Building API documentation..."
    cargo doc --workspace --no-deps --open

# CI/CD checks (what CI would run)
ci: lint test check
    @echo "All CI checks passed!"

# Development shortcuts
d: dev
t: test
tw: test-watch
l: lint
f: fmt
b: build
c: clean

# Help
help:
    @echo "Quizlr Development Commands"
    @echo "=========================="
    @echo ""
    @echo "Development:"
    @echo "  just dev          - Start development server"
    @echo "  just test         - Run all tests"
    @echo "  just test-watch   - Run tests in watch mode"
    @echo "  just lint         - Run linters"
    @echo "  just fmt          - Format code"
    @echo ""
    @echo "Building:"
    @echo "  just build        - Build all crates"
    @echo "  just build-core   - Build core library"
    @echo "  just build-web    - Build web frontend"
    @echo "  just build-docs   - Build documentation"
    @echo ""
    @echo "Documentation:"
    @echo "  just docs-serve   - Serve documentation locally"
    @echo "  just docs-open    - Build and open documentation"
    @echo "  just doc          - Build API documentation"
    @echo ""
    @echo "Utilities:"
    @echo "  just setup        - Install required tools"
    @echo "  just clean        - Clean build artifacts"
    @echo "  just check-deps   - Check for outdated dependencies"
    @echo ""
    @echo "Shortcuts:"
    @echo "  just d            - dev"
    @echo "  just t            - test"
    @echo "  just tw           - test-watch"
    @echo "  just l            - lint"
    @echo "  just f            - fmt"
    @echo "  just b            - build"
    @echo "  just c            - clean"
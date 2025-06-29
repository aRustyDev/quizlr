#!/usr/bin/env just --justfile

# Default recipe to display help
default:
    @just --list

# Development
dev: setup compile-scss
    @echo "Starting development server..."
    cd quizlr-web && trunk serve --port 3001

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
build-web: setup-web compile-scss
    @echo "Building web frontend..."
    cd quizlr-web && trunk build

# Compile SCSS to CSS
compile-scss:
    @echo "Compiling SCSS to CSS..."
    @if command -v sass >/dev/null 2>&1; then \
        sass quizlr-web/style/main.scss quizlr-web/style/main.css; \
    else \
        echo "Warning: sass not installed, skipping SCSS compilation"; \
    fi

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
    @if [ -z "${GITHUB_ACTIONS}" ]; then \
        echo "Error: This target is designed to run in GitHub Actions."; \
        echo "For local testing, use 'just docs-open' instead."; \
        exit 1; \
    fi
    @echo "Documentation built successfully. Deploy step will be handled by GitHub Actions."

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
    @echo "Installing git hooks..."
    ./scripts/install-hooks.sh || true
    @echo "Development environment ready!"

# Install Rust toolchain and tools
setup-rust:
    @echo "Setting up Rust toolchain..."
    rustup target add wasm32-unknown-unknown
    @echo "Installing cargo tools..."
    cargo install cargo-watch || true
    cargo install cargo-edit || true
    cargo install cargo-audit || true
    cargo install cargo-tarpaulin || true

# Install web development tools
setup-web:
    @echo "Installing web development tools..."
    cargo install trunk --version 0.21.7 --locked || true
    cargo install wasm-pack || true
    @echo "Installing sass for SCSS compilation..."
    @if command -v brew >/dev/null 2>&1; then \
        brew install sass/sass/sass || true; \
    else \
        cargo install grass || true; \
    fi

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

# Run E2E tests
test-e2e: setup-e2e
    @echo "Running E2E tests..."
    cd e2e && npm test

# Install E2E test dependencies
setup-e2e:
    @echo "Setting up E2E tests..."
    cd e2e && npm install
    cd e2e && npx playwright install

# CI/CD checks (what CI would run)
ci: lint test check build test-e2e build-docs
    @echo "All CI checks passed!"

# Quick CI checks (without E2E tests)
ci-quick: lint test check build
    @echo "Quick CI checks passed!"

# Development shortcuts
d: dev
t: test
tw: test-watch
l: lint
f: fmt
b: build
c: clean

# Watch SCSS files for changes
watch-scss:
    @echo "Watching SCSS files for changes..."
    @if command -v sass >/dev/null 2>&1; then \
        sass --watch quizlr-web/style/main.scss:quizlr-web/style/main.css; \
    else \
        echo "Error: sass not installed, run 'just setup-web' first"; \
        exit 1; \
    fi

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
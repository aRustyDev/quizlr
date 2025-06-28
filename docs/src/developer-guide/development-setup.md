# Development Setup

This guide walks you through setting up a complete development environment for Quizlr. Whether you're contributing code, documentation, or just exploring the codebase, this guide will get you up and running.

## Prerequisites

### Required Software

#### Rust Toolchain
Quizlr requires Rust 1.75.0 or later.

```bash
# Install Rust via rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version

# Update to latest stable
rustup update stable
```

#### WebAssembly Target
Required for building the web application:

```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-pack for building
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Verify wasm-pack
wasm-pack --version
```

#### Node.js and npm
Required for web development tooling:

```bash
# Install Node.js 18+ (using nvm)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18

# Verify installation
node --version
npm --version
```

#### Just (Task Runner)
We use `just` for task automation:

```bash
# macOS
brew install just

# Linux
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin

# Windows (via Scoop)
scoop install just

# Verify installation
just --version
```

### Optional but Recommended

#### mdBook
For building and viewing documentation:

```bash
cargo install mdbook
cargo install mdbook-mermaid  # For diagrams
cargo install mdbook-toc       # For table of contents
```

#### Development Tools
```bash
# Cargo extensions
cargo install cargo-watch      # Auto-rebuild on changes
cargo install cargo-edit       # Add/remove dependencies
cargo install cargo-outdated   # Check for updates
cargo install cargo-audit      # Security audits
cargo install cargo-expand     # Macro expansion
cargo install cargo-criterion  # Benchmarking

# Code quality tools
cargo install clippy          # Linting
cargo install rustfmt         # Formatting
```

#### IDE Setup
We recommend Visual Studio Code with these extensions:
- rust-analyzer: Rust language support
- CodeLLDB: Debugging support
- Even Better TOML: TOML file support
- Error Lens: Inline error display
- GitLens: Git integration

For VS Code, create `.vscode/settings.json`:
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "rust-analyzer.imports.granularity.group": "module",
  "rust-analyzer.imports.prefix": "crate"
}
```

## Getting Started

### 1. Clone the Repository

```bash
# Clone via HTTPS
git clone https://github.com/yourusername/quizlr.git

# Or via SSH
git clone git@github.com:yourusername/quizlr.git

# Enter the project directory
cd quizlr
```

### 2. Initial Setup

```bash
# Install git hooks (optional but recommended)
./scripts/install-hooks.sh

# Install all dependencies
cargo fetch

# Build all workspace members
just build

# Run initial tests to verify setup
just test
```

### 3. Environment Configuration

Create a `.env` file for local development:

```bash
# Copy example environment file
cp .env.example .env

# Edit with your preferred editor
$EDITOR .env
```

Example `.env` contents:
```env
# Development environment
RUST_LOG=debug,quizlr=trace
RUST_BACKTRACE=1

# LLM Provider Keys (optional for basic development)
OPENAI_API_KEY=your_key_here
ANTHROPIC_API_KEY=your_key_here

# Storage Configuration
STORAGE_PATH=./data
STORAGE_ENCRYPTION_KEY=development_only_key

# Web Server
WEB_PORT=8080
WEB_HOST=127.0.0.1
```

### 4. Database Setup (Future)

Currently, Quizlr uses file-based storage. Future versions will support:

```bash
# SQLite (default for development)
touch data/quizlr.db

# PostgreSQL (optional)
docker-compose up -d postgres
just migrate
```

## Development Workflows

### Basic Development Cycle

```bash
# Start development server with auto-reload
just dev

# In another terminal, run tests in watch mode
just test-watch

# Format code before committing
just fmt

# Run linter
just lint
```

### Web Development

```bash
# Build and serve web application
just web

# Build web in release mode
just build-web-release

# Run web tests
just test-web

# Check bundle size
just analyze-bundle
```

### Working with Documentation

```bash
# Serve documentation locally
just docs

# Build documentation
just build-docs

# Check for broken links
just check-docs
```

## Project-Specific Setup

### Core Library Development

When working on `quizlr-core`:

```bash
# Run core tests only
cargo test -p quizlr-core

# Run specific test
cargo test -p quizlr-core quiz::tests::test_quiz_creation

# Run benchmarks
cargo bench -p quizlr-core

# Check feature combinations
cargo hack check --feature-powerset
```

### Web Application Development

When working on `quizlr-web`:

```bash
# Install frontend dependencies
cd quizlr-web
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Run Playwright tests
npm run test:e2e
```

### Mobile Development (Future)

#### iOS Setup
```bash
# Install Xcode and command line tools
xcode-select --install

# Install CocoaPods
sudo gem install cocoapods

# Generate iOS bindings
just build-ios
```

#### Android Setup
```bash
# Install Android Studio and SDK
# Set ANDROID_HOME environment variable

# Install NDK
sdkmanager "ndk-bundle"

# Generate Android bindings
just build-android
```

## Troubleshooting Common Issues

### Rust Compilation Errors

#### Issue: "can't find crate"
```bash
# Clear cargo cache
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build
```

#### Issue: "WASM target not found"
```bash
# Reinstall WASM target
rustup target add wasm32-unknown-unknown --toolchain stable

# Verify installation
rustup target list --installed
```

### WebAssembly Build Issues

#### Issue: "wasm-pack build failed"
```bash
# Clear WASM build cache
rm -rf pkg/ target/wasm32-unknown-unknown/

# Rebuild with verbose output
wasm-pack build --dev -- --verbose

# Check wasm-bindgen version compatibility
cargo tree -p wasm-bindgen
```

#### Issue: "Module not found" in browser
```bash
# Ensure correct build output
ls -la pkg/

# Check generated package.json
cat pkg/package.json

# Rebuild with correct target
wasm-pack build --target web
```

### Development Server Issues

#### Issue: "Port already in use"
```bash
# Find process using port
lsof -i :8080

# Kill process
kill -9 <PID>

# Or use different port
WEB_PORT=8081 just dev
```

#### Issue: "CORS errors in browser"
```bash
# Check server configuration
# Ensure proper headers in development

# Use proxy configuration
# Update vite.config.js or webpack.config.js
```

## Performance Optimization

### Development Build Performance

```bash
# Use mold linker (Linux)
cargo install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"

# Use lld linker (macOS/Windows)
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"

# Enable incremental compilation
export CARGO_INCREMENTAL=1

# Use sccache for caching
cargo install sccache
export RUSTC_WRAPPER=sccache
```

### Profile-Guided Optimization

```bash
# Build with PGO instrumentation
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release

# Run typical workload
./target/release/quizlr-bench

# Build with PGO data
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data" cargo build --release
```

## Debug Configuration

### VS Code Launch Configuration

Create `.vscode/launch.json`:
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug unit tests in library",
      "cargo": {
        "args": ["test", "--no-run", "--lib", "--package=quizlr-core"],
        "filter": {
          "name": "quizlr-core",
          "kind": "lib"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    },
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug web server",
      "cargo": {
        "args": ["build", "--package=quizlr-web"],
        "filter": {
          "name": "quizlr-web",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}",
      "env": {
        "RUST_LOG": "debug"
      }
    }
  ]
}
```

### Logging Configuration

```rust
// Enable detailed logging for debugging
RUST_LOG=trace cargo run

// Filter by module
RUST_LOG=quizlr_core::quiz=debug cargo run

// Use pretty printing
RUST_LOG=debug RUST_LOG_STYLE=pretty cargo run
```

## Continuous Integration Setup

### Pre-commit Hooks

Create `.git/hooks/pre-commit`:
```bash
#!/bin/sh
set -e

# Format code
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings

# Run tests
cargo test --workspace

echo "Pre-commit checks passed!"
```

### GitHub Actions Integration

The project uses GitHub Actions for CI/CD. To test workflows locally:

```bash
# Install act
brew install act  # macOS
# or
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Run workflows locally
act -j test
act -j build
```

## Advanced Setup

### Cross-Compilation

```bash
# Install cross
cargo install cross

# Build for different targets
cross build --target aarch64-unknown-linux-gnu
cross build --target x86_64-pc-windows-gnu
```

### Profiling and Benchmarking

```bash
# Install profiling tools
cargo install flamegraph
cargo install cargo-profiling

# Generate flamegraph
cargo flamegraph --bin quizlr-web

# Run criterion benchmarks
cargo criterion

# Memory profiling
cargo install cargo-instruments  # macOS only
cargo instruments -t Allocations
```

### Security Auditing

```bash
# Install security tools
cargo install cargo-audit
cargo install cargo-deny

# Run security audit
cargo audit

# Check dependencies
cargo deny check
```

## Development Best Practices

### Code Quality Checklist

Before submitting a PR, ensure:

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code is formatted with `rustfmt`
- [ ] No clippy warnings
- [ ] Documentation is updated
- [ ] Benchmarks show no regression
- [ ] Security audit passes

### Performance Guidelines

1. **Use `cargo flamegraph`** to identify bottlenecks
2. **Profile before optimizing** - measure, don't guess
3. **Benchmark critical paths** with Criterion
4. **Monitor WASM size** - aim for < 500KB gzipped
5. **Use `wee_alloc`** for smaller WASM binaries

### Development Tips

1. **Use `cargo watch`** for auto-recompilation:
   ```bash
   cargo watch -x test -x run
   ```

2. **Enable all features during development**:
   ```bash
   cargo build --all-features
   ```

3. **Use feature flags for experimental code**:
   ```toml
   [features]
   experimental = []
   ```

4. **Keep compile times fast**:
   - Use `cargo check` instead of `cargo build`
   - Split large modules
   - Minimize dependencies

## Conclusion

With this development environment set up, you're ready to contribute to Quizlr! Remember to:

- Keep your tools updated
- Run tests before committing
- Follow the project's coding standards
- Ask questions in issues or discussions

For specific development tasks, refer to:
- [Building](./building.md) - Build instructions
- [Testing](./testing.md) - Testing guide
- [Contributing](./contributing.md) - Contribution guidelines
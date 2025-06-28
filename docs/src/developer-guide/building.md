# Building Quizlr

This guide covers all aspects of building Quizlr, from development builds to production releases across different platforms. Quizlr uses a monorepo structure with multiple build targets.

## Quick Start

```bash
# Build everything
just build

# Build and run web application
just web

# Build for release
just release
```

## Build System Overview

Quizlr uses several build tools:
- **Cargo**: Rust's build system for core library
- **wasm-pack**: WebAssembly bundler
- **Vite/Webpack**: Web application bundler
- **Just**: Task runner for orchestration

### Build Targets

```
┌─────────────────────────────────────────┐
│          Build Targets                  │
├─────────────────────────────────────────┤
│ • quizlr-core (rlib)                    │
│ • quizlr-core (cdylib → WASM)           │
│ • quizlr-web (Yew app → WASM)          │
│ • quizlr-cli (binary)                   │
│ • iOS Framework (via UniFFI)            │
│ • Android Library (via JNI)             │
└─────────────────────────────────────────┘
```

## Core Library Builds

### Development Build

```bash
# Debug build with all features
cargo build -p quizlr-core --all-features

# Specific feature set
cargo build -p quizlr-core --features "llm,storage"

# Check without building
cargo check -p quizlr-core
```

### Release Build

```bash
# Optimized release build
cargo build -p quizlr-core --release

# Size-optimized build
cargo build -p quizlr-core --release --profile=min-size
```

### Custom Build Profiles

Add to `Cargo.toml`:
```toml
[profile.min-size]
inherits = "release"
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit
strip = true        # Strip symbols
panic = "abort"     # Smaller panic handler

[profile.max-perf]
inherits = "release"
opt-level = 3       # Maximum optimization
lto = "fat"         # Full LTO
codegen-units = 1   # Better optimization
```

## WebAssembly Builds

### Web Application Build

```bash
# Development build with debug info
wasm-pack build quizlr-web --dev --target web --out-dir pkg

# Production build
wasm-pack build quizlr-web --release --target web --out-dir pkg

# Build with specific features
wasm-pack build quizlr-web --release --target web -- --features "full"
```

### Build Script (`build-web.sh`)

```bash
#!/bin/bash
set -e

echo "Building Quizlr Web..."

# Clean previous builds
rm -rf pkg dist

# Build WASM module
wasm-pack build quizlr-web \
  --release \
  --target web \
  --out-dir pkg \
  --no-typescript

# Optimize WASM binary
wasm-opt pkg/*_bg.wasm \
  -O4 \
  --enable-simd \
  -o pkg/*_bg.wasm

# Bundle with Vite
cd quizlr-web
npm run build

echo "Build complete! Output in dist/"
```

### WASM Optimization

```bash
# Install wasm-opt
npm install -g wasm-opt

# Optimize for size
wasm-opt input.wasm -Oz -o output.wasm

# Optimize for speed
wasm-opt input.wasm -O3 -o output.wasm

# Enable SIMD
wasm-opt input.wasm -O3 --enable-simd -o output.wasm
```

## Platform-Specific Builds

### Linux

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y \
  build-essential \
  pkg-config \
  libssl-dev \
  libsqlite3-dev

# Build with native CPU features
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Static linking
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release
```

### macOS

```bash
# Universal binary (Intel + Apple Silicon)
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create universal binary
lipo -create \
  target/x86_64-apple-darwin/release/quizlr \
  target/aarch64-apple-darwin/release/quizlr \
  -output target/release/quizlr-universal

# Code signing
codesign --sign "Developer ID" --timestamp target/release/quizlr-universal
```

### Windows

```bash
# MSVC toolchain (recommended)
cargo build --release

# GNU toolchain
cargo build --release --target x86_64-pc-windows-gnu

# Generate import library
cargo build --release --target x86_64-pc-windows-msvc
lib /def:quizlr.def /out:quizlr.lib /machine:x64
```

### iOS (Future)

```bash
# Install iOS targets
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios

# Build for iOS
cargo build --release --target aarch64-apple-ios

# Generate XCFramework
./scripts/build-ios-framework.sh

# Output structure
# Quizlr.xcframework/
# ├── ios-arm64/
# │   └── Quizlr.framework
# ├── ios-arm64-simulator/
# │   └── Quizlr.framework
# └── Info.plist
```

### Android (Future)

```bash
# Install Android targets
rustup target add \
  aarch64-linux-android \
  armv7-linux-androideabi \
  i686-linux-android \
  x86_64-linux-android

# Build with cargo-ndk
cargo install cargo-ndk
cargo ndk -t arm64-v8a -o ./jniLibs build --release

# Generate AAR
./scripts/build-android-aar.sh
```

## Build Configuration

### Feature Flags

```toml
[features]
default = ["llm", "storage", "auth"]
full = ["default", "experimental", "telemetry"]

# Core features
llm = ["dep:reqwest", "dep:async-openai"]
storage = ["dep:aws-sdk-s3", "dep:sqlx"]
auth = ["dep:oauth2", "dep:jsonwebtoken"]

# Optional features
experimental = []
telemetry = ["dep:opentelemetry", "dep:tracing-opentelemetry"]
offline = ["storage", "dep:sled"]

# Platform-specific
mobile = ["dep:uniffi"]
wasm = ["dep:wasm-bindgen", "dep:web-sys"]
```

### Conditional Compilation

```rust
// Platform-specific code
#[cfg(target_arch = "wasm32")]
mod wasm_impl;

#[cfg(not(target_arch = "wasm32"))]
mod native_impl;

// Feature-gated code
#[cfg(feature = "llm")]
pub mod llm_provider;

// Debug-only code
#[cfg(debug_assertions)]
println!("Debug mode: {}", value);
```

## Build Optimization

### Binary Size Optimization

```toml
# Cargo.toml
[profile.release]
opt-level = "z"          # Optimize for size
lto = true               # Link-time optimization
codegen-units = 1        # Better optimization
strip = true             # Strip debug symbols
panic = "abort"          # Smaller panic handler

[dependencies]
# Use lighter alternatives
uuid = { version = "1.0", default-features = false, features = ["v4"] }
chrono = { version = "0.4", default-features = false, features = ["clock"] }
```

### Compile Time Optimization

```toml
# .cargo/config.toml
[build]
# Parallel compilation
jobs = 8

[target.x86_64-unknown-linux-gnu]
# Use mold linker (Linux)
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[target.x86_64-apple-darwin]
# Use lld linker (macOS)
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

### Build Caching

```bash
# Install sccache
cargo install sccache

# Configure
export RUSTC_WRAPPER=sccache
export SCCACHE_CACHE_SIZE="10G"

# Use with Docker
docker build \
  --build-arg RUSTC_WRAPPER=sccache \
  --mount=type=cache,target=/root/.cache/sccache \
  .
```

## Continuous Integration Builds

### GitHub Actions

```yaml
name: Build

on: [push, pull_request]

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]
    
    runs-on: ${{ matrix.os }}
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@master
      with:
        toolchain: ${{ matrix.rust }}
        targets: wasm32-unknown-unknown
    
    - name: Cache cargo
      uses: Swatinem/rust-cache@v2
    
    - name: Build
      run: |
        cargo build --workspace
        cargo build --workspace --release
    
    - name: Build WASM
      run: |
        cargo install wasm-pack
        wasm-pack build quizlr-web --release
```

### Docker Builds

```dockerfile
# Multi-stage build for minimal image size
FROM rust:1.75 as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build dependencies separately for caching
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY quizlr-core/Cargo.toml ./quizlr-core/
COPY quizlr-web/Cargo.toml ./quizlr-web/
RUN mkdir -p quizlr-core/src quizlr-web/src && \
    echo "fn main() {}" > quizlr-core/src/lib.rs && \
    echo "fn main() {}" > quizlr-web/src/main.rs && \
    cargo build --release && \
    rm -rf quizlr-core/src quizlr-web/src

# Build application
COPY . .
RUN cargo build --release && \
    wasm-pack build quizlr-web --release --target web

# Runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/quizlr /usr/local/bin/
COPY --from=builder /app/quizlr-web/pkg /usr/share/quizlr/web

EXPOSE 8080
CMD ["quizlr"]
```

## Build Troubleshooting

### Common Build Issues

#### Issue: "error: linker `cc` not found"
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# macOS
xcode-select --install

# Windows
# Install Visual Studio Build Tools
```

#### Issue: "error[E0463]: can't find crate"
```bash
# Update dependencies
cargo update

# Clean build
cargo clean
cargo build
```

#### Issue: "wasm-pack build failed"
```bash
# Check Node.js version (need 16+)
node --version

# Clear wasm-pack cache
rm -rf ~/.cache/wasm-pack

# Try with verbose output
wasm-pack build --verbose
```

### Platform-Specific Issues

#### macOS: "symbol not found" errors
```bash
# Update macOS SDK
xcode-select --install
sudo xcode-select --switch /Applications/Xcode.app

# Use correct deployment target
export MACOSX_DEPLOYMENT_TARGET=10.15
```

#### Windows: Long path issues
```powershell
# Enable long paths in Windows
New-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" `
  -Name "LongPathsEnabled" -Value 1 -PropertyType DWORD -Force
```

#### Linux: OpenSSL issues
```bash
# Install OpenSSL development files
sudo apt-get install libssl-dev  # Debian/Ubuntu
sudo yum install openssl-devel   # RHEL/CentOS
sudo pacman -S openssl           # Arch
```

## Build Artifacts

### Output Structure

```
target/
├── debug/                    # Debug builds
│   ├── deps/                 # Dependencies
│   ├── build/                # Build scripts
│   ├── examples/             # Example binaries
│   └── quizlr               # Debug binary
├── release/                  # Release builds
│   └── quizlr               # Release binary
├── wasm32-unknown-unknown/   # WASM builds
│   └── release/
│       └── quizlr_web.wasm
└── doc/                      # Generated docs
    └── quizlr_core/
        └── index.html

quizlr-web/
├── pkg/                      # WASM package
│   ├── quizlr_web_bg.wasm
│   ├── quizlr_web.js
│   └── package.json
└── dist/                     # Web distribution
    ├── index.html
    ├── assets/
    └── quizlr_web_bg.wasm
```

### Build Verification

```bash
# Verify binary
file target/release/quizlr
ldd target/release/quizlr  # Linux
otool -L target/release/quizlr  # macOS

# Verify WASM module
wasm-objdump -x pkg/quizlr_web_bg.wasm
wasm-dis pkg/quizlr_web_bg.wasm | head -20

# Check binary size
du -h target/release/quizlr
du -h pkg/quizlr_web_bg.wasm
```

## Release Process

### Version Bumping

```bash
# Update workspace version
cargo workspaces version --all patch

# Update specific package
cargo workspaces version -p quizlr-core minor

# Create git tag
git tag -a v0.1.2 -m "Release version 0.1.2"
git push origin v0.1.2
```

### Release Checklist

1. **Update version numbers**
   ```toml
   # Cargo.toml
   [workspace.package]
   version = "0.1.2"
   ```

2. **Update CHANGELOG.md**
   ```markdown
   ## [0.1.2] - 2024-01-15
   ### Added
   - New feature X
   ### Fixed
   - Bug Y
   ```

3. **Run full test suite**
   ```bash
   just test-all
   ```

4. **Build all targets**
   ```bash
   just release-all
   ```

5. **Create release artifacts**
   ```bash
   ./scripts/create-release.sh
   ```

### Distribution

#### Cargo Registry
```bash
# Publish to crates.io
cargo publish -p quizlr-core
cargo publish -p quizlr-web
```

#### NPM Registry
```bash
cd quizlr-web/pkg
npm publish
```

#### GitHub Releases
```bash
# Using GitHub CLI
gh release create v0.1.2 \
  --title "Release v0.1.2" \
  --notes-file RELEASE_NOTES.md \
  target/release/quizlr-* \
  dist/quizlr-web-*.tar.gz
```

## Conclusion

Building Quizlr involves multiple build systems and targets. Key points:

- Use `just` commands for common build tasks
- Optimize builds based on target platform
- Enable appropriate features for your use case
- Test builds across platforms before release
- Monitor binary sizes, especially for WASM

For development workflows, see [Development Setup](./development-setup.md).
For testing builds, see [Testing](./testing.md).
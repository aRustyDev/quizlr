#!/bin/bash
set -e

echo "🛠️  Setting up Quizlr development environment..."
echo "=============================================="

# Check for required tools
check_tool() {
    local tool=$1
    local install_cmd=$2
    
    if ! command -v "$tool" > /dev/null 2>&1; then
        echo "❌ $tool not found!"
        echo "   Install with: $install_cmd"
        return 1
    else
        echo "✅ $tool found"
        return 0
    fi
}

echo -e "\n📋 Checking required tools..."
MISSING_TOOLS=0

check_tool "rustc" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" || MISSING_TOOLS=$((MISSING_TOOLS + 1))
check_tool "cargo" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" || MISSING_TOOLS=$((MISSING_TOOLS + 1))
check_tool "git" "https://git-scm.com/downloads" || MISSING_TOOLS=$((MISSING_TOOLS + 1))
check_tool "npm" "https://nodejs.org/" || MISSING_TOOLS=$((MISSING_TOOLS + 1))

if [ $MISSING_TOOLS -gt 0 ]; then
    echo -e "\n❌ Please install missing tools before continuing"
    exit 1
fi

# Install Rust toolchain components
echo -e "\n🦀 Setting up Rust toolchain..."
rustup target add wasm32-unknown-unknown
rustup component add rustfmt clippy

# Install cargo tools
echo -e "\n📦 Installing cargo tools..."
cargo_install() {
    local tool=$1
    local version=$2
    
    if cargo install --list | grep -q "^$tool "; then
        echo "   ✅ $tool already installed"
    else
        echo "   📥 Installing $tool..."
        if [ -n "$version" ]; then
            cargo install "$tool" --version "$version" --locked
        else
            cargo install "$tool"
        fi
    fi
}

cargo_install "cargo-watch"
cargo_install "cargo-edit"
cargo_install "cargo-audit"
cargo_install "cargo-outdated"
cargo_install "cargo-license"
cargo_install "cargo-tarpaulin"
cargo_install "trunk" "0.21.7"
cargo_install "wasm-pack"
cargo_install "mdbook"
cargo_install "just"

# Set up git hooks
echo -e "\n🔗 Installing git hooks..."
if [ -f "scripts/install-hooks.sh" ]; then
    ./scripts/install-hooks.sh
else
    echo "   ⚠️  Git hooks script not found"
fi

# Install E2E test dependencies
echo -e "\n🧪 Setting up E2E tests..."
if [ -d "e2e" ]; then
    cd e2e
    npm install
    npx playwright install
    cd ..
else
    echo "   ⚠️  E2E directory not found"
fi

# Create .env file if it doesn't exist
if [ ! -f ".env" ]; then
    echo -e "\n📝 Creating .env file..."
    cat > .env << 'EOF'
# Quizlr Environment Variables

# API Keys (optional)
# OPENAI_API_KEY=
# ANTHROPIC_API_KEY=
# GOOGLE_API_KEY=
# COHERE_API_KEY=

# GitHub Integration (optional)
# GITHUB_TOKEN=

# Development Settings
RUST_LOG=debug
RUST_BACKTRACE=1
EOF
    echo "   ✅ .env file created (add your API keys)"
else
    echo -e "\n✅ .env file already exists"
fi

# Build the project
echo -e "\n🔨 Building project..."
cargo build --workspace

# Run tests
echo -e "\n🧪 Running tests..."
cargo test --workspace --quiet

# Build documentation
echo -e "\n📚 Building documentation..."
cd docs && mdbook build && cd ..

echo -e "\n✨ Development environment setup complete!"
echo ""
echo "Quick start commands:"
echo "  just dev       - Start development server"
echo "  just test      - Run tests"
echo "  just lint      - Run linters"
echo "  just docs      - Build documentation"
echo "  just help      - Show all commands"
echo ""
echo "Happy coding! 🚀"
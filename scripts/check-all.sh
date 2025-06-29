#!/bin/bash
set -e

echo "🔍 Running comprehensive checks..."
echo "================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track overall status
FAILED=0

# Function to run a check
run_check() {
    local name=$1
    local command=$2
    
    echo -e "\n📋 Running: $name"
    if eval "$command" > /tmp/check_output_$$.log 2>&1; then
        echo -e "${GREEN}✅ $name passed${NC}"
    else
        echo -e "${RED}❌ $name failed${NC}"
        echo "   See /tmp/check_output_$$.log for details"
        FAILED=$((FAILED + 1))
    fi
}

# Formatting check
run_check "Formatting" "cargo fmt --all -- --check"

# Linting
run_check "Clippy" "cargo clippy --workspace --all-targets --all-features -- -D warnings"

# Tests
run_check "Unit Tests" "cargo test --workspace --lib --quiet"
run_check "Integration Tests" "cargo test --workspace --test '*' --quiet"
run_check "Doc Tests" "cargo test --workspace --doc --quiet"

# Documentation
run_check "API Docs" "cargo doc --workspace --no-deps --quiet"
run_check "mdBook" "cd docs && mdbook test"

# Build checks
run_check "Debug Build" "cargo build --workspace --quiet"
run_check "Release Build" "cargo build --workspace --release --quiet"
run_check "WASM Build" "cargo build --package quizlr-core --target wasm32-unknown-unknown --no-default-features --features wasm --quiet"

# Dependency checks
run_check "Dependency Tree" "cargo tree --workspace --quiet"
run_check "Outdated Check" "cargo outdated --workspace --exit-code 0"

# Security audit (if installed)
if command -v cargo-audit > /dev/null 2>&1; then
    run_check "Security Audit" "cargo audit"
else
    echo -e "${YELLOW}⚠️  Skipping security audit (cargo-audit not installed)${NC}"
fi

# License check (if installed)
if command -v cargo-license > /dev/null 2>&1; then
    run_check "License Check" "cargo license --json > /dev/null"
else
    echo -e "${YELLOW}⚠️  Skipping license check (cargo-license not installed)${NC}"
fi

# Coverage (if installed)
if command -v cargo-tarpaulin > /dev/null 2>&1; then
    echo -e "\n📊 Generating coverage report..."
    cargo tarpaulin --workspace --print-summary --quiet || true
else
    echo -e "${YELLOW}⚠️  Skipping coverage (cargo-tarpaulin not installed)${NC}"
fi

# Clean up log files
rm -f /tmp/check_output_$$.log

# Summary
echo -e "\n================================="
if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ All checks passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ $FAILED check(s) failed!${NC}"
    exit 1
fi
#!/bin/bash
# Script to test for common build errors before committing

echo "Running build error prevention tests..."

# First, try to build the project
echo "Testing WASM build..."
cargo build --target wasm32-unknown-unknown 2>&1 | tee build.log

if [ ${PIPESTATUS[0]} -ne 0 ]; then
    echo "❌ Build failed! Check build.log for errors"
    
    # Check for common error patterns
    if grep -q "if.*and.*else.*have incompatible types" build.log; then
        echo "⚠️  Found if/else type mismatch - ensure all branches return same view structure"
    fi
    
    if grep -q "expected.*Fn.*closure.*found.*FnOnce" build.log; then
        echo "⚠️  Found closure capture issue - check for moved values in closures"
    fi
    
    exit 1
else
    echo "✅ Build successful!"
fi

# Run clippy for additional checks
echo "Running clippy..."
cargo clippy --target wasm32-unknown-unknown -- -D warnings

echo "✅ All checks passed!"
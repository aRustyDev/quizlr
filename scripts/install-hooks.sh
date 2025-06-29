#!/bin/bash
set -e

echo "🔧 Installing git hooks..."

# Create scripts directory if it doesn't exist
mkdir -p scripts

# Check if .git directory exists
if [ ! -d ".git" ]; then
    echo "❌ Error: Not in a git repository"
    exit 1
fi

# Set git hooks path
git config core.hooksPath .githooks

echo "✅ Git hooks installed successfully!"
echo "ℹ️  Hooks location: .githooks/"
echo ""
echo "Available hooks:"
echo "  - pre-commit: Runs formatting, linting, and tests"
echo "  - pre-push: Runs full test suite, security audit, and builds"
echo ""
echo "To disable hooks temporarily, use:"
echo "  git commit --no-verify"
echo "  git push --no-verify"
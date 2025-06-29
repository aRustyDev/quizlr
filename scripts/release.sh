#!/bin/bash
set -e

echo "🚀 Quizlr Release Script"
echo "======================="

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "❌ Error: Releases must be created from main branch"
    echo "   Current branch: $CURRENT_BRANCH"
    exit 1
fi

# Check for uncommitted changes
if ! git diff --quiet || ! git diff --cached --quiet; then
    echo "❌ Error: Uncommitted changes detected"
    echo "   Please commit or stash your changes"
    exit 1
fi

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2)
echo "Current version: $CURRENT_VERSION"

# Prompt for new version
echo -e "\nEnter new version (current: $CURRENT_VERSION):"
read -r NEW_VERSION

if [ -z "$NEW_VERSION" ]; then
    echo "❌ Error: Version cannot be empty"
    exit 1
fi

# Validate version format
if ! echo "$NEW_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?$'; then
    echo "❌ Error: Invalid version format"
    echo "   Expected: X.Y.Z or X.Y.Z-suffix"
    exit 1
fi

echo -e "\nRelease version $NEW_VERSION? (y/N)"
read -r CONFIRM

if [ "$CONFIRM" != "y" ] && [ "$CONFIRM" != "Y" ]; then
    echo "❌ Release cancelled"
    exit 1
fi

# Update version in all Cargo.toml files
echo -e "\n📝 Updating version to $NEW_VERSION..."
find . -name "Cargo.toml" -not -path "./target/*" -not -path "./e2e/*" -exec sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" {} \;
find . -name "Cargo.toml.bak" -delete

# Update Cargo.lock
cargo update --workspace

# Run all checks
echo -e "\n🔍 Running pre-release checks..."
./scripts/check-all.sh

# Update CHANGELOG.md
echo -e "\n📋 Updating CHANGELOG.md..."
if [ -f "CHANGELOG.md" ]; then
    # Add new version header
    sed -i.bak "s/## \[Unreleased\]/## [Unreleased]\n\n## [$NEW_VERSION] - $(date +%Y-%m-%d)/" CHANGELOG.md
    rm CHANGELOG.md.bak
    echo "   ✅ CHANGELOG.md updated"
else
    echo "   ⚠️  CHANGELOG.md not found"
fi

# Commit version changes
echo -e "\n💾 Committing version changes..."
git add -A
git commit -m "chore: release v$NEW_VERSION

🚀 Release version $NEW_VERSION

🤖 Generated with release script

Co-Authored-By: Release Bot <noreply@quizlr.app>"

# Create git tag
echo -e "\n🏷️  Creating git tag..."
git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"

# Build release artifacts
echo -e "\n🔨 Building release artifacts..."
cargo build --workspace --release

# Build web app
echo -e "\n🌐 Building web app..."
cd quizlr-web
trunk build --release
cd ..

# Create release archive
echo -e "\n📦 Creating release archive..."
mkdir -p releases
RELEASE_DIR="releases/quizlr-v$NEW_VERSION"
mkdir -p "$RELEASE_DIR"

# Copy artifacts
cp target/release/quizlr "$RELEASE_DIR/" 2>/dev/null || true
cp -r quizlr-web/dist "$RELEASE_DIR/web"
cp README.md "$RELEASE_DIR/"
cp LICENSE* "$RELEASE_DIR/" 2>/dev/null || true
cp CHANGELOG.md "$RELEASE_DIR/" 2>/dev/null || true

# Create tarball
tar -czf "releases/quizlr-v$NEW_VERSION.tar.gz" -C releases "quizlr-v$NEW_VERSION"
rm -rf "$RELEASE_DIR"

echo -e "\n✅ Release preparation complete!"
echo ""
echo "Next steps:"
echo "1. Review the changes: git log --oneline -5"
echo "2. Push to remote: git push origin main --tags"
echo "3. Create GitHub release from tag v$NEW_VERSION"
echo "4. Upload release archive: releases/quizlr-v$NEW_VERSION.tar.gz"
echo "5. Publish to crates.io: cargo publish -p quizlr-core"
echo ""
echo "To undo this release (before pushing):"
echo "  git reset --hard HEAD~1"
echo "  git tag -d v$NEW_VERSION"
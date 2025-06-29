#!/bin/bash
set -e

echo "📊 Quizlr Project Health Monitor"
echo "================================"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Get project stats
echo -e "\n📈 Project Statistics:"

# Line count
RUST_LINES=$(find . -name "*.rs" -not -path "./target/*" -not -path "./e2e/*" | xargs wc -l | tail -1 | awk '{print $1}')
echo "   Rust code lines: $RUST_LINES"

# File count
RUST_FILES=$(find . -name "*.rs" -not -path "./target/*" -not -path "./e2e/*" | wc -l)
echo "   Rust files: $RUST_FILES"

# Test count
TEST_COUNT=$(grep -r "#\[test\]" --include="*.rs" . 2>/dev/null | wc -l)
echo "   Test functions: $TEST_COUNT"

# Dependencies
echo -e "\n📦 Dependencies:"
DEPS=$(cargo tree --workspace --depth 1 2>/dev/null | grep -c "├──\|└──" || echo "0")
echo "   Direct dependencies: $DEPS"

# Check for outdated dependencies
echo -e "\n🔄 Outdated Dependencies:"
if command -v cargo-outdated > /dev/null 2>&1; then
    OUTDATED=$(cargo outdated --workspace 2>/dev/null | grep -c "^[[:alnum:]]" || echo "0")
    if [ "$OUTDATED" -gt 0 ]; then
        echo -e "   ${YELLOW}$OUTDATED packages have updates available${NC}"
    else
        echo -e "   ${GREEN}All dependencies up to date${NC}"
    fi
else
    echo "   ⚠️  cargo-outdated not installed"
fi

# Security audit
echo -e "\n🔒 Security Audit:"
if command -v cargo-audit > /dev/null 2>&1; then
    if cargo audit --quiet 2>/dev/null; then
        echo -e "   ${GREEN}No known vulnerabilities${NC}"
    else
        VULNS=$(cargo audit 2>/dev/null | grep -c "^│" || echo "unknown")
        echo -e "   ${RED}$VULNS vulnerabilities found${NC}"
    fi
else
    echo "   ⚠️  cargo-audit not installed"
fi

# Code coverage
echo -e "\n📊 Code Coverage:"
if command -v cargo-tarpaulin > /dev/null 2>&1; then
    COVERAGE=$(cargo tarpaulin --workspace --print-summary 2>/dev/null | grep "Coverage" | awk '{print $2}' || echo "0%")
    COVERAGE_NUM=${COVERAGE%\%}
    
    if (( $(echo "$COVERAGE_NUM >= 80" | bc -l 2>/dev/null || echo 0) )); then
        echo -e "   ${GREEN}$COVERAGE${NC}"
    elif (( $(echo "$COVERAGE_NUM >= 60" | bc -l 2>/dev/null || echo 0) )); then
        echo -e "   ${YELLOW}$COVERAGE${NC}"
    else
        echo -e "   ${RED}$COVERAGE${NC}"
    fi
else
    echo "   ⚠️  cargo-tarpaulin not installed"
fi

# Build size
echo -e "\n💾 Build Artifacts:"
if [ -f "target/release/quizlr" ]; then
    SIZE=$(ls -lh target/release/quizlr 2>/dev/null | awk '{print $5}')
    echo "   Release binary: $SIZE"
fi

if [ -d "quizlr-web/dist" ]; then
    WASM_SIZE=$(find quizlr-web/dist -name "*.wasm" -exec ls -lh {} \; 2>/dev/null | awk '{total += $5} END {print total}' || echo "0")
    if [ -n "$WASM_SIZE" ] && [ "$WASM_SIZE" != "0" ]; then
        echo "   WASM size: ${WASM_SIZE}B"
    fi
fi

# Documentation
echo -e "\n📚 Documentation:"
if [ -d "docs/book" ]; then
    DOC_PAGES=$(find docs/book -name "*.html" 2>/dev/null | wc -l)
    echo "   Documentation pages: $DOC_PAGES"
else
    echo "   Documentation not built"
fi

# Git statistics
echo -e "\n🔀 Git Statistics:"
COMMITS=$(git rev-list --count HEAD 2>/dev/null || echo "0")
CONTRIBUTORS=$(git shortlog -sn 2>/dev/null | wc -l || echo "0")
BRANCHES=$(git branch -r 2>/dev/null | wc -l || echo "0")

echo "   Total commits: $COMMITS"
echo "   Contributors: $CONTRIBUTORS"
echo "   Remote branches: $BRANCHES"

# Recent activity
echo -e "\n📅 Recent Activity:"
echo "   Last commit: $(git log -1 --format='%cr' 2>/dev/null || echo 'unknown')"
RECENT_COMMITS=$(git log --since="1 week ago" --oneline 2>/dev/null | wc -l || echo "0")
echo "   Commits in last week: $RECENT_COMMITS"

# Health score calculation
echo -e "\n🏥 Overall Health Score:"
SCORE=100

# Deduct points for issues
[ "$OUTDATED" -gt 0 ] && SCORE=$((SCORE - 5))
[ "$OUTDATED" -gt 5 ] && SCORE=$((SCORE - 10))

if [ -n "$VULNS" ] && [ "$VULNS" != "0" ] && [ "$VULNS" != "unknown" ]; then
    SCORE=$((SCORE - 20))
fi

if [ -n "$COVERAGE_NUM" ]; then
    if (( $(echo "$COVERAGE_NUM < 60" | bc -l 2>/dev/null || echo 0) )); then
        SCORE=$((SCORE - 10))
    elif (( $(echo "$COVERAGE_NUM < 80" | bc -l 2>/dev/null || echo 0) )); then
        SCORE=$((SCORE - 5))
    fi
fi

# Display health score
if [ $SCORE -ge 90 ]; then
    echo -e "   ${GREEN}Health: $SCORE/100 - Excellent!${NC}"
elif [ $SCORE -ge 70 ]; then
    echo -e "   ${YELLOW}Health: $SCORE/100 - Good${NC}"
else
    echo -e "   ${RED}Health: $SCORE/100 - Needs attention${NC}"
fi

echo -e "\n✨ Monitoring complete!"
# Claude Development Guidelines for Quizlr

This document contains critical guidelines and requirements for Claude when working on the Quizlr project.

## IMPORTANT!!

- Ask questions whenever you are unsure about something
- Ask for clarification whenever something is ambiguous or unclear
- Ask follow up questions when an answer was not helpful or did not fully address your question
- Repeat your understanding of my response back to me to ensure clarity and accuracy

## Using the Justfile

The Justfile contains all development automation commands. When working on this project:

1. **Always use Just commands** instead of raw cargo/npm commands
2. **Test your changes** with `just test` before any commit
3. **Check code quality** with `just lint` and `just fmt`
4. **Build documentation** with `just docs` after API changes
5. **Run full CI locally** with `just ci` before pushing

Common commands:
- `just dev` - Start development server
- `just test` - Run all tests
- `just lint` - Run clippy and format check
- `just build` - Build all targets
- `just docs-serve` - Serve documentation locally

## STRICT REQUIREMENTS

### 1. Test Driven Development (TDD)

**REQUIREMENT**: Follow strict TDD practices with RED-GREEN-REFACTOR pattern:

1. **RED**: Write a failing test FIRST
2. **GREEN**: Write minimal code to make the test pass
3. **REFACTOR**: Improve the code while keeping tests green

**NEVER** write implementation code before writing tests.

### 2. 100% Test Passing Rate

**REQUIREMENT**: ALL tests must pass 100% before any commit:
- Cargo unit tests: `cargo test --workspace`
- Integration tests: `cargo test --workspace --test '*'`
- End-to-end tests: `just test-e2e`

**NO EXCEPTIONS** - If a test fails, fix it before proceeding.

### 3. Documentation Updates

**REQUIREMENT**: Update documentation to reflect current state BEFORE version bumps:
- Documentation must be a **snapshot** of the current code
- Update function names, descriptions, and examples to match implementation
- Do NOT include historical notes like "changed from X to Y"
- Run `just docs` and `just docs-test` to verify

### 4. Conventional Commits

**REQUIREMENT**: Every commit must follow conventional commit standards:
```
<type>(<scope>): <subject>

<body>

<footer>
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`

Example:
```
feat(quiz): add time-based scoring algorithm

Implements adaptive scoring that considers response time
in addition to correctness.

Closes #123
```

### 5. Version Management

**REQUIREMENT**: Keep ALL Cargo.toml versions synchronized:
- Workspace version in root Cargo.toml
- All member crates must use `version.workspace = true`
- Update version before creating git tags

### 6. Patch Version Updates

**REQUIREMENT**: Update patch version on EVERY fully passing commit:
- New test added + passing = new patch version
- Bug fix + tests passing = new patch version
- Documentation update + passing = new patch version

Example: 0.1.1 → 0.1.2

### 7. Minor Version Updates

**REQUIREMENT**: Update minor version on completion of atomic features:
- New question type implemented = minor version
- New API endpoint added = minor version
- New UI component completed = minor version

Example: 0.1.x → 0.2.0

### 8. Pre-commit Validation

**REQUIREMENT**: 100% pre-commit hook passing rate:
- Run `./scripts/install-hooks.sh` to install hooks
- NEVER use `--no-verify` flag
- Fix all issues before committing

Pre-commit checks:
1. Code formatting
2. Clippy warnings
3. Test execution
4. TODO/FIXME detection

### 9. Issue Tracking
**REQUIREMENT**: Create GitHub issues for EVERY problem:

Common Commands:
- `git config remote.origin.url | cat` to get the remote URL for use with `gh` commands
- `git config user.name | cat` to get the person to assign things with the `gh` commands

1. **Create Issue**: Document the problem clearly
  ```markdown
  Title: Build fails with "tokio feature mismatch"
  Body:
  - Steps to reproduce
  - Expected behavior
  - Actual behavior
  - Environment details (Language version, OS, etc.)
  - Latest related commit hash
  ```

2. **Update Issue**: Add investigation notes and potential solutions as comments on the issue, update labels as needed
  ```markdown
  Investigation:
   - Found tokio is using "net" feature in WASM build
   - WASM doesn't support networking features

  Possible-Solution:
   - Add conditional compilation for features
  ```

3. **Close Issue**: Reference issue in commit message, add a comment with the SOLUTION & LESSONS LEARNED from the issue, and close the issue
  ```markdown
  fix(build): conditionally compile tokio features for WASM

   Adds feature flags to separate native and WASM builds,
   preventing tokio networking features from being included
   in WASM targets.

  Fixes #42
   ```

## Development Workflow

1. Pick a task from TODO.md or create an issue
2. Write failing tests (RED)
3. Implement minimal solution (GREEN)
4. Refactor if needed (REFACTOR)
5. Update documentation
6. Run `just ci` to verify everything
7. Update patch/minor version as appropriate
8. Commit with conventional commit message
9. Reference and close the issue

## Debugging Process

When encountering errors:

1. **Document**: Create an issue immediately
2. **Reproduce**: Create minimal test case
3. **Research**: Check documentation, similar issues
4. **Experiment**: Try solutions in isolated environment
5. **Test**: Verify fix doesn't break other functionality
6. **Document**: Update issue with solution
7. **Implement**: Apply fix with proper tests
8. **Close**: Reference issue in commit

## Commands Quick Reference

```bash
# Development
just dev          # Start dev server
just test         # Run tests
just test-watch   # Watch mode
just lint         # Lint code
just fmt          # Format code

# Building
just build        # Build all
just build-web    # Build web only
just build-core   # Build core only

# Documentation
just docs         # Build docs
just docs-serve   # Serve docs
just docs-test    # Test docs

# E2E Testing
just setup-e2e    # Install E2E deps
just test-e2e     # Run E2E tests

# Utilities
just clean        # Clean artifacts
just check-deps   # Check outdated
just ci           # Run full CI
```

## Remember

- **Quality over speed** - Better to do it right than do it twice
- **Tests are documentation** - Write clear, descriptive tests
- **Small commits** - Each commit should be atomic and meaningful
- **Communication** - Update issues with progress and blockers

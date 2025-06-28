# Quizlr Development Challenges

This document tracks challenges encountered during the development of Quizlr and their solutions.

## Format

Each challenge should be documented with:
- **Date**: When the challenge was encountered
- **Challenge**: Description of the problem
- **Context**: What was being worked on when this arose
- **Solution**: How the challenge was resolved
- **Status**: Open/Resolved

---

## Challenges

### 2025-06-28: WASM-Compatible Dependencies

**Challenge**: Ensuring all dependencies are WASM-compatible for browser deployment

**Context**: Setting up the initial project structure with dependencies that need to work in both native and WASM environments

**Solution**: 
- Using `getrandom` with "js" feature for WASM compatibility
- Selecting `reqwest` with `rustls-tls` instead of native TLS
- Configuring target-specific dependencies for web APIs

**Status**: Resolved

---

### 2025-06-28: Cross-Platform Architecture Design

**Challenge**: Designing a core library that can be used from Rust/WASM, Swift, and Kotlin

**Context**: Initial architecture planning for cross-platform support

**Solution**: 
- Using a library crate with both `cdylib` and `rlib` types
- Planning FFI layer for future mobile bindings
- Keeping platform-specific code isolated

**Status**: In Progress

---

### 2025-06-28: Rust Crate Version Compatibility

**Challenge**: Multiple dependency version issues when setting up the project
- octocrab 0.45 doesn't exist (latest is 0.44)
- genai features like "anthropic" don't exist in version 0.3
- leptos_meta doesn't have "csr" or "nightly" features
- d3-rs crate doesn't exist
- indexed_db_futures version compatibility

**Context**: Initial project setup with multiple external dependencies

**Solution**: 
- Downgraded octocrab to 0.44
- Removed specific features from genai
- Removed features from Leptos dependencies
- Commented out d3-rs, will use JS interop instead
- Adjusted indexed_db_futures version

**Status**: Resolved

---

### 2025-06-28: Test-Driven Bug Discovery

**Challenge**: Multiple scoring calculation bugs found when writing tests
- Empty sessions returned incorrect scores in adaptive strategy
- Difficulty weighted scoring didn't account for all questions
- Time score calculation gave perfect score for no responses

**Context**: Writing comprehensive test suite for existing quiz engine

**Solution**: 
- Fixed adaptive scoring to return 0 for time/consistency when no responses
- Updated difficulty scoring to calculate max_possible from all questions
- Added explicit checks for empty response scenarios

**Status**: Resolved

---
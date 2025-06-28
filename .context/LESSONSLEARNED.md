# Quizlr Development Lessons Learned

This document captures key insights and lessons learned during the development of Quizlr.

## Format

Each lesson should include:
- **Date**: When the lesson was learned
- **Lesson**: The key insight or principle discovered
- **Context**: The problem/solution that led to this lesson
- **Application**: How this lesson can be applied in the future

---

## Lessons Learned

### 2025-06-28: Workspace Dependencies Simplify Multi-Crate Projects

**Lesson**: Using Cargo workspace dependencies significantly reduces version conflicts and maintenance overhead

**Context**: Setting up a multi-crate Rust project with shared dependencies between core library and web frontend

**Application**: 
- Define common dependencies at the workspace level
- Use `workspace = true` in member crates
- This ensures consistent versions across all crates

---

### 2025-06-28: Target-Specific Dependencies for WASM

**Lesson**: Use `[target.'cfg(target_arch = "wasm32")'.dependencies]` to conditionally include web-specific dependencies

**Context**: Building a library that works in both native and WASM environments

**Application**:
- Keep web-sys and js-sys dependencies only for WASM targets
- Use feature flags for optional platform-specific functionality
- This reduces binary size and compilation time for non-WASM builds

---

### 2025-06-28: Library + Binary Pattern for Reusability

**Lesson**: Separating core logic into a library crate enables maximum code reuse across different platforms

**Context**: Designing architecture for future iOS/Android support while building a web app

**Application**:
- Core business logic goes in the library crate
- Platform-specific code (UI, platform APIs) goes in binary crates
- Use trait abstractions for platform-specific implementations

---
# Project Structure

This guide provides a detailed overview of Quizlr's project structure, explaining the purpose of each directory and key files. Understanding this structure is essential for contributing to the project effectively.

## Repository Overview

```
quizlr/
├── Cargo.toml              # Workspace configuration
├── Cargo.lock              # Dependency lock file
├── LICENSE                 # Dual MIT/Apache-2.0 license
├── README.md               # Project introduction
├── CHANGELOG.md            # Version history
├── justfile                # Task runner configuration
├── build-web.sh            # Web build script
├── .gitignore              # Git ignore patterns
├── .github/                # GitHub-specific files
│   ├── workflows/          # CI/CD workflows
│   ├── ISSUE_TEMPLATE/     # Issue templates
│   └── pull_request_template.md
├── quizlr-core/            # Core library (Rust)
├── quizlr-web/             # Web application
├── quizlr-ios/             # iOS application (future)
├── quizlr-android/         # Android application (future)
├── docs/                   # Documentation
├── examples/               # Example code
├── benches/                # Performance benchmarks
└── target/                 # Build artifacts (git-ignored)
```

## Core Library (`quizlr-core/`)

The heart of Quizlr, containing all business logic and core functionality.

```
quizlr-core/
├── Cargo.toml              # Package manifest
├── src/
│   ├── lib.rs              # Library entry point
│   ├── error.rs            # Error types and handling
│   ├── quiz/               # Quiz engine module
│   │   ├── mod.rs          # Module declaration
│   │   ├── question.rs     # Question types and traits
│   │   ├── question_tests.rs
│   │   ├── quiz.rs         # Quiz structure and logic
│   │   ├── quiz_tests.rs
│   │   ├── scoring.rs      # Scoring algorithms
│   │   ├── scoring_tests.rs
│   │   ├── session.rs      # Quiz session management
│   │   └── session_tests.rs
│   ├── adaptive/           # Adaptive learning algorithms
│   │   ├── mod.rs
│   │   ├── sm2.rs          # SuperMemo 2 algorithm
│   │   ├── irt.rs          # Item Response Theory
│   │   └── engine.rs       # Adaptive engine
│   ├── llm/                # LLM integration
│   │   ├── mod.rs
│   │   ├── provider.rs     # Provider trait
│   │   ├── openai.rs       # OpenAI implementation
│   │   ├── anthropic.rs    # Anthropic implementation
│   │   ├── local.rs        # Local model support
│   │   └── manager.rs      # Provider management
│   ├── storage/            # Storage abstraction
│   │   ├── mod.rs
│   │   ├── backend.rs      # Storage trait
│   │   ├── local.rs        # Local storage
│   │   ├── cloud.rs        # Cloud storage
│   │   ├── encrypted.rs    # Encryption wrapper
│   │   └── sync.rs         # Synchronization logic
│   ├── auth/               # Authentication
│   │   ├── mod.rs
│   │   ├── provider.rs     # Auth provider trait
│   │   ├── local.rs        # Local auth
│   │   ├── oauth.rs        # OAuth2 support
│   │   └── session.rs      # Session management
│   ├── curriculum/         # Curriculum management
│   │   ├── mod.rs
│   │   ├── course.rs       # Course structure
│   │   ├── module.rs       # Learning modules
│   │   └── progress.rs     # Progress tracking
│   ├── graph/              # Knowledge graph
│   │   ├── mod.rs
│   │   ├── node.rs         # Graph nodes
│   │   ├── edge.rs         # Relationships
│   │   ├── traversal.rs    # Graph algorithms
│   │   └── index.rs        # Graph indexing
│   └── ffi/                # Foreign Function Interface
│       ├── mod.rs
│       ├── ios.rs          # iOS bindings
│       └── android.rs      # Android bindings
├── tests/                  # Integration tests
│   ├── common/             # Shared test utilities
│   │   ├── mod.rs
│   │   └── fixtures.rs
│   ├── quiz_integration.rs
│   ├── storage_integration.rs
│   └── sync_integration.rs
├── benches/                # Benchmarks
│   ├── quiz_bench.rs
│   ├── scoring_bench.rs
│   └── storage_bench.rs
└── examples/               # Usage examples
    ├── basic_quiz.rs
    ├── custom_provider.rs
    └── adaptive_session.rs
```

### Key Core Files

#### `lib.rs`
Entry point defining public API and WASM bindings:
```rust
pub mod quiz;
pub mod curriculum;
pub mod adaptive;
// ... other modules

#[wasm_bindgen]
pub struct QuizlrCore {
    // Core state
}
```

#### `error.rs`
Centralized error handling:
```rust
#[derive(Error, Debug)]
pub enum QuizlrError {
    #[error("Quiz not found: {0}")]
    QuizNotFound(Uuid),
    // ... other variants
}
```

## Web Application (`quizlr-web/`)

Yew-based web application compiled to WebAssembly.

```
quizlr-web/
├── Cargo.toml              # Package manifest
├── index.html              # HTML template
├── src/
│   ├── main.rs             # Application entry
│   ├── app.rs              # Root component
│   ├── components/         # Reusable components
│   │   ├── mod.rs
│   │   ├── quiz/           # Quiz components
│   │   │   ├── mod.rs
│   │   │   ├── question_view.rs
│   │   │   ├── quiz_list.rs
│   │   │   └── session_view.rs
│   │   ├── auth/           # Auth components
│   │   │   ├── mod.rs
│   │   │   ├── login_form.rs
│   │   │   └── user_menu.rs
│   │   ├── layout/         # Layout components
│   │   │   ├── mod.rs
│   │   │   ├── header.rs
│   │   │   ├── sidebar.rs
│   │   │   └── footer.rs
│   │   └── common/         # Common components
│   │       ├── mod.rs
│   │       ├── button.rs
│   │       ├── modal.rs
│   │       └── spinner.rs
│   ├── pages/              # Page components
│   │   ├── mod.rs
│   │   ├── home.rs
│   │   ├── quiz.rs
│   │   ├── library.rs
│   │   ├── profile.rs
│   │   └── settings.rs
│   ├── hooks/              # Custom React-like hooks
│   │   ├── mod.rs
│   │   ├── use_quiz.rs
│   │   ├── use_auth.rs
│   │   └── use_storage.rs
│   ├── services/           # Frontend services
│   │   ├── mod.rs
│   │   ├── api.rs          # API client
│   │   ├── storage.rs      # Browser storage
│   │   └── worker.rs       # Web Worker interface
│   ├── state/              # State management
│   │   ├── mod.rs
│   │   ├── store.rs        # Global store
│   │   ├── actions.rs      # State actions
│   │   └── reducers.rs     # State reducers
│   ├── utils/              # Utility functions
│   │   ├── mod.rs
│   │   ├── date.rs
│   │   └── format.rs
│   └── routes.rs           # Route definitions
├── style/                  # CSS/SCSS files
│   ├── main.css
│   ├── components/
│   │   ├── _quiz.css
│   │   ├── _buttons.css
│   │   └── _forms.css
│   └── themes/
│       ├── light.css
│       └── dark.css
├── static/                 # Static assets
│   ├── images/
│   ├── fonts/
│   └── manifest.json
├── tests/                  # Frontend tests
│   ├── components/
│   └── e2e/
└── build.rs                # Build script
```

## Documentation (`docs/`)

mdBook-based documentation system.

```
docs/
├── book.toml               # mdBook configuration
├── src/
│   ├── SUMMARY.md          # Table of contents
│   ├── introduction.md
│   ├── user-guide/         # End-user documentation
│   │   ├── README.md
│   │   ├── installation.md
│   │   ├── getting-started.md
│   │   ├── configuration.md
│   │   ├── troubleshooting.md
│   │   └── ...
│   ├── developer-guide/    # Developer documentation
│   │   ├── README.md
│   │   ├── architecture.md
│   │   ├── project-structure.md
│   │   ├── development-setup.md
│   │   ├── building.md
│   │   ├── testing.md
│   │   ├── contributing.md
│   │   └── ...
│   ├── reference/          # API reference
│   │   ├── README.md
│   │   ├── quiz-engine.md
│   │   ├── question-types.md
│   │   └── ...
│   └── appendix/           # Additional resources
│       ├── changelog.md
│       ├── roadmap.md
│       └── glossary.md
├── book/                   # Generated documentation (git-ignored)
└── theme/                  # Custom mdBook theme
```

## Configuration Files

### Root Configuration

#### `Cargo.toml` (Workspace)
```toml
[workspace]
resolver = "2"
members = ["quizlr-core", "quizlr-web"]

[workspace.package]
version = "0.1.1"
authors = ["Quizlr Contributors"]
edition = "2021"

[workspace.dependencies]
# Shared dependencies
```

#### `justfile`
Task automation for common operations:
```just
# Default recipe
default:
    just --list

# Build all targets
build:
    cargo build --workspace

# Run tests
test:
    cargo test --workspace

# Build web app
build-web:
    ./build-web.sh
```

#### `.gitignore`
```gitignore
# Build artifacts
/target/
/dist/
/book/

# IDE files
.idea/
.vscode/
*.swp

# OS files
.DS_Store
Thumbs.db

# Environment
.env
.env.local
```

## Build Artifacts

### Development Builds
```
target/
├── debug/                  # Debug build artifacts
│   ├── deps/               # Dependencies
│   ├── build/              # Build scripts output
│   ├── libquizlr_core.rlib # Core library
│   └── quizlr-web          # Web executable
├── wasm32-unknown-unknown/ # WASM target
│   └── release/
│       └── quizlr_web.wasm
└── release/                # Release builds
```

### Distribution
```
dist/                       # Web distribution
├── index.html
├── quizlr_bg.wasm          # WASM binary
├── quizlr.js               # JS bindings
├── style/
│   └── main.css
└── static/
    └── ...
```

## Platform-Specific Directories (Future)

### iOS (`quizlr-ios/`)
```
quizlr-ios/
├── Quizlr.xcodeproj/       # Xcode project
├── Quizlr/                 # Swift sources
│   ├── App/                # App lifecycle
│   ├── Views/              # SwiftUI views
│   ├── Models/             # Swift models
│   ├── ViewModels/         # View models
│   └── Services/           # iOS services
├── QuizlrCore/             # Core library bindings
├── Resources/              # Assets and resources
└── Tests/                  # iOS tests
```

### Android (`quizlr-android/`)
```
quizlr-android/
├── app/                    # Android app module
│   ├── src/
│   │   ├── main/
│   │   │   ├── java/       # Kotlin sources
│   │   │   ├── res/        # Resources
│   │   │   └── AndroidManifest.xml
│   │   └── test/           # Unit tests
│   └── build.gradle.kts
├── quizlr-core/            # Core library module
├── gradle/                 # Gradle wrapper
└── settings.gradle.kts
```

## Development Workflows

### Feature Development
1. Create feature branch from `main`
2. Implement in `quizlr-core/src/`
3. Add tests in module's `*_tests.rs`
4. Update web UI in `quizlr-web/src/`
5. Add documentation in `docs/src/`
6. Run `just test` and `just build-web`

### Adding a New Module
1. Create directory in `quizlr-core/src/`
2. Add `mod.rs` with public interface
3. Update `lib.rs` to export module
4. Add tests in same directory
5. Document in `docs/src/reference/`

### Creating Components
1. Add component in `quizlr-web/src/components/`
2. Export from parent `mod.rs`
3. Style in `style/components/`
4. Add component tests
5. Use in pages or other components

## Best Practices

### File Organization
- Keep related code together
- One primary type per file
- Tests alongside implementation
- Clear module boundaries

### Naming Conventions
- Snake_case for files and modules
- PascalCase for types and traits
- snake_case for functions and variables
- SCREAMING_SNAKE_CASE for constants

### Module Structure
```rust
// mod.rs
mod implementation;  // Private implementation
mod types;          // Type definitions
mod traits;         // Trait definitions

pub use types::*;   // Public exports
pub use traits::*;

// Re-export commonly used items
pub use implementation::ImportantStruct;
```

### Test Organization
- Unit tests in `*_tests.rs` files
- Integration tests in `tests/` directory
- E2E tests in component directories
- Benchmark in `benches/`

## Dependency Management

### Core Dependencies
Located in workspace `Cargo.toml`:
- `serde`: Serialization
- `tokio`: Async runtime
- `thiserror`: Error handling
- `uuid`: Unique identifiers
- `chrono`: Date/time handling

### Web Dependencies
Additional in `quizlr-web/Cargo.toml`:
- `yew`: Web framework
- `wasm-bindgen`: JS bindings
- `web-sys`: Web APIs
- `gloo`: Yew utilities

### Development Dependencies
- `mockall`: Mocking framework
- `criterion`: Benchmarking
- `proptest`: Property testing
- `pretty_assertions`: Better test output

## Conclusion

Understanding Quizlr's project structure is crucial for effective development. The modular organization supports:
- Clear separation of concerns
- Easy navigation and discovery
- Parallel development
- Comprehensive testing
- Platform-specific implementations

When contributing, maintain this structure and follow the established patterns for consistency.
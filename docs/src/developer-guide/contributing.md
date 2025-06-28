# Contributing to Quizlr

Thank you for your interest in contributing to Quizlr! This guide will help you get started with contributing code, documentation, or ideas to the project.

## Code of Conduct

Before contributing, please read our Code of Conduct. We are committed to providing a welcoming and inclusive environment for all contributors.

### Our Standards

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive criticism
- Show empathy towards other community members
- Respect differing viewpoints and experiences

## Getting Started

### Prerequisites

1. Read the [Development Setup](./development-setup.md) guide
2. Fork the repository on GitHub
3. Clone your fork locally
4. Set up the development environment

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/quizlr.git
cd quizlr

# Add upstream remote
git remote add upstream https://github.com/quizlr/quizlr.git

# Install dependencies and run tests
just setup
just test
```

### Finding Something to Work On

#### Good First Issues

Look for issues labeled `good-first-issue`:
- Simple bug fixes
- Documentation improvements
- Test additions
- Small feature enhancements

#### Feature Requests

Check issues labeled `enhancement` or `feature-request`:
- Discuss the feature in the issue first
- Get consensus on the approach
- Break down large features into smaller PRs

#### Bug Reports

Issues labeled `bug`:
- Reproduce the bug locally
- Add a failing test case
- Fix the bug
- Ensure all tests pass

## Development Workflow

### 1. Create a Feature Branch

```bash
# Update your local main branch
git checkout main
git pull upstream main

# Create a feature branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

### 2. Make Your Changes

Follow these guidelines:
- Write clean, idiomatic Rust code
- Follow existing patterns and conventions
- Add tests for new functionality
- Update documentation as needed

### 3. Commit Your Changes

#### Commit Message Format

We follow the Conventional Commits specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Test additions or modifications
- `chore`: Maintenance tasks
- `perf`: Performance improvements

**Examples:**

```bash
# Feature commit
git commit -m "feat(quiz): add support for image questions

- Add new ImageQuestion variant to Question enum
- Implement image rendering in web UI
- Add tests for image question validation

Closes #123"

# Bug fix commit
git commit -m "fix(storage): prevent data loss on sync conflict

The previous implementation would overwrite local changes when
a sync conflict occurred. This fix implements proper three-way
merge logic.

Fixes #456"

# Documentation commit
git commit -m "docs(api): update LLM provider examples

Add examples for custom prompt templates and response parsing"
```

### 4. Keep Your Branch Updated

```bash
# Fetch upstream changes
git fetch upstream

# Rebase your branch
git rebase upstream/main

# Force push to your fork (only for feature branches!)
git push --force-with-lease origin feature/your-feature-name
```

### 5. Run Quality Checks

Before submitting a PR, ensure all checks pass:

```bash
# Format code
just fmt

# Run linter
just lint

# Run all tests
just test-all

# Check documentation builds
just build-docs

# Run benchmarks (optional)
just bench
```

### 6. Submit a Pull Request

1. Push your branch to your fork
2. Go to the main Quizlr repository
3. Click "New Pull Request"
4. Select your fork and branch
5. Fill out the PR template
6. Submit the PR

#### PR Template

```markdown
## Description
Brief description of what this PR does.

## Related Issue
Fixes #(issue number)

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Checklist
- [ ] My code follows the project's style guidelines
- [ ] I have self-reviewed my code
- [ ] I have added tests that prove my fix/feature works
- [ ] I have updated the documentation accordingly
- [ ] My changes generate no new warnings
```

## Code Style

### Rust Style Guide

We use `rustfmt` with the following configuration:

```toml
# rustfmt.toml
edition = "2021"
max_width = 100
use_small_heuristics = "Max"
imports_granularity = "Module"
group_imports = "StdExternalCrate"
```

#### Naming Conventions

```rust
// Modules: snake_case
mod quiz_engine;

// Types: PascalCase
struct QuizSession { }
enum QuestionType { }
trait ScoringStrategy { }

// Functions and methods: snake_case
fn calculate_score() { }
fn get_next_question() { }

// Constants: SCREAMING_SNAKE_CASE
const MAX_QUESTIONS: usize = 100;
const DEFAULT_TIME_LIMIT: Duration = Duration::from_secs(300);

// Variables: snake_case
let quiz_id = Uuid::new_v4();
let mut current_score = 0.0;
```

#### Code Organization

```rust
// Standard library imports first
use std::collections::HashMap;
use std::sync::Arc;

// External crate imports
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Local imports
use crate::error::{QuizlrError, Result};
use crate::quiz::Question;

// Then declarations
pub struct Quiz {
    // fields...
}

impl Quiz {
    // Associated functions first
    pub fn new() -> Self { }
    
    // Then methods
    pub fn add_question(&mut self, question: Question) { }
    
    // Private methods last
    fn validate(&self) -> Result<()> { }
}
```

### Documentation Style

#### Code Documentation

```rust
/// A quiz containing multiple questions.
///
/// # Examples
///
/// ```
/// use quizlr_core::quiz::Quiz;
///
/// let quiz = Quiz::builder()
///     .title("Math Quiz")
///     .description("Test your math skills")
///     .build();
/// ```
pub struct Quiz {
    /// Unique identifier for the quiz
    id: Uuid,
    
    /// Human-readable title
    title: String,
    
    /// Optional description providing context
    description: Option<String>,
}

/// Calculates the final score for a quiz session.
///
/// # Arguments
///
/// * `correct_answers` - Number of correctly answered questions
/// * `total_questions` - Total number of questions in the quiz
///
/// # Returns
///
/// The score as a percentage (0.0 to 100.0)
///
/// # Panics
///
/// Panics if `total_questions` is 0
pub fn calculate_score(correct_answers: usize, total_questions: usize) -> f32 {
    assert!(total_questions > 0, "Cannot calculate score for empty quiz");
    (correct_answers as f32 / total_questions as f32) * 100.0
}
```

#### Error Documentation

```rust
/// Errors that can occur during quiz operations.
#[derive(Debug, thiserror::Error)]
pub enum QuizError {
    /// The requested quiz was not found.
    #[error("Quiz not found: {id}")]
    NotFound { id: Uuid },
    
    /// The quiz data is invalid or corrupted.
    #[error("Invalid quiz data: {reason}")]
    InvalidData { reason: String },
    
    /// An error occurred while accessing storage.
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
}
```

### Testing Style

#### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Group related tests in nested modules
    mod quiz_creation {
        use super::*;
        
        #[test]
        fn creates_quiz_with_valid_data() {
            // test implementation
        }
        
        #[test]
        fn fails_with_empty_title() {
            // test implementation
        }
    }
    
    mod scoring {
        use super::*;
        
        #[test]
        fn calculates_percentage_correctly() {
            // test implementation
        }
    }
}
```

#### Test Naming

```rust
// Good test names - descriptive and specific
#[test]
fn quiz_builder_creates_quiz_with_all_fields() { }

#[test]
fn score_calculation_handles_zero_questions() { }

#[test]
fn session_expires_after_timeout_period() { }

// Bad test names - vague or unclear
#[test]
fn test_quiz() { }

#[test]
fn works() { }
```

## Review Process

### What We Look For

#### Code Quality
- [ ] Clean, readable code
- [ ] Appropriate abstractions
- [ ] No unnecessary complexity
- [ ] Performance considerations

#### Testing
- [ ] Adequate test coverage
- [ ] Tests are clear and focused
- [ ] Edge cases are handled

#### Documentation
- [ ] Public APIs are documented
- [ ] Complex logic is explained
- [ ] Examples provided where helpful

#### Design
- [ ] Follows existing patterns
- [ ] Maintains backward compatibility
- [ ] Considers future extensibility

### Review Timeline

- Initial review: Within 48 hours
- Subsequent reviews: Within 24 hours
- Small PRs (< 100 lines): Expedited review

### Addressing Feedback

```rust
// Before review feedback
pub fn process_quiz(data: String) -> Quiz {
    let parsed = serde_json::from_str(&data).unwrap();
    Quiz::from(parsed)
}

// After review feedback
/// Parses quiz data from JSON string.
///
/// # Errors
///
/// Returns `QuizError::InvalidData` if the JSON is malformed
/// or doesn't match the expected schema.
pub fn process_quiz(data: &str) -> Result<Quiz> {
    let parsed: QuizData = serde_json::from_str(data)
        .map_err(|e| QuizError::InvalidData {
            reason: format!("JSON parsing failed: {}", e),
        })?;
    
    Quiz::try_from(parsed)
}
```

## Types of Contributions

### Code Contributions

#### Bug Fixes
1. Create a failing test that reproduces the bug
2. Fix the bug
3. Ensure all tests pass
4. Update documentation if needed

#### New Features
1. Discuss the feature in an issue first
2. Design the API and get feedback
3. Implement with tests
4. Add documentation and examples
5. Update the changelog

#### Performance Improvements
1. Benchmark the current performance
2. Implement the optimization
3. Benchmark again to prove improvement
4. Ensure no functionality is broken

### Documentation Contributions

#### API Documentation
- Document all public items
- Include examples
- Explain error conditions
- Cross-reference related items

#### Guide Improvements
- Fix typos and grammar
- Add clarifying examples
- Update outdated information
- Improve organization

#### Tutorial Creation
- Create step-by-step guides
- Include working code examples
- Explain concepts clearly
- Test all instructions

### Other Contributions

#### Bug Reports
Include:
- Clear description
- Steps to reproduce
- Expected vs actual behavior
- System information
- Error messages/logs

#### Feature Requests
Include:
- Use case description
- Proposed API/interface
- Alternative solutions considered
- Mockups/examples if applicable

#### Code Reviews
- Review others' PRs
- Provide constructive feedback
- Test changes locally
- Suggest improvements

## Release Process

### Version Numbering

We follow Semantic Versioning (SemVer):
- MAJOR: Breaking API changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes

### Release Checklist

1. **Update Version Numbers**
   ```toml
   # Cargo.toml
   [workspace.package]
   version = "0.2.0"
   ```

2. **Update CHANGELOG.md**
   ```markdown
   ## [0.2.0] - 2024-01-20
   
   ### Added
   - Image question support (#123)
   - Custom scoring strategies (#124)
   
   ### Fixed
   - Memory leak in session manager (#125)
   
   ### Changed
   - Improved quiz generation performance by 50%
   ```

3. **Create Release PR**
   - Title: "Release v0.2.0"
   - Include changelog in description
   - Tag reviewers

4. **After Merge**
   - Create git tag
   - Publish to crates.io
   - Update documentation site
   - Announce release

## Recognition

### Contributors

All contributors are recognized in:
- `AUTHORS.md` file
- GitHub contributors page
- Release notes

### Core Contributors

Regular contributors may be invited to become core contributors with:
- Write access to the repository
- Ability to review and merge PRs
- Voice in project direction

## Getting Help

### Resources

- [Development Setup](./development-setup.md)
- [Architecture Guide](./architecture.md)
- [Testing Guide](./testing.md)
- [API Documentation](https://docs.quizlr.dev)

### Communication

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and ideas
- **Discord**: Real-time chat and support
- **Email**: security@quizlr.dev (security issues only)

### Asking Questions

Good question example:
```markdown
I'm trying to implement a custom scoring strategy but I'm getting a lifetime error.

Here's what I've tried:
```rust
impl ScoringStrategy for MyStrategy<'a> {
    fn calculate(&self, answers: &[Answer]) -> Score {
        // implementation
    }
}
```

The error is:
```
error[E0495]: cannot infer an appropriate lifetime
```

I've read the lifetime documentation but I'm still confused about how to structure this.
```

## Legal

### License

By contributing to Quizlr, you agree that your contributions will be licensed under the same license as the project (MIT/Apache 2.0 dual license).

### Developer Certificate of Origin

By making a contribution, you certify that:

1. The contribution was created by you
2. You have the right to submit it under the project license
3. You understand the contribution is public and may be redistributed

## Thank You!

Your contributions make Quizlr better for everyone. We appreciate your time and effort in improving the project. Welcome to the Quizlr community!
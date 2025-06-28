# Testing Guide

Comprehensive testing is crucial for Quizlr's reliability and maintainability. This guide covers all aspects of testing, from unit tests to end-to-end testing across platforms.

## Testing Philosophy

Quizlr follows these testing principles:
- **Test Pyramid**: Many unit tests, fewer integration tests, minimal E2E tests
- **Fast Feedback**: Tests should run quickly and provide clear feedback
- **Isolation**: Tests should not depend on external services or state
- **Documentation**: Tests serve as living documentation
- **Coverage**: Aim for 80%+ coverage, 100% for critical paths

## Test Organization

```
quizlr/
├── quizlr-core/
│   ├── src/
│   │   ├── quiz/
│   │   │   ├── quiz.rs           # Implementation
│   │   │   └── quiz_tests.rs     # Unit tests
│   │   └── lib.rs
│   ├── tests/                    # Integration tests
│   │   ├── common/
│   │   │   └── mod.rs           # Shared test utilities
│   │   └── integration_test.rs
│   └── benches/                  # Performance benchmarks
│       └── quiz_bench.rs
├── quizlr-web/
│   ├── src/
│   │   └── components/
│   │       └── quiz_view.rs
│   ├── tests/
│   │   ├── unit/                # Unit tests
│   │   └── e2e/                 # End-to-end tests
│   └── cypress/                 # E2E test specs
└── tests/                       # Workspace-level tests
    └── smoke_tests.rs
```

## Unit Testing

### Basic Unit Tests

```rust
// In quiz_tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_quiz_creation() {
        let quiz = Quiz::new(
            "Test Quiz",
            "A test quiz description",
            vec![],
        );
        
        assert_eq!(quiz.title(), "Test Quiz");
        assert_eq!(quiz.description(), "A test quiz description");
        assert!(quiz.questions().is_empty());
    }

    #[test]
    fn test_add_question() {
        let mut quiz = Quiz::new("Test", "Description", vec![]);
        let question = Question::MultipleChoice {
            id: Uuid::new_v4(),
            text: "What is 2+2?".to_string(),
            options: vec!["3", "4", "5", "6"].iter().map(|s| s.to_string()).collect(),
            correct_answer: 1,
        };
        
        quiz.add_question(question.clone());
        
        assert_eq!(quiz.questions().len(), 1);
        assert_eq!(quiz.questions()[0], question);
    }
}
```

### Testing Error Cases

```rust
#[test]
fn test_invalid_answer_index() {
    let question = Question::MultipleChoice {
        id: Uuid::new_v4(),
        text: "Test question".to_string(),
        options: vec!["A", "B", "C"].iter().map(|s| s.to_string()).collect(),
        correct_answer: 0,
    };
    
    let result = question.validate_answer(&Answer::MultipleChoice(5));
    
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Invalid answer index: 5"
    );
}

#[test]
#[should_panic(expected = "Quiz must have at least one question")]
fn test_empty_quiz_validation() {
    let quiz = Quiz::new("Empty", "No questions", vec![]);
    quiz.validate().unwrap(); // Should panic
}
```

### Parameterized Tests

```rust
use rstest::rstest;

#[rstest]
#[case("", false)]
#[case("test", false)]
#[case("test@example", false)]
#[case("test@example.com", true)]
#[case("user.name+tag@example.co.uk", true)]
fn test_email_validation(#[case] email: &str, #[case] expected: bool) {
    assert_eq!(is_valid_email(email), expected);
}

#[rstest]
#[case(0, 1, 0.0)]
#[case(1, 1, 100.0)]
#[case(3, 4, 75.0)]
#[case(10, 10, 100.0)]
fn test_score_calculation(
    #[case] correct: usize,
    #[case] total: usize,
    #[case] expected: f32,
) {
    let score = calculate_score(correct, total);
    assert!((score - expected).abs() < f32::EPSILON);
}
```

### Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_quiz_serialization_roundtrip(
        title in "[a-zA-Z ]{1,100}",
        description in "[a-zA-Z ]{1,500}",
        question_count in 1..20usize,
    ) {
        let questions: Vec<Question> = (0..question_count)
            .map(|i| Question::TrueFalse {
                id: Uuid::new_v4(),
                text: format!("Question {}", i),
                correct_answer: i % 2 == 0,
            })
            .collect();
        
        let original = Quiz::new(&title, &description, questions);
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: Quiz = serde_json::from_str(&serialized).unwrap();
        
        prop_assert_eq!(original, deserialized);
    }
    
    #[test]
    fn test_score_bounds(correct in 0..=100usize, total in 1..=100usize) {
        prop_assume!(correct <= total);
        
        let score = calculate_score(correct, total);
        
        prop_assert!(score >= 0.0);
        prop_assert!(score <= 100.0);
    }
}
```

### Mock Testing

```rust
use mockall::*;

#[automock]
#[async_trait]
pub trait StorageBackend {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    async fn put(&self, key: &str, value: Vec<u8>) -> Result<()>;
}

#[tokio::test]
async fn test_quiz_storage() {
    let mut mock = MockStorageBackend::new();
    
    mock.expect_put()
        .with(eq("quiz:123"), mockall::predicate::always())
        .times(1)
        .returning(|_, _| Ok(()));
    
    mock.expect_get()
        .with(eq("quiz:123"))
        .times(1)
        .returning(|_| Ok(Some(b"quiz data".to_vec())));
    
    let storage = QuizStorage::new(Box::new(mock));
    let quiz = Quiz::new("Test", "Description", vec![]);
    
    storage.save_quiz("123", &quiz).await.unwrap();
    let loaded = storage.load_quiz("123").await.unwrap();
    
    assert!(loaded.is_some());
}
```

## Integration Testing

### Database Integration Tests

```rust
// tests/storage_integration.rs
use quizlr_core::storage::{Storage, SqliteStorage};
use tempfile::TempDir;

#[tokio::test]
async fn test_sqlite_storage_integration() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    let storage = SqliteStorage::new(&db_path).await.unwrap();
    
    // Test CRUD operations
    let quiz = create_test_quiz();
    let id = storage.create_quiz(&quiz).await.unwrap();
    
    let loaded = storage.get_quiz(&id).await.unwrap();
    assert_eq!(loaded.unwrap().title(), quiz.title());
    
    let quizzes = storage.list_quizzes().await.unwrap();
    assert_eq!(quizzes.len(), 1);
    
    storage.delete_quiz(&id).await.unwrap();
    let deleted = storage.get_quiz(&id).await.unwrap();
    assert!(deleted.is_none());
}
```

### API Integration Tests

```rust
// tests/api_integration.rs
use quizlr_core::llm::{LLMProvider, OpenAIProvider};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, header};

#[tokio::test]
async fn test_openai_integration() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .and(header("authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({
                "choices": [{
                    "message": {
                        "content": "Generated quiz question"
                    }
                }]
            })))
        .mount(&mock_server)
        .await;
    
    let provider = OpenAIProvider::new(
        "test-key",
        Some(mock_server.uri()),
    );
    
    let response = provider.generate_question("Math", "Easy").await;
    
    assert!(response.is_ok());
    assert_eq!(response.unwrap(), "Generated quiz question");
}
```

### Service Integration Tests

```rust
#[tokio::test]
async fn test_quiz_service_integration() {
    // Setup test dependencies
    let storage = Arc::new(InMemoryStorage::new());
    let llm = Arc::new(MockLLMProvider::new());
    let auth = Arc::new(MockAuthProvider::new());
    
    let service = QuizService::new(storage, llm, auth);
    
    // Test complete workflow
    let user_id = "test-user";
    let quiz_config = QuizConfig {
        topic: "History".to_string(),
        difficulty: Difficulty::Medium,
        question_count: 5,
    };
    
    let quiz = service.create_quiz(user_id, quiz_config).await.unwrap();
    assert_eq!(quiz.questions().len(), 5);
    
    let session = service.start_session(user_id, quiz.id()).await.unwrap();
    assert_eq!(session.state(), SessionState::InProgress);
    
    // Answer questions
    for (i, question) in quiz.questions().iter().enumerate() {
        let answer = Answer::MultipleChoice(0); // Always pick first option
        service.submit_answer(&session.id(), question.id(), answer).await.unwrap();
    }
    
    let result = service.complete_session(&session.id()).await.unwrap();
    assert!(result.score >= 0.0 && result.score <= 100.0);
}
```

## End-to-End Testing

### Web E2E Tests (Playwright)

```typescript
// quizlr-web/tests/e2e/quiz.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Quiz Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8080');
  });

  test('complete quiz workflow', async ({ page }) => {
    // Navigate to quiz creation
    await page.click('text=Create Quiz');
    
    // Fill quiz form
    await page.fill('input[name="topic"]', 'Mathematics');
    await page.selectOption('select[name="difficulty"]', 'medium');
    await page.fill('input[name="questions"]', '5');
    await page.click('button[type="submit"]');
    
    // Wait for quiz generation
    await expect(page.locator('.quiz-container')).toBeVisible();
    
    // Answer all questions
    for (let i = 0; i < 5; i++) {
      await page.click('.answer-option:first-child');
      await page.click('text=Next Question');
    }
    
    // Check results
    await expect(page.locator('.quiz-results')).toBeVisible();
    await expect(page.locator('.score')).toContainText(/\d+%/);
  });

  test('quiz persistence', async ({ page, context }) => {
    // Create a quiz
    await page.click('text=Create Quiz');
    await page.fill('input[name="topic"]', 'Science');
    await page.click('button[type="submit"]');
    
    // Start answering
    await page.click('.answer-option:first-child');
    
    // Reload page
    await page.reload();
    
    // Check quiz state is preserved
    await expect(page.locator('.quiz-container')).toBeVisible();
    await expect(page.locator('.question-counter')).toContainText('2 of');
  });
});
```

### Mobile E2E Tests (Detox)

```javascript
// e2e/quiz.e2e.js
describe('Quiz Flow', () => {
  beforeAll(async () => {
    await device.launchApp();
  });

  beforeEach(async () => {
    await device.reloadReactNative();
  });

  it('should create and complete a quiz', async () => {
    // Navigate to quiz creation
    await element(by.id('create-quiz-button')).tap();
    
    // Fill form
    await element(by.id('topic-input')).typeText('Geography');
    await element(by.id('difficulty-picker')).tap();
    await element(by.text('Hard')).tap();
    
    // Create quiz
    await element(by.id('create-button')).tap();
    
    // Answer questions
    for (let i = 0; i < 5; i++) {
      await waitFor(element(by.id('question-text')))
        .toBeVisible()
        .withTimeout(5000);
      
      await element(by.id('answer-0')).tap();
      await element(by.id('next-button')).tap();
    }
    
    // Verify results
    await expect(element(by.id('quiz-score'))).toBeVisible();
  });
});
```

## Performance Testing

### Benchmark Tests

```rust
// benches/quiz_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quizlr_core::quiz::{Quiz, Question};

fn benchmark_quiz_creation(c: &mut Criterion) {
    c.bench_function("create quiz with 100 questions", |b| {
        b.iter(|| {
            let questions: Vec<Question> = (0..100)
                .map(|i| Question::TrueFalse {
                    id: Uuid::new_v4(),
                    text: format!("Question {}", i),
                    correct_answer: i % 2 == 0,
                })
                .collect();
            
            Quiz::new(
                black_box("Benchmark Quiz"),
                black_box("Description"),
                black_box(questions),
            )
        });
    });
}

fn benchmark_scoring(c: &mut Criterion) {
    let quiz = create_large_quiz(1000);
    let answers = create_random_answers(1000);
    
    c.bench_function("score 1000 answers", |b| {
        b.iter(|| {
            quiz.calculate_score(black_box(&answers))
        });
    });
}

criterion_group!(benches, benchmark_quiz_creation, benchmark_scoring);
criterion_main!(benches);
```

### Load Testing

```rust
// tests/load_test.rs
use tokio::time::{Duration, Instant};
use futures::future::join_all;

#[tokio::test]
async fn test_concurrent_quiz_sessions() {
    let service = create_test_service().await;
    let quiz = create_test_quiz();
    
    let start = Instant::now();
    let handles: Vec<_> = (0..100)
        .map(|i| {
            let service = service.clone();
            let quiz_id = quiz.id().clone();
            
            tokio::spawn(async move {
                let user_id = format!("user-{}", i);
                let session = service.start_session(&user_id, &quiz_id).await.unwrap();
                
                // Simulate answering questions
                for question_id in quiz.question_ids() {
                    service.submit_answer(
                        &session.id(),
                        &question_id,
                        Answer::MultipleChoice(0),
                    ).await.unwrap();
                }
                
                service.complete_session(&session.id()).await.unwrap()
            })
        })
        .collect();
    
    let results = join_all(handles).await;
    let duration = start.elapsed();
    
    println!("Completed 100 concurrent sessions in {:?}", duration);
    assert!(duration < Duration::from_secs(5));
    assert!(results.iter().all(|r| r.is_ok()));
}
```

## Test Utilities

### Test Fixtures

```rust
// tests/common/fixtures.rs
use quizlr_core::quiz::{Quiz, Question, Difficulty};
use uuid::Uuid;

pub fn create_test_quiz() -> Quiz {
    Quiz::new(
        "Test Quiz",
        "A quiz for testing",
        vec![
            Question::MultipleChoice {
                id: Uuid::new_v4(),
                text: "What is 2+2?".to_string(),
                options: vec!["3", "4", "5", "6"].iter().map(|s| s.to_string()).collect(),
                correct_answer: 1,
            },
            Question::TrueFalse {
                id: Uuid::new_v4(),
                text: "The Earth is flat".to_string(),
                correct_answer: false,
            },
        ],
    )
}

pub fn create_quiz_with_difficulty(difficulty: Difficulty, questions: usize) -> Quiz {
    let questions = (0..questions)
        .map(|i| match i % 3 {
            0 => create_multiple_choice_question(i, difficulty),
            1 => create_true_false_question(i),
            _ => create_short_answer_question(i),
        })
        .collect();
    
    Quiz::new(
        &format!("{:?} Quiz", difficulty),
        "Auto-generated quiz",
        questions,
    )
}
```

### Test Builders

```rust
// tests/common/builders.rs
pub struct QuizBuilder {
    title: String,
    description: String,
    questions: Vec<Question>,
}

impl QuizBuilder {
    pub fn new() -> Self {
        Self {
            title: "Test Quiz".to_string(),
            description: "Description".to_string(),
            questions: vec![],
        }
    }
    
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }
    
    pub fn with_questions(mut self, count: usize) -> Self {
        self.questions = (0..count)
            .map(|i| Question::TrueFalse {
                id: Uuid::new_v4(),
                text: format!("Question {}", i),
                correct_answer: true,
            })
            .collect();
        self
    }
    
    pub fn build(self) -> Quiz {
        Quiz::new(&self.title, &self.description, self.questions)
    }
}

// Usage
let quiz = QuizBuilder::new()
    .with_title("Custom Quiz")
    .with_questions(10)
    .build();
```

### Test Helpers

```rust
// tests/common/helpers.rs
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_test_logging() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter("debug,quizlr=trace")
            .with_test_writer()
            .init();
    });
}

pub async fn with_test_db<F, Fut>(f: F)
where
    F: FnOnce(SqliteStorage) -> Fut,
    Fut: Future<Output = ()>,
{
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let storage = SqliteStorage::new(&db_path).await.unwrap();
    
    f(storage).await;
    
    // Cleanup happens automatically when temp_dir is dropped
}

pub fn assert_quiz_equal(left: &Quiz, right: &Quiz) {
    assert_eq!(left.title(), right.title());
    assert_eq!(left.description(), right.description());
    assert_eq!(left.questions().len(), right.questions().len());
    
    for (l, r) in left.questions().iter().zip(right.questions().iter()) {
        assert_eq!(l, r);
    }
}
```

## Test Coverage

### Measuring Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# With specific features
cargo tarpaulin --features "llm storage" --out Lcov

# Exclude test files
cargo tarpaulin --exclude-files "**/tests/*" --exclude-files "**/*_test.rs"
```

### Coverage Configuration

```toml
# tarpaulin.toml
[default]
exclude-files = ["*/tests/*", "*_test.rs", "*/benches/*"]
ignored = ["quizlr-web"]  # WASM not supported
timeout = "300s"
all-features = true

[report]
out = ["Html", "Lcov", "Json"]
output-dir = "coverage"
```

### CI Coverage Integration

```yaml
# .github/workflows/coverage.yml
name: Coverage

on: [push, pull_request]

jobs:
  coverage:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Install tarpaulin
      run: cargo install cargo-tarpaulin
    
    - name: Generate coverage
      run: cargo tarpaulin --out Lcov
    
    - name: Upload to Codecov
      uses: codecov/codecov-action@v3
      with:
        files: ./lcov.info
```

## Testing Best Practices

### Test Naming

```rust
// Good test names
#[test]
fn quiz_creation_with_valid_data_succeeds() { }

#[test]
fn quiz_validation_fails_when_no_questions() { }

#[test]
fn score_calculation_returns_percentage() { }

// Bad test names
#[test]
fn test1() { }

#[test]
fn quiz_test() { }
```

### Test Structure (AAA Pattern)

```rust
#[test]
fn test_quiz_completion() {
    // Arrange
    let quiz = create_test_quiz();
    let mut session = Session::new(quiz);
    let answers = create_test_answers();
    
    // Act
    for (question_id, answer) in answers {
        session.submit_answer(question_id, answer);
    }
    let result = session.complete();
    
    // Assert
    assert_eq!(result.status, SessionStatus::Completed);
    assert!(result.score >= 0.0 && result.score <= 100.0);
    assert_eq!(result.answered_count, quiz.question_count());
}
```

### Test Isolation

```rust
// Bad - Tests depend on shared state
static mut COUNTER: u32 = 0;

#[test]
fn test_increment() {
    unsafe {
        COUNTER += 1;
        assert_eq!(COUNTER, 1); // Fails if tests run in parallel
    }
}

// Good - Each test is independent
#[test]
fn test_increment() {
    let mut counter = 0;
    counter += 1;
    assert_eq!(counter, 1);
}
```

### Async Test Patterns

```rust
// Test with timeout
#[tokio::test(flavor = "multi_thread")]
#[timeout(Duration::from_secs(5))]
async fn test_async_operation() {
    let result = long_running_operation().await;
    assert!(result.is_ok());
}

// Test with custom runtime
#[test]
fn test_with_custom_runtime() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    
    runtime.block_on(async {
        let result = async_operation().await;
        assert!(result.is_ok());
    });
}
```

## Debugging Tests

### Running Specific Tests

```bash
# Run single test
cargo test test_quiz_creation

# Run tests matching pattern
cargo test quiz

# Run tests in specific module
cargo test quiz::tests::

# Run with output
cargo test -- --nocapture

# Run with specific log level
RUST_LOG=debug cargo test
```

### Debugging Failed Tests

```rust
// Add debug information
#[test]
fn test_complex_calculation() {
    let input = create_complex_input();
    dbg!(&input); // Print debug representation
    
    let result = complex_calculation(input);
    
    eprintln!("Result: {:?}", result); // Print to stderr
    
    assert_eq!(result.value, 42, "Expected 42, got {}", result.value);
}

// Use custom assertions
#[test]
fn test_quiz_equality() {
    let quiz1 = create_quiz("A");
    let quiz2 = create_quiz("B");
    
    // Better error messages with pretty_assertions
    use pretty_assertions::assert_eq;
    assert_eq!(quiz1, quiz2);
}
```

## Conclusion

Effective testing is essential for Quizlr's quality and reliability. Key takeaways:

- Write tests at multiple levels (unit, integration, E2E)
- Keep tests fast, isolated, and deterministic
- Use appropriate testing tools for each scenario
- Maintain high test coverage for critical paths
- Tests are documentation - keep them readable

For more information:
- [Development Setup](./development-setup.md) - Setting up test environment
- [Contributing](./contributing.md) - Testing requirements for PRs
- [Architecture](./architecture.md) - Understanding system design for better tests
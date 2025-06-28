# Extensions API

The Quizlr Extensions API enables developers to extend the platform's functionality through plugins, custom question types, scoring algorithms, and integrations. The system is designed for flexibility while maintaining security and performance.

## Architecture

The extension system is built on a trait-based architecture:

```rust
pub trait Extension: Send + Sync {
    fn metadata(&self) -> ExtensionMetadata;
    fn initialize(&mut self, context: &ExtensionContext) -> Result<(), ExtensionError>;
    fn shutdown(&mut self) -> Result<(), ExtensionError>;
}

pub struct ExtensionMetadata {
    pub id: String,
    pub name: String,
    pub version: Version,
    pub author: String,
    pub description: String,
    pub capabilities: Vec<Capability>,
    pub dependencies: Vec<Dependency>,
}
```

## Extension Types

### Question Type Extensions

Create custom question types beyond the built-in ones:

```rust
pub trait QuestionTypeExtension: Extension {
    fn question_type_id(&self) -> &str;
    fn validate_question(&self, data: &serde_json::Value) -> Result<(), ValidationError>;
    fn validate_answer(&self, question: &serde_json::Value, answer: &serde_json::Value) -> Result<bool, ValidationError>;
    fn render_question(&self, question: &serde_json::Value) -> Result<QuestionDisplay, RenderError>;
    fn score_answer(&self, question: &serde_json::Value, answer: &serde_json::Value) -> Result<f32, ScoringError>;
}
```

**Example - Code Review Question**:
```rust
pub struct CodeReviewExtension {
    metadata: ExtensionMetadata,
    syntax_highlighter: SyntaxHighlighter,
    code_analyzer: CodeAnalyzer,
}

impl QuestionTypeExtension for CodeReviewExtension {
    fn question_type_id(&self) -> &str {
        "code_review"
    }
    
    fn validate_question(&self, data: &serde_json::Value) -> Result<(), ValidationError> {
        let code = data.get("code")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingField("code".to_string()))?;
        
        let language = data.get("language")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ValidationError::MissingField("language".to_string()))?;
        
        // Validate syntax
        self.syntax_highlighter.validate(code, language)?;
        
        Ok(())
    }
    
    fn score_answer(&self, question: &serde_json::Value, answer: &serde_json::Value) -> Result<f32, ScoringError> {
        let issues_found = answer.get("issues_found")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ScoringError::InvalidAnswer)?;
        
        let expected_issues = question.get("expected_issues")
            .and_then(|v| v.as_array())
            .ok_or_else(|| ScoringError::InvalidQuestion)?;
        
        // Calculate score based on found vs expected issues
        let found_count = issues_found.len() as f32;
        let expected_count = expected_issues.len() as f32;
        
        let precision = self.calculate_precision(issues_found, expected_issues);
        let recall = found_count / expected_count.max(1.0);
        
        // F1 score
        let score = 2.0 * (precision * recall) / (precision + recall).max(0.001);
        
        Ok(score.clamp(0.0, 1.0))
    }
}
```

### Scoring Extensions

Implement custom scoring algorithms:

```rust
pub trait ScoringExtension: Extension {
    fn scoring_strategy_id(&self) -> &str;
    fn calculate_score(
        &self,
        session: &QuizSession,
        questions: &[Question],
        config: &serde_json::Value,
    ) -> Result<Score, ScoringError>;
}
```

**Example - Competency-Based Scoring**:
```rust
pub struct CompetencyBasedScoring {
    metadata: ExtensionMetadata,
    competency_map: HashMap<Uuid, Vec<Competency>>,
}

impl ScoringExtension for CompetencyBasedScoring {
    fn scoring_strategy_id(&self) -> &str {
        "competency_based"
    }
    
    fn calculate_score(
        &self,
        session: &QuizSession,
        questions: &[Question],
        config: &serde_json::Value,
    ) -> Result<Score, ScoringError> {
        let mut competency_scores: HashMap<String, CompetencyScore> = HashMap::new();
        
        // Map questions to competencies
        for (question, response) in questions.iter().zip(&session.responses) {
            if let Some(competencies) = self.competency_map.get(&question.id) {
                for competency in competencies {
                    let score = competency_scores
                        .entry(competency.id.clone())
                        .or_insert_with(|| CompetencyScore::default());
                    
                    score.add_result(
                        response.is_correct,
                        competency.weight,
                        question.difficulty,
                    );
                }
            }
        }
        
        // Calculate overall score
        let total_score = competency_scores.values()
            .map(|cs| cs.weighted_score())
            .sum::<f32>() / competency_scores.len().max(1) as f32;
        
        Ok(Score {
            raw_score: total_score,
            weighted_score: total_score,
            competency_breakdown: Some(competency_scores),
            ..Default::default()
        })
    }
}
```

### Storage Extensions

Add custom storage backends:

```rust
pub trait StorageExtension: Extension {
    fn storage_backend_id(&self) -> &str;
    fn create_storage(&self, config: &serde_json::Value) -> Result<Box<dyn Storage>, StorageError>;
}
```

**Example - Database Storage**:
```rust
pub struct PostgresStorageExtension {
    metadata: ExtensionMetadata,
}

impl StorageExtension for PostgresStorageExtension {
    fn storage_backend_id(&self) -> &str {
        "postgres"
    }
    
    fn create_storage(&self, config: &serde_json::Value) -> Result<Box<dyn Storage>, StorageError> {
        let connection_string = config.get("connection_string")
            .and_then(|v| v.as_str())
            .ok_or_else(|| StorageError::InvalidConfig("Missing connection_string".to_string()))?;
        
        let pool = PgPool::connect(connection_string)
            .await
            .map_err(|e| StorageError::ConnectionFailed(e.to_string()))?;
        
        Ok(Box::new(PostgresStorage::new(pool)))
    }
}
```

### Analytics Extensions

Custom analytics and reporting:

```rust
pub trait AnalyticsExtension: Extension {
    fn analyze_session(&self, session: &QuizSession) -> Result<SessionAnalytics, AnalyticsError>;
    fn analyze_quiz(&self, quiz_id: Uuid, sessions: &[QuizSession]) -> Result<QuizAnalytics, AnalyticsError>;
    fn generate_report(&self, analytics: &dyn Any, format: ReportFormat) -> Result<Vec<u8>, ReportError>;
}
```

## Extension Development

### Project Structure

```
my-extension/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── extension.rs
│   └── tests.rs
├── assets/
│   ├── icon.png
│   └── templates/
└── extension.toml
```

### Extension Manifest

```toml
[extension]
id = "my-company.advanced-scoring"
name = "Advanced Scoring Extension"
version = "1.0.0"
author = "My Company"
description = "Adds advanced scoring capabilities"

[capabilities]
types = ["scoring", "analytics"]
permissions = ["read_sessions", "write_scores"]

[dependencies]
quizlr-core = "1.0"
other-extension = { version = "2.0", optional = true }

[configuration]
schema = "config-schema.json"
defaults = "default-config.json"
```

### Basic Extension

```rust
use quizlr_extensions::*;

pub struct MyExtension {
    metadata: ExtensionMetadata,
    config: Config,
}

impl Extension for MyExtension {
    fn metadata(&self) -> ExtensionMetadata {
        self.metadata.clone()
    }
    
    fn initialize(&mut self, context: &ExtensionContext) -> Result<(), ExtensionError> {
        // Access Quizlr services
        let storage = context.storage();
        let llm = context.llm_manager();
        
        // Initialize your extension
        self.setup_database(storage)?;
        self.register_handlers(context)?;
        
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<(), ExtensionError> {
        // Cleanup resources
        self.close_connections()?;
        Ok(())
    }
}

// Export the extension
#[no_mangle]
pub extern "C" fn create_extension() -> Box<dyn Extension> {
    Box::new(MyExtension::new())
}
```

## Extension Context

The context provides access to Quizlr services:

```rust
pub struct ExtensionContext {
    storage: Arc<dyn Storage>,
    llm_manager: Arc<LlmManager>,
    event_bus: Arc<EventBus>,
    config: Arc<Config>,
    logger: Logger,
}

impl ExtensionContext {
    pub fn storage(&self) -> Arc<dyn Storage> {
        self.storage.clone()
    }
    
    pub fn llm_manager(&self) -> Arc<LlmManager> {
        self.llm_manager.clone()
    }
    
    pub fn subscribe<E: Event>(&self, handler: impl EventHandler<E>) {
        self.event_bus.subscribe(handler);
    }
    
    pub fn publish(&self, event: impl Event) -> Result<(), EventError> {
        self.event_bus.publish(event)
    }
}
```

## Event System

Extensions can subscribe to and publish events:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuizlrEvent {
    QuizStarted { quiz_id: Uuid, user_id: Uuid },
    QuestionAnswered { session_id: Uuid, question_id: Uuid, is_correct: bool },
    QuizCompleted { session_id: Uuid, score: f32 },
    ExtensionEvent { extension_id: String, data: serde_json::Value },
}

pub trait EventHandler<E: Event>: Send + Sync {
    fn handle(&self, event: &E) -> Result<(), EventError>;
}
```

**Example Event Handler**:
```rust
struct AchievementHandler {
    achievement_service: AchievementService,
}

impl EventHandler<QuizlrEvent> for AchievementHandler {
    fn handle(&self, event: &QuizlrEvent) -> Result<(), EventError> {
        match event {
            QuizlrEvent::QuizCompleted { session_id, score } => {
                if *score >= 0.95 {
                    self.achievement_service.award(
                        session_id,
                        Achievement::PerfectScore,
                    )?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}
```

## Security

### Sandboxing

Extensions run in a sandboxed environment:

```rust
pub struct ExtensionSandbox {
    memory_limit: usize,
    cpu_quota: f32,
    allowed_syscalls: HashSet<String>,
    network_access: NetworkPolicy,
}

impl ExtensionSandbox {
    pub fn execute<F, R>(&self, extension: &dyn Extension, operation: F) -> Result<R, SandboxError>
    where
        F: FnOnce() -> R,
    {
        // Apply resource limits
        self.apply_limits()?;
        
        // Execute in restricted context
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(operation))
            .map_err(|_| SandboxError::Panic)?;
        
        Ok(result)
    }
}
```

### Permission System

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Permission {
    ReadQuizzes,
    WriteQuizzes,
    ReadSessions,
    WriteSessions,
    AccessLlm,
    AccessNetwork,
    Custom(String),
}

pub struct PermissionManager {
    grants: HashMap<String, HashSet<Permission>>,
}

impl PermissionManager {
    pub fn check(&self, extension_id: &str, permission: &Permission) -> Result<(), PermissionError> {
        let grants = self.grants.get(extension_id)
            .ok_or_else(|| PermissionError::ExtensionNotFound)?;
        
        if grants.contains(permission) {
            Ok(())
        } else {
            Err(PermissionError::Denied(permission.clone()))
        }
    }
}
```

## Extension Manager

Load and manage extensions:

```rust
pub struct ExtensionManager {
    extensions: HashMap<String, Box<dyn Extension>>,
    loader: ExtensionLoader,
    sandbox: ExtensionSandbox,
    permissions: PermissionManager,
}

impl ExtensionManager {
    pub async fn load_extension(&mut self, path: &Path) -> Result<String, ExtensionError> {
        // Load extension metadata
        let manifest = self.loader.load_manifest(path)?;
        
        // Verify signature
        self.loader.verify_signature(&manifest)?;
        
        // Load extension code
        let extension = self.loader.load_extension(path)?;
        
        // Initialize in sandbox
        let context = self.create_context(&manifest);
        self.sandbox.execute(&*extension, || {
            extension.initialize(&context)
        })?;
        
        // Register extension
        let id = manifest.id.clone();
        self.extensions.insert(id.clone(), extension);
        
        Ok(id)
    }
    
    pub fn get_extension<T: Extension>(&self, id: &str) -> Option<&T> {
        self.extensions.get(id)
            .and_then(|ext| ext.as_any().downcast_ref::<T>())
    }
}
```

## Testing Extensions

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use quizlr_extensions::testing::*;
    
    #[test]
    fn test_scoring_extension() {
        let mut extension = MyScoreExtension::new();
        let context = MockExtensionContext::new();
        
        // Initialize
        extension.initialize(&context).unwrap();
        
        // Create test data
        let session = create_test_session();
        let questions = create_test_questions();
        
        // Calculate score
        let score = extension.calculate_score(&session, &questions, &json!({})).unwrap();
        
        assert!(score.raw_score >= 0.0 && score.raw_score <= 1.0);
    }
}
```

### Integration Testing

```rust
#[tokio::test]
async fn test_extension_integration() {
    let mut manager = ExtensionManager::new();
    
    // Load extension
    let extension_id = manager.load_extension(Path::new("./my-extension")).await.unwrap();
    
    // Create quiz with custom question type
    let quiz = QuizBuilder::new("Test Quiz")
        .add_question(Question {
            question_type: QuestionType::Extension {
                type_id: "code_review".to_string(),
                data: json!({
                    "code": "fn main() { println!(\"Hello\") }",
                    "language": "rust",
                    "expected_issues": ["missing semicolon"]
                }),
            },
            ..Default::default()
        })
        .build();
    
    // Run quiz and verify scoring
    let session = run_quiz_with_extension(&quiz, &manager).await;
    assert!(session.score > 0.0);
}
```

## Best Practices

### Performance

1. **Lazy Loading**: Load resources only when needed
2. **Caching**: Cache computed results
3. **Async Operations**: Use async for I/O operations
4. **Resource Limits**: Respect memory and CPU limits

### Error Handling

1. **Graceful Degradation**: Handle failures without crashing
2. **Clear Messages**: Provide helpful error messages
3. **Logging**: Use structured logging
4. **Recovery**: Implement retry logic where appropriate

### Compatibility

1. **Version Checks**: Check Quizlr version compatibility
2. **Feature Detection**: Check for optional features
3. **Backward Compatibility**: Maintain compatibility
4. **Migration**: Provide upgrade paths

## Examples

### Complete Extension

```rust
use quizlr_extensions::*;

pub struct MathQuestionExtension {
    metadata: ExtensionMetadata,
    math_engine: MathEngine,
    latex_renderer: LatexRenderer,
}

impl MathQuestionExtension {
    pub fn new() -> Self {
        Self {
            metadata: ExtensionMetadata {
                id: "math-questions".to_string(),
                name: "Math Question Types".to_string(),
                version: Version::new(1, 0, 0),
                author: "Math Extensions Inc".to_string(),
                description: "Adds mathematical question types".to_string(),
                capabilities: vec![
                    Capability::QuestionType,
                    Capability::Rendering,
                ],
                dependencies: vec![],
            },
            math_engine: MathEngine::new(),
            latex_renderer: LatexRenderer::new(),
        }
    }
}

impl Extension for MathQuestionExtension {
    fn metadata(&self) -> ExtensionMetadata {
        self.metadata.clone()
    }
    
    fn initialize(&mut self, context: &ExtensionContext) -> Result<(), ExtensionError> {
        // Register question types
        context.register_question_type("equation_solving", self)?;
        context.register_question_type("graph_plotting", self)?;
        
        // Subscribe to events
        context.subscribe(MathEventHandler::new());
        
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<(), ExtensionError> {
        Ok(())
    }
}

impl QuestionTypeExtension for MathQuestionExtension {
    fn question_type_id(&self) -> &str {
        "math"
    }
    
    fn render_question(&self, question: &serde_json::Value) -> Result<QuestionDisplay, RenderError> {
        let latex = question.get("equation")
            .and_then(|v| v.as_str())
            .ok_or_else(|| RenderError::MissingField("equation".to_string()))?;
        
        let rendered = self.latex_renderer.render(latex)?;
        
        Ok(QuestionDisplay {
            html: format!("<div class='math-question'>{}</div>", rendered),
            assets: vec![
                Asset {
                    path: "katex.min.css".to_string(),
                    content: include_bytes!("../assets/katex.min.css").to_vec(),
                },
            ],
        })
    }
    
    fn score_answer(&self, question: &serde_json::Value, answer: &serde_json::Value) -> Result<f32, ScoringError> {
        let expected = question.get("solution")
            .ok_or_else(|| ScoringError::InvalidQuestion)?;
        
        let student_answer = answer.get("expression")
            .ok_or_else(|| ScoringError::InvalidAnswer)?;
        
        // Use math engine to compare mathematical equivalence
        let score = self.math_engine.compare_expressions(expected, student_answer)?;
        
        Ok(score)
    }
}
```

### Extension Usage

```rust
// In your application
let extension_manager = ExtensionManager::new();

// Load extensions
extension_manager.load_extension("./extensions/math-questions").await?;
extension_manager.load_extension("./extensions/advanced-scoring").await?;

// Use in quiz
let quiz = QuizBuilder::new("Advanced Math")
    .add_question(Question {
        question_type: QuestionType::Extension {
            type_id: "equation_solving".to_string(),
            data: json!({
                "equation": "\\frac{d}{dx}(x^2 + 3x) = ?",
                "solution": "2x + 3",
                "variables": ["x"],
            }),
        },
        ..Default::default()
    })
    .scoring_strategy(ScoringStrategy::Extension {
        strategy_id: "competency_based".to_string(),
        config: json!({
            "competencies": ["calculus", "derivatives"],
        }),
    })
    .build();
```
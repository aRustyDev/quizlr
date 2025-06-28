# LLM Integration

Quizlr's LLM integration enables AI-powered features like dynamic question generation, intelligent answer evaluation, and adaptive learning experiences. The system supports multiple LLM providers through a unified interface.

## Architecture

The LLM system is built around the `LlmClient` trait and provider implementations:

```rust
#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String, QuizlrError>;
    async fn generate_with_options(&self, request: LlmRequest) -> Result<LlmResponse, QuizlrError>;
    async fn embed(&self, text: &str) -> Result<Vec<f32>, QuizlrError>;
    async fn classify(&self, text: &str, categories: &[String]) -> Result<Classification, QuizlrError>;
}
```

## Supported Providers

### Claude (Anthropic)

Advanced reasoning and educational content generation.

```rust
pub struct ClaudeClient {
    api_key: String,
    model: ClaudeModel,
    max_tokens: usize,
    temperature: f32,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: ClaudeModel::Claude3Sonnet,
            max_tokens: 4000,
            temperature: 0.7,
        }
    }
    
    pub fn with_model(mut self, model: ClaudeModel) -> Self {
        self.model = model;
        self
    }
    
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature.clamp(0.0, 1.0);
        self
    }
}
```

**Models**:
- Claude 3 Opus (most capable)
- Claude 3 Sonnet (balanced)
- Claude 3 Haiku (fastest)

**Example**:
```rust
let client = ClaudeClient::new(api_key)
    .with_model(ClaudeModel::Claude3Sonnet)
    .with_temperature(0.5);

let prompt = "Generate a multiple choice question about photosynthesis";
let response = client.generate(prompt).await?;
```

### Gemini (Google)

Multimodal capabilities and efficient processing.

```rust
pub struct GeminiClient {
    api_key: String,
    model: GeminiModel,
    safety_settings: SafetySettings,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: GeminiModel::GeminiPro,
            safety_settings: SafetySettings::default(),
        }
    }
    
    pub fn with_safety(mut self, settings: SafetySettings) -> Self {
        self.safety_settings = settings;
        self
    }
}
```

**Features**:
- Text and image understanding
- Safety filtering
- Function calling
- Streaming responses

### OpenAI

GPT models for versatile text generation.

```rust
pub struct OpenAIClient {
    api_key: String,
    model: OpenAIModel,
    organization_id: Option<String>,
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: OpenAIModel::Gpt4,
            organization_id: None,
        }
    }
    
    pub fn with_organization(mut self, org_id: String) -> Self {
        self.organization_id = Some(org_id);
        self
    }
}
```

**Models**:
- GPT-4 (most capable)
- GPT-4 Turbo (faster, cheaper)
- GPT-3.5 Turbo (economical)

## Core Features

### Question Generation

Generate questions dynamically based on topics and parameters:

```rust
pub struct QuestionGenerator {
    llm: Box<dyn LlmClient>,
    validator: QuestionValidator,
}

impl QuestionGenerator {
    pub async fn generate_question(
        &self,
        params: QuestionParams,
    ) -> Result<Question, QuizlrError> {
        let prompt = self.build_prompt(&params);
        let response = self.llm.generate(&prompt).await?;
        
        let question = self.parse_question(&response)?;
        self.validator.validate(&question)?;
        
        Ok(question)
    }
    
    fn build_prompt(&self, params: &QuestionParams) -> String {
        format!(
            r#"Generate a {} question about {}.
            Difficulty: {} (0.0 to 1.0 scale)
            Include: {}
            Format as JSON with fields: question, options, correct_answer, explanation
            "#,
            params.question_type,
            params.topic,
            params.difficulty,
            params.requirements.join(", ")
        )
    }
}
```

**Usage**:
```rust
let generator = QuestionGenerator::new(llm_client);

let params = QuestionParams {
    topic: "World War II".to_string(),
    question_type: QuestionType::MultipleChoice,
    difficulty: 0.6,
    requirements: vec![
        "Focus on causes".to_string(),
        "Include dates".to_string(),
    ],
};

let question = generator.generate_question(params).await?;
```

### Answer Evaluation

Intelligent evaluation of free-form answers:

```rust
pub struct AnswerEvaluator {
    llm: Box<dyn LlmClient>,
    rubric_engine: RubricEngine,
}

impl AnswerEvaluator {
    pub async fn evaluate_answer(
        &self,
        question: &str,
        expected: &str,
        student_answer: &str,
        rubric: Option<&Rubric>,
    ) -> Result<Evaluation, QuizlrError> {
        let prompt = self.build_evaluation_prompt(
            question,
            expected,
            student_answer,
            rubric,
        );
        
        let response = self.llm.generate_with_options(LlmRequest {
            prompt,
            temperature: 0.2, // Lower temperature for consistency
            max_tokens: 1000,
            ..Default::default()
        }).await?;
        
        self.parse_evaluation(&response.text)
    }
}
```

**Evaluation Structure**:
```rust
pub struct Evaluation {
    pub score: f32,              // 0.0 to 1.0
    pub feedback: String,         // Detailed feedback
    pub strengths: Vec<String>,   // What was done well
    pub improvements: Vec<String>, // Areas to improve
    pub misconceptions: Vec<String>, // Identified misunderstandings
}
```

### Interactive Interviews

Dynamic conversational assessments:

```rust
pub struct InterviewConductor {
    llm: Box<dyn LlmClient>,
    conversation_manager: ConversationManager,
}

impl InterviewConductor {
    pub async fn start_interview(
        &self,
        topic: &str,
        initial_question: &str,
    ) -> Result<InterviewSession, QuizlrError> {
        let session = InterviewSession::new(topic);
        
        let response = self.llm.generate_with_options(LlmRequest {
            prompt: initial_question,
            system_prompt: Some(self.build_system_prompt(topic)),
            ..Default::default()
        }).await?;
        
        session.add_exchange(initial_question, &response.text);
        Ok(session)
    }
    
    pub async fn continue_interview(
        &self,
        session: &mut InterviewSession,
        student_response: &str,
    ) -> Result<InterviewExchange, QuizlrError> {
        let context = session.get_context();
        let follow_up = self.generate_follow_up(&context, student_response).await?;
        
        let exchange = InterviewExchange {
            question: follow_up.question,
            expected_concepts: follow_up.concepts,
            difficulty_adjustment: follow_up.difficulty_delta,
        };
        
        session.add_student_response(student_response);
        session.add_exchange(&exchange.question, "");
        
        Ok(exchange)
    }
}
```

### Content Enhancement

Improve and expand existing quiz content:

```rust
pub struct ContentEnhancer {
    llm: Box<dyn LlmClient>,
}

impl ContentEnhancer {
    pub async fn enhance_explanation(
        &self,
        question: &Question,
        target_audience: &str,
    ) -> Result<String, QuizlrError> {
        let prompt = format!(
            r#"Enhance this explanation for {}: 
            Question: {}
            Current explanation: {}
            
            Make it more engaging and educational while maintaining accuracy."#,
            target_audience,
            question.get_text(),
            question.get_explanation().unwrap_or("No explanation provided")
        );
        
        self.llm.generate(&prompt).await
    }
    
    pub async fn generate_hints(
        &self,
        question: &Question,
        num_hints: usize,
    ) -> Result<Vec<String>, QuizlrError> {
        let prompt = format!(
            r#"Generate {} progressive hints for this question:
            {}
            
            Each hint should provide more information than the last,
            but not give away the answer directly."#,
            num_hints,
            question.get_text()
        );
        
        let response = self.llm.generate(&prompt).await?;
        self.parse_hints(&response)
    }
}
```

## Advanced Features

### Embeddings and Semantic Search

```rust
pub struct SemanticSearch {
    llm: Box<dyn LlmClient>,
    vector_store: VectorStore,
}

impl SemanticSearch {
    pub async fn index_questions(&self, questions: &[Question]) -> Result<(), QuizlrError> {
        for question in questions {
            let text = question.to_searchable_text();
            let embedding = self.llm.embed(&text).await?;
            
            self.vector_store.insert(
                question.id,
                embedding,
                question.metadata.clone(),
            ).await?;
        }
        Ok(())
    }
    
    pub async fn find_similar(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<Question>, QuizlrError> {
        let query_embedding = self.llm.embed(query).await?;
        let results = self.vector_store.search(&query_embedding, limit).await?;
        
        // Load full questions from IDs
        self.load_questions(results)
    }
}
```

### Difficulty Calibration

```rust
pub struct DifficultyCalibrator {
    llm: Box<dyn LlmClient>,
    performance_analyzer: PerformanceAnalyzer,
}

impl DifficultyCalibrator {
    pub async fn calibrate_difficulty(
        &self,
        question: &Question,
        performance_data: &[QuestionPerformance],
    ) -> Result<f32, QuizlrError> {
        let analysis = self.performance_analyzer.analyze(performance_data);
        
        let prompt = format!(
            r#"Analyze this question's difficulty based on student performance:
            Question: {}
            Success rate: {:.1}%
            Average time: {}s
            Common mistakes: {:?}
            
            Provide a difficulty rating from 0.0 (easiest) to 1.0 (hardest)."#,
            question.get_text(),
            analysis.success_rate * 100.0,
            analysis.avg_time_seconds,
            analysis.common_errors
        );
        
        let response = self.llm.generate(&prompt).await?;
        self.parse_difficulty(&response)
    }
}
```

### Personalized Learning Paths

```rust
pub struct LearningPathGenerator {
    llm: Box<dyn LlmClient>,
    curriculum_engine: CurriculumEngine,
}

impl LearningPathGenerator {
    pub async fn generate_path(
        &self,
        learner_profile: &LearnerProfile,
        learning_goals: &[String],
    ) -> Result<LearningPath, QuizlrError> {
        let prompt = self.build_path_prompt(learner_profile, learning_goals);
        
        let response = self.llm.generate_with_options(LlmRequest {
            prompt,
            response_format: ResponseFormat::Json,
            ..Default::default()
        }).await?;
        
        let path_data: PathData = serde_json::from_str(&response.text)?;
        
        Ok(LearningPath {
            modules: self.create_modules(path_data.modules),
            prerequisites: path_data.prerequisites,
            estimated_duration: path_data.duration_hours,
            difficulty_progression: path_data.difficulty_curve,
        })
    }
}
```

## Configuration

### LLM Manager

Central configuration for LLM services:

```rust
pub struct LlmManager {
    providers: HashMap<String, Box<dyn LlmClient>>,
    default_provider: String,
    fallback_chain: Vec<String>,
    cache: Option<LlmCache>,
}

impl LlmManager {
    pub fn builder() -> LlmManagerBuilder {
        LlmManagerBuilder::default()
    }
    
    pub async fn generate(&self, prompt: &str) -> Result<String, QuizlrError> {
        // Try default provider first
        if let Some(client) = self.providers.get(&self.default_provider) {
            match client.generate(prompt).await {
                Ok(response) => return Ok(response),
                Err(e) if e.is_transient() => {
                    // Try fallback providers
                    for provider_name in &self.fallback_chain {
                        if let Some(fallback) = self.providers.get(provider_name) {
                            if let Ok(response) = fallback.generate(prompt).await {
                                return Ok(response);
                            }
                        }
                    }
                }
                Err(e) => return Err(e),
            }
        }
        
        Err(QuizlrError::LlmError("All providers failed".to_string()))
    }
}
```

### Builder Pattern

```rust
let llm_manager = LlmManager::builder()
    .add_provider("claude", ClaudeClient::new(claude_key))
    .add_provider("gemini", GeminiClient::new(gemini_key))
    .add_provider("openai", OpenAIClient::new(openai_key))
    .default_provider("claude")
    .fallback_chain(vec!["gemini", "openai"])
    .with_cache(LlmCache::new(cache_size))
    .build()?;
```

## Prompt Engineering

### Template System

```rust
pub struct PromptTemplate {
    template: String,
    variables: Vec<String>,
    constraints: Vec<String>,
}

impl PromptTemplate {
    pub fn new(template: &str) -> Self {
        Self {
            template: template.to_string(),
            variables: Self::extract_variables(template),
            constraints: vec![],
        }
    }
    
    pub fn with_constraint(mut self, constraint: String) -> Self {
        self.constraints.push(constraint);
        self
    }
    
    pub fn render(&self, values: &HashMap<String, String>) -> Result<String, QuizlrError> {
        let mut result = self.template.clone();
        
        for var in &self.variables {
            let value = values.get(var)
                .ok_or_else(|| QuizlrError::InvalidInput(
                    format!("Missing variable: {}", var)
                ))?;
            result = result.replace(&format!("{{{}}}", var), value);
        }
        
        if !self.constraints.is_empty() {
            result.push_str("\n\nConstraints:\n");
            for constraint in &self.constraints {
                result.push_str(&format!("- {}\n", constraint));
            }
        }
        
        Ok(result)
    }
}
```

### Prompt Library

```rust
pub struct PromptLibrary {
    templates: HashMap<String, PromptTemplate>,
}

impl PromptLibrary {
    pub fn standard() -> Self {
        let mut templates = HashMap::new();
        
        templates.insert(
            "question_generation".to_string(),
            PromptTemplate::new(
                r#"Generate a {question_type} question about {topic}.
                Difficulty level: {difficulty}
                Target audience: {audience}
                
                Format the response as JSON with these fields:
                - question: The question text
                - options: Array of options (for multiple choice)
                - correct_answer: The correct answer
                - explanation: Why this answer is correct
                - learning_objectives: What this tests"#
            )
            .with_constraint("Ensure factual accuracy".to_string())
            .with_constraint("Make questions clear and unambiguous".to_string())
        );
        
        // Add more templates...
        
        Self { templates }
    }
}
```

## Performance Optimization

### Response Caching

```rust
pub struct LlmCache {
    store: Arc<RwLock<LruCache<String, CachedResponse>>>,
    ttl: Duration,
}

#[derive(Clone)]
struct CachedResponse {
    content: String,
    created_at: Instant,
    hit_count: u32,
}

impl LlmCache {
    pub fn get(&self, prompt_hash: &str) -> Option<String> {
        let mut store = self.store.write().unwrap();
        
        if let Some(cached) = store.get_mut(prompt_hash) {
            if cached.created_at.elapsed() < self.ttl {
                cached.hit_count += 1;
                return Some(cached.content.clone());
            }
        }
        
        None
    }
}
```

### Batch Processing

```rust
pub struct BatchProcessor {
    llm: Box<dyn LlmClient>,
    batch_size: usize,
    max_concurrent: usize,
}

impl BatchProcessor {
    pub async fn process_questions(
        &self,
        questions: Vec<QuestionParams>,
    ) -> Result<Vec<Question>, QuizlrError> {
        let chunks = questions.chunks(self.batch_size);
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));
        
        let futures = chunks.map(|chunk| {
            let sem = semaphore.clone();
            let llm = self.llm.clone();
            
            async move {
                let _permit = sem.acquire().await.unwrap();
                self.process_batch(&llm, chunk).await
            }
        });
        
        let results = futures::future::try_join_all(futures).await?;
        Ok(results.into_iter().flatten().collect())
    }
}
```

## Error Handling

### Retry Strategy

```rust
pub struct LlmRetryPolicy {
    max_attempts: u32,
    base_delay: Duration,
    max_delay: Duration,
    exponential_base: f32,
}

impl LlmRetryPolicy {
    pub async fn execute<F, T>(&self, operation: F) -> Result<T, QuizlrError>
    where
        F: Fn() -> futures::future::BoxFuture<'static, Result<T, QuizlrError>>,
    {
        let mut attempt = 0;
        let mut delay = self.base_delay;
        
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) if attempt >= self.max_attempts => return Err(e),
                Err(e) if e.is_rate_limit() => {
                    tokio::time::sleep(delay).await;
                    delay = (delay.as_secs_f32() * self.exponential_base)
                        .min(self.max_delay.as_secs_f32());
                    attempt += 1;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
```

## Best Practices

### Prompt Design

1. **Clear Instructions**: Be explicit about format and constraints
2. **Examples**: Include examples for complex tasks
3. **Validation**: Always validate LLM outputs
4. **Temperature**: Use lower values for consistency

### Cost Management

1. **Caching**: Cache common prompts and responses
2. **Model Selection**: Use appropriate models for each task
3. **Token Limits**: Set reasonable limits
4. **Monitoring**: Track usage and costs

### Safety

1. **Content Filtering**: Filter inappropriate content
2. **Injection Prevention**: Sanitize user inputs
3. **Output Validation**: Verify responses meet requirements
4. **Rate Limiting**: Prevent abuse

## Examples

### Complete Question Generation

```rust
use quizlr_core::llm::*;

// Setup
let llm_client = ClaudeClient::new(api_key)
    .with_model(ClaudeModel::Claude3Sonnet)
    .with_temperature(0.7);

let generator = QuestionGenerator::new(Box::new(llm_client));

// Generate a set of questions
let topics = vec!["Photosynthesis", "Cell Division", "DNA Replication"];
let mut questions = vec![];

for topic in topics {
    for difficulty in [0.3, 0.5, 0.7] {
        let params = QuestionParams {
            topic: topic.to_string(),
            question_type: QuestionType::MultipleChoice,
            difficulty,
            requirements: vec![
                "Include real-world application".to_string(),
                "Test conceptual understanding".to_string(),
            ],
        };
        
        match generator.generate_question(params).await {
            Ok(question) => questions.push(question),
            Err(e) => eprintln!("Failed to generate question: {}", e),
        }
    }
}

// Create quiz from generated questions
let quiz = QuizBuilder::new("Biology Fundamentals")
    .add_questions(questions)
    .build();
```

### Interactive Assessment

```rust
// Setup interview conductor
let conductor = InterviewConductor::new(llm_client);

// Start interview
let mut session = conductor.start_interview(
    "Machine Learning",
    "Can you explain what supervised learning is?"
).await?;

// Continue based on response
let student_response = "Supervised learning is when you train a model with labeled data...";

let follow_up = conductor.continue_interview(
    &mut session,
    student_response
).await?;

println!("Follow-up question: {}", follow_up.question);

// Evaluate final performance
let evaluation = conductor.evaluate_interview(&session).await?;
println!("Comprehension score: {:.1}%", evaluation.score * 100.0);
```
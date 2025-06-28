# Question Types

Quizlr supports seven different question types, each designed for specific learning scenarios. All question types share common properties but have unique validation and display requirements.

## Common Properties

Every question has these base properties:

```rust
pub struct Question {
    pub id: Uuid,
    pub question_type: QuestionType,
    pub topic_id: Uuid,
    pub difficulty: f32,        // 0.0 to 1.0
    pub estimated_time_seconds: u32,
    pub tags: Vec<String>,
    pub citations: Vec<Citation>,
    pub metadata: HashMap<String, Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## True/False

Simple binary choice questions.

```rust
QuestionType::TrueFalse {
    statement: String,
    correct_answer: bool,
    explanation: Option<String>,
}
```

**Example**:
```rust
QuestionType::TrueFalse {
    statement: "The Earth is flat".to_string(),
    correct_answer: false,
    explanation: Some("The Earth is an oblate spheroid".to_string()),
}
```

**Use Cases**:
- Fact checking
- Quick knowledge verification
- Warm-up questions

## Multiple Choice

Single selection from multiple options.

```rust
QuestionType::MultipleChoice {
    question: String,
    options: Vec<String>,
    correct_index: usize,
    explanation: Option<String>,
}
```

**Example**:
```rust
QuestionType::MultipleChoice {
    question: "What is the capital of France?".to_string(),
    options: vec![
        "London".to_string(),
        "Berlin".to_string(),
        "Paris".to_string(),
        "Madrid".to_string(),
    ],
    correct_index: 2,
    explanation: Some("Paris has been the capital since 987 AD".to_string()),
}
```

**Validation**:
- Ensures answer index is within bounds
- At least 2 options required

## Multi-Select

Multiple selections from a list of options.

```rust
QuestionType::MultiSelect {
    question: String,
    options: Vec<String>,
    correct_indices: Vec<usize>,
    explanation: Option<String>,
}
```

**Example**:
```rust
QuestionType::MultiSelect {
    question: "Which are primary colors?".to_string(),
    options: vec![
        "Red".to_string(),
        "Green".to_string(),
        "Blue".to_string(),
        "Yellow".to_string(),
    ],
    correct_indices: vec![0, 2], // Red and Blue
    explanation: Some("In additive color, RGB are primary".to_string()),
}
```

**Features**:
- Order-independent validation
- Partial credit possible

## Fill in the Blank

Text completion with one or more blanks.

```rust
QuestionType::FillInTheBlank {
    template: String,           // Contains {} for blanks
    correct_answers: Vec<String>,
    case_sensitive: bool,
    explanation: Option<String>,
}
```

**Example**:
```rust
QuestionType::FillInTheBlank {
    template: "The {} is the powerhouse of the {}".to_string(),
    correct_answers: vec!["mitochondria".to_string(), "cell".to_string()],
    case_sensitive: false,
    explanation: Some("Basic cell biology".to_string()),
}
```

**Features**:
- Multiple blanks support
- Case sensitivity option
- Exact match validation

## Match Pairs

Connect items from two lists.

```rust
QuestionType::MatchPairs {
    instruction: String,
    left_items: Vec<String>,
    right_items: Vec<String>,
    correct_pairs: Vec<(usize, usize)>,
    explanation: Option<String>,
}
```

**Example**:
```rust
QuestionType::MatchPairs {
    instruction: "Match countries with capitals".to_string(),
    left_items: vec!["France", "Germany", "Italy"],
    right_items: vec!["Berlin", "Rome", "Paris"],
    correct_pairs: vec![(0, 2), (1, 0), (2, 1)],
    explanation: None,
}
```

**Use Cases**:
- Vocabulary matching
- Concept associations
- Relationship mapping

## Interactive Interview

AI-powered conversational assessment.

```rust
QuestionType::InteractiveInterview {
    topic: String,
    initial_question: String,
    follow_up_rules: Vec<FollowUpRule>,
    comprehension_threshold: f32,
}

pub struct FollowUpRule {
    pub condition: String,
    pub follow_up_question: String,
    pub weight: f32,
}
```

**Example**:
```rust
QuestionType::InteractiveInterview {
    topic: "Photosynthesis".to_string(),
    initial_question: "Explain how photosynthesis works".to_string(),
    follow_up_rules: vec![
        FollowUpRule {
            condition: "mentions chlorophyll".to_string(),
            follow_up_question: "What role does chlorophyll play?".to_string(),
            weight: 0.8,
        },
    ],
    comprehension_threshold: 0.7,
}
```

**Features**:
- Dynamic follow-up questions
- LLM-powered evaluation
- Comprehension scoring

## Topic Explanation

Long-form explanatory responses.

```rust
QuestionType::TopicExplanation {
    topic: String,
    prompt: String,
    key_concepts: Vec<String>,
    min_word_count: usize,
}
```

**Example**:
```rust
QuestionType::TopicExplanation {
    topic: "Machine Learning".to_string(),
    prompt: "Explain supervised vs unsupervised learning".to_string(),
    key_concepts: vec![
        "labeled data".to_string(),
        "training".to_string(),
        "clustering".to_string(),
    ],
    min_word_count: 200,
}
```

**Evaluation**:
- Concept coverage check
- Length requirements
- LLM-assisted grading

## Answer Types

Each question type has a corresponding answer type:

```rust
pub enum Answer {
    TrueFalse(bool),
    MultipleChoice(usize),
    MultiSelect(Vec<usize>),
    FillInTheBlank(Vec<String>),
    MatchPairs(Vec<(usize, usize)>),
    InteractiveResponse {
        responses: Vec<String>,
        time_taken_seconds: u32,
    },
    TopicExplanation {
        explanation: String,
        time_taken_seconds: u32,
    },
}
```

## Best Practices

### Choosing Question Types

1. **Knowledge Check**: True/False, Multiple Choice
2. **Deeper Understanding**: Multi-Select, Fill in the Blank
3. **Relationships**: Match Pairs
4. **Comprehension**: Interactive Interview, Topic Explanation

### Writing Good Questions

1. **Clear and Concise**: Avoid ambiguity
2. **Single Concept**: Test one thing at a time
3. **Appropriate Difficulty**: Match target audience
4. **Good Distractors**: Make wrong answers plausible
5. **Helpful Explanations**: Teach, don't just test

### Difficulty Guidelines

- **0.0 - 0.3**: Basic recall, definitions
- **0.3 - 0.6**: Application, understanding
- **0.6 - 0.8**: Analysis, synthesis
- **0.8 - 1.0**: Expert level, edge cases

## Future Question Types

Planned for future releases:

- **Code Completion**: Programming exercises
- **Diagram Labeling**: Visual learning
- **Sequence Ordering**: Process understanding
- **Calculation**: Numerical problems
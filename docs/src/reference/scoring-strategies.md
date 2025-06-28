# Scoring Strategies

Quizlr provides a flexible scoring system with multiple strategies to evaluate quiz performance. Each strategy emphasizes different aspects of learning assessment.

## Architecture

The scoring system is built around the `ScoringStrategy` enum and produces a comprehensive `Score` struct:

```rust
pub struct Score {
    pub raw_score: f32,         // Basic correctness (0.0 to 1.0)
    pub weighted_score: f32,    // Strategy-adjusted score
    pub percentile: Option<f32>, // Relative performance
    pub time_bonus: f32,        // Speed bonus/penalty
    pub difficulty_bonus: f32,   // Difficulty adjustment
    pub streak_bonus: f32,      // Consecutive correct bonus
    pub components: ScoreComponents,
}

pub struct ScoreComponents {
    pub correctness: f32,    // Raw accuracy
    pub speed: f32,          // Time efficiency
    pub difficulty: f32,     // Difficulty mastery
    pub consistency: f32,    // Response uniformity
}
```

## Available Strategies

### Simple Scoring

Basic correct/incorrect scoring without any modifiers.

```rust
ScoringStrategy::Simple
```

**Algorithm**:
```
score = correct_answers / total_questions
```

**Use Cases**:
- Quick assessments
- Pass/fail evaluations
- Learning checkpoints

**Example**:
```rust
let strategy = ScoringStrategy::Simple;
let score = strategy.calculate_score(&session, &questions);
println!("You got {:.0}% correct", score.raw_score * 100.0);
```

### Time-Weighted Scoring

Factors response time into the final score, rewarding quick accurate answers.

```rust
ScoringStrategy::TimeWeighted {
    base_time_seconds: u32,      // Expected time per question
    penalty_per_second: f32,     // Score reduction per extra second
}
```

**Algorithm**:
```
for each question:
    base_points = is_correct ? 1.0 : 0.0
    time_penalty = max(0, time_taken - base_time) * penalty_per_second
    points = max(0, base_points - time_penalty)

score = total_points / question_count
```

**Use Cases**:
- Timed assessments
- Competitive quizzes
- Skill proficiency tests

**Example**:
```rust
let strategy = ScoringStrategy::TimeWeighted {
    base_time_seconds: 30,
    penalty_per_second: 0.01,
};
// A correct answer in 45 seconds loses 0.15 points
```

### Difficulty-Weighted Scoring

Rewards correct answers to harder questions more than easier ones.

```rust
ScoringStrategy::DifficultyWeighted {
    easy_multiplier: f32,     // Points for easy (< 0.33)
    medium_multiplier: f32,   // Points for medium (0.33-0.67)
    hard_multiplier: f32,     // Points for hard (> 0.67)
}
```

**Algorithm**:
```
total_score = sum(correct_answer ? difficulty_multiplier : 0)
max_possible = sum(all_difficulty_multipliers)
score = total_score / max_possible
```

**Use Cases**:
- Comprehensive assessments
- Skill level determination
- Advanced topic mastery

**Example**:
```rust
let strategy = ScoringStrategy::DifficultyWeighted {
    easy_multiplier: 1.0,
    medium_multiplier: 1.5,
    hard_multiplier: 2.0,
};
// Hard questions worth twice as much as easy ones
```

### Adaptive Scoring

Comprehensive scoring combining multiple factors for nuanced assessment.

```rust
ScoringStrategy::Adaptive {
    time_weight: f32,         // Importance of speed (0.0-1.0)
    difficulty_weight: f32,   // Importance of difficulty
    streak_weight: f32,       // Importance of consistency
    consistency_weight: f32,  // Importance of uniform timing
}
```

**Algorithm**:
```
correctness_score = correct / total

time_score = expected_avg_time / actual_avg_time (capped at 1.0)

difficulty_score = sum(correct_difficulties) / sum(all_difficulties)

streak_score = longest_correct_streak / total_questions

consistency_score = 1 / (1 + coefficient_of_variation)

weighted_score = (
    correctness_score + 
    time_score * time_weight +
    difficulty_score * difficulty_weight +
    streak_score * streak_weight +
    consistency_score * consistency_weight
) / (1 + sum_of_weights)
```

**Use Cases**:
- Comprehensive evaluations
- Learning analytics
- Personalized feedback

**Example**:
```rust
let strategy = ScoringStrategy::Adaptive {
    time_weight: 0.2,
    difficulty_weight: 0.3,
    streak_weight: 0.1,
    consistency_weight: 0.1,
};
```

## Implementation Details

### Score Calculation

All strategies implement the same interface:

```rust
impl ScoringStrategy {
    pub fn calculate_score(
        &self,
        session: &QuizSession,
        questions: &[Question],
    ) -> Score;
}
```

### Performance Considerations

- **O(n) complexity**: All strategies iterate through responses once
- **Memory efficient**: No additional allocations beyond result struct
- **Thread safe**: All calculations are immutable
- **Deterministic**: Same inputs always produce same outputs

### Custom Scoring

To implement a custom scoring strategy:

1. Extend the `ScoringStrategy` enum:
```rust
ScoringStrategy::Custom {
    // Your parameters
}
```

2. Add calculation logic in `calculate_score`:
```rust
ScoringStrategy::Custom { params } => {
    self.custom_score(session, questions, params)
}
```

3. Implement the scoring algorithm:
```rust
fn custom_score(&self, session: &QuizSession, questions: &[Question], params: CustomParams) -> Score {
    // Your scoring logic
}
```

## Usage Patterns

### Basic Usage

```rust
// Simple scoring
let score = ScoringStrategy::Simple.calculate_score(&session, &questions);

// Time-based scoring
let score = ScoringStrategy::TimeWeighted {
    base_time_seconds: 45,
    penalty_per_second: 0.02,
}.calculate_score(&session, &questions);
```

### Score Analysis

```rust
let score = strategy.calculate_score(&session, &questions);

// Overall performance
println!("Final score: {:.1}%", score.weighted_score * 100.0);

// Component breakdown
println!("Correctness: {:.1}%", score.components.correctness * 100.0);
println!("Speed bonus: {:.1}%", score.components.speed * 100.0);
println!("Difficulty bonus: {:.1}%", score.components.difficulty * 100.0);
println!("Consistency: {:.1}%", score.components.consistency * 100.0);

// Specific bonuses
println!("Time bonus: +{:.1}%", score.time_bonus * 100.0);
println!("Streak bonus: +{:.1}%", score.streak_bonus * 100.0);
```

### Strategy Selection

Choose strategies based on assessment goals:

```rust
let strategy = match quiz_type {
    QuizType::Practice => ScoringStrategy::Simple,
    QuizType::Timed => ScoringStrategy::TimeWeighted {
        base_time_seconds: 30,
        penalty_per_second: 0.01,
    },
    QuizType::Certification => ScoringStrategy::DifficultyWeighted {
        easy_multiplier: 0.8,
        medium_multiplier: 1.0,
        hard_multiplier: 1.5,
    },
    QuizType::Comprehensive => ScoringStrategy::Adaptive {
        time_weight: 0.15,
        difficulty_weight: 0.35,
        streak_weight: 0.10,
        consistency_weight: 0.10,
    },
};
```

## Best Practices

### Strategy Configuration

1. **Time Weights**: Keep penalties reasonable (0.01-0.05 per second)
2. **Difficulty Multipliers**: Use progressive scaling (1.0, 1.5, 2.0)
3. **Adaptive Weights**: Ensure weights sum to less than 1.0
4. **Consistency**: Use same strategy across related assessments

### Score Interpretation

1. **Raw vs Weighted**: Always show both scores for transparency
2. **Component Analysis**: Break down scores for detailed feedback
3. **Percentiles**: Compare against historical data when available
4. **Thresholds**: Set clear pass/fail criteria

### Performance Optimization

1. **Cache Calculations**: Store intermediate results
2. **Batch Processing**: Calculate multiple sessions together
3. **Async Scoring**: Use async for large datasets
4. **Incremental Updates**: Update scores as responses come in

## Examples

### Complete Scoring Example

```rust
use quizlr_core::quiz::*;

// Create quiz and session
let quiz = QuizBuilder::new("Math Fundamentals")
    .pass_threshold(0.7)
    .build();

let mut session = QuizSession::new(quiz.id, Some(user_id));
session.start().unwrap();

// Submit answers
for (i, question) in quiz.questions.iter().enumerate() {
    let answer = get_user_answer(question);
    session.submit_answer(question, answer, time_taken).unwrap();
}

// Calculate scores with different strategies
let strategies = vec![
    ("Simple", ScoringStrategy::Simple),
    ("Time-Based", ScoringStrategy::TimeWeighted {
        base_time_seconds: 60,
        penalty_per_second: 0.02,
    }),
    ("Difficulty-Based", ScoringStrategy::DifficultyWeighted {
        easy_multiplier: 1.0,
        medium_multiplier: 1.5,
        hard_multiplier: 2.0,
    }),
    ("Adaptive", ScoringStrategy::Adaptive {
        time_weight: 0.2,
        difficulty_weight: 0.3,
        streak_weight: 0.1,
        consistency_weight: 0.1,
    }),
];

for (name, strategy) in strategies {
    let score = strategy.calculate_score(&session, &quiz.questions);
    println!("{}: {:.1}%", name, score.weighted_score * 100.0);
}
```

### Custom Analysis

```rust
fn analyze_performance(session: &QuizSession, questions: &[Question]) {
    let score = ScoringStrategy::Adaptive {
        time_weight: 0.25,
        difficulty_weight: 0.35,
        streak_weight: 0.15,
        consistency_weight: 0.15,
    }.calculate_score(session, questions);
    
    // Performance report
    println!("Performance Analysis");
    println!("===================");
    println!("Overall: {:.1}%", score.weighted_score * 100.0);
    println!();
    
    // Strengths and weaknesses
    let components = &score.components;
    let mut strengths = vec![];
    let mut weaknesses = vec![];
    
    if components.correctness > 0.8 { strengths.push("Accuracy"); }
    else if components.correctness < 0.6 { weaknesses.push("Accuracy"); }
    
    if components.speed > 0.8 { strengths.push("Speed"); }
    else if components.speed < 0.6 { weaknesses.push("Speed"); }
    
    if components.difficulty > 0.8 { strengths.push("Challenging questions"); }
    else if components.difficulty < 0.6 { weaknesses.push("Challenging questions"); }
    
    if components.consistency > 0.8 { strengths.push("Consistency"); }
    else if components.consistency < 0.6 { weaknesses.push("Consistency"); }
    
    println!("Strengths: {}", strengths.join(", "));
    println!("Areas for improvement: {}", weaknesses.join(", "));
}
```
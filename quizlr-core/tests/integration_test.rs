use quizlr_core::quiz::{Answer, Question, QuestionType, QuizBuilder, QuizSession};
use quizlr_core::quiz::{ScoringStrategy, SessionState};
use uuid::Uuid;

#[test]
fn test_complete_quiz_workflow() {
    // Create a quiz with multiple question types
    let topic_id = Uuid::new_v4();

    let questions = vec![
        Question::new(
            QuestionType::TrueFalse {
                statement: "Rust is memory safe".to_string(),
                correct_answer: true,
                explanation: Some("Rust prevents memory errors at compile time".to_string()),
            },
            topic_id,
            0.3,
        ),
        Question::new(
            QuestionType::MultipleChoice {
                question: "What is Rust's package manager?".to_string(),
                options: vec![
                    "npm".to_string(),
                    "pip".to_string(),
                    "cargo".to_string(),
                    "gem".to_string(),
                ],
                correct_index: 2,
                explanation: Some("Cargo is Rust's build system and package manager".to_string()),
            },
            topic_id,
            0.4,
        ),
        Question::new(
            QuestionType::MultiSelect {
                question: "Which are Rust's zero-cost abstractions?".to_string(),
                options: vec![
                    "Traits".to_string(),
                    "Garbage Collection".to_string(),
                    "Iterators".to_string(),
                    "Pattern Matching".to_string(),
                ],
                correct_indices: vec![0, 2, 3],
                explanation: Some(
                    "Traits, iterators, and pattern matching have no runtime overhead".to_string(),
                ),
            },
            topic_id,
            0.6,
        ),
    ];

    let quiz = QuizBuilder::new("Rust Fundamentals".to_string())
        .description("Test your knowledge of Rust basics".to_string())
        .pass_threshold(0.7)
        .show_explanations(true)
        .add_questions(questions.clone())
        .build();

    // Start a quiz session
    let mut session = QuizSession::new(quiz.id, Some(Uuid::new_v4()));
    assert_eq!(session.state, SessionState::NotStarted);

    session.start().unwrap();
    assert_eq!(session.state, SessionState::InProgress);

    // Answer questions
    let correct1 = session
        .submit_answer(&questions[0], Answer::TrueFalse(true), 15)
        .unwrap();
    assert!(correct1);

    let correct2 = session
        .submit_answer(&questions[1], Answer::MultipleChoice(2), 20)
        .unwrap();
    assert!(correct2);

    let correct3 = session
        .submit_answer(&questions[2], Answer::MultiSelect(vec![0, 2, 3]), 30)
        .unwrap();
    assert!(correct3);

    // Complete the session
    let summary = session.complete().unwrap();
    assert_eq!(summary.correct_answers, 3);
    assert_eq!(summary.total_questions, 3);
    assert_eq!(summary.score, 1.0);
    assert!(summary.passed(0.7));

    // Test different scoring strategies
    let simple_score = ScoringStrategy::Simple.calculate_score(&session, &questions);
    assert_eq!(simple_score.raw_score, 1.0);

    let time_weighted = ScoringStrategy::TimeWeighted {
        base_time_seconds: 30,
        penalty_per_second: 0.01,
    };
    let time_score = time_weighted.calculate_score(&session, &questions);
    assert!(time_score.weighted_score <= 1.0);

    let difficulty_weighted = ScoringStrategy::DifficultyWeighted {
        easy_multiplier: 1.0,
        medium_multiplier: 1.5,
        hard_multiplier: 2.0,
    };
    let diff_score = difficulty_weighted.calculate_score(&session, &questions);
    assert!(diff_score.weighted_score > 0.0);
}

#[test]
fn test_session_pause_resume() {
    let quiz_id = Uuid::new_v4();
    let mut session = QuizSession::new(quiz_id, None);

    session.start().unwrap();
    assert_eq!(session.state, SessionState::InProgress);

    // Pause the session
    session.pause().unwrap();
    assert_eq!(session.state, SessionState::Paused);

    // Try to submit answer while paused (should fail)
    let question = Question::new(
        QuestionType::TrueFalse {
            statement: "Test".to_string(),
            correct_answer: true,
            explanation: None,
        },
        Uuid::new_v4(),
        0.5,
    );

    let result = session.submit_answer(&question, Answer::TrueFalse(true), 10);
    assert!(result.is_err());

    // Resume the session
    session.resume().unwrap();
    assert_eq!(session.state, SessionState::InProgress);

    // Now submission should work
    let result = session.submit_answer(&question, Answer::TrueFalse(true), 10);
    assert!(result.is_ok());
}

#[test]
fn test_skip_questions() {
    let quiz_id = Uuid::new_v4();
    let mut session = QuizSession::new(quiz_id, None);

    session.start().unwrap();

    // Skip some questions
    session.skip_question(0);
    session.skip_question(2);

    assert_eq!(session.skipped_questions.len(), 2);
    assert!(session.skipped_questions.contains(&0));
    assert!(session.skipped_questions.contains(&2));

    // Complete session with skipped questions
    let summary = session.complete().unwrap();
    assert_eq!(summary.skipped_questions, 2);
    assert_eq!(summary.total_questions, 2); // Only skipped questions
    assert_eq!(summary.completion_rate, 0.0); // No questions answered
}

#[test]
fn test_question_navigation() {
    let quiz_id = Uuid::new_v4();
    let mut session = QuizSession::new(quiz_id, None);

    session.start().unwrap();
    assert_eq!(session.current_question_index, 0);

    // Navigate forward
    session.next_question().unwrap();
    assert_eq!(session.current_question_index, 1);

    session.next_question().unwrap();
    assert_eq!(session.current_question_index, 2);

    // Navigate backward
    session.previous_question().unwrap();
    assert_eq!(session.current_question_index, 1);

    session.previous_question().unwrap();
    assert_eq!(session.current_question_index, 0);

    // Try to go before first question
    let result = session.previous_question();
    assert!(result.is_err());
}

#[test]
fn test_answer_validation_errors() {
    let question = Question::new(
        QuestionType::MultipleChoice {
            question: "Test".to_string(),
            options: vec!["A".to_string(), "B".to_string()],
            correct_index: 0,
            explanation: None,
        },
        Uuid::new_v4(),
        0.5,
    );

    // Wrong answer type
    let result = question.validate_answer(&Answer::TrueFalse(true));
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Answer type does not match question type"
    );

    // Invalid index
    let result = question.validate_answer(&Answer::MultipleChoice(5));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid option index");
}

#[test]
fn test_quiz_builder_configuration() {
    let quiz = QuizBuilder::new("Test Quiz".to_string())
        .description("A comprehensive test quiz".to_string())
        .pass_threshold(0.85)
        .allow_skip(true)
        .show_explanations(false)
        .randomize_questions(true)
        .randomize_options(true)
        .add_tag("test".to_string())
        .add_tag("integration".to_string())
        .add_metadata("difficulty".to_string(), serde_json::json!("medium"))
        .build();

    assert_eq!(quiz.title, "Test Quiz");
    assert_eq!(
        quiz.description,
        Some("A comprehensive test quiz".to_string())
    );
    assert_eq!(quiz.pass_threshold, 0.85);
    assert!(quiz.allow_skip);
    assert!(!quiz.show_explanations);
    assert!(quiz.randomize_questions);
    assert!(quiz.randomize_options);
    assert_eq!(quiz.tags.len(), 2);
    assert!(quiz.metadata.contains_key("difficulty"));
}

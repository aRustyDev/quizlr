//! Comprehensive tests for quiz management
//!
//! DEVNOTES: Testing quiz creation, modification, and configuration
//! to ensure proper quiz lifecycle management

use crate::quiz::question::{Question, QuestionType};
use crate::quiz::quiz_impl::{Quiz, QuizBuilder};
use uuid::Uuid;

#[cfg(test)]
mod quiz_management_tests {
    use super::*;

    fn create_sample_question(difficulty: f32) -> Question {
        Question::new(
            QuestionType::TrueFalse {
                statement: format!("Test question with difficulty {}", difficulty),
                correct_answer: true,
                explanation: None,
            },
            Uuid::new_v4(),
            difficulty,
        )
    }

    #[test]
    fn test_quiz_creation_defaults() {
        // Test default values on quiz creation
        let quiz = Quiz::new("Test Quiz".to_string());

        assert_eq!(quiz.title, "Test Quiz");
        assert_eq!(quiz.description, None);
        assert_eq!(quiz.questions.len(), 0);
        assert_eq!(quiz.topic_ids.len(), 0);
        assert_eq!(quiz.difficulty_range, (0.0, 1.0));
        assert_eq!(quiz.estimated_duration_minutes, 30);
        assert_eq!(quiz.pass_threshold, 0.7);
        assert!(quiz.allow_skip);
        assert!(quiz.show_explanations);
        assert!(!quiz.randomize_questions);
        assert!(!quiz.randomize_options);
    }

    #[test]
    fn test_quiz_builder_fluent_api() {
        // Test the builder pattern
        let quiz = QuizBuilder::new("Advanced Rust Quiz".to_string())
            .description("Test your knowledge of advanced Rust concepts".to_string())
            .pass_threshold(0.85)
            .allow_skip(false)
            .show_explanations(false)
            .randomize_questions(true)
            .randomize_options(true)
            .add_tag("rust".to_string())
            .add_tag("advanced".to_string())
            .add_metadata(
                "difficulty_level".to_string(),
                serde_json::Value::String("expert".to_string()),
            )
            .build();

        assert_eq!(quiz.title, "Advanced Rust Quiz");
        assert_eq!(
            quiz.description,
            Some("Test your knowledge of advanced Rust concepts".to_string())
        );
        assert_eq!(quiz.pass_threshold, 0.85);
        assert!(!quiz.allow_skip);
        assert!(!quiz.show_explanations);
        assert!(quiz.randomize_questions);
        assert!(quiz.randomize_options);
        assert_eq!(quiz.tags.len(), 2);
        assert!(quiz.tags.contains(&"rust".to_string()));
        assert!(quiz.tags.contains(&"advanced".to_string()));
        assert_eq!(
            quiz.metadata.get("difficulty_level"),
            Some(&serde_json::Value::String("expert".to_string()))
        );
    }

    #[test]
    fn test_pass_threshold_clamping() {
        // Test that pass threshold is clamped between 0.0 and 1.0
        let quiz1 = QuizBuilder::new("Quiz 1".to_string())
            .pass_threshold(1.5)
            .build();
        assert_eq!(quiz1.pass_threshold, 1.0);

        let quiz2 = QuizBuilder::new("Quiz 2".to_string())
            .pass_threshold(-0.5)
            .build();
        assert_eq!(quiz2.pass_threshold, 0.0);
    }

    #[test]
    fn test_add_questions_updates_metadata() {
        // Test that adding questions updates quiz metadata
        let mut quiz = Quiz::new("Dynamic Quiz".to_string());
        let topic_id1 = Uuid::new_v4();
        let topic_id2 = Uuid::new_v4();

        let q1 = create_sample_question(0.3);
        let mut q2 = create_sample_question(0.7);
        q2.topic_id = topic_id1;
        q2.estimated_time_seconds = 120;

        let mut q3 = create_sample_question(0.5);
        q3.topic_id = topic_id2;
        q3.estimated_time_seconds = 90;

        quiz.add_question(q1);
        assert_eq!(quiz.questions.len(), 1);
        assert_eq!(quiz.difficulty_range, (0.3, 0.3));
        assert_eq!(quiz.estimated_duration_minutes, 1); // 60s / 60
                                                        // First question has default topic ID, so we have 1 topic
        assert_eq!(quiz.topic_ids.len(), 1);

        quiz.add_question(q2);
        assert_eq!(quiz.questions.len(), 2);
        assert_eq!(quiz.difficulty_range, (0.3, 0.7));
        assert_eq!(quiz.estimated_duration_minutes, 3); // 180s / 60
        assert!(quiz.topic_ids.contains(&topic_id1));
        // Now we have 2 topics (default + topic_id1)
        assert_eq!(quiz.topic_ids.len(), 2);

        quiz.add_question(q3);
        assert_eq!(quiz.questions.len(), 3);
        assert_eq!(quiz.difficulty_range, (0.3, 0.7));
        assert_eq!(quiz.estimated_duration_minutes, 4); // 270s / 60
        assert!(quiz.topic_ids.contains(&topic_id2));
        // Now we have 3 topics (default + topic_id1 + topic_id2)
        assert_eq!(quiz.topic_ids.len(), 3);
    }

    #[test]
    fn test_remove_question() {
        // Test removing questions
        let mut quiz = Quiz::new("Test Quiz".to_string());
        let q1 = create_sample_question(0.2);
        let q2 = create_sample_question(0.8);
        let q3 = create_sample_question(0.5);

        let q1_id = q1.id;
        let q2_id = q2.id;

        quiz.add_question(q1);
        quiz.add_question(q2);
        quiz.add_question(q3);

        assert_eq!(quiz.questions.len(), 3);
        assert_eq!(quiz.difficulty_range, (0.2, 0.8));

        // Remove middle question
        let removed = quiz.remove_question(q2_id);
        assert!(removed.is_some());
        assert_eq!(quiz.questions.len(), 2);
        assert_eq!(quiz.difficulty_range, (0.2, 0.5));

        // Try to remove non-existent question
        let not_found = quiz.remove_question(Uuid::new_v4());
        assert!(not_found.is_none());

        // Remove first question
        quiz.remove_question(q1_id);
        assert_eq!(quiz.questions.len(), 1);
        assert_eq!(quiz.difficulty_range, (0.5, 0.5));
    }

    #[test]
    fn test_empty_quiz_difficulty_range() {
        // Test difficulty range for empty quiz
        let mut quiz = Quiz::new("Empty Quiz".to_string());
        let q = create_sample_question(0.5);
        let q_id = q.id;

        quiz.add_question(q);
        quiz.remove_question(q_id);

        assert_eq!(quiz.questions.len(), 0);
        assert_eq!(quiz.difficulty_range, (0.0, 1.0)); // Reset to defaults
    }

    #[test]
    fn test_quiz_builder_with_questions() {
        // Test adding multiple questions via builder
        let questions = vec![
            create_sample_question(0.3),
            create_sample_question(0.5),
            create_sample_question(0.7),
        ];

        let quiz = QuizBuilder::new("Multi-Question Quiz".to_string())
            .add_questions(questions.clone())
            .build();

        assert_eq!(quiz.questions.len(), 3);
        assert_eq!(quiz.difficulty_range, (0.3, 0.7));
    }

    #[test]
    fn test_duplicate_tags() {
        // Test that duplicate tags are not added
        let quiz = QuizBuilder::new("Tagged Quiz".to_string())
            .add_tag("rust".to_string())
            .add_tag("rust".to_string())
            .add_tag("programming".to_string())
            .build();

        assert_eq!(quiz.tags.len(), 2);
        assert!(quiz.tags.contains(&"rust".to_string()));
        assert!(quiz.tags.contains(&"programming".to_string()));
    }

    #[test]
    fn test_get_questions_for_session_no_randomization() {
        // Test getting questions without randomization
        let mut quiz = Quiz::new("Ordered Quiz".to_string());

        for i in 0..5 {
            let mut q = create_sample_question(0.5);
            q.id = Uuid::from_u128(i as u128); // Predictable IDs for testing
            quiz.add_question(q);
        }

        let session_questions = quiz.get_questions_for_session();
        assert_eq!(session_questions.len(), 5);

        // Verify order is preserved
        for (i, q) in session_questions.iter().enumerate() {
            assert_eq!(q.id, Uuid::from_u128(i as u128));
        }
    }

    #[test]
    fn test_get_questions_for_session_with_randomization() {
        // Test that randomization flag works
        let mut quiz = Quiz::new("Random Quiz".to_string());
        quiz.randomize_questions = true;

        // Add enough questions to make random order likely different
        for i in 0..20 {
            quiz.add_question(create_sample_question((i as f32) / 20.0));
        }

        let session_questions = quiz.get_questions_for_session();
        assert_eq!(session_questions.len(), 20);

        // Note: We can't test actual randomization without controlling the RNG
        // but we can verify the function runs without error
    }

    #[test]
    fn test_updated_timestamp() {
        use std::thread;
        use std::time::Duration;

        let mut quiz = Quiz::new("Timestamp Quiz".to_string());
        let initial_updated = quiz.updated_at;

        // Small delay to ensure timestamp difference
        thread::sleep(Duration::from_millis(10));

        quiz.add_question(create_sample_question(0.5));
        assert!(quiz.updated_at > initial_updated);

        let after_add = quiz.updated_at;
        thread::sleep(Duration::from_millis(10));

        quiz.remove_question(quiz.questions[0].id);
        assert!(quiz.updated_at > after_add);
    }
}

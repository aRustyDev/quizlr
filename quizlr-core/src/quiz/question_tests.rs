//! Comprehensive tests for question types and validation
//! 
//! DEVNOTES: Testing all question types and edge cases to ensure
//! proper validation and behavior across the quiz engine

use crate::quiz::question::{Question, QuestionType, Answer, Citation, FollowUpRule};
use uuid::Uuid;

#[cfg(test)]
mod question_type_tests {
    use super::*;

    #[test]
    fn test_true_false_correct_validation() {
        // Test correct answer validation
        let question = Question::new(
            QuestionType::TrueFalse {
                statement: "Rust has a garbage collector".to_string(),
                correct_answer: false,
                explanation: Some("Rust uses ownership system instead".to_string()),
            },
            Uuid::new_v4(),
            0.4,
        );

        assert_eq!(
            question.validate_answer(&Answer::TrueFalse(false)).unwrap(),
            true
        );
        assert_eq!(
            question.validate_answer(&Answer::TrueFalse(true)).unwrap(),
            false
        );
    }

    #[test]
    fn test_multiple_choice_boundary_validation() {
        // Test boundary conditions for multiple choice
        let question = Question::new(
            QuestionType::MultipleChoice {
                question: "Which is a Rust keyword?".to_string(),
                options: vec![
                    "var".to_string(),
                    "let".to_string(),
                    "const".to_string(),
                ],
                correct_index: 1,
                explanation: None,
            },
            Uuid::new_v4(),
            0.3,
        );

        // Valid indices
        assert!(question.validate_answer(&Answer::MultipleChoice(0)).is_ok());
        assert!(question.validate_answer(&Answer::MultipleChoice(1)).is_ok());
        assert!(question.validate_answer(&Answer::MultipleChoice(2)).is_ok());

        // Invalid index
        let result = question.validate_answer(&Answer::MultipleChoice(3));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid option index");
    }

    #[test]
    fn test_multi_select_order_independence() {
        // Test that order doesn't matter for multi-select
        let question = Question::new(
            QuestionType::MultiSelect {
                question: "Which are Rust's zero-cost abstractions?".to_string(),
                options: vec![
                    "Traits".to_string(),
                    "Garbage Collection".to_string(),
                    "Iterators".to_string(),
                    "Reflection".to_string(),
                ],
                correct_indices: vec![0, 2],
                explanation: Some("Traits and Iterators are zero-cost".to_string()),
            },
            Uuid::new_v4(),
            0.6,
        );

        // Same indices, different order
        assert!(question.validate_answer(&Answer::MultiSelect(vec![0, 2])).unwrap());
        assert!(question.validate_answer(&Answer::MultiSelect(vec![2, 0])).unwrap());
        
        // Wrong indices
        assert!(!question.validate_answer(&Answer::MultiSelect(vec![1, 3])).unwrap());
        assert!(!question.validate_answer(&Answer::MultiSelect(vec![0])).unwrap());
    }

    #[test]
    fn test_fill_in_blank_case_sensitivity() {
        // Test case sensitive validation
        let case_sensitive = Question::new(
            QuestionType::FillInTheBlank {
                template: "The {} macro is used for printing in Rust".to_string(),
                correct_answers: vec!["println!".to_string()],
                case_sensitive: true,
                explanation: None,
            },
            Uuid::new_v4(),
            0.2,
        );

        assert!(case_sensitive
            .validate_answer(&Answer::FillInTheBlank(vec!["println!".to_string()]))
            .unwrap());
        assert!(!case_sensitive
            .validate_answer(&Answer::FillInTheBlank(vec!["PRINTLN!".to_string()]))
            .unwrap());

        // Test case insensitive
        let case_insensitive = Question::new(
            QuestionType::FillInTheBlank {
                template: "The {} keyword declares a variable".to_string(),
                correct_answers: vec!["let".to_string()],
                case_sensitive: false,
                explanation: None,
            },
            Uuid::new_v4(),
            0.2,
        );

        assert!(case_insensitive
            .validate_answer(&Answer::FillInTheBlank(vec!["let".to_string()]))
            .unwrap());
        assert!(case_insensitive
            .validate_answer(&Answer::FillInTheBlank(vec!["LET".to_string()]))
            .unwrap());
        assert!(case_insensitive
            .validate_answer(&Answer::FillInTheBlank(vec!["Let".to_string()]))
            .unwrap());
    }

    #[test]
    fn test_fill_in_blank_multiple_blanks() {
        // Test multiple blanks
        let question = Question::new(
            QuestionType::FillInTheBlank {
                template: "{} is to Rust as {} is to JavaScript".to_string(),
                correct_answers: vec!["cargo".to_string(), "npm".to_string()],
                case_sensitive: false,
                explanation: None,
            },
            Uuid::new_v4(),
            0.4,
        );

        // Correct answers
        assert!(question
            .validate_answer(&Answer::FillInTheBlank(vec![
                "cargo".to_string(),
                "npm".to_string()
            ]))
            .unwrap());

        // Wrong number of answers
        let result = question.validate_answer(&Answer::FillInTheBlank(vec!["cargo".to_string()]));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Wrong number of answers");
    }

    #[test]
    fn test_match_pairs_validation() {
        // Test match pairs with various combinations
        let question = Question::new(
            QuestionType::MatchPairs {
                instruction: "Match Rust concepts with descriptions".to_string(),
                left_items: vec![
                    "ownership".to_string(),
                    "borrowing".to_string(),
                    "lifetimes".to_string(),
                ],
                right_items: vec![
                    "temporary access".to_string(),
                    "memory safety".to_string(),
                    "reference validity".to_string(),
                ],
                correct_pairs: vec![(0, 1), (1, 0), (2, 2)],
                explanation: None,
            },
            Uuid::new_v4(),
            0.5,
        );

        // Correct pairing (order doesn't matter)
        assert!(question
            .validate_answer(&Answer::MatchPairs(vec![(0, 1), (1, 0), (2, 2)]))
            .unwrap());
        assert!(question
            .validate_answer(&Answer::MatchPairs(vec![(2, 2), (0, 1), (1, 0)]))
            .unwrap());

        // Incorrect pairing
        assert!(!question
            .validate_answer(&Answer::MatchPairs(vec![(0, 0), (1, 1), (2, 2)]))
            .unwrap());
    }

    #[test]
    fn test_wrong_answer_type() {
        // Test mismatched answer types
        let question = Question::new(
            QuestionType::TrueFalse {
                statement: "Test".to_string(),
                correct_answer: true,
                explanation: None,
            },
            Uuid::new_v4(),
            0.5,
        );

        let result = question.validate_answer(&Answer::MultipleChoice(0));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Answer type does not match question type");
    }

    #[test]
    fn test_question_metadata() {
        // Test question metadata and properties
        let mut question = Question::new(
            QuestionType::TrueFalse {
                statement: "Test".to_string(),
                correct_answer: true,
                explanation: Some("Test explanation".to_string()),
            },
            Uuid::new_v4(),
            0.7,
        );

        // Add tags
        question.tags.push("rust".to_string());
        question.tags.push("basics".to_string());

        // Add metadata
        question.metadata.insert(
            "category".to_string(),
            serde_json::Value::String("language-features".to_string()),
        );

        assert_eq!(question.tags.len(), 2);
        assert!(question.tags.contains(&"rust".to_string()));
        assert_eq!(question.difficulty, 0.7);
        assert_eq!(question.estimated_time_seconds, 60);
        assert_eq!(
            question.get_explanation(),
            Some("Test explanation")
        );
    }

    #[test]
    fn test_citation_creation() {
        // Test citation handling
        let mut question = Question::new(
            QuestionType::TrueFalse {
                statement: "Rust was first released in 2015".to_string(),
                correct_answer: true,
                explanation: None,
            },
            Uuid::new_v4(),
            0.3,
        );

        let citation = Citation {
            id: Uuid::new_v4(),
            source: "The Rust Programming Language".to_string(),
            url: Some("https://doc.rust-lang.org/book/".to_string()),
            excerpt: Some("Rust 1.0 was released in May 2015".to_string()),
            confidence: 0.95,
        };

        question.citations.push(citation.clone());

        assert_eq!(question.citations.len(), 1);
        assert_eq!(question.citations[0].source, "The Rust Programming Language");
        assert_eq!(question.citations[0].confidence, 0.95);
    }

    #[test]
    fn test_interactive_interview_type() {
        // Test the interactive interview question type structure
        let follow_up_rules = vec![
            FollowUpRule {
                condition: "mentions ownership".to_string(),
                follow_up_question: "Can you explain how borrowing relates to ownership?".to_string(),
                weight: 0.8,
            },
            FollowUpRule {
                condition: "mentions memory safety".to_string(),
                follow_up_question: "How does Rust guarantee memory safety?".to_string(),
                weight: 0.9,
            },
        ];

        let question = Question::new(
            QuestionType::InteractiveInterview {
                topic: "Rust Ownership".to_string(),
                initial_question: "What is ownership in Rust?".to_string(),
                follow_up_rules,
                comprehension_threshold: 0.7,
            },
            Uuid::new_v4(),
            0.8,
        );

        // Verify the question was created correctly
        if let QuestionType::InteractiveInterview { follow_up_rules, .. } = &question.question_type {
            assert_eq!(follow_up_rules.len(), 2);
            assert_eq!(follow_up_rules[0].weight, 0.8);
        } else {
            panic!("Wrong question type");
        }
    }

    #[test]
    fn test_topic_explanation_type() {
        // Test topic explanation question type
        let question = Question::new(
            QuestionType::TopicExplanation {
                topic: "Rust Lifetimes".to_string(),
                prompt: "Explain how lifetimes work in Rust".to_string(),
                key_concepts: vec![
                    "borrow checker".to_string(),
                    "reference validity".to_string(),
                    "compile-time".to_string(),
                ],
                min_word_count: 100,
            },
            Uuid::new_v4(),
            0.9,
        );

        // Verify structure
        if let QuestionType::TopicExplanation { key_concepts, min_word_count, .. } = &question.question_type {
            assert_eq!(key_concepts.len(), 3);
            assert_eq!(*min_word_count, 100);
        } else {
            panic!("Wrong question type");
        }
    }
}
//! Comprehensive tests for scoring strategies
//!
//! DEVNOTES: Testing all scoring strategies to ensure accurate
//! and fair assessment of quiz performance

use crate::quiz::question::{Question, QuestionType, Answer};
use crate::quiz::session::{QuizSession, QuestionResponse};
use crate::quiz::scoring::ScoringStrategy;
use uuid::Uuid;
use chrono::Utc;

#[cfg(test)]
mod scoring_strategy_tests {
    use super::*;

    fn create_questions_with_difficulties(difficulties: Vec<f32>) -> Vec<Question> {
        difficulties
            .into_iter()
            .map(|diff| {
                let mut q = Question::new(
                    QuestionType::TrueFalse {
                        statement: format!("Question with difficulty {}", diff),
                        correct_answer: true,
                        explanation: None,
                    },
                    Uuid::new_v4(),
                    diff,
                );
                q.estimated_time_seconds = 60;
                q
            })
            .collect()
    }

    fn create_session_with_responses(
        questions: &[Question],
        correct_mask: Vec<bool>,
        times: Vec<u32>,
    ) -> QuizSession {
        let mut session = QuizSession::new(Uuid::new_v4(), None);
        
        for ((question, is_correct), time) in questions.iter()
            .zip(correct_mask.iter())
            .zip(times.iter())
        {
            session.responses.push(QuestionResponse {
                question_id: question.id,
                answer: Answer::TrueFalse(*is_correct),
                is_correct: *is_correct,
                time_taken_seconds: *time,
                attempts: 1,
                submitted_at: Utc::now(),
            });
        }
        
        session
    }

    #[test]
    fn test_simple_scoring_all_correct() {
        let strategy = ScoringStrategy::Simple;
        let questions = create_questions_with_difficulties(vec![0.3, 0.5, 0.7]);
        let session = create_session_with_responses(
            &questions,
            vec![true, true, true],
            vec![30, 45, 60],
        );

        let score = strategy.calculate_score(&session, &questions);
        
        assert_eq!(score.raw_score, 1.0); // 3/3
        assert_eq!(score.weighted_score, 1.0);
        assert_eq!(score.time_bonus, 0.0);
        assert_eq!(score.difficulty_bonus, 0.0);
        assert_eq!(score.streak_bonus, 0.0);
        assert_eq!(score.components.correctness, 1.0);
    }

    #[test]
    fn test_simple_scoring_partial() {
        let strategy = ScoringStrategy::Simple;
        let questions = create_questions_with_difficulties(vec![0.3, 0.5, 0.7, 0.9]);
        let session = create_session_with_responses(
            &questions,
            vec![true, false, true, false],
            vec![30, 45, 60, 90],
        );

        let score = strategy.calculate_score(&session, &questions);
        
        assert_eq!(score.raw_score, 0.5); // 2/4
        assert_eq!(score.weighted_score, 0.5);
    }

    #[test]
    fn test_simple_scoring_empty() {
        let strategy = ScoringStrategy::Simple;
        let questions = create_questions_with_difficulties(vec![0.5]);
        let session = QuizSession::new(Uuid::new_v4(), None);

        let score = strategy.calculate_score(&session, &questions);
        
        assert_eq!(score.raw_score, 0.0);
        assert_eq!(score.weighted_score, 0.0);
    }

    #[test]
    fn test_time_weighted_scoring() {
        let strategy = ScoringStrategy::TimeWeighted {
            base_time_seconds: 60,
            penalty_per_second: 0.01,
        };
        
        let questions = create_questions_with_difficulties(vec![0.5, 0.5, 0.5]);
        let session = create_session_with_responses(
            &questions,
            vec![true, true, true],
            vec![50, 60, 90], // Fast, on-time, slow
        );

        let score = strategy.calculate_score(&session, &questions);
        
        // First question: 1.0 (no penalty, under base time)
        // Second question: 1.0 (exactly base time)
        // Third question: 1.0 - (90-60)*0.01 = 0.7
        // Total: (1.0 + 1.0 + 0.7) / 3 = 0.9
        assert!((score.weighted_score - 0.9).abs() < 0.001);
        assert!(score.time_bonus < 0.0); // Negative bonus due to slow response
    }

    #[test]
    fn test_time_weighted_scoring_with_incorrect() {
        let strategy = ScoringStrategy::TimeWeighted {
            base_time_seconds: 60,
            penalty_per_second: 0.02,
        };
        
        let questions = create_questions_with_difficulties(vec![0.5, 0.5]);
        let session = create_session_with_responses(
            &questions,
            vec![false, true],
            vec![120, 30], // Slow wrong, fast right
        );

        let score = strategy.calculate_score(&session, &questions);
        
        // First question: 0 (incorrect, no points regardless of time)
        // Second question: 1.0 (correct and fast)
        // Total: 1.0 / 2 = 0.5
        assert_eq!(score.weighted_score, 0.5);
    }

    #[test]
    fn test_difficulty_weighted_scoring() {
        let strategy = ScoringStrategy::DifficultyWeighted {
            easy_multiplier: 1.0,
            medium_multiplier: 1.5,
            hard_multiplier: 2.0,
        };
        
        // Easy (< 0.33), Medium (0.33-0.67), Hard (>= 0.67)
        let questions = create_questions_with_difficulties(vec![0.2, 0.5, 0.8]);
        let session = create_session_with_responses(
            &questions,
            vec![true, true, true],
            vec![30, 45, 60],
        );

        let score = strategy.calculate_score(&session, &questions);
        
        // Easy: 1.0 * 1.0 = 1.0
        // Medium: 1.0 * 1.5 = 1.5
        // Hard: 1.0 * 2.0 = 2.0
        // Total: 4.5 / 4.5 = 1.0
        assert_eq!(score.weighted_score, 1.0);
        assert_eq!(score.raw_score, 1.0); // 3/3 correct
        assert_eq!(score.difficulty_bonus, 0.0); // No bonus when perfect score
    }

    #[test]
    fn test_difficulty_weighted_bonus_when_harder_correct() {
        let strategy = ScoringStrategy::DifficultyWeighted {
            easy_multiplier: 1.0,
            medium_multiplier: 1.5,
            hard_multiplier: 2.0,
        };
        
        // Answer hard questions correctly, miss easy ones
        let questions = create_questions_with_difficulties(vec![0.2, 0.5, 0.8]);
        let session = create_session_with_responses(
            &questions,
            vec![false, true, true], // Miss easy, get medium and hard
            vec![30, 45, 60],
        );

        let score = strategy.calculate_score(&session, &questions);
        
        // Max possible: 1.0 + 1.5 + 2.0 = 4.5
        // Score: 0 + 1.5 + 2.0 = 3.5
        // Weighted: 3.5 / 4.5 = 0.778
        // Raw: 2/3 = 0.667
        // Bonus: 0.778 - 0.667 = 0.111
        assert!((score.weighted_score - 0.778).abs() < 0.001);
        assert!((score.raw_score - 0.667).abs() < 0.001);
        assert!(score.difficulty_bonus > 0.0);
    }

    #[test]
    fn test_difficulty_weighted_with_skipped() {
        let strategy = ScoringStrategy::DifficultyWeighted {
            easy_multiplier: 1.0,
            medium_multiplier: 1.5,
            hard_multiplier: 2.0,
        };
        
        let questions = create_questions_with_difficulties(vec![0.2, 0.5, 0.8]);
        let mut session = create_session_with_responses(
            &questions[..2], // Only answer first two
            vec![true, true],
            vec![30, 45],
        );
        
        // Skip the hard question
        session.skipped_questions.push(2);

        let score = strategy.calculate_score(&session, &questions);
        
        // Max possible: Easy (1.0) + Medium (1.5) + Hard (2.0) = 4.5
        // Score: Easy (1.0) + Medium (1.5) = 2.5
        // Total: 2.5 / 4.5 = 0.556
        assert!((score.weighted_score - 0.556).abs() < 0.001);
    }

    #[test]
    fn test_adaptive_scoring_comprehensive() {
        let strategy = ScoringStrategy::Adaptive {
            time_weight: 0.2,
            difficulty_weight: 0.3,
            streak_weight: 0.2,
            consistency_weight: 0.1,
        };
        
        let questions = create_questions_with_difficulties(vec![0.3, 0.5, 0.7, 0.8]);
        let session = create_session_with_responses(
            &questions,
            vec![true, true, true, false],
            vec![50, 55, 60, 100],
        );

        let score = strategy.calculate_score(&session, &questions);
        
        // Verify all components are calculated
        assert!(score.components.correctness > 0.0);
        assert!(score.components.speed > 0.0);
        assert!(score.components.difficulty > 0.0);
        assert!(score.components.consistency > 0.0);
        
        // Weighted score should be between raw score and 1.0
        assert!(score.weighted_score >= score.raw_score);
        assert!(score.weighted_score <= 1.0);
    }

    #[test]
    fn test_streak_calculation() {
        let strategy = ScoringStrategy::Adaptive {
            time_weight: 0.0,
            difficulty_weight: 0.0,
            streak_weight: 1.0,
            consistency_weight: 0.0,
        };
        
        let questions = create_questions_with_difficulties(vec![0.5; 6]);
        
        // Pattern: correct, correct, wrong, correct, correct, correct
        let session = create_session_with_responses(
            &questions,
            vec![true, true, false, true, true, true],
            vec![60; 6],
        );

        let score = strategy.calculate_score(&session, &questions);
        
        // Max streak is 3 (last three correct)
        // Streak score = 3/6 = 0.5
        let expected_streak_score = 0.5;
        
        // Since only streak weight is 1.0, the bonus should reflect this
        assert!((score.streak_bonus - expected_streak_score).abs() < 0.1);
    }

    #[test]
    fn test_consistency_calculation() {
        let strategy = ScoringStrategy::Adaptive {
            time_weight: 0.0,
            difficulty_weight: 0.0,
            streak_weight: 0.0,
            consistency_weight: 1.0,
        };
        
        let questions = create_questions_with_difficulties(vec![0.5; 4]);
        
        // Very consistent times
        let consistent_session = create_session_with_responses(
            &questions,
            vec![true; 4],
            vec![60, 61, 59, 60],
        );
        
        let consistent_score = strategy.calculate_score(&consistent_session, &questions);
        
        // Very inconsistent times
        let inconsistent_session = create_session_with_responses(
            &questions,
            vec![true; 4],
            vec![30, 90, 45, 120],
        );
        
        let inconsistent_score = strategy.calculate_score(&inconsistent_session, &questions);
        
        // Consistent timing should score higher
        assert!(consistent_score.components.consistency > inconsistent_score.components.consistency);
    }

    #[test]
    fn test_adaptive_all_weights_zero() {
        let strategy = ScoringStrategy::Adaptive {
            time_weight: 0.0,
            difficulty_weight: 0.0,
            streak_weight: 0.0,
            consistency_weight: 0.0,
        };
        
        let questions = create_questions_with_difficulties(vec![0.5, 0.5]);
        let session = create_session_with_responses(
            &questions,
            vec![true, false],
            vec![60, 60],
        );

        let score = strategy.calculate_score(&session, &questions);
        
        // With all weights at 0, weighted score should equal raw score
        assert_eq!(score.weighted_score, score.raw_score);
        assert_eq!(score.raw_score, 0.5); // 1/2 correct
    }

    #[test]
    fn test_scoring_with_no_responses() {
        let strategies = vec![
            ScoringStrategy::Simple,
            ScoringStrategy::TimeWeighted {
                base_time_seconds: 60,
                penalty_per_second: 0.01,
            },
            ScoringStrategy::DifficultyWeighted {
                easy_multiplier: 1.0,
                medium_multiplier: 1.5,
                hard_multiplier: 2.0,
            },
            ScoringStrategy::Adaptive {
                time_weight: 0.5,
                difficulty_weight: 0.5,
                streak_weight: 0.5,
                consistency_weight: 0.5,
            },
        ];
        
        let questions = create_questions_with_difficulties(vec![0.5]);
        let empty_session = QuizSession::new(Uuid::new_v4(), None);
        
        for (i, strategy) in strategies.into_iter().enumerate() {
            let score = strategy.calculate_score(&empty_session, &questions);
            assert_eq!(score.raw_score, 0.0, "Strategy {} raw score mismatch", i);
            assert_eq!(score.weighted_score, 0.0, "Strategy {} weighted score mismatch", i);
        }
    }

    #[test]
    fn test_edge_case_single_response() {
        let strategy = ScoringStrategy::Adaptive {
            time_weight: 0.3,
            difficulty_weight: 0.3,
            streak_weight: 0.2,
            consistency_weight: 0.2,
        };
        
        let questions = create_questions_with_difficulties(vec![0.5]);
        let session = create_session_with_responses(
            &questions,
            vec![true],
            vec![60],
        );

        let score = strategy.calculate_score(&session, &questions);
        
        assert_eq!(score.raw_score, 1.0);
        assert_eq!(score.components.correctness, 1.0);
        assert_eq!(score.components.consistency, 1.0); // Perfect consistency with 1 response
        
        // Streak score should be 1.0 (1 correct out of 1)
        assert_eq!(score.components.speed, 1.0); // Assuming on target time
    }

    #[test]
    fn test_scoring_preserves_percentile_field() {
        // Test that percentile field is available for future use
        let strategy = ScoringStrategy::Simple;
        let questions = create_questions_with_difficulties(vec![0.5]);
        let session = create_session_with_responses(&questions, vec![true], vec![60]);

        let score = strategy.calculate_score(&session, &questions);
        
        assert!(score.percentile.is_none()); // Not implemented yet
    }
}
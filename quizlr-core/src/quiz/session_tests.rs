//! Comprehensive tests for quiz session management
//!
//! DEVNOTES: Testing session lifecycle, state transitions, and
//! answer submission to ensure reliable quiz-taking experience

use crate::quiz::question::{Question, QuestionType, Answer};
use crate::quiz::session::{QuizSession, SessionState, SessionSummary};
use uuid::Uuid;
use chrono::Duration;

#[cfg(test)]
mod session_management_tests {
    use super::*;

    fn create_test_question() -> Question {
        Question::new(
            QuestionType::TrueFalse {
                statement: "Test statement".to_string(),
                correct_answer: true,
                explanation: None,
            },
            Uuid::new_v4(),
            0.5,
        )
    }

    #[test]
    fn test_session_creation() {
        // Test session initialization
        let quiz_id = Uuid::new_v4();
        let user_id = Some(Uuid::new_v4());
        let session = QuizSession::new(quiz_id, user_id);

        assert_eq!(session.quiz_id, quiz_id);
        assert_eq!(session.user_id, user_id);
        assert_eq!(session.state, SessionState::NotStarted);
        assert_eq!(session.current_question_index, 0);
        assert_eq!(session.responses.len(), 0);
        assert_eq!(session.skipped_questions.len(), 0);
        assert!(session.start_time.is_none());
        assert!(session.end_time.is_none());
        assert_eq!(session.pause_duration, Duration::zero());
    }

    #[test]
    fn test_anonymous_session() {
        // Test session without user ID
        let session = QuizSession::new(Uuid::new_v4(), None);
        assert!(session.user_id.is_none());
    }

    #[test]
    fn test_session_state_transitions() {
        // Test valid state transitions
        let mut session = QuizSession::new(Uuid::new_v4(), None);

        // Start session
        assert!(session.start().is_ok());
        assert_eq!(session.state, SessionState::InProgress);
        assert!(session.start_time.is_some());

        // Can't start again
        assert!(session.start().is_err());

        // Pause session
        assert!(session.pause().is_ok());
        assert_eq!(session.state, SessionState::Paused);

        // Can't pause when already paused
        assert!(session.pause().is_err());

        // Resume session
        assert!(session.resume().is_ok());
        assert_eq!(session.state, SessionState::InProgress);

        // Complete session
        let summary = session.complete();
        assert!(summary.is_ok());
        assert_eq!(session.state, SessionState::Completed);
        assert!(session.end_time.is_some());
    }

    #[test]
    fn test_session_abandon() {
        // Test abandoning a session
        let mut session = QuizSession::new(Uuid::new_v4(), None);
        session.start().unwrap();
        
        session.abandon();
        assert_eq!(session.state, SessionState::Abandoned);
        assert!(session.end_time.is_some());
    }

    #[test]
    fn test_pause_duration_tracking() {
        use std::thread;
        use std::time::Duration as StdDuration;

        let mut session = QuizSession::new(Uuid::new_v4(), None);
        session.start().unwrap();
        
        // Initial pause duration should be zero
        assert_eq!(session.pause_duration, Duration::zero());
        
        session.pause().unwrap();
        let pause_start = session.last_activity;
        
        // Simulate time passing
        thread::sleep(StdDuration::from_millis(50));
        
        session.resume().unwrap();
        
        // Pause duration should be recorded
        assert!(session.pause_duration > Duration::zero());
        assert!(session.last_activity > pause_start);
    }

    #[test]
    fn test_submit_answer_correct() {
        // Test submitting correct answer
        let mut session = QuizSession::new(Uuid::new_v4(), None);
        session.start().unwrap();
        
        let question = create_test_question();
        let result = session.submit_answer(
            &question,
            Answer::TrueFalse(true),
            30,
        );

        assert!(result.is_ok());
        assert!(result.unwrap());
        assert_eq!(session.responses.len(), 1);
        
        let response = &session.responses[0];
        assert_eq!(response.question_id, question.id);
        assert!(response.is_correct);
        assert_eq!(response.time_taken_seconds, 30);
        assert_eq!(response.attempts, 1);
    }

    #[test]
    fn test_submit_answer_incorrect() {
        // Test submitting incorrect answer
        let mut session = QuizSession::new(Uuid::new_v4(), None);
        session.start().unwrap();
        
        let question = create_test_question();
        let result = session.submit_answer(
            &question,
            Answer::TrueFalse(false),
            25,
        );

        assert!(result.is_ok());
        assert!(!result.unwrap());
        assert_eq!(session.responses.len(), 1);
        assert!(!session.responses[0].is_correct);
    }

    #[test]
    fn test_submit_answer_not_in_progress() {
        // Test submitting answer when session not in progress
        let mut session = QuizSession::new(Uuid::new_v4(), None);
        let question = create_test_question();
        
        // Not started
        let result = session.submit_answer(&question, Answer::TrueFalse(true), 30);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Session is not in progress");
        
        // Paused
        session.start().unwrap();
        session.pause().unwrap();
        let result = session.submit_answer(&question, Answer::TrueFalse(true), 30);
        assert!(result.is_err());
    }

    #[test]
    fn test_resubmit_answer() {
        // Test resubmitting answer to same question
        let mut session = QuizSession::new(Uuid::new_v4(), None);
        session.start().unwrap();
        
        let question = create_test_question();
        
        // First attempt - wrong
        session.submit_answer(&question, Answer::TrueFalse(false), 20).unwrap();
        assert_eq!(session.responses.len(), 1);
        assert!(!session.responses[0].is_correct);
        assert_eq!(session.responses[0].attempts, 1);
        assert_eq!(session.responses[0].time_taken_seconds, 20);
        
        // Second attempt - correct
        session.submit_answer(&question, Answer::TrueFalse(true), 15).unwrap();
        assert_eq!(session.responses.len(), 1); // Still only one response
        assert!(session.responses[0].is_correct);
        assert_eq!(session.responses[0].attempts, 2);
        assert_eq!(session.responses[0].time_taken_seconds, 35); // 20 + 15
    }

    #[test]
    fn test_skip_question() {
        // Test skipping questions
        let mut session = QuizSession::new(Uuid::new_v4(), None);
        session.start().unwrap();
        
        session.skip_question(0);
        assert_eq!(session.skipped_questions.len(), 1);
        assert!(session.skipped_questions.contains(&0));
        
        // Skip same question again - should not duplicate
        session.skip_question(0);
        assert_eq!(session.skipped_questions.len(), 1);
        
        session.skip_question(2);
        assert_eq!(session.skipped_questions.len(), 2);
        assert!(session.skipped_questions.contains(&2));
    }

    #[test]
    fn test_navigation() {
        // Test next/previous question navigation
        let mut session = QuizSession::new(Uuid::new_v4(), None);
        session.start().unwrap();
        
        assert_eq!(session.current_question_index, 0);
        
        // Next question
        assert!(session.next_question().is_ok());
        assert_eq!(session.current_question_index, 1);
        
        assert!(session.next_question().is_ok());
        assert_eq!(session.current_question_index, 2);
        
        // Previous question
        assert!(session.previous_question().is_ok());
        assert_eq!(session.current_question_index, 1);
        
        assert!(session.previous_question().is_ok());
        assert_eq!(session.current_question_index, 0);
        
        // Can't go before first question
        assert!(session.previous_question().is_err());
        assert_eq!(session.current_question_index, 0);
    }

    #[test]
    fn test_session_summary_calculation() {
        // Test summary generation
        let mut session = QuizSession::new(Uuid::new_v4(), None);
        session.start().unwrap();
        
        // Answer some questions
        let q1 = create_test_question();
        let q2 = create_test_question();
        let _q3 = create_test_question();
        
        session.submit_answer(&q1, Answer::TrueFalse(true), 30).unwrap(); // Correct
        session.submit_answer(&q2, Answer::TrueFalse(false), 45).unwrap(); // Incorrect
        session.skip_question(2);
        
        let summary = session.complete().unwrap();
        
        assert_eq!(summary.session_id, session.id);
        assert_eq!(summary.correct_answers, 1);
        assert_eq!(summary.total_questions, 3); // 2 answered + 1 skipped
        assert_eq!(summary.skipped_questions, 1);
        assert_eq!(summary.total_time_seconds, 75); // 30 + 45
        assert_eq!(summary.average_time_per_question, 37); // 75 / 2
        assert_eq!(summary.completion_rate, 2.0 / 3.0);
        assert_eq!(summary.score, 1.0 / 3.0);
    }

    #[test]
    fn test_session_summary_grades() {
        let session = QuizSession::new(Uuid::new_v4(), None);
        let summary = SessionSummary {
            session_id: session.id,
            quiz_id: session.quiz_id,
            score: 0.0,
            correct_answers: 0,
            total_questions: 0,
            skipped_questions: 0,
            total_time_seconds: 0,
            duration: Duration::zero(),
            average_time_per_question: 0,
            completion_rate: 0.0,
        };

        // Test grade assignments
        let mut test_summary = summary.clone();
        
        test_summary.score = 0.95;
        assert_eq!(test_summary.get_grade(), "A");
        
        test_summary.score = 0.85;
        assert_eq!(test_summary.get_grade(), "B");
        
        test_summary.score = 0.75;
        assert_eq!(test_summary.get_grade(), "C");
        
        test_summary.score = 0.65;
        assert_eq!(test_summary.get_grade(), "D");
        
        test_summary.score = 0.55;
        assert_eq!(test_summary.get_grade(), "F");
    }

    #[test]
    fn test_session_summary_pass_threshold() {
        let session = QuizSession::new(Uuid::new_v4(), None);
        let mut summary = SessionSummary {
            session_id: session.id,
            quiz_id: session.quiz_id,
            score: 0.7,
            correct_answers: 7,
            total_questions: 10,
            skipped_questions: 0,
            total_time_seconds: 300,
            duration: Duration::seconds(300),
            average_time_per_question: 30,
            completion_rate: 1.0,
        };

        assert!(summary.passed(0.7)); // Exactly at threshold
        assert!(summary.passed(0.6)); // Above threshold
        assert!(!summary.passed(0.8)); // Below threshold
        
        summary.score = 0.69;
        assert!(!summary.passed(0.7)); // Just below threshold
    }

    #[test]
    fn test_progress_calculation() {
        let mut session = QuizSession::new(Uuid::new_v4(), None);
        session.start().unwrap();
        
        // No questions answered
        assert_eq!(session.get_progress(10), 0.0);
        
        // Answer some questions
        let q1 = create_test_question();
        let q2 = create_test_question();
        session.submit_answer(&q1, Answer::TrueFalse(true), 30).unwrap();
        session.submit_answer(&q2, Answer::TrueFalse(false), 30).unwrap();
        
        assert_eq!(session.get_progress(10), 0.2); // 2/10
        assert_eq!(session.get_progress(4), 0.5); // 2/4
        assert_eq!(session.get_progress(2), 1.0); // 2/2
        
        // Edge case: no total questions
        assert_eq!(session.get_progress(0), 0.0);
    }

    #[test]
    fn test_empty_session_summary() {
        // Test summary for session with no activity
        let mut session = QuizSession::new(Uuid::new_v4(), None);
        session.start().unwrap();
        
        let summary = session.complete().unwrap();
        
        assert_eq!(summary.score, 0.0);
        assert_eq!(summary.correct_answers, 0);
        assert_eq!(summary.total_questions, 0);
        assert_eq!(summary.average_time_per_question, 0);
        assert_eq!(summary.completion_rate, 0.0);
    }
}
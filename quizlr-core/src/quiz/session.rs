use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use super::{Question, Answer, Quiz};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionState {
    NotStarted,
    InProgress,
    Paused,
    Completed,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizSession {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Option<Uuid>, // Optional for anonymous users
    pub state: SessionState,
    pub current_question_index: usize,
    pub responses: Vec<QuestionResponse>,
    pub skipped_questions: Vec<usize>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub pause_duration: Duration,
    pub last_activity: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionResponse {
    pub question_id: Uuid,
    pub answer: Answer,
    pub is_correct: bool,
    pub time_taken_seconds: u32,
    pub attempts: u32,
    pub submitted_at: DateTime<Utc>,
}

impl QuizSession {
    pub fn new(quiz_id: Uuid, user_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            quiz_id,
            user_id,
            state: SessionState::NotStarted,
            current_question_index: 0,
            responses: Vec::new(),
            skipped_questions: Vec::new(),
            start_time: None,
            end_time: None,
            pause_duration: Duration::zero(),
            last_activity: Utc::now(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn start(&mut self) -> Result<(), String> {
        match self.state {
            SessionState::NotStarted => {
                self.state = SessionState::InProgress;
                self.start_time = Some(Utc::now());
                self.last_activity = Utc::now();
                Ok(())
            }
            _ => Err("Session already started".to_string()),
        }
    }
    
    pub fn pause(&mut self) -> Result<(), String> {
        match self.state {
            SessionState::InProgress => {
                self.state = SessionState::Paused;
                self.last_activity = Utc::now();
                Ok(())
            }
            _ => Err("Can only pause an in-progress session".to_string()),
        }
    }
    
    pub fn resume(&mut self) -> Result<(), String> {
        match self.state {
            SessionState::Paused => {
                let pause_time = Utc::now() - self.last_activity;
                self.pause_duration = self.pause_duration + pause_time;
                self.state = SessionState::InProgress;
                self.last_activity = Utc::now();
                Ok(())
            }
            _ => Err("Can only resume a paused session".to_string()),
        }
    }
    
    pub fn submit_answer(
        &mut self,
        question: &Question,
        answer: Answer,
        time_taken_seconds: u32,
    ) -> Result<bool, String> {
        if self.state != SessionState::InProgress {
            return Err("Session is not in progress".to_string());
        }
        
        let is_correct = question.validate_answer(&answer)?;
        
        // Check if we already have a response for this question
        let existing_response = self.responses.iter_mut()
            .find(|r| r.question_id == question.id);
        
        if let Some(response) = existing_response {
            response.attempts += 1;
            response.answer = answer;
            response.is_correct = is_correct;
            response.time_taken_seconds += time_taken_seconds;
            response.submitted_at = Utc::now();
        } else {
            self.responses.push(QuestionResponse {
                question_id: question.id,
                answer,
                is_correct,
                time_taken_seconds,
                attempts: 1,
                submitted_at: Utc::now(),
            });
        }
        
        self.last_activity = Utc::now();
        Ok(is_correct)
    }
    
    pub fn skip_question(&mut self, question_index: usize) {
        if !self.skipped_questions.contains(&question_index) {
            self.skipped_questions.push(question_index);
        }
        self.last_activity = Utc::now();
    }
    
    pub fn next_question(&mut self) -> Result<(), String> {
        if self.state != SessionState::InProgress {
            return Err("Session is not in progress".to_string());
        }
        
        self.current_question_index += 1;
        self.last_activity = Utc::now();
        Ok(())
    }
    
    pub fn previous_question(&mut self) -> Result<(), String> {
        if self.state != SessionState::InProgress {
            return Err("Session is not in progress".to_string());
        }
        
        if self.current_question_index > 0 {
            self.current_question_index -= 1;
            self.last_activity = Utc::now();
            Ok(())
        } else {
            Err("Already at first question".to_string())
        }
    }
    
    pub fn complete(&mut self) -> Result<SessionSummary, String> {
        match self.state {
            SessionState::InProgress => {
                self.state = SessionState::Completed;
                self.end_time = Some(Utc::now());
                Ok(self.generate_summary())
            }
            _ => Err("Can only complete an in-progress session".to_string()),
        }
    }
    
    pub fn abandon(&mut self) {
        self.state = SessionState::Abandoned;
        self.end_time = Some(Utc::now());
    }
    
    pub fn generate_summary(&self) -> SessionSummary {
        let total_questions = self.responses.len() + self.skipped_questions.len();
        let correct_answers = self.responses.iter()
            .filter(|r| r.is_correct)
            .count();
        let total_time_seconds: u32 = self.responses.iter()
            .map(|r| r.time_taken_seconds)
            .sum();
        
        let score = if total_questions > 0 {
            correct_answers as f32 / total_questions as f32
        } else {
            0.0
        };
        
        let duration = if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            end - start - self.pause_duration
        } else if let Some(start) = self.start_time {
            Utc::now() - start - self.pause_duration
        } else {
            Duration::zero()
        };
        
        SessionSummary {
            session_id: self.id,
            quiz_id: self.quiz_id,
            score,
            correct_answers,
            total_questions,
            skipped_questions: self.skipped_questions.len(),
            total_time_seconds,
            duration,
            average_time_per_question: if !self.responses.is_empty() {
                total_time_seconds / self.responses.len() as u32
            } else {
                0
            },
            completion_rate: if total_questions > 0 {
                self.responses.len() as f32 / total_questions as f32
            } else {
                0.0
            },
        }
    }
    
    pub fn get_progress(&self, total_questions: usize) -> f32 {
        if total_questions == 0 {
            return 0.0;
        }
        
        let answered = self.responses.len();
        answered as f32 / total_questions as f32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub session_id: Uuid,
    pub quiz_id: Uuid,
    pub score: f32,
    pub correct_answers: usize,
    pub total_questions: usize,
    pub skipped_questions: usize,
    pub total_time_seconds: u32,
    pub duration: Duration,
    pub average_time_per_question: u32,
    pub completion_rate: f32,
}

impl SessionSummary {
    pub fn passed(&self, pass_threshold: f32) -> bool {
        self.score >= pass_threshold
    }
    
    pub fn get_grade(&self) -> &'static str {
        match self.score {
            s if s >= 0.9 => "A",
            s if s >= 0.8 => "B",
            s if s >= 0.7 => "C",
            s if s >= 0.6 => "D",
            _ => "F",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quiz::question::QuestionType;
    
    #[test]
    fn test_session_lifecycle() {
        let mut session = QuizSession::new(Uuid::new_v4(), None);
        
        assert_eq!(session.state, SessionState::NotStarted);
        assert!(session.start().is_ok());
        assert_eq!(session.state, SessionState::InProgress);
        
        assert!(session.pause().is_ok());
        assert_eq!(session.state, SessionState::Paused);
        
        assert!(session.resume().is_ok());
        assert_eq!(session.state, SessionState::InProgress);
        
        let summary = session.complete().unwrap();
        assert_eq!(session.state, SessionState::Completed);
        assert_eq!(summary.score, 0.0); // No questions answered
    }
    
    #[test]
    fn test_submit_answer() {
        let mut session = QuizSession::new(Uuid::new_v4(), None);
        session.start().unwrap();
        
        let question = Question::new(
            QuestionType::TrueFalse {
                statement: "Test".to_string(),
                correct_answer: true,
                explanation: None,
            },
            Uuid::new_v4(),
            0.5,
        );
        
        let result = session.submit_answer(
            &question,
            Answer::TrueFalse(true),
            30,
        ).unwrap();
        
        assert!(result);
        assert_eq!(session.responses.len(), 1);
        assert!(session.responses[0].is_correct);
    }
}
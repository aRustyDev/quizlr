use super::question::Question;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub questions: Vec<Question>,
    pub topic_ids: Vec<Uuid>,
    pub difficulty_range: (f32, f32), // min, max
    pub estimated_duration_minutes: u32,
    pub pass_threshold: f32, // 0.0 to 1.0
    pub allow_skip: bool,
    pub show_explanations: bool,
    pub randomize_questions: bool,
    pub randomize_options: bool,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Quiz {
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            questions: Vec::new(),
            topic_ids: Vec::new(),
            difficulty_range: (0.0, 1.0),
            estimated_duration_minutes: 30,
            pass_threshold: 0.7,
            allow_skip: true,
            show_explanations: true,
            randomize_questions: false,
            randomize_options: false,
            tags: Vec::new(),
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_question(&mut self, question: Question) {
        if !self.topic_ids.contains(&question.topic_id) {
            self.topic_ids.push(question.topic_id);
        }
        self.questions.push(question);
        self.update_difficulty_range();
        self.update_estimated_duration();
        self.updated_at = Utc::now();
    }

    pub fn remove_question(&mut self, question_id: Uuid) -> Option<Question> {
        if let Some(pos) = self.questions.iter().position(|q| q.id == question_id) {
            let removed = self.questions.remove(pos);
            self.update_difficulty_range();
            self.update_estimated_duration();
            self.updated_at = Utc::now();
            Some(removed)
        } else {
            None
        }
    }

    fn update_difficulty_range(&mut self) {
        if self.questions.is_empty() {
            self.difficulty_range = (0.0, 1.0);
        } else {
            let min = self
                .questions
                .iter()
                .map(|q| q.difficulty)
                .fold(1.0, f32::min);
            let max = self
                .questions
                .iter()
                .map(|q| q.difficulty)
                .fold(0.0, f32::max);
            self.difficulty_range = (min, max);
        }
    }

    fn update_estimated_duration(&mut self) {
        let total_seconds: u32 = self
            .questions
            .iter()
            .map(|q| q.estimated_time_seconds)
            .sum();
        self.estimated_duration_minutes = (total_seconds / 60).max(1);
    }

    pub fn get_questions_for_session(&self) -> Vec<Question> {
        let mut questions = self.questions.clone();

        if self.randomize_questions {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            questions.shuffle(&mut rng);
        }

        if self.randomize_options {
            // This would need to be implemented for each question type
            // that supports option randomization
        }

        questions
    }
}

pub struct QuizBuilder {
    quiz: Quiz,
}

impl QuizBuilder {
    pub fn new(title: String) -> Self {
        Self {
            quiz: Quiz::new(title),
        }
    }

    pub fn description(mut self, desc: String) -> Self {
        self.quiz.description = Some(desc);
        self
    }

    pub fn pass_threshold(mut self, threshold: f32) -> Self {
        self.quiz.pass_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    pub fn allow_skip(mut self, allow: bool) -> Self {
        self.quiz.allow_skip = allow;
        self
    }

    pub fn show_explanations(mut self, show: bool) -> Self {
        self.quiz.show_explanations = show;
        self
    }

    pub fn randomize_questions(mut self, randomize: bool) -> Self {
        self.quiz.randomize_questions = randomize;
        self
    }

    pub fn randomize_options(mut self, randomize: bool) -> Self {
        self.quiz.randomize_options = randomize;
        self
    }

    pub fn add_question(mut self, question: Question) -> Self {
        self.quiz.add_question(question);
        self
    }

    pub fn add_questions(mut self, questions: Vec<Question>) -> Self {
        for question in questions {
            self.quiz.add_question(question);
        }
        self
    }

    pub fn add_tag(mut self, tag: String) -> Self {
        if !self.quiz.tags.contains(&tag) {
            self.quiz.tags.push(tag);
        }
        self
    }

    pub fn add_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.quiz.metadata.insert(key, value);
        self
    }

    pub fn build(self) -> Quiz {
        self.quiz
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quiz::question::QuestionType;

    #[test]
    fn test_quiz_builder() {
        let quiz = QuizBuilder::new("Test Quiz".to_string())
            .description("A test quiz".to_string())
            .pass_threshold(0.8)
            .randomize_questions(true)
            .add_tag("test".to_string())
            .build();

        assert_eq!(quiz.title, "Test Quiz");
        assert_eq!(quiz.description, Some("A test quiz".to_string()));
        assert_eq!(quiz.pass_threshold, 0.8);
        assert!(quiz.randomize_questions);
        assert!(quiz.tags.contains(&"test".to_string()));
    }

    #[test]
    fn test_add_remove_questions() {
        let mut quiz = Quiz::new("Test Quiz".to_string());
        let topic_id = Uuid::new_v4();

        let question = Question::new(
            QuestionType::TrueFalse {
                statement: "Test statement".to_string(),
                correct_answer: true,
                explanation: None,
            },
            topic_id,
            0.5,
        );

        let question_id = question.id;
        quiz.add_question(question);

        assert_eq!(quiz.questions.len(), 1);
        assert!(quiz.topic_ids.contains(&topic_id));

        quiz.remove_question(question_id);
        assert_eq!(quiz.questions.len(), 0);
    }
}

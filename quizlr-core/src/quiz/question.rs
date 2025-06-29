use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum QuestionType {
    TrueFalse {
        statement: String,
        correct_answer: bool,
        explanation: Option<String>,
    },
    MultipleChoice {
        question: String,
        options: Vec<String>,
        correct_index: usize,
        explanation: Option<String>,
    },
    MultiSelect {
        question: String,
        options: Vec<String>,
        correct_indices: Vec<usize>,
        explanation: Option<String>,
    },
    FillInTheBlank {
        template: String, // Contains {} for blanks
        correct_answers: Vec<String>,
        case_sensitive: bool,
        explanation: Option<String>,
    },
    MatchPairs {
        instruction: String,
        left_items: Vec<String>,
        right_items: Vec<String>,
        correct_pairs: Vec<(usize, usize)>,
        explanation: Option<String>,
    },
    InteractiveInterview {
        topic: String,
        initial_question: String,
        follow_up_rules: Vec<FollowUpRule>,
        comprehension_threshold: f32,
    },
    TopicExplanation {
        topic: String,
        prompt: String,
        key_concepts: Vec<String>,
        min_word_count: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FollowUpRule {
    pub condition: String,
    pub follow_up_question: String,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub question_type: QuestionType,
    pub topic_id: Uuid,
    pub difficulty: f32, // 0.0 to 1.0
    pub estimated_time_seconds: u32,
    pub tags: Vec<String>,
    pub citations: Vec<Citation>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub id: Uuid,
    pub source: String,
    pub url: Option<String>,
    pub excerpt: Option<String>,
    pub confidence: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
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

impl Question {
    pub fn new(question_type: QuestionType, topic_id: Uuid, difficulty: f32) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            question_type,
            topic_id,
            difficulty,
            estimated_time_seconds: 60, // Default 1 minute
            tags: Vec::new(),
            citations: Vec::new(),
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn validate_answer(&self, answer: &Answer) -> Result<bool, String> {
        match (&self.question_type, answer) {
            (QuestionType::TrueFalse { correct_answer, .. }, Answer::TrueFalse(user_answer)) => {
                Ok(correct_answer == user_answer)
            }
            (
                QuestionType::MultipleChoice {
                    correct_index,
                    options,
                    ..
                },
                Answer::MultipleChoice(user_index),
            ) => {
                if *user_index >= options.len() {
                    Err("Invalid option index".to_string())
                } else {
                    Ok(correct_index == user_index)
                }
            }
            (
                QuestionType::MultiSelect {
                    correct_indices,
                    options,
                    ..
                },
                Answer::MultiSelect(user_indices),
            ) => {
                if user_indices.iter().any(|&idx| idx >= options.len()) {
                    Err("Invalid option index".to_string())
                } else {
                    let mut user_sorted = user_indices.clone();
                    let mut correct_sorted = correct_indices.clone();
                    user_sorted.sort();
                    correct_sorted.sort();
                    Ok(user_sorted == correct_sorted)
                }
            }
            (
                QuestionType::FillInTheBlank {
                    correct_answers,
                    case_sensitive,
                    ..
                },
                Answer::FillInTheBlank(user_answers),
            ) => {
                if user_answers.len() != correct_answers.len() {
                    Err("Wrong number of answers".to_string())
                } else {
                    let all_correct =
                        user_answers
                            .iter()
                            .zip(correct_answers.iter())
                            .all(|(user, correct)| {
                                if *case_sensitive {
                                    user == correct
                                } else {
                                    user.to_lowercase() == correct.to_lowercase()
                                }
                            });
                    Ok(all_correct)
                }
            }
            (QuestionType::MatchPairs { correct_pairs, .. }, Answer::MatchPairs(user_pairs)) => {
                let mut user_sorted = user_pairs.clone();
                let mut correct_sorted = correct_pairs.clone();
                user_sorted.sort();
                correct_sorted.sort();
                Ok(user_sorted == correct_sorted)
            }
            _ => Err("Answer type does not match question type".to_string()),
        }
    }

    pub fn get_explanation(&self) -> Option<&str> {
        match &self.question_type {
            QuestionType::TrueFalse { explanation, .. }
            | QuestionType::MultipleChoice { explanation, .. }
            | QuestionType::MultiSelect { explanation, .. }
            | QuestionType::FillInTheBlank { explanation, .. }
            | QuestionType::MatchPairs { explanation, .. } => explanation.as_deref(),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_false_validation() {
        let question = Question::new(
            QuestionType::TrueFalse {
                statement: "Rust is memory safe".to_string(),
                correct_answer: true,
                explanation: Some("Rust provides memory safety guarantees".to_string()),
            },
            Uuid::new_v4(),
            0.3,
        );

        assert!(question.validate_answer(&Answer::TrueFalse(true)).unwrap());
        assert!(!question.validate_answer(&Answer::TrueFalse(false)).unwrap());
    }

    #[test]
    fn test_multiple_choice_validation() {
        let question = Question::new(
            QuestionType::MultipleChoice {
                question: "What is 2+2?".to_string(),
                options: vec!["3".to_string(), "4".to_string(), "5".to_string()],
                correct_index: 1,
                explanation: None,
            },
            Uuid::new_v4(),
            0.1,
        );

        assert!(question
            .validate_answer(&Answer::MultipleChoice(1))
            .unwrap());
        assert!(!question
            .validate_answer(&Answer::MultipleChoice(0))
            .unwrap());
    }
}

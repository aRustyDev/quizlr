use serde::{Deserialize, Serialize};
use super::session::{QuizSession, QuestionResponse};
use super::Question;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub raw_score: f32,
    pub weighted_score: f32,
    pub percentile: Option<f32>,
    pub time_bonus: f32,
    pub difficulty_bonus: f32,
    pub streak_bonus: f32,
    pub components: ScoreComponents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreComponents {
    pub correctness: f32,
    pub speed: f32,
    pub difficulty: f32,
    pub consistency: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScoringStrategy {
    Simple,                    // Just correct/incorrect
    TimeWeighted {            // Factor in response time
        base_time_seconds: u32,
        penalty_per_second: f32,
    },
    DifficultyWeighted {      // Factor in question difficulty
        easy_multiplier: f32,
        medium_multiplier: f32,
        hard_multiplier: f32,
    },
    Adaptive {                // Comprehensive scoring
        time_weight: f32,
        difficulty_weight: f32,
        streak_weight: f32,
        consistency_weight: f32,
    },
}

impl ScoringStrategy {
    pub fn calculate_score(
        &self,
        session: &QuizSession,
        questions: &[Question],
    ) -> Score {
        match self {
            ScoringStrategy::Simple => self.simple_score(session, questions),
            ScoringStrategy::TimeWeighted { base_time_seconds, penalty_per_second } => {
                self.time_weighted_score(session, questions, *base_time_seconds, *penalty_per_second)
            }
            ScoringStrategy::DifficultyWeighted { easy_multiplier, medium_multiplier, hard_multiplier } => {
                self.difficulty_weighted_score(
                    session,
                    questions,
                    *easy_multiplier,
                    *medium_multiplier,
                    *hard_multiplier,
                )
            }
            ScoringStrategy::Adaptive { time_weight, difficulty_weight, streak_weight, consistency_weight } => {
                self.adaptive_score(
                    session,
                    questions,
                    *time_weight,
                    *difficulty_weight,
                    *streak_weight,
                    *consistency_weight,
                )
            }
        }
    }
    
    fn simple_score(&self, session: &QuizSession, questions: &[Question]) -> Score {
        let total = questions.len() as f32;
        let correct = session.responses.iter()
            .filter(|r| r.is_correct)
            .count() as f32;
        
        let raw_score = if total > 0.0 { correct / total } else { 0.0 };
        
        Score {
            raw_score,
            weighted_score: raw_score,
            percentile: None,
            time_bonus: 0.0,
            difficulty_bonus: 0.0,
            streak_bonus: 0.0,
            components: ScoreComponents {
                correctness: raw_score,
                speed: 0.0,
                difficulty: 0.0,
                consistency: 0.0,
            },
        }
    }
    
    fn time_weighted_score(
        &self,
        session: &QuizSession,
        questions: &[Question],
        base_time_seconds: u32,
        penalty_per_second: f32,
    ) -> Score {
        let mut total_score = 0.0;
        let question_map: std::collections::HashMap<_, _> = questions.iter()
            .map(|q| (q.id, q))
            .collect();
        
        for response in &session.responses {
            if let Some(question) = question_map.get(&response.question_id) {
                let base_points = if response.is_correct { 1.0 } else { 0.0 };
                let time_penalty = if response.time_taken_seconds > base_time_seconds {
                    (response.time_taken_seconds - base_time_seconds) as f32 * penalty_per_second
                } else {
                    0.0
                };
                
                let points = (base_points - time_penalty).max(0.0);
                total_score += points;
            }
        }
        
        let max_score = questions.len() as f32;
        let weighted_score = if max_score > 0.0 { total_score / max_score } else { 0.0 };
        
        Score {
            raw_score: self.simple_score(session, questions).raw_score,
            weighted_score,
            percentile: None,
            time_bonus: weighted_score - self.simple_score(session, questions).raw_score,
            difficulty_bonus: 0.0,
            streak_bonus: 0.0,
            components: ScoreComponents {
                correctness: self.simple_score(session, questions).raw_score,
                speed: weighted_score - self.simple_score(session, questions).raw_score,
                difficulty: 0.0,
                consistency: 0.0,
            },
        }
    }
    
    fn difficulty_weighted_score(
        &self,
        session: &QuizSession,
        questions: &[Question],
        easy_multiplier: f32,
        medium_multiplier: f32,
        hard_multiplier: f32,
    ) -> Score {
        let mut total_score = 0.0;
        let mut max_possible = 0.0;
        let question_map: std::collections::HashMap<_, _> = questions.iter()
            .map(|q| (q.id, q))
            .collect();
        
        for response in &session.responses {
            if let Some(question) = question_map.get(&response.question_id) {
                let multiplier = match question.difficulty {
                    d if d < 0.33 => easy_multiplier,
                    d if d < 0.67 => medium_multiplier,
                    _ => hard_multiplier,
                };
                
                max_possible += multiplier;
                if response.is_correct {
                    total_score += multiplier;
                }
            }
        }
        
        // Account for skipped questions
        for &question_index in &session.skipped_questions {
            if let Some(question) = questions.get(question_index) {
                let multiplier = match question.difficulty {
                    d if d < 0.33 => easy_multiplier,
                    d if d < 0.67 => medium_multiplier,
                    _ => hard_multiplier,
                };
                max_possible += multiplier;
            }
        }
        
        let weighted_score = if max_possible > 0.0 { total_score / max_possible } else { 0.0 };
        
        Score {
            raw_score: self.simple_score(session, questions).raw_score,
            weighted_score,
            percentile: None,
            time_bonus: 0.0,
            difficulty_bonus: weighted_score - self.simple_score(session, questions).raw_score,
            streak_bonus: 0.0,
            components: ScoreComponents {
                correctness: self.simple_score(session, questions).raw_score,
                speed: 0.0,
                difficulty: weighted_score - self.simple_score(session, questions).raw_score,
                consistency: 0.0,
            },
        }
    }
    
    fn adaptive_score(
        &self,
        session: &QuizSession,
        questions: &[Question],
        time_weight: f32,
        difficulty_weight: f32,
        streak_weight: f32,
        consistency_weight: f32,
    ) -> Score {
        let total_weight = time_weight + difficulty_weight + streak_weight + consistency_weight;
        
        // Calculate base correctness score
        let correctness_score = self.simple_score(session, questions).raw_score;
        
        // Calculate time score
        let avg_time: f32 = session.responses.iter()
            .map(|r| r.time_taken_seconds as f32)
            .sum::<f32>() / session.responses.len().max(1) as f32;
        let expected_avg_time: f32 = questions.iter()
            .map(|q| q.estimated_time_seconds as f32)
            .sum::<f32>() / questions.len().max(1) as f32;
        let time_score = (expected_avg_time / avg_time.max(1.0)).min(1.0);
        
        // Calculate difficulty score
        let difficulty_score = self.calculate_difficulty_score(session, questions);
        
        // Calculate streak score
        let streak_score = self.calculate_streak_score(&session.responses);
        
        // Calculate consistency score
        let consistency_score = self.calculate_consistency_score(&session.responses);
        
        // Combine scores
        let weighted_score = (
            correctness_score * 1.0 + // Base score always counts
            time_score * time_weight +
            difficulty_score * difficulty_weight +
            streak_score * streak_weight +
            consistency_score * consistency_weight
        ) / (1.0 + total_weight);
        
        Score {
            raw_score: correctness_score,
            weighted_score,
            percentile: None,
            time_bonus: time_score * time_weight,
            difficulty_bonus: difficulty_score * difficulty_weight,
            streak_bonus: streak_score * streak_weight,
            components: ScoreComponents {
                correctness: correctness_score,
                speed: time_score,
                difficulty: difficulty_score,
                consistency: consistency_score,
            },
        }
    }
    
    fn calculate_difficulty_score(&self, session: &QuizSession, questions: &[Question]) -> f32 {
        let question_map: std::collections::HashMap<_, _> = questions.iter()
            .map(|q| (q.id, q))
            .collect();
        
        let mut difficulty_sum = 0.0;
        let mut correct_difficulty_sum = 0.0;
        
        for response in &session.responses {
            if let Some(question) = question_map.get(&response.question_id) {
                difficulty_sum += question.difficulty;
                if response.is_correct {
                    correct_difficulty_sum += question.difficulty;
                }
            }
        }
        
        if difficulty_sum > 0.0 {
            correct_difficulty_sum / difficulty_sum
        } else {
            0.0
        }
    }
    
    fn calculate_streak_score(&self, responses: &[QuestionResponse]) -> f32 {
        if responses.is_empty() {
            return 0.0;
        }
        
        let mut max_streak = 0;
        let mut current_streak = 0;
        
        for response in responses {
            if response.is_correct {
                current_streak += 1;
                max_streak = max_streak.max(current_streak);
            } else {
                current_streak = 0;
            }
        }
        
        max_streak as f32 / responses.len() as f32
    }
    
    fn calculate_consistency_score(&self, responses: &[QuestionResponse]) -> f32 {
        if responses.len() < 2 {
            return 1.0; // Perfect consistency with 0 or 1 responses
        }
        
        // Calculate variance in response times
        let times: Vec<f32> = responses.iter()
            .map(|r| r.time_taken_seconds as f32)
            .collect();
        
        let mean_time = times.iter().sum::<f32>() / times.len() as f32;
        let variance = times.iter()
            .map(|t| (t - mean_time).powi(2))
            .sum::<f32>() / times.len() as f32;
        
        let std_dev = variance.sqrt();
        let cv = std_dev / mean_time; // Coefficient of variation
        
        // Lower CV means more consistent, map to 0-1 score
        (1.0 / (1.0 + cv)).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use crate::quiz::question::{QuestionType, Answer};
    
    #[test]
    fn test_simple_scoring() {
        let strategy = ScoringStrategy::Simple;
        let mut session = QuizSession::new(Uuid::new_v4(), None);
        
        // Create test questions
        let questions = vec![
            Question::new(
                QuestionType::TrueFalse {
                    statement: "Test 1".to_string(),
                    correct_answer: true,
                    explanation: None,
                },
                Uuid::new_v4(),
                0.5,
            ),
            Question::new(
                QuestionType::TrueFalse {
                    statement: "Test 2".to_string(),
                    correct_answer: false,
                    explanation: None,
                },
                Uuid::new_v4(),
                0.5,
            ),
        ];
        
        // Add responses
        session.responses.push(QuestionResponse {
            question_id: questions[0].id,
            answer: Answer::TrueFalse(true),
            is_correct: true,
            time_taken_seconds: 10,
            attempts: 1,
            submitted_at: chrono::Utc::now(),
        });
        
        session.responses.push(QuestionResponse {
            question_id: questions[1].id,
            answer: Answer::TrueFalse(true),
            is_correct: false,
            time_taken_seconds: 15,
            attempts: 1,
            submitted_at: chrono::Utc::now(),
        });
        
        let score = strategy.calculate_score(&session, &questions);
        assert_eq!(score.raw_score, 0.5); // 1 correct out of 2
        assert_eq!(score.weighted_score, 0.5);
    }
}
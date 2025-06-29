//! Test utilities for creating test data
//!
//! This module provides builders and factories for creating test instances
//! of various quiz-related types. These utilities are used in tests throughout
//! the crate to reduce boilerplate and ensure consistent test data.

#[cfg(test)]
pub mod builders {
    use crate::quiz::{Answer, Question, QuestionType, Quiz, QuizBuilder};
    use uuid::Uuid;

    /// Builder for creating test questions with sensible defaults
    pub struct TestQuestionBuilder {
        question_type: QuestionType,
        topic_id: Uuid,
        difficulty: f32,
        tags: Vec<String>,
        estimated_time: u32,
    }

    impl Default for TestQuestionBuilder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl TestQuestionBuilder {
        /// Create a new test question builder with a true/false question
        pub fn new() -> Self {
            Self {
                question_type: QuestionType::TrueFalse {
                    statement: "Test statement".to_string(),
                    correct_answer: true,
                    explanation: None,
                },
                topic_id: Uuid::new_v4(),
                difficulty: 0.5,
                tags: vec![],
                estimated_time: 30,
            }
        }

        /// Create a true/false question
        pub fn true_false(statement: &str, correct: bool) -> Self {
            Self {
                question_type: QuestionType::TrueFalse {
                    statement: statement.to_string(),
                    correct_answer: correct,
                    explanation: None,
                },
                ..Self::new()
            }
        }

        /// Create a multiple choice question
        pub fn multiple_choice(question: &str, options: Vec<&str>, correct_index: usize) -> Self {
            Self {
                question_type: QuestionType::MultipleChoice {
                    question: question.to_string(),
                    options: options.into_iter().map(|s| s.to_string()).collect(),
                    correct_index,
                    explanation: None,
                },
                ..Self::new()
            }
        }

        /// Create a multi-select question
        pub fn multi_select(
            question: &str,
            options: Vec<&str>,
            correct_indices: Vec<usize>,
        ) -> Self {
            Self {
                question_type: QuestionType::MultiSelect {
                    question: question.to_string(),
                    options: options.into_iter().map(|s| s.to_string()).collect(),
                    correct_indices,
                    explanation: None,
                },
                ..Self::new()
            }
        }

        /// Set the difficulty
        pub fn difficulty(mut self, difficulty: f32) -> Self {
            self.difficulty = difficulty;
            self
        }

        /// Set the topic ID
        pub fn topic(mut self, topic_id: Uuid) -> Self {
            self.topic_id = topic_id;
            self
        }

        /// Add a tag
        pub fn tag(mut self, tag: &str) -> Self {
            self.tags.push(tag.to_string());
            self
        }

        /// Set estimated time
        pub fn time(mut self, seconds: u32) -> Self {
            self.estimated_time = seconds;
            self
        }

        /// Build the question
        pub fn build(self) -> Question {
            let mut question = Question::new(self.question_type, self.topic_id, self.difficulty);
            question.tags = self.tags;
            question.estimated_time_seconds = self.estimated_time;
            question
        }
    }

    /// Builder for creating test quizzes
    pub struct TestQuizBuilder {
        title: String,
        questions: Vec<Question>,
        pass_threshold: f32,
    }

    impl TestQuizBuilder {
        /// Create a new test quiz builder
        pub fn new(title: &str) -> Self {
            Self {
                title: title.to_string(),
                questions: vec![],
                pass_threshold: 0.7,
            }
        }

        /// Add a question to the quiz
        pub fn question(mut self, question: Question) -> Self {
            self.questions.push(question);
            self
        }

        /// Add multiple questions
        pub fn questions(mut self, questions: Vec<Question>) -> Self {
            self.questions.extend(questions);
            self
        }

        /// Set pass threshold
        pub fn pass_threshold(mut self, threshold: f32) -> Self {
            self.pass_threshold = threshold;
            self
        }

        /// Build the quiz
        pub fn build(self) -> Quiz {
            let mut builder = QuizBuilder::new(self.title);
            builder = builder.pass_threshold(self.pass_threshold);
            for question in self.questions {
                builder = builder.add_question(question);
            }
            builder.build()
        }
    }

    /// Factory functions for common test scenarios
    pub mod factories {
        use super::*;

        /// Create a simple quiz with 3 questions of varying difficulty
        pub fn simple_quiz() -> Quiz {
            let topic_id = Uuid::new_v4();

            TestQuizBuilder::new("Simple Test Quiz")
                .question(
                    TestQuestionBuilder::true_false("Easy question", true)
                        .difficulty(0.2)
                        .topic(topic_id)
                        .build(),
                )
                .question(
                    TestQuestionBuilder::multiple_choice(
                        "Medium question",
                        vec!["Wrong", "Also wrong", "Correct", "Nope"],
                        2,
                    )
                    .difficulty(0.5)
                    .topic(topic_id)
                    .build(),
                )
                .question(
                    TestQuestionBuilder::multi_select(
                        "Hard question",
                        vec!["A", "B", "C", "D"],
                        vec![1, 3],
                    )
                    .difficulty(0.8)
                    .topic(topic_id)
                    .build(),
                )
                .build()
        }

        /// Create a quiz with all question types
        pub fn all_question_types_quiz() -> Quiz {
            let topic_id = Uuid::new_v4();

            TestQuizBuilder::new("All Question Types")
                .question(
                    TestQuestionBuilder::true_false("True or false?", false)
                        .topic(topic_id)
                        .build(),
                )
                .question(
                    TestQuestionBuilder::multiple_choice("Choose one", vec!["A", "B", "C"], 1)
                        .topic(topic_id)
                        .build(),
                )
                .question(
                    TestQuestionBuilder::multi_select(
                        "Choose multiple",
                        vec!["1", "2", "3", "4"],
                        vec![0, 2],
                    )
                    .topic(topic_id)
                    .build(),
                )
                .build()
        }

        /// Create a set of correct answers for a quiz
        pub fn correct_answers_for(quiz: &Quiz) -> Vec<Answer> {
            quiz.questions
                .iter()
                .map(|q| match &q.question_type {
                    QuestionType::TrueFalse { correct_answer, .. } => {
                        Answer::TrueFalse(*correct_answer)
                    }
                    QuestionType::MultipleChoice { correct_index, .. } => {
                        Answer::MultipleChoice(*correct_index)
                    }
                    QuestionType::MultiSelect {
                        correct_indices, ..
                    } => Answer::MultiSelect(correct_indices.clone()),
                    QuestionType::FillInTheBlank {
                        correct_answers, ..
                    } => Answer::FillInTheBlank(correct_answers.clone()),
                    QuestionType::MatchPairs { correct_pairs, .. } => {
                        Answer::MatchPairs(correct_pairs.clone())
                    }
                    _ => panic!("Unsupported question type in test"),
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::builders::*;

    #[test]
    fn test_question_builder() {
        let question = TestQuestionBuilder::true_false("Rust is fast", true)
            .difficulty(0.3)
            .tag("performance")
            .tag("rust")
            .time(20)
            .build();

        assert_eq!(question.difficulty, 0.3);
        assert_eq!(question.tags.len(), 2);
        assert_eq!(question.estimated_time_seconds, 20);
    }

    #[test]
    fn test_quiz_builder() {
        let quiz = TestQuizBuilder::new("Test Quiz")
            .pass_threshold(0.8)
            .question(TestQuestionBuilder::new().build())
            .question(TestQuestionBuilder::new().build())
            .build();

        assert_eq!(quiz.title, "Test Quiz");
        assert_eq!(quiz.pass_threshold, 0.8);
        assert_eq!(quiz.questions.len(), 2);
    }

    #[test]
    fn test_factories() {
        let simple = factories::simple_quiz();
        assert_eq!(simple.questions.len(), 3);

        let all_types = factories::all_question_types_quiz();
        assert_eq!(all_types.questions.len(), 3);

        let answers = factories::correct_answers_for(&simple);
        assert_eq!(answers.len(), simple.questions.len());
    }
}

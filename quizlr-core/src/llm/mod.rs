use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LlmProvider {
    Claude,
    Gemini,
    OpenAI,
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String, crate::error::QuizlrError>;
}

pub struct LlmManager {
    // Placeholder for LLM integration
}

impl LlmManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for LlmManager {
    fn default() -> Self {
        Self::new()
    }
}

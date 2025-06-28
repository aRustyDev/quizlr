use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageBackend {
    Local,
    GitHub,
    Custom(String),
}

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save(&self, key: &str, data: &[u8]) -> Result<(), crate::error::QuizlrError>;
    async fn load(&self, key: &str) -> Result<Vec<u8>, crate::error::QuizlrError>;
    async fn delete(&self, key: &str) -> Result<(), crate::error::QuizlrError>;
    async fn list(&self, prefix: &str) -> Result<Vec<String>, crate::error::QuizlrError>;
}

pub struct StorageManager {
    // Placeholder for storage implementation
}

impl StorageManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for StorageManager {
    fn default() -> Self {
        Self::new()
    }
}
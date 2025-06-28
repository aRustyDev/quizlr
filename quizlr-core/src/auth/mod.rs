use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthProvider {
    Google,
    GitHub,
    Microsoft,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub provider: AuthProvider,
}

pub struct AuthManager {
    // Placeholder for auth implementation
}

impl AuthManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}
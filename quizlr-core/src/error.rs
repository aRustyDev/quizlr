use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuizlrError {
    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("LLM API error: {0}")]
    LlmApi(String),

    #[error("Quiz engine error: {0}")]
    QuizEngine(String),

    #[error("Graph operation error: {0}")]
    Graph(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, QuizlrError>;

#[cfg(target_arch = "wasm32")]
impl From<QuizlrError> for wasm_bindgen::JsValue {
    fn from(error: QuizlrError) -> Self {
        wasm_bindgen::JsValue::from_str(&error.to_string())
    }
}

# LLM Providers

Guide for integrating language models with Quizlr.

## Supported Providers

Quizlr supports multiple LLM providers:
- OpenAI (GPT-3.5, GPT-4)
- Anthropic (Claude)
- Local models via Ollama

## Adding a New Provider

1. **Implement the LLMProvider trait**:
   ```rust
   pub trait LLMProvider {
       async fn generate_question(&self, topic: &str) -> Result<Question>;
       async fn validate_answer(&self, question: &Question, answer: &str) -> Result<bool>;
   }
   ```

2. **Handle API authentication**:
   - Secure key storage
   - Rate limiting
   - Error handling

3. **Configure prompts**:
   - Question generation templates
   - Answer validation logic

## Example: Custom Provider

```rust
pub struct CustomLLM {
    api_key: String,
    endpoint: String,
}

impl LLMProvider for CustomLLM {
    async fn generate_question(&self, topic: &str) -> Result<Question> {
        // Call your LLM API
        // Parse response into Question
    }
}
```

For detailed integration docs, see [LLM Integration](../reference/llm-integration.md).
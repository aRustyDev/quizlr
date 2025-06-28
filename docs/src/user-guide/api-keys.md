# API Keys

This guide explains how to obtain, configure, and manage API keys for Quizlr's AI-powered features.

## Overview

Quizlr integrates with various AI services to provide:
- Automatic quiz generation
- Intelligent question creation
- Answer validation
- Content suggestions
- Learning assistance

```
Supported AI Providers
├─ OpenAI (GPT-3.5, GPT-4)
├─ Anthropic (Claude)
├─ Google (Gemini)
├─ Cohere
├─ Open Source (Ollama, LlamaCpp)
└─ Custom Endpoints
```

## Getting API Keys

### OpenAI

1. **Sign up** at [platform.openai.com](https://platform.openai.com)
2. Navigate to **API Keys** section
3. Click **Create new secret key**
4. Copy the key (starts with `sk-`)
5. Save it securely - you won't see it again!

**Pricing:** 
- GPT-3.5: ~$0.002 per 1K tokens
- GPT-4: ~$0.03 per 1K tokens
- Free tier: $5 credit for new users

### Anthropic (Claude)

1. **Apply for access** at [console.anthropic.com](https://console.anthropic.com)
2. Once approved, go to **API Keys**
3. Generate a new key
4. Copy the key (starts with `sk-ant-`)

**Pricing:**
- Claude Instant: ~$0.002 per 1K tokens
- Claude 2: ~$0.008 per 1K tokens
- Free tier: Limited availability

### Google Gemini

1. **Visit** [makersuite.google.com](https://makersuite.google.com)
2. Click **Get API Key**
3. Create a new project or select existing
4. Generate API key
5. Enable Gemini API for your project

**Pricing:**
- Gemini Pro: Free tier available
- 60 requests per minute limit
- Paid tiers for higher usage

### Cohere

1. **Sign up** at [dashboard.cohere.ai](https://dashboard.cohere.ai)
2. Navigate to **API Keys**
3. Create a trial or production key
4. Copy the generated key

**Pricing:**
- Free tier: 1000 API calls/month
- Generate: ~$0.015 per 1K tokens

## Adding API Keys to Quizlr

### Method 1: Through Settings UI

1. Open Quizlr
2. Go to **Settings** → **API Configuration**
3. Click **Add API Key**
4. Select provider and enter details:

```
Provider: OpenAI
Key Name: Personal GPT-4 Key
API Key: sk-...........................
Model: gpt-4
[✓] Set as default for quiz generation
```

### Method 2: Configuration File

Edit `config.yaml`:

```yaml
api_keys:
  openai:
    - name: "primary"
      key: "${OPENAI_API_KEY}"  # Read from environment
      model: "gpt-4"
      default: true
    - name: "backup"
      key: "sk-..."  # Direct key (not recommended)
      model: "gpt-3.5-turbo"
      
  anthropic:
    - name: "claude-main"
      key: "${CLAUDE_API_KEY}"
      model: "claude-3-opus"
      
  google:
    - name: "gemini"
      key: "${GEMINI_API_KEY}"
      model: "gemini-pro"
```

### Method 3: Environment Variables

Most secure method:

```bash
# .env file (git ignored)
QUIZLR_OPENAI_KEY=sk-...
QUIZLR_CLAUDE_KEY=sk-ant-...
QUIZLR_GEMINI_KEY=...

# Or export in shell
export QUIZLR_OPENAI_KEY="sk-..."
```

## API Key Security

### Best Practices

1. **Never commit keys to version control**
   ```gitignore
   # .gitignore
   .env
   .env.local
   config.local.yaml
   **/api_keys.json
   ```

2. **Use environment variables**
   ```yaml
   # Good - Reference environment variable
   key: "${OPENAI_API_KEY}"
   
   # Bad - Hardcoded key
   key: "sk-abc123..."
   ```

3. **Rotate keys regularly**
   - Set reminders every 90 days
   - Immediately rotate if exposed
   - Keep backup keys ready

4. **Use separate keys for different environments**
   ```yaml
   development:
     key: "${OPENAI_DEV_KEY}"
   production:
     key: "${OPENAI_PROD_KEY}"
   ```

### Key Storage

Quizlr stores API keys securely:

**macOS:** Keychain
```bash
# View stored keys
security find-generic-password -s "Quizlr" -a "openai"
```

**Windows:** Credential Manager
```powershell
# View stored credentials
cmdkey /list:Quizlr*
```

**Linux:** Secret Service (GNOME Keyring/KWallet)
```bash
# Using secret-tool
secret-tool lookup application Quizlr provider openai
```

### Encryption

All API keys are encrypted at rest:

```yaml
security:
  encryption:
    algorithm: "AES-256-GCM"
    key_derivation: "PBKDF2"
    iterations: 100000
```

## Managing Multiple Keys

### Key Rotation

Set up automatic rotation:

```yaml
api_management:
  rotation:
    enabled: true
    interval_days: 90
    notify_before_days: 7
    
  usage_limits:
    daily_limit: 1000
    monthly_limit: 25000
    alert_at_percent: 80
```

### Load Balancing

Distribute requests across multiple keys:

```yaml
load_balancing:
  strategy: "round-robin"  # round-robin, least-used, weighted
  
  keys:
    - name: "key1"
      weight: 2  # Gets 2x more requests
      rate_limit: 100  # requests per minute
    - name: "key2"
      weight: 1
      rate_limit: 60
```

### Fallback Configuration

Handle API failures gracefully:

```yaml
fallback:
  enabled: true
  order:
    - openai_primary
    - openai_backup
    - anthropic_primary
    - google_gemini
    
  retry_policy:
    max_attempts: 3
    backoff: "exponential"
    initial_delay_ms: 1000
```

## Usage Monitoring

### Built-in Dashboard

View API usage in Quizlr:

```
API Usage Dashboard
├─ OpenAI
│  ├─ Requests Today: 156
│  ├─ Tokens Used: 45,230
│  ├─ Estimated Cost: $1.36
│  └─ Rate Limit: 45% used
├─ Claude
│  ├─ Requests Today: 89
│  ├─ Tokens Used: 23,100
│  └─ Estimated Cost: $0.42
```

### Usage Alerts

Configure usage notifications:

```yaml
monitoring:
  alerts:
    - type: "daily_limit"
      threshold: 80  # percent
      notify: ["email", "in_app"]
    - type: "cost"
      threshold: 10.00  # dollars
      notify: ["email"]
    - type: "rate_limit"
      threshold: 90  # percent
      notify: ["in_app"]
```

### Export Usage Data

Export usage for analysis:

```bash
# Export as CSV
quizlr export-usage --format csv --output usage.csv

# Export date range
quizlr export-usage --from 2024-01-01 --to 2024-01-31
```

## Provider-Specific Features

### OpenAI Features

```yaml
openai:
  features:
    function_calling: true
    json_mode: true
    vision: true  # For GPT-4V
    
  parameters:
    temperature: 0.7
    top_p: 1.0
    frequency_penalty: 0.0
    presence_penalty: 0.0
```

### Claude Features

```yaml
anthropic:
  features:
    constitutional_ai: true
    long_context: true  # 100K+ tokens
    
  parameters:
    temperature: 0.7
    max_tokens: 4000
```

### Model Selection

Choose models based on use case:

| Use Case | Recommended Model | Why |
|----------|------------------|-----|
| Quick quiz generation | GPT-3.5 | Fast, cheap, good quality |
| Complex topics | GPT-4 | Better reasoning |
| Long content | Claude-3 | 200K context window |
| Free usage | Gemini Pro | Generous free tier |
| Offline | Ollama | Runs locally |

## Local AI Models

### Using Ollama

Run AI models locally without API keys:

1. **Install Ollama**:
   ```bash
   # macOS/Linux
   curl -fsSL https://ollama.ai/install.sh | sh
   
   # Pull a model
   ollama pull llama2
   ```

2. **Configure in Quizlr**:
   ```yaml
   local_ai:
     provider: "ollama"
     endpoint: "http://localhost:11434"
     model: "llama2"
     enabled: true
   ```

### Using LlamaCpp

For more control over local models:

```yaml
local_ai:
  provider: "llamacpp"
  model_path: "~/models/llama-2-7b.gguf"
  context_size: 4096
  gpu_layers: 32  # GPU acceleration
```

## Troubleshooting

### Common Issues

**Invalid API Key:**
```
Error: OpenAI API key invalid
Solution: 
1. Check key starts with correct prefix (sk-)
2. Verify key hasn't been rotated
3. Ensure no extra spaces
```

**Rate Limiting:**
```
Error: Rate limit exceeded
Solution:
1. Wait for limit reset
2. Use backup keys
3. Upgrade API plan
4. Implement request queuing
```

**Network Issues:**
```
Error: Failed to connect to API
Solution:
1. Check internet connection
2. Verify firewall settings
3. Test with curl:
   curl https://api.openai.com/v1/models \
     -H "Authorization: Bearer $OPENAI_API_KEY"
```

### Debug Mode

Enable detailed API logging:

```yaml
debug:
  api_logging:
    enabled: true
    log_requests: true
    log_responses: false  # Be careful - may log sensitive data
    log_file: "~/quizlr-api.log"
```

### Testing API Keys

Test keys without using credits:

```bash
# Built-in test command
quizlr test-api --provider openai --key sk-...

# Test all configured keys
quizlr test-api --all
```

## Cost Optimization

### Strategies

1. **Use appropriate models**:
   - Simple tasks: GPT-3.5 or Claude Instant
   - Complex tasks: GPT-4 or Claude 2

2. **Cache responses**:
   ```yaml
   caching:
     enabled: true
     ttl_hours: 24
     max_size_mb: 100
   ```

3. **Batch requests**:
   ```yaml
   batching:
     enabled: true
     batch_size: 10
     wait_time_ms: 100
   ```

4. **Implement token limits**:
   ```yaml
   limits:
     max_tokens_per_request: 2000
     max_tokens_per_day: 100000
   ```

### Cost Tracking

Monitor spending:

```
Monthly API Costs
├─ OpenAI: $24.50
│  ├─ GPT-4: $18.30 (610 requests)
│  └─ GPT-3.5: $6.20 (3,100 requests)
├─ Claude: $12.80
└─ Total: $37.30

Daily Average: $1.24
Projected Monthly: $37.30
Budget Remaining: $12.70 (75% used)
```

## Integration Examples

### Quiz Generation

```yaml
quiz_generation:
  provider: "openai"
  model: "gpt-4"
  prompt_template: |
    Create {num_questions} questions about {topic}.
    Difficulty: {difficulty}
    Types: {question_types}
    Format: JSON
```

### Answer Validation

```yaml
answer_validation:
  provider: "anthropic"
  model: "claude-instant"
  prompt_template: |
    Evaluate this answer:
    Question: {question}
    Student Answer: {answer}
    Correct Answer: {correct}
    Provide feedback and partial credit.
```

## Next Steps

- [Configure Authentication](./authentication.md) for security
- [Explore Data Sources](./data-sources.md) for content
- [Set Up Data Management](./data-management.md)
- [Review Configuration](./configuration.md) options

## Related Resources

- [OpenAI Documentation](https://platform.openai.com/docs)
- [Anthropic Documentation](https://docs.anthropic.com)
- [Google AI Documentation](https://ai.google.dev)
- [Quizlr API Reference](../reference/api.md)
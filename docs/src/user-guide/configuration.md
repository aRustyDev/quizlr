# Configuration

This guide covers all configuration options in Quizlr, from basic preferences to advanced customization.

## Configuration Overview

Quizlr's configuration is organized into several categories:

```
Settings
├─ General
│  ├─ Language & Region
│  ├─ Theme & Appearance
│  └─ Default Quiz Settings
├─ Learning
│  ├─ Adaptive Algorithm
│  ├─ Spaced Repetition
│  └─ Performance Tracking
├─ Storage & Sync
│  ├─ Local Storage
│  ├─ Cloud Sync
│  └─ Backup Schedule
└─ Advanced
   ├─ API Configuration
   ├─ Developer Options
   └─ Experimental Features
```

## General Settings

### Language & Region

Configure your language and regional preferences:

```yaml
# config.yaml
general:
  language: "en-US"
  region: "US"
  date_format: "MM/DD/YYYY"
  time_format: "12h"
  week_start: "sunday"
```

**Available Languages:**
- English (US/UK)
- Spanish
- French
- German
- Japanese
- Chinese (Simplified/Traditional)

### Theme & Appearance

Customize Quizlr's look and feel:

```yaml
appearance:
  theme: "auto"  # light, dark, auto
  accent_color: "#0066CC"
  font_size: "medium"  # small, medium, large
  font_family: "system"
  animations: true
  reduced_motion: false
```

**Theme Options:**
- **Light Mode**: Best for daytime use
- **Dark Mode**: Easier on eyes in low light
- **Auto**: Follows system preferences

### Default Quiz Settings

Set defaults for new quizzes:

```yaml
quiz_defaults:
  questions_per_session: 20
  time_limit: null  # minutes, null for unlimited
  pass_threshold: 70  # percentage
  allow_skip: true
  show_explanations: "after_quiz"  # immediate, after_quiz, never
  randomize_questions: true
  randomize_answers: true
```

## Learning Configuration

### Adaptive Algorithm

Fine-tune how Quizlr adapts to your performance:

```yaml
adaptive_learning:
  enabled: true
  algorithm: "elo"  # elo, simple, custom
  
  # ELO Rating System
  elo_settings:
    initial_rating: 1200
    k_factor: 32
    difficulty_bands:
      easy: [0, 1000]
      medium: [1001, 1400]
      hard: [1401, 2000]
  
  # Adjustment Rules
  adjustment:
    consecutive_correct: 3  # Questions before difficulty increase
    consecutive_wrong: 2    # Questions before difficulty decrease
    confidence_threshold: 0.8
```

### Spaced Repetition

Configure the spaced repetition algorithm:

```yaml
spaced_repetition:
  enabled: true
  algorithm: "sm2"  # sm2, anki, custom
  
  # SuperMemo 2 Algorithm
  sm2_settings:
    initial_interval: 1  # days
    easy_bonus: 1.3
    interval_modifier: 1.0
    max_interval: 365
    
  # Review Scheduling
  scheduling:
    new_cards_per_day: 20
    review_cards_per_day: 100
    learning_steps: [1, 10]  # minutes
    relearning_steps: [10]   # minutes
```

### Performance Tracking

Set up metrics and analytics:

```yaml
performance:
  track_time: true
  track_attempts: true
  
  # Metrics to Calculate
  metrics:
    - accuracy_rate
    - speed_improvement
    - topic_mastery
    - difficulty_progression
    - retention_rate
  
  # Reporting
  reports:
    daily_summary: true
    weekly_digest: true
    monthly_analysis: true
    export_format: "pdf"  # pdf, csv, json
```

## Storage & Sync Configuration

### Local Storage

Configure where and how data is stored locally:

```yaml
storage:
  # Storage Location
  data_directory: "~/Library/Application Support/Quizlr"  # macOS
  # data_directory: "%APPDATA%/Quizlr"  # Windows
  # data_directory: "~/.config/quizlr"  # Linux
  
  # Database Settings
  database:
    type: "sqlite"
    path: "quizlr.db"
    wal_mode: true
    auto_vacuum: true
    
  # File Storage
  files:
    max_size_mb: 100
    allowed_types: ["json", "yaml", "md", "csv"]
    compression: true
```

### Cloud Sync

Set up synchronization with cloud services:

```yaml
sync:
  provider: "github"  # github, gitlab, custom
  enabled: true
  
  # GitHub Configuration
  github:
    repo: "username/quizlr-data"
    branch: "main"
    path: "/quizzes"
    auto_sync: true
    sync_interval: 300  # seconds
    
  # Sync Options
  options:
    sync_on_startup: true
    sync_on_change: true
    sync_on_exit: true
    conflict_resolution: "newest"  # newest, prompt, merge
```

### Backup Configuration

Automated backup settings:

```yaml
backup:
  enabled: true
  
  # Backup Schedule
  schedule:
    frequency: "daily"  # hourly, daily, weekly
    time: "02:00"
    keep_versions: 7
    
  # Backup Destinations
  destinations:
    - type: "local"
      path: "~/Documents/Quizlr Backups"
    - type: "github"
      repo: "username/quizlr-backups"
      
  # Backup Options
  options:
    include_media: true
    compress: true
    encrypt: false
    format: "zip"  # zip, tar.gz
```

## Advanced Configuration

### API Configuration

Configure external API integrations:

```yaml
apis:
  # OpenAI Configuration
  openai:
    enabled: true
    model: "gpt-4"
    temperature: 0.7
    max_tokens: 2000
    
  # Claude Configuration
  claude:
    enabled: true
    model: "claude-3-opus"
    max_tokens: 4000
    
  # Custom LLM
  custom_llm:
    enabled: false
    endpoint: "https://api.example.com/v1/completions"
    headers:
      Authorization: "Bearer ${CUSTOM_API_KEY}"
```

### Developer Options

Advanced options for power users:

```yaml
developer:
  debug_mode: false
  verbose_logging: false
  
  # Console Options
  console:
    enabled: false
    log_level: "info"  # debug, info, warn, error
    
  # Network
  network:
    timeout: 30  # seconds
    retry_count: 3
    proxy: null
    
  # Cache
  cache:
    enabled: true
    size_mb: 100
    ttl: 3600  # seconds
```

### Experimental Features

Enable beta features:

```yaml
experimental:
  # Feature Flags
  features:
    ai_tutor: false
    voice_questions: false
    collaborative_mode: false
    ar_mode: false
    
  # Beta Options
  beta:
    channel: "stable"  # stable, beta, nightly
    auto_update: false
    telemetry: false
```

## Configuration Files

### File Locations

Configuration files are stored in platform-specific locations:

**macOS:**
```
~/Library/Application Support/Quizlr/
├── config.yaml          # Main configuration
├── preferences.json     # UI preferences
└── cache/              # Temporary files
```

**Windows:**
```
%APPDATA%\Quizlr\
├── config.yaml
├── preferences.json
└── cache\
```

**Linux:**
```
~/.config/quizlr/
├── config.yaml
├── preferences.json
└── cache/
```

### Loading Order

Configuration is loaded in this order (later overrides earlier):

1. Built-in defaults
2. System-wide config (`/etc/quizlr/config.yaml`)
3. User config (`~/.config/quizlr/config.yaml`)
4. Environment variables (`QUIZLR_*`)
5. Command-line arguments

### Environment Variables

Override configuration with environment variables:

```bash
# API Keys
export QUIZLR_OPENAI_KEY="sk-..."
export QUIZLR_CLAUDE_KEY="sk-ant-..."

# Paths
export QUIZLR_DATA_DIR="/custom/path"
export QUIZLR_CONFIG_FILE="/custom/config.yaml"

# Features
export QUIZLR_DEBUG=true
export QUIZLR_THEME=dark
```

## Import/Export Configuration

### Exporting Configuration

Export your configuration for backup or sharing:

1. Go to **Settings** → **Advanced** → **Export**
2. Choose what to export:
   - Configuration only
   - Configuration + Data
   - Full backup
3. Select format (YAML, JSON, ZIP)
4. Save to file

### Importing Configuration

Import configuration from another installation:

1. Go to **Settings** → **Advanced** → **Import**
2. Select configuration file
3. Review changes
4. Choose merge strategy:
   - Replace all
   - Merge (keep existing)
   - Interactive merge

### Configuration Templates

Use pre-made templates for common scenarios:

**Student Template:**
```yaml
# Optimized for studying
adaptive_learning:
  enabled: true
  algorithm: "elo"
spaced_repetition:
  enabled: true
  new_cards_per_day: 30
performance:
  track_time: true
  daily_summary: true
```

**Teacher Template:**
```yaml
# Optimized for creating content
quiz_defaults:
  show_explanations: "immediate"
  allow_skip: false
apis:
  openai:
    enabled: true
    model: "gpt-4"
developer:
  debug_mode: true
```

## Validation & Troubleshooting

### Configuration Validation

Quizlr validates configuration on startup:

```
Validating configuration...
✓ General settings
✓ Learning configuration
✓ Storage paths accessible
✗ API key missing for OpenAI
✓ Sync configuration

1 warning found. Continue anyway? [Y/n]
```

### Common Issues

**Invalid Configuration:**
```yaml
# WRONG - Invalid theme
appearance:
  theme: "midnight"  # Not a valid option

# CORRECT
appearance:
  theme: "dark"
```

**Path Issues:**
```yaml
# WRONG - Relative path
storage:
  data_directory: "./data"

# CORRECT - Absolute path
storage:
  data_directory: "~/Documents/Quizlr/data"
```

### Reset Configuration

Reset to defaults if needed:

```bash
# Reset all settings
quizlr --reset-config

# Reset specific section
quizlr --reset-config appearance

# Backup before reset
quizlr --backup-config --reset-config
```

## Best Practices

### Security

1. **Never commit API keys** to version control
2. Use environment variables for sensitive data
3. Enable encryption for cloud sync
4. Regularly rotate API keys

### Performance

1. **Disable unused features** to save resources
2. Limit cache size on low-storage devices
3. Adjust sync frequency based on usage
4. Use local storage for better performance

### Organization

1. **Use comments** in YAML files:
   ```yaml
   # This controls quiz difficulty progression
   adaptive_learning:
     enabled: true  # Set false for fixed difficulty
   ```

2. **Group related settings** logically
3. **Document custom values** for future reference
4. **Version your configuration** with Git

## Next Steps

- [Set up API Keys](./api-keys.md) for AI features
- [Configure Authentication](./authentication.md) for security
- [Explore Data Sources](./data-sources.md) for content
- [Review Troubleshooting](./troubleshooting.md) for issues

## Related Documentation

- [Getting Started](./getting-started.md)
- [Advanced Features](./advanced-features.md)
- [Developer Guide](../developer-guide/README.md)
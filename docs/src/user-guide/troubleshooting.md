# Troubleshooting

This guide helps you resolve common issues with Quizlr and provides solutions for various problems you might encounter.

## Troubleshooting Overview

Quick navigation to common issues:

```
Common Issues
├─ Installation Problems
│  ├─ Dependencies
│  ├─ Permissions
│  └─ Platform-specific
├─ Performance Issues
│  ├─ Slow Loading
│  ├─ High Memory Usage
│  └─ Freezing/Crashes
├─ Data & Sync Issues
│  ├─ Sync Failures
│  ├─ Data Loss
│  └─ Import Errors
├─ Authentication Problems
│  ├─ Login Issues
│  ├─ OAuth Errors
│  └─ Session Problems
└─ Feature-Specific Issues
   ├─ AI Generation
   ├─ Quiz Taking
   └─ Export/Import
```

## Installation Issues

### Dependencies Missing

**Problem:** Error about missing dependencies during installation

**Solution:**
```bash
# Check system requirements
quizlr doctor

# Common fixes by platform:

# macOS
brew install sqlite3
brew install git

# Ubuntu/Debian
sudo apt-get update
sudo apt-get install libsqlite3-dev
sudo apt-get install git

# Windows (using chocolatey)
choco install sqlite
choco install git
```

### Permission Errors

**Problem:** "Permission denied" errors

**Solution:**
```bash
# macOS/Linux - Fix permissions
sudo chown -R $(whoami) ~/.quizlr
chmod -R 755 ~/.quizlr

# Windows - Run as Administrator
# Right-click Quizlr → Run as administrator

# Alternative: Install in user directory
quizlr install --user-dir ~/my-quizlr
```

### Platform-Specific Issues

**macOS - "App is damaged":**
```bash
# Clear quarantine attribute
xattr -cr /Applications/Quizlr.app

# If still blocked
# System Preferences → Security & Privacy → Allow
```

**Windows - SmartScreen Warning:**
```
1. Click "More info"
2. Click "Run anyway"
3. Or disable SmartScreen temporarily
```

**Linux - Desktop Entry Missing:**
```bash
# Create desktop entry
cat > ~/.local/share/applications/quizlr.desktop << EOF
[Desktop Entry]
Name=Quizlr
Exec=/usr/local/bin/quizlr
Icon=quizlr
Type=Application
Categories=Education;
EOF
```

## Performance Issues

### Slow Loading

**Symptoms:** Quizlr takes long to start or load quizzes

**Diagnostics:**
```bash
# Check startup time
quizlr --profile startup

# Results:
Database init: 245ms
Plugin load: 890ms  ← Slow!
UI render: 156ms
Total: 1291ms
```

**Solutions:**

1. **Disable unnecessary plugins:**
   ```yaml
   plugins:
     disabled:
       - "heavy-analytics"
       - "unused-integration"
   ```

2. **Optimize database:**
   ```bash
   quizlr db optimize
   quizlr db vacuum
   ```

3. **Clear cache:**
   ```bash
   quizlr cache clear --all
   ```

4. **Reduce startup items:**
   ```yaml
   startup:
     preload_quizzes: false
     check_updates: false
     sync_on_start: false
   ```

### High Memory Usage

**Symptoms:** Quizlr using excessive RAM

**Monitor usage:**
```
Memory Usage Monitor
├─ Quizlr Process: 847 MB
├─ Breakdown:
│  ├─ UI: 234 MB
│  ├─ Database: 156 MB
│  ├─ Cache: 389 MB ← Too high!
│  └─ Plugins: 68 MB
```

**Solutions:**

1. **Limit cache size:**
   ```yaml
   cache:
     memory:
       max_size_mb: 100  # Was 500
       ttl_minutes: 30   # Was 120
   ```

2. **Reduce media quality:**
   ```yaml
   media:
     image_quality: 80  # Was 100
     max_size_kb: 500   # Was 2000
     lazy_load: true
   ```

3. **Disable memory-intensive features:**
   ```yaml
   features:
     live_preview: false
     auto_save_interval: 300  # Was 10
   ```

### Freezing/Crashes

**Symptoms:** Quizlr becomes unresponsive or crashes

**Debug steps:**

1. **Check logs:**
   ```bash
   # View recent logs
   quizlr logs --tail 100
   
   # Watch logs in real-time
   quizlr logs --follow
   ```

2. **Run in safe mode:**
   ```bash
   quizlr --safe-mode
   ```

3. **Generate crash report:**
   ```bash
   quizlr debug --crash-report
   ```

**Common fixes:**

```yaml
# Disable problematic features
stability:
  disable_animations: true
  disable_gpu_acceleration: true
  single_process_mode: true
  
# Increase timeouts
timeouts:
  api_calls: 30000     # 30 seconds
  database_ops: 10000  # 10 seconds
```

## Data & Sync Issues

### Sync Failures

**Problem:** "Sync failed" or conflicts

**Diagnostic:**
```
Sync Status
├─ Last Sync: Failed (2 hours ago)
├─ Error: Merge conflict in quiz_123
├─ Pending Changes: 15
└─ Conflicts: 3
```

**Solutions:**

1. **Force sync:**
   ```bash
   # Pull latest changes
   quizlr sync --force-pull
   
   # Push local changes
   quizlr sync --force-push
   
   # Reset sync state
   quizlr sync --reset
   ```

2. **Resolve conflicts:**
   ```yaml
   sync:
     conflict_resolution:
       strategy: "manual"  # or "newest", "local", "remote"
       
     merge_tool: "vimdiff"  # or "meld", "kdiff3"
   ```

3. **Check connectivity:**
   ```bash
   # Test sync endpoint
   quizlr sync --test
   
   # Check API status
   curl -I https://api.github.com
   ```

### Data Loss Prevention

**Problem:** Lost quizzes or progress

**Recovery options:**

1. **Check auto-saves:**
   ```bash
   # List auto-saved versions
   quizlr recover --list
   
   # Recover specific item
   quizlr recover --id quiz_123
   ```

2. **Browse backups:**
   ```
   Backup Browser
   ├─ Local Backups
   │  ├─ 2024-01-20-0200.zip (Daily)
   │  ├─ 2024-01-13-0200.zip (Weekly)
   │  └─ 2024-01-01-0200.zip (Monthly)
   └─ Cloud Backups
      └─ [Browse Cloud Storage]
   ```

3. **Emergency recovery:**
   ```bash
   # Scan for recoverable data
   quizlr recover --deep-scan
   
   # Restore from transaction log
   quizlr recover --from-wal
   ```

### Import Errors

**Problem:** Failed to import quiz content

**Common errors and fixes:**

1. **"Invalid format":**
   ```bash
   # Validate file
   quizlr validate quiz.json
   
   # Auto-fix common issues
   quizlr import quiz.json --fix-errors
   ```

2. **"Encoding error":**
   ```bash
   # Specify encoding
   quizlr import quiz.csv --encoding utf-8
   
   # Convert encoding
   iconv -f ISO-8859-1 -t UTF-8 quiz.csv > quiz-utf8.csv
   ```

3. **"Size limit exceeded":**
   ```yaml
   import:
     limits:
       max_file_size_mb: 100  # Increase limit
       chunk_size: 1000       # Process in chunks
   ```

## Authentication Problems

### Login Issues

**Problem:** Cannot log in with correct credentials

**Troubleshooting steps:**

1. **Check caps lock** and keyboard layout
2. **Clear cookies:**
   ```bash
   quizlr auth --clear-cookies
   ```
3. **Reset password:**
   ```bash
   quizlr auth --reset-password
   ```
4. **Check authentication method:**
   ```yaml
   auth:
     methods:
       - local     # Username/password
       - oauth     # Social login
       - biometric # Touch/Face ID
   ```

### OAuth Errors

**Problem:** "OAuth authentication failed"

**Common fixes:**

1. **GitHub OAuth:**
   ```bash
   # Re-authorize
   quizlr auth logout --provider github
   quizlr auth login --provider github
   
   # Check token permissions
   quizlr auth check --provider github
   ```

2. **Token expired:**
   ```yaml
   oauth:
     auto_refresh: true
     refresh_before_expiry: 300  # 5 minutes
   ```

3. **Callback URL mismatch:**
   ```
   Registered: http://localhost:3000/auth/callback
   Actual: http://127.0.0.1:3000/auth/callback
   
   Fix: Use consistent URLs
   ```

### Session Problems

**Problem:** Getting logged out repeatedly

**Solutions:**

1. **Extend session timeout:**
   ```yaml
   session:
     timeout_minutes: 1440  # 24 hours
     remember_me: true
     refresh_on_activity: true
   ```

2. **Fix clock sync:**
   ```bash
   # Check system time
   date
   
   # Sync time (varies by OS)
   # macOS
   sudo sntp -sS time.apple.com
   
   # Linux
   sudo ntpdate pool.ntp.org
   ```

3. **Disable strict mode:**
   ```yaml
   session:
     strict_mode: false
     bind_to_ip: false
     check_user_agent: false
   ```

## Feature-Specific Issues

### AI Generation Problems

**Problem:** AI quiz generation fails or gives poor results

**Diagnostics:**
```bash
# Test AI connection
quizlr ai test

# Check API key
quizlr ai check-key

# View AI logs
quizlr logs --filter ai
```

**Solutions:**

1. **API key issues:**
   ```bash
   # Re-enter API key
   quizlr config set openai.api_key "sk-..."
   
   # Test with curl
   curl https://api.openai.com/v1/models \
     -H "Authorization: Bearer $OPENAI_API_KEY"
   ```

2. **Rate limiting:**
   ```yaml
   ai:
     rate_limit:
       requests_per_minute: 20  # Reduce if hitting limits
       retry_on_limit: true
       backoff_seconds: 60
   ```

3. **Poor quality results:**
   ```yaml
   ai:
     generation:
       model: "gpt-4"  # Use better model
       temperature: 0.7
       
     prompts:
       system: |
         You are an expert educator creating high-quality quiz questions.
         Ensure questions are clear, accurate, and educational.
       
       examples: true  # Include examples in prompt
   ```

### Quiz Taking Issues

**Problem:** Questions not displaying correctly

**Fixes by symptom:**

1. **Missing images:**
   ```yaml
   media:
     fallback_image: "/assets/placeholder.png"
     retry_failed: true
     cache_duration: 86400
   ```

2. **Math formulas not rendering:**
   ```bash
   # Install MathJax
   quizlr plugins install mathjax
   
   # Or use KaTeX
   quizlr plugins install katex
   ```

3. **Code highlighting broken:**
   ```yaml
   highlighting:
     engine: "prism"  # or "highlight.js"
     theme: "tomorrow"
     languages: ["javascript", "python", "java"]
   ```

### Export/Import Issues

**Problem:** Export fails or produces corrupted files

**Solutions:**

1. **Memory issues with large exports:**
   ```yaml
   export:
     streaming: true
     chunk_size: 100
     compress: true
   ```

2. **Format-specific issues:**
   ```bash
   # Validate before export
   quizlr validate --all
   
   # Export with specific options
   quizlr export --format pdf --split-files --max-size 10mb
   ```

3. **Character encoding:**
   ```yaml
   export:
     encoding: "utf-8"
     bom: true  # For Excel compatibility
     escape_unicode: false
   ```

## Diagnostic Tools

### Built-in Diagnostics

Run comprehensive checks:

```bash
# Full system check
quizlr doctor --full

# Output:
System Check Results
├─ ✓ Operating System: macOS 14.0
├─ ✓ Quizlr Version: 2.1.0
├─ ✓ Database: SQLite 3.39.0
├─ ✗ Network: GitHub API unreachable
├─ ⚠ Storage: 89% full
└─ ✓ Dependencies: All satisfied

Issues found: 2
Run 'quizlr doctor --fix' to attempt repairs
```

### Debug Mode

Enable detailed logging:

```yaml
debug:
  enabled: true
  level: "trace"  # error, warn, info, debug, trace
  
  modules:
    - "sync"
    - "auth"
    - "database"
    
  output:
    console: true
    file: "~/quizlr-debug.log"
    max_size_mb: 100
```

### Performance Profiling

Identify bottlenecks:

```bash
# Profile specific operation
quizlr profile --operation "quiz-load"

# Results:
Performance Profile: quiz-load
├─ Database Query: 45ms (15%)
├─ JSON Parsing: 123ms (41%)
├─ UI Render: 89ms (30%)
└─ Plugin Hooks: 42ms (14%)
Total: 299ms

Recommendation: Optimize JSON parsing
```

## Getting Help

### Self-Help Resources

1. **Search error messages:**
   ```bash
   quizlr help --search "error message"
   ```

2. **View documentation:**
   ```bash
   quizlr docs --open troubleshooting
   ```

3. **Check known issues:**
   - [GitHub Issues](https://github.com/quizlr/quizlr/issues)
   - [Status Page](https://status.quizlr.app)

### Community Support

1. **Discord Server:** Real-time help
2. **Forums:** Detailed discussions
3. **Stack Overflow:** Tag: `quizlr`

### Reporting Issues

Create helpful bug reports:

```bash
# Generate bug report
quizlr report --bug

# Includes:
# - System info
# - Configuration (sanitized)
# - Recent logs
# - Crash dumps
```

**Bug report template:**
```markdown
## Description
Brief description of the issue

## Steps to Reproduce
1. Open Quizlr
2. Navigate to...
3. Click on...

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: macOS 14.0
- Quizlr: 2.1.0
- Browser: Chrome 120

## Logs
```
[Attach relevant logs]
```
```

## Emergency Procedures

### Data Recovery

If all else fails:

```bash
# Emergency backup
quizlr emergency-backup

# Factory reset (keeps data)
quizlr reset --keep-data

# Full reset (WARNING: deletes everything)
quizlr reset --full --confirm
```

### Safe Mode

Start with minimal features:

```bash
# Safe mode
quizlr --safe-mode

# Minimal mode (no plugins, default config)
quizlr --minimal

# Recovery mode (repair tools only)
quizlr --recovery
```

## Prevention Tips

1. **Regular backups** - Enable automatic backups
2. **Keep updated** - Install updates promptly
3. **Monitor health** - Check system status weekly
4. **Document changes** - Note configuration modifications
5. **Test in staging** - Try major changes in test environment

## Related Documentation

- [Configuration Guide](./configuration.md)
- [Data Management](./data-management.md)
- [API Reference](../reference/api.md)
- [FAQ](./faq.md)
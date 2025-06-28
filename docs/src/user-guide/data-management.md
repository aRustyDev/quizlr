# Data Management

This guide covers how to organize, backup, export, and manage your quiz data in Quizlr.

## Data Management Overview

Quizlr's data management system:

```
Data Management
├─ Organization
│  ├─ Collections & Folders
│  ├─ Tags & Categories
│  └─ Search & Filters
├─ Storage
│  ├─ Local Database
│  ├─ Cloud Sync
│  └─ External Storage
├─ Backup & Recovery
│  ├─ Automated Backups
│  ├─ Manual Exports
│  └─ Version History
└─ Privacy & Security
   ├─ Encryption
   ├─ Access Control
   └─ Data Retention
```

## Data Organization

### Collections and Folders

Organize quizzes hierarchically:

```
My Quizzes
├─ 📁 Programming
│  ├─ 📁 Web Development
│  │  ├─ 📋 HTML Basics (25 questions)
│  │  ├─ 📋 CSS Flexbox (15 questions)
│  │  └─ 📋 JavaScript ES6 (30 questions)
│  └─ 📁 Backend
│     ├─ 📋 Node.js Fundamentals (20 questions)
│     └─ 📋 Database Design (18 questions)
├─ 📁 Mathematics
│  ├─ 📋 Algebra Review (40 questions)
│  └─ 📋 Calculus Concepts (35 questions)
└─ 📁 Shared with Me
   └─ 📋 Team Quiz (12 questions)
```

**Creating Collections:**
```yaml
collections:
  - name: "Computer Science"
    description: "All CS-related quizzes"
    icon: "💻"
    color: "#0066CC"
    
    subfolders:
      - "Algorithms"
      - "Data Structures"
      - "Operating Systems"
      
    permissions:
      owner: "user@example.com"
      shared_with: ["team@example.com"]
      public: false
```

### Tags and Categories

Multi-dimensional organization:

```yaml
tagging:
  # Hierarchical Tags
  taxonomy:
    subject:
      - programming
        - javascript
        - python
        - java
      - mathematics
        - algebra
        - calculus
        
    difficulty:
      - beginner
      - intermediate
      - advanced
      
    type:
      - practice
      - exam
      - review
      
  # Auto-tagging Rules
  auto_tag:
    - rule: "title contains 'JavaScript'"
      tags: ["programming", "javascript"]
      
    - rule: "question_count > 50"
      tags: ["comprehensive"]
      
    - rule: "difficulty == 'hard'"
      tags: ["advanced", "challenging"]
```

### Smart Filters

Create dynamic collections:

```yaml
smart_collections:
  - name: "Need Review"
    filter:
      last_score: "< 70"
      attempts: "> 0"
      
  - name: "Daily Practice"
    filter:
      tags: ["practice"]
      last_taken: "> 7 days ago"
      difficulty: ["easy", "medium"]
      
  - name: "Upcoming Exams"
    filter:
      tags: ["exam"]
      due_date: "< 30 days"
      completed: false
```

### Search Functionality

Advanced search capabilities:

```
Search Query Examples:

Basic Search:
"python functions"

Advanced Search:
title:"JavaScript" AND difficulty:medium
tags:exam AND created:>2024-01-01
author:"John Doe" OR shared:true

Search Operators:
• AND, OR, NOT
• Wildcards: pyth*
• Phrases: "exact phrase"
• Ranges: score:70..100
• Fields: title:, tags:, content:
```

## Data Storage

### Local Database

Understanding local storage:

```yaml
database:
  # SQLite Configuration
  type: "sqlite"
  path: "~/Library/Application Support/Quizlr/quizlr.db"
  
  # Performance Settings
  settings:
    journal_mode: "WAL"  # Write-Ahead Logging
    cache_size: 10000    # Pages in memory
    temp_store: "MEMORY"
    synchronous: "NORMAL"
    
  # Maintenance
  maintenance:
    auto_vacuum: "INCREMENTAL"
    analyze_on_startup: true
    optimize_interval_days: 7
```

**Database Structure:**
```sql
-- Main Tables
quizzes
├─ id, title, description, created, modified
├─ settings (JSON)
└─ metadata (JSON)

questions
├─ id, quiz_id, type, content
├─ options (JSON)
├─ correct_answer, explanation
└─ difficulty, points, tags

results
├─ id, quiz_id, user_id, started, completed
├─ score, time_taken
└─ answers (JSON)

-- Indexes for Performance
CREATE INDEX idx_quiz_created ON quizzes(created);
CREATE INDEX idx_question_quiz ON questions(quiz_id);
CREATE INDEX idx_results_user ON results(user_id);
```

### Cloud Storage

Configure cloud backends:

```yaml
cloud_storage:
  # Primary Storage
  primary:
    provider: "github"
    repository: "username/quizlr-data"
    branch: "main"
    path: "/data"
    
  # Backup Storage
  backup:
    provider: "s3"
    bucket: "quizlr-backups"
    region: "us-east-1"
    prefix: "user-123/"
    
  # Media Storage
  media:
    provider: "cloudinary"
    cloud_name: "quizlr"
    folder: "quiz-media"
```

### Storage Optimization

Manage storage efficiently:

```yaml
optimization:
  # Compression
  compression:
    enabled: true
    algorithm: "zstd"
    level: 3
    exclude: ["*.jpg", "*.png"]
    
  # Deduplication
  deduplication:
    enabled: true
    check_content_hash: true
    
  # Archival
  archive:
    enabled: true
    after_days: 365
    compress: true
    move_to: "archive/"
```

## Backup and Recovery

### Automated Backups

Configure automatic backups:

```yaml
backup:
  # Schedule
  schedule:
    frequency: "daily"
    time: "02:00"
    timezone: "America/New_York"
    
  # Retention Policy
  retention:
    daily_backups: 7
    weekly_backups: 4
    monthly_backups: 12
    yearly_backups: 2
    
  # Backup Locations
  destinations:
    - type: "local"
      path: "~/Documents/Quizlr Backups"
      
    - type: "cloud"
      provider: "dropbox"
      folder: "/Apps/Quizlr/Backups"
      
    - type: "github"
      repository: "username/quizlr-backups"
      branch: "backups"
```

**Backup Process:**
```
Backup Running...
├─ Preparing database snapshot
├─ Compressing data (2.3 MB → 456 KB)
├─ Encrypting backup
├─ Uploading to destinations
│  ├─ ✓ Local: ~/Documents/Quizlr Backups/
│  ├─ ✓ Dropbox: /Apps/Quizlr/Backups/
│  └─ ✓ GitHub: username/quizlr-backups
└─ Backup complete (15.3s)

Next backup: Tomorrow at 02:00 AM
```

### Manual Exports

Export data on demand:

```
Export Options
├─ 📦 Full Backup
│  ├─ Format: [ZIP/TAR/7Z]
│  ├─ Include: ☑ Quizzes ☑ Results ☑ Media ☑ Settings
│  └─ Encryption: [Optional password]
├─ 📊 Quiz Data Only
│  ├─ Format: [JSON/YAML/XML]
│  ├─ Pretty Print: ☑
│  └─ Include Metadata: ☑
├─ 📈 Results/Analytics
│  ├─ Format: [CSV/Excel/PDF]
│  ├─ Date Range: [Last 30 days ▼]
│  └─ Group By: [Quiz/Date/User]
└─ 🎯 Selective Export
   ├─ Select Items: [Browse...]
   └─ Export Format: [Choose...]
```

### Version History

Track changes over time:

```yaml
versioning:
  enabled: true
  
  # What to Track
  track:
    - quiz_content
    - question_changes
    - settings_updates
    
  # Version Storage
  storage:
    max_versions_per_item: 50
    compress_old_versions: true
    
  # Diff Display
  diff_viewer:
    show_inline: true
    highlight_changes: true
    context_lines: 3
```

**Version Browser:**
```
Version History: JavaScript Basics Quiz
├─ v15 (Current) - 2024-01-20 15:30
│  └─ Added 5 new questions on promises
├─ v14 - 2024-01-18 10:15
│  └─ Fixed typo in question #3
├─ v13 - 2024-01-15 14:22
│  └─ Updated difficulty levels
└─ [Show More...]

[Compare Versions] [Restore Version]
```

### Disaster Recovery

Restore from backups:

```
Recovery Options
├─ 🔄 Quick Restore
│  ├─ Source: Latest backup (2 hours ago)
│  ├─ Items: All data
│  └─ [Restore Now]
├─ 📅 Point-in-Time Recovery
│  ├─ Select Date: [Calendar]
│  ├─ Select Time: [14:30]
│  └─ [Preview & Restore]
├─ 🎯 Selective Restore
│  ├─ Choose Items: [Browse backup...]
│  ├─ Merge Strategy: [Replace/Merge/Skip]
│  └─ [Start Restore]
└─ 🆘 Emergency Recovery
   ├─ Recovery Key: [Enter 24-word phrase]
   └─ [Decrypt & Restore]
```

## Import/Export Formats

### Supported Export Formats

Export for different purposes:

| Format | Use Case | Includes |
|--------|----------|----------|
| JSON | Complete backup | Everything |
| YAML | Human-readable | All data |
| CSV | Spreadsheet analysis | Tabular data |
| Markdown | Documentation | Questions & answers |
| PDF | Printing/Sharing | Formatted quizzes |
| HTML | Web publishing | Interactive preview |
| SCORM | LMS integration | Course package |
| QTI | Standard format | IMS compliant |

### Format Conversion

Convert between formats:

```yaml
conversion:
  # JSON to Markdown
  json_to_markdown:
    template: |
      # {title}
      
      {description}
      
      ## Questions
      
      {#questions}
      ### Question {number}
      
      {question}
      
      {#options}
      - [{correct}] {text}
      {/options}
      
      **Answer:** {answer}
      
      **Explanation:** {explanation}
      {/questions}
      
  # CSV Mapping
  csv_export:
    columns:
      - question_number
      - question_text
      - question_type
      - correct_answer
      - difficulty
      - tags
      - explanation
```

### Bulk Operations

Handle large-scale exports:

```yaml
bulk_export:
  # Export Multiple Quizzes
  batch_size: 10
  parallel_exports: 3
  
  # Naming Convention
  filename_template: "{category}_{title}_{date}"
  
  # Organization
  folder_structure:
    by_category: true
    by_date: true
    by_author: false
    
  # Post-processing
  after_export:
    compress: true
    encrypt: optional
    upload_to_cloud: true
```

## Data Privacy

### Encryption

Protect sensitive data:

```yaml
encryption:
  # At-Rest Encryption
  database:
    enabled: true
    algorithm: "AES-256-GCM"
    key_derivation: "Argon2id"
    
  # Field-Level Encryption
  sensitive_fields:
    - user_email
    - api_keys
    - personal_notes
    
  # Backup Encryption
  backups:
    always_encrypt: true
    password_required: true
    key_escrow: false
```

### Data Anonymization

Remove personal information:

```yaml
anonymization:
  # Automatic Anonymization
  auto_anonymize:
    after_days: 180
    
  # What to Anonymize
  fields:
    user_name: "User{random_id}"
    email: "user{random_id}@example.com"
    ip_address: "0.0.0.0"
    
  # Preserve Analytics
  preserve:
    - aggregate_scores
    - difficulty_ratings
    - time_statistics
```

### GDPR Compliance

Handle data requests:

```
GDPR Tools
├─ 📥 Data Export
│  ├─ Export all my data
│  ├─ Machine-readable format (JSON)
│  └─ Human-readable format (PDF)
├─ 🗑️ Data Deletion
│  ├─ Delete account and all data
│  ├─ Delete specific items
│  └─ Anonymize instead
├─ 📊 Data Usage Report
│  ├─ What data we collect
│  ├─ How we use it
│  └─ Who we share with
└─ ⚙️ Privacy Settings
   ├─ Analytics opt-out
   ├─ Data retention period
   └─ Sharing preferences
```

## Performance Optimization

### Database Maintenance

Keep database performant:

```bash
# Maintenance Commands
quizlr db vacuum        # Reclaim space
quizlr db analyze       # Update statistics
quizlr db optimize      # Full optimization
quizlr db check         # Integrity check

# Scheduled Maintenance
0 3 * * 0 quizlr db optimize --quiet
```

### Caching Strategy

Improve performance:

```yaml
cache:
  # Memory Cache
  memory:
    size_mb: 100
    ttl_seconds: 3600
    
  # Disk Cache
  disk:
    path: "~/.cache/quizlr"
    size_mb: 500
    
  # What to Cache
  cache_types:
    - quiz_metadata
    - search_results
    - user_preferences
    - api_responses
```

### Monitoring

Track data usage:

```
Storage Analytics
├─ Total Size: 847 MB
├─ Growth Rate: +12 MB/month
├─ Breakdown:
│  ├─ Quizzes: 234 MB (27.6%)
│  ├─ Results: 156 MB (18.4%)
│  ├─ Media: 389 MB (45.9%)
│  └─ Backups: 68 MB (8.1%)
└─ Projections:
   ├─ 1 month: 859 MB
   ├─ 6 months: 919 MB
   └─ 1 year: 991 MB

[Cleanup Wizard] [Storage Settings]
```

## Best Practices

### Regular Maintenance

1. **Weekly tasks:**
   - Review and organize new content
   - Check backup status
   - Clear unnecessary cache

2. **Monthly tasks:**
   - Run database optimization
   - Review storage usage
   - Update tags and categories

3. **Quarterly tasks:**
   - Audit data retention
   - Test backup restoration
   - Review privacy settings

### Data Hygiene

1. **Remove duplicates** regularly
2. **Archive old content** instead of deleting
3. **Use consistent naming** conventions
4. **Tag thoroughly** for easy retrieval
5. **Document changes** in version notes

### Security Practices

1. **Encrypt sensitive data** always
2. **Use strong passwords** for backups
3. **Test recovery procedures** regularly
4. **Limit access** appropriately
5. **Audit data access** logs

## Troubleshooting

### Common Issues

**Database Corruption:**
```bash
# Check integrity
quizlr db check

# Repair if needed
quizlr db repair --backup-first

# Restore from backup
quizlr restore --from-backup latest
```

**Storage Full:**
```bash
# Analyze usage
quizlr storage analyze

# Clean up
quizlr storage cleanup --old-cache --temp-files

# Archive old data
quizlr archive --older-than 1y
```

**Sync Conflicts:**
```yaml
conflict_resolution:
  strategy: "newest"  # newest, prompt, merge
  
  merge_rules:
    - field: "content"
      action: "combine"
    - field: "metadata"
      action: "newest"
```

## Next Steps

- [Review Troubleshooting Guide](./troubleshooting.md)
- [Explore Advanced Features](./advanced-features.md)
- [Configure Backups](./configuration.md#backup)
- [Set Up Cloud Sync](./configuration.md#sync)

## Related Documentation

- [Database Schema](../reference/database-schema.md)
- [API Reference](../reference/api.md)
- [Security Guide](../security/guide.md)
- [Privacy Policy](../legal/privacy.md)
# Data Sources

This guide explains how to import, connect, and manage various data sources in Quizlr for quiz content.

## Data Sources Overview

Quizlr supports multiple content sources:

```
Supported Data Sources
├─ File Imports
│  ├─ Markdown (.md)
│  ├─ JSON (.json)
│  ├─ YAML (.yaml)
│  ├─ CSV (.csv)
│  └─ GIFT Format (.gift)
├─ Version Control
│  ├─ GitHub
│  ├─ GitLab
│  └─ Bitbucket
├─ Learning Platforms
│  ├─ Moodle
│  ├─ Canvas
│  └─ Blackboard
├─ APIs & Webhooks
│  ├─ REST APIs
│  ├─ GraphQL
│  └─ Webhooks
└─ AI Generation
   ├─ From Documents
   ├─ From Websites
   └─ From Topics
```

## File Imports

### Markdown Format

Import questions from Markdown files:

**Example Format:**
```markdown
# Quiz: JavaScript Fundamentals

## Question 1
What is the result of `typeof null` in JavaScript?

- [ ] "null"
- [x] "object"
- [ ] "undefined"
- [ ] "boolean"

**Explanation:** This is a historical bug in JavaScript where `typeof null` returns "object".

## Question 2 {difficulty: hard}
Fill in the blank: The _____ method creates a new array with the results of calling a function on every element.

**Answer:** map

**Explanation:** The `map()` method creates a new array populated with the results of calling a provided function on every element in the calling array.
```

**Import Process:**
1. Click **Import** → **From File**
2. Select Markdown files
3. Preview parsed questions
4. Map metadata fields
5. Import to quiz library

### JSON Format

Structure for JSON imports:

```json
{
  "quiz": {
    "title": "Python Basics",
    "description": "Test your Python knowledge",
    "settings": {
      "passThreshold": 70,
      "timeLimit": 30,
      "shuffleQuestions": true
    }
  },
  "questions": [
    {
      "id": "q1",
      "type": "multiple_choice",
      "question": "What is a Python decorator?",
      "options": [
        "A function that modifies another function",
        "A way to add comments",
        "A type of variable",
        "A loop construct"
      ],
      "correct": 0,
      "explanation": "Decorators are functions that modify the behavior of other functions.",
      "difficulty": "medium",
      "tags": ["functions", "advanced"],
      "points": 2
    },
    {
      "id": "q2",
      "type": "true_false",
      "question": "Python is a statically typed language",
      "correct": false,
      "explanation": "Python is dynamically typed.",
      "difficulty": "easy"
    }
  ]
}
```

### YAML Format

Human-readable format for quiz data:

```yaml
quiz:
  title: "Data Structures"
  description: "Understanding common data structures"
  metadata:
    author: "John Doe"
    version: "1.0"
    language: "en"
    
questions:
  - id: ds_001
    type: multiple_choice
    question: "Which data structure uses LIFO?"
    options:
      - Queue
      - Stack
      - Heap
      - Tree
    correct: 1
    explanation: "Stack uses Last In, First Out (LIFO)"
    difficulty: easy
    tags: [basics, stack]
    
  - id: ds_002
    type: fill_blank
    question: "A _____ is a hierarchical data structure with nodes"
    answer: tree
    case_sensitive: false
    difficulty: medium
```

### CSV Format

Import bulk questions from spreadsheets:

```csv
question_type,question,option_a,option_b,option_c,option_d,correct_answer,explanation,difficulty,tags
multiple_choice,"What is 2+2?",3,4,5,6,B,"Basic addition",easy,"math,arithmetic"
true_false,"The Earth is flat",TRUE,FALSE,,,B,"The Earth is spherical",easy,"science,geography"
fill_blank,"The capital of France is _____",,,,,"Paris","Geography basics",easy,"geography,europe"
```

**CSV Import Options:**
```yaml
csv_import:
  delimiter: ","
  quote_char: '"'
  escape_char: "\\"
  skip_rows: 1  # Header row
  encoding: "utf-8"
  
  column_mapping:
    question: "Question Text"
    type: "Question Type"
    correct: "Correct Answer"
```

### GIFT Format

Import from Moodle's GIFT format:

```
// Question 1
::Q1:: What is the capital of France? {
  =Paris
  ~London
  ~Berlin
  ~Madrid
}

// Question 2 with feedback
::Q2:: Python uses {
  =dynamic typing#Correct! Python determines types at runtime
  ~static typing#Python is not statically typed
  ~no typing#Python is strongly typed
}

// Fill in the blank
::Q3:: The keyword {=def} is used to define functions in Python.

// True/False
::Q4:: Java is a compiled language {T}
```

## Version Control Integration

### GitHub Integration

Connect to GitHub repositories:

1. **Setup GitHub App:**
   ```yaml
   github:
     app_id: "12345"
     private_key: "${GITHUB_PRIVATE_KEY}"
     installation_id: "67890"
   ```

2. **Repository Configuration:**
   ```yaml
   repositories:
     - owner: "myorg"
       name: "quiz-content"
       branch: "main"
       path: "/quizzes"
       auto_sync: true
       
     - owner: "educator"
       name: "course-materials"
       branch: "master"
       path: "/assessments"
       watch_files: ["*.md", "*.json"]
   ```

3. **Import Process:**
   ```
   GitHub Import
   ├─ Select Repository
   ├─ Choose Branch
   ├─ Browse Files
   │  ├─ /quizzes
   │  │  ├─ javascript-basics.md ✓
   │  │  ├─ python-advanced.json ✓
   │  │  └─ data-structures.yaml ✓
   │  └─ /examples
   │     └─ sample-quiz.md
   └─ Import Selected (3 files)
   ```

### GitLab Integration

Connect to GitLab:

```yaml
gitlab:
  host: "gitlab.com"  # or self-hosted
  token: "${GITLAB_TOKEN}"
  
  projects:
    - id: "12345"
      path: "/content/quizzes"
      ref: "main"
      
  sync:
    webhook_secret: "${WEBHOOK_SECRET}"
    events: ["push", "merge_request"]
```

### Auto-sync Configuration

Keep content synchronized:

```yaml
auto_sync:
  enabled: true
  interval: 300  # seconds
  
  strategies:
    conflict_resolution: "newest"  # newest, prompt, merge
    
  notifications:
    on_sync: true
    on_conflict: true
    on_error: true
    
  filters:
    include_patterns:
      - "*.md"
      - "*.json"
      - "quizzes/**/*"
    exclude_patterns:
      - "*.draft.*"
      - "archive/**"
```

## Learning Management Systems

### Moodle Integration

Import from Moodle:

1. **API Configuration:**
   ```yaml
   moodle:
     url: "https://moodle.school.edu"
     token: "${MOODLE_TOKEN}"
     service: "quiz_export"
   ```

2. **Import Courses:**
   ```
   Moodle Courses
   ├─ CS101: Introduction to Programming
   │  ├─ Quiz 1: Variables and Types (20 questions)
   │  ├─ Quiz 2: Control Flow (15 questions)
   │  └─ Final Exam (50 questions)
   └─ MATH201: Calculus II
      ├─ Weekly Quiz 1 (10 questions)
      └─ Midterm Exam (30 questions)
   
   [Select All] [Import Selected]
   ```

### Canvas LMS

Connect to Canvas:

```yaml
canvas:
  domain: "school.instructure.com"
  access_token: "${CANVAS_TOKEN}"
  
  import_options:
    include_banks: true
    include_outcomes: true
    preserve_ids: false
    
  course_filters:
    enrollment_type: ["teacher", "ta"]
    state: ["available", "completed"]
```

### Blackboard

Import from Blackboard:

```yaml
blackboard:
  learn_url: "https://learn.school.edu"
  app_key: "${BB_APP_KEY}"
  app_secret: "${BB_APP_SECRET}"
  
  content_types:
    - tests
    - pools
    - assessments
```

## API & Webhook Sources

### REST API Integration

Connect to external APIs:

```yaml
api_sources:
  - name: "Question Bank API"
    type: "rest"
    base_url: "https://api.questionbank.com/v1"
    auth:
      type: "bearer"
      token: "${API_TOKEN}"
      
    endpoints:
      list_quizzes:
        path: "/quizzes"
        method: "GET"
        params:
          subject: "{subject}"
          grade: "{grade}"
          
      get_quiz:
        path: "/quizzes/{id}"
        method: "GET"
        
    mapping:
      quiz_title: "$.title"
      questions: "$.questions[*]"
      question_text: "$.text"
      options: "$.choices[*].text"
      correct: "$.correct_index"
```

### GraphQL Sources

Query GraphQL endpoints:

```yaml
graphql_sources:
  - name: "Educational Content API"
    endpoint: "https://api.education.com/graphql"
    headers:
      Authorization: "Bearer ${TOKEN}"
      
    queries:
      fetch_quiz: |
        query GetQuiz($id: ID!) {
          quiz(id: $id) {
            title
            description
            questions {
              id
              text
              type
              options {
                text
                isCorrect
              }
            }
          }
        }
```

### Webhook Reception

Receive content via webhooks:

```yaml
webhooks:
  enabled: true
  endpoint: "/api/webhooks/content"
  
  security:
    verify_signature: true
    allowed_ips: ["192.168.1.0/24"]
    
  handlers:
    - event: "quiz.created"
      action: "import"
      
    - event: "quiz.updated"
      action: "sync"
      
    - event: "quiz.deleted"
      action: "archive"
```

## AI-Powered Generation

### Generate from Documents

Create quizzes from uploaded documents:

```
Document Analysis
├─ Upload Document (PDF, DOCX, TXT)
├─ AI Processing
│  ├─ Extract key concepts
│  ├─ Identify learning objectives
│  ├─ Generate questions
│  └─ Create explanations
└─ Review & Import

Supported Formats:
• PDF documents
• Word documents (.docx)
• Plain text (.txt)
• EPub books
• Web pages (via URL)
```

**Configuration:**
```yaml
ai_generation:
  document_analysis:
    max_file_size_mb: 50
    
    extraction:
      method: "smart"  # smart, ocr, text-only
      languages: ["en", "es", "fr"]
      
    generation:
      questions_per_page: 2
      difficulty_distribution:
        easy: 40
        medium: 40
        hard: 20
      include_page_references: true
```

### Generate from Websites

Extract quiz content from web pages:

```yaml
web_extraction:
  enabled: true
  
  sources:
    - url: "https://docs.python.org/3/tutorial/"
      depth: 2  # Follow links up to 2 levels
      selectors:
        content: ".body"
        exclude: [".navigation", ".footer"]
        
  processing:
    summarize_content: true
    extract_code_examples: true
    generate_practical_questions: true
```

### Topic-based Generation

Generate quizzes from topics:

```
AI Quiz Generator
├─ Enter Topic: "Machine Learning Basics"
├─ Configure Options:
│  ├─ Number of Questions: 20
│  ├─ Difficulty: Mixed
│  ├─ Question Types:
│  │  ├─ [✓] Multiple Choice (60%)
│  │  ├─ [✓] True/False (20%)
│  │  └─ [✓] Fill in Blank (20%)
│  └─ Include:
│     ├─ [✓] Explanations
│     ├─ [✓] References
│     └─ [✓] Practice tips
└─ Generate Quiz
```

## Data Transformation

### Format Conversion

Convert between formats:

```yaml
transformations:
  # Markdown to JSON
  - from: "markdown"
    to: "json"
    options:
      parse_frontmatter: true
      extract_metadata: true
      
  # CSV to YAML
  - from: "csv"
    to: "yaml"
    options:
      group_by: "category"
      nest_questions: true
```

### Content Enhancement

Enhance imported content:

```yaml
enhancement:
  # Add missing explanations
  auto_explain:
    enabled: true
    provider: "openai"
    style: "educational"
    
  # Generate additional questions
  expand_content:
    enabled: true
    variations_per_question: 2
    difficulty_variants: true
    
  # Translate content
  translation:
    enabled: true
    target_languages: ["es", "fr", "de"]
    preserve_formatting: true
```

### Validation & Cleanup

Ensure content quality:

```yaml
validation:
  rules:
    - type: "required_fields"
      fields: ["question", "correct_answer"]
      
    - type: "answer_validation"
      ensure_correct_exists: true
      min_options: 2
      max_options: 6
      
    - type: "content_quality"
      min_question_length: 10
      max_question_length: 500
      check_grammar: true
      check_spelling: true
      
  cleanup:
    remove_duplicates: true
    fix_formatting: true
    standardize_difficulty: true
```

## Bulk Operations

### Batch Import

Import multiple sources:

```
Batch Import Queue
├─ GitHub: myorg/quiz-content (273 files)
├─ CSV: student_questions.csv (150 questions)
├─ Moodle: CS101 Course (5 quizzes)
└─ AI Generate: "Biology Basics" (30 questions)

Total: 458 questions from 4 sources

[Start Import] [Schedule for Later]

Progress: ████████░░ 80% (367/458)
```

### Import Mapping

Map fields during import:

```yaml
field_mapping:
  # Source -> Destination
  "title": "question"
  "correct_option": "correct_answer"
  "topic": "tags[0]"
  "points_value": "points"
  
  # Transformations
  transforms:
    difficulty:
      type: "map"
      mapping:
        "1": "easy"
        "2": "medium"
        "3": "hard"
        
    tags:
      type: "split"
      delimiter: ","
      trim: true
```

### Scheduling

Schedule regular imports:

```yaml
scheduled_imports:
  - name: "Weekly GitHub Sync"
    source: "github"
    repository: "myorg/quiz-content"
    schedule: "0 0 * * 0"  # Weekly on Sunday
    
  - name: "Daily Moodle Sync"
    source: "moodle"
    courses: ["CS101", "CS102"]
    schedule: "0 2 * * *"  # Daily at 2 AM
    
  - name: "Monthly AI Generation"
    source: "ai"
    topics_file: "topics.txt"
    schedule: "0 0 1 * *"  # First day of month
```

## Monitoring & Logs

### Import History

Track all imports:

```
Import History
├─ 2024-01-20 14:32
│  ├─ Source: GitHub (myorg/quiz-content)
│  ├─ Status: Success
│  ├─ Imported: 45 questions
│  └─ Duration: 12s
├─ 2024-01-20 10:15
│  ├─ Source: CSV Upload
│  ├─ Status: Partial (3 errors)
│  ├─ Imported: 27/30 questions
│  └─ View Errors
```

### Error Handling

Handle import errors:

```yaml
error_handling:
  strategies:
    malformed_data: "skip"  # skip, fix, prompt
    duplicate_content: "update"  # skip, update, create_new
    missing_fields: "use_defaults"
    
  notifications:
    email_on_error: true
    log_errors: true
    
  retry:
    max_attempts: 3
    backoff: "exponential"
```

## Best Practices

### Content Organization

1. **Use consistent naming**:
   ```
   /quizzes
   ├─ programming/
   │  ├─ javascript-basics-v1.json
   │  ├─ python-intermediate-v2.yaml
   │  └─ java-advanced-v1.md
   └─ mathematics/
      ├─ algebra-fundamentals-v1.json
      └─ calculus-applications-v1.yaml
   ```

2. **Version your content**
3. **Tag appropriately** for easy filtering
4. **Include metadata** for better organization
5. **Validate before importing**

### Performance Tips

1. **Batch similar imports** together
2. **Use webhooks** for real-time updates
3. **Schedule large imports** during off-hours
4. **Enable caching** for frequently accessed content
5. **Monitor API rate limits**

### Security Considerations

1. **Validate all imported content**
2. **Sanitize HTML** in questions
3. **Use secure connections** (HTTPS)
4. **Rotate API tokens** regularly
5. **Audit import sources**

## Next Steps

- [Configure Data Management](./data-management.md)
- [Set Up Authentication](./authentication.md)
- [Review Troubleshooting](./troubleshooting.md)
- [Explore Advanced Features](./advanced-features.md)

## Related Documentation

- [Quiz Format Reference](../reference/quiz-formats.md)
- [API Documentation](../reference/api.md)
- [Import Examples](../examples/imports/)
- [Developer Guide](../developer-guide/README.md)
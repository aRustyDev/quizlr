# Getting Started

This guide will walk you through your first experience with Quizlr, from creating your first quiz to understanding your learning progress.

## First Launch

When you first open Quizlr, you'll see the welcome screen with three options:

1. **Quick Start** - Jump right in with sample quizzes
2. **Import Content** - Load existing questions from files or GitHub
3. **Create New** - Build your own quiz from scratch

## Creating Your First Quiz

### Quick Method: Using AI

1. Click **Create New Quiz**
2. Select **Generate with AI**
3. Enter a topic (e.g., "Python basics", "World War II", "Cell Biology")
4. Choose settings:
   - Number of questions (default: 10)
   - Difficulty level (Easy/Medium/Hard/Mixed)
   - Question types to include
5. Click **Generate**

The AI will create a complete quiz in seconds!

### Manual Method: Building from Scratch

1. Click **Create New Quiz**
2. Select **Build Manually**
3. Enter quiz details:
   ```
   Title: Introduction to Rust
   Description: Test your knowledge of Rust fundamentals
   Pass Threshold: 70%
   ```
4. Add questions one by one:
   - Choose question type
   - Enter question content
   - Set correct answer(s)
   - Add explanations (optional)
   - Set difficulty level

### Importing from GitHub

1. Click **Import Content**
2. Select **From GitHub**
3. Enter repository URL or search by topic
4. Choose files to import (supports Markdown, JSON, YAML)
5. Review and import questions

## Taking a Quiz

### Starting a Quiz

1. Select a quiz from your library
2. Review quiz details:
   - Number of questions
   - Estimated time
   - Pass threshold
3. Click **Start Quiz**

### During the Quiz

- **Answer questions** using the appropriate input method
- **Skip questions** if allowed (come back later)
- **Pause** anytime - your progress is saved
- **View progress** in the top bar
- **Time tracking** shows time per question

### Question Types

#### True/False
```
Statement: Rust has automatic memory management
○ True  ● False
```

#### Multiple Choice
```
What is the ownership rule in Rust?
○ A. Multiple owners allowed
● B. Single owner at a time
○ C. No ownership concept
○ D. Shared ownership by default
```

#### Fill in the Blank
```
The _____ keyword is used to define a function in Rust.
[fn]
```

### After Completion

When you finish a quiz, you'll see:

1. **Score Summary**
   - Raw score percentage
   - Pass/Fail status
   - Time taken
   - Questions by difficulty

2. **Detailed Review**
   - Each question with your answer
   - Correct answers highlighted
   - Explanations (if enabled)
   - Time spent per question

3. **Learning Insights**
   - Strengths and weaknesses
   - Topic performance
   - Difficulty analysis
   - Recommendations for improvement

## Understanding Adaptive Learning

Quizlr adapts to your learning style:

### Difficulty Adjustment
- **Getting questions right** → Harder questions appear
- **Struggling** → Easier questions to build confidence
- **Mixed performance** → Balanced difficulty

### Spaced Repetition
- Questions you miss appear more frequently
- Mastered content appears less often
- Optimal intervals for memory retention

### Performance Tracking
```
Topic: JavaScript Basics
├─ Variables & Types: 85% ████████▌ 
├─ Functions: 72% ███████▏  
├─ Arrays: 91% █████████▏
└─ Objects: 68% ██████▊   
```

## Managing Your Content

### Organizing Quizzes

1. **Collections**: Group related quizzes
   - "Programming Languages"
   - "History Topics"
   - "Science Concepts"

2. **Tags**: Add searchable labels
   - #beginner
   - #javascript
   - #exam-prep

3. **Favorites**: Star important quizzes

### Search and Filter

```
Search: "python"
Filters: Difficulty: Medium | Topic: Programming | Created: Last 30 days

Results (12 quizzes):
✓ Python Data Structures (85%)
✓ Python Functions (92%)
○ Python Classes (Not attempted)
...
```

## Syncing Your Data

### Automatic Sync

By default, Quizlr saves everything locally. To enable cloud sync:

1. Go to **Settings** → **Storage**
2. Click **Connect GitHub**
3. Authorize Quizlr
4. Choose sync options:
   - Auto-sync on changes
   - Manual sync only
   - Sync frequency

### Manual Backup

1. Go to **Settings** → **Data**
2. Click **Export All Data**
3. Choose format:
   - JSON (complete backup)
   - Markdown (human-readable)
   - CSV (spreadsheet-compatible)

## Tips for Success

### Daily Practice
- Set a daily goal (e.g., 10 questions)
- Use the **Daily Challenge** feature
- Review missed questions before bed

### Effective Learning
1. **Read explanations** even when correct
2. **Take notes** on difficult concepts
3. **Create custom quizzes** for weak areas
4. **Use different question types** for variety

### Progress Tracking
- Check weekly progress reports
- Set learning goals
- Celebrate milestones!

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Next Question | `Enter` or `→` |
| Previous Question | `←` |
| Skip Question | `S` |
| Pause Quiz | `P` |
| Submit Quiz | `Ctrl+Enter` |
| Toggle Explanations | `E` |
| Search | `Ctrl+K` |

## Next Steps

Now that you know the basics:

1. **[Configure API Keys](./api-keys.md)** - Enable AI features
2. **[Set Up Authentication](./authentication.md)** - Secure your data
3. **[Explore Data Sources](./data-sources.md)** - Import more content
4. **[Learn Advanced Features](./advanced-features.md)** - Power user tips

## Getting Help

- **In-app help**: Click the `?` icon
- **Keyboard shortcuts**: Press `?` anytime
- **Troubleshooting**: See our [guide](./troubleshooting.md)
- **Community**: Join our Discord server
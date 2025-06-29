# Issue: Lint errors preventing build

## Problem
`just lint` fails with two errors:
1. Unused import `Quiz` in integration_test.rs
2. Missing Default implementation for TestQuestionBuilder

## Steps to Reproduce
1. Run `just lint`
2. Observe clippy errors

## Expected Behavior
All lint checks should pass without warnings or errors.

## Actual Behavior
```
error: unused import: `Quiz`
 --> quizlr-core/tests/integration_test.rs:1:57

error: you should consider adding a `Default` implementation for `TestQuestionBuilder`
  --> quizlr-core/src/test_utils.rs:23:9
```

## Investigation
- The `Quiz` import is not used directly in integration tests (QuizBuilder is used instead)
- TestQuestionBuilder has a `new()` method but no Default implementation

## Solution
1. Remove unused `Quiz` import from integration_test.rs
2. Add Default implementation for TestQuestionBuilder that calls new()
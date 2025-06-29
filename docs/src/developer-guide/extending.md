# Extending Quizlr

Quizlr is designed to be extensible, allowing developers to add new functionality and integrate with external systems.

## Extension Points

Quizlr provides several extension points:

1. **Storage Backends**: Implement custom storage solutions
2. **LLM Providers**: Add support for different language models
3. **Question Types**: Create custom question formats
4. **Scoring Strategies**: Implement custom scoring algorithms

## Getting Started

To create an extension:

1. Understand the relevant API
2. Implement the required traits
3. Register your extension
4. Test thoroughly

## Best Practices

- Follow the existing patterns in the codebase
- Write comprehensive tests
- Document your extension
- Consider performance implications
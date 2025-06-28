# Developer Guide

Welcome to the Quizlr Developer Guide! This guide provides everything you need to contribute to Quizlr or build extensions.

## Overview

Quizlr is built with:

- **Rust** - Core logic and performance
- **WebAssembly** - Browser compatibility
- **Leptos** - Reactive UI framework
- **IndexedDB** - Local storage
- **GitHub API** - Cloud sync and content

## Quick Start for Developers

```bash
# Clone the repository
git clone https://github.com/yourusername/quizlr.git
cd quizlr

# Install tools
just setup

# Run development server
just dev

# Run tests
just test

# Build for production
just build
```

## Guide Contents

### Getting Started
- [Architecture Overview](./architecture.md) - System design and principles
- [Project Structure](./project-structure.md) - Repository organization
- [Development Setup](./development-setup.md) - Environment configuration

### Development
- [Building](./building.md) - Compilation and packaging
- [Testing](./testing.md) - Test strategies and execution
- [Contributing](./contributing.md) - Guidelines for contributors

### Advanced Topics
- [Extensions](./extensions.md) - Plugin development
- [Deployment](./deployment.md) - Production deployment
- [API Reference](./api-reference.md) - Core API documentation

## Key Concepts

### Workspace Structure
Quizlr uses a Cargo workspace with multiple crates:
- `quizlr-core` - Business logic library
- `quizlr-web` - Web frontend application

### Cross-Platform Design
The core library is designed for reuse across:
- Web (via WASM)
- Mobile (via FFI bindings)
- Desktop (native compilation)

### Extension Points
Quizlr is designed to be extensible:
- Storage backends
- LLM providers
- Question types
- UI themes

## Development Philosophy

1. **Test-Driven Development**: Write tests first
2. **Type Safety**: Leverage Rust's type system
3. **Performance**: Optimize for user experience
4. **Accessibility**: Support all users
5. **Documentation**: Code should be self-documenting

## Getting Help

- **Discord**: Join our developer community
- **GitHub Issues**: Report bugs or request features
- **Discussions**: Architecture and design decisions

## Contributing

We welcome contributions! See our [Contributing Guide](./contributing.md) for:
- Code style guidelines
- Commit message format
- Pull request process
- Code of conduct

Ready to start developing? Head to [Development Setup](./development-setup.md)!
# Glossary

This glossary defines technical terms and concepts used throughout Quizlr documentation and codebase.

## A

### Adaptive Learning
A personalized education approach where the system adjusts content difficulty and presentation based on the learner's performance and needs.

### Adaptive Scoring
A scoring strategy that considers the user's historical performance, question difficulty, and response patterns to provide personalized score calculations.

### API (Application Programming Interface)
A set of protocols and tools for building software applications. Quizlr provides APIs for extending functionality and integrating with external systems.

### API Key
A unique identifier used to authenticate requests to external services, particularly for LLM providers like OpenAI or Anthropic.

### Asynchronous Processing
Operations that don't block the main thread, allowing the application to remain responsive while handling time-consuming tasks.

## B

### Builder Pattern
A design pattern used in Quizlr for constructing complex objects (like quizzes) step by step. The `QuizBuilder` allows flexible quiz creation.

### Bundle
The compiled and optimized JavaScript/WASM files that make up the web application, created during the build process.

## C

### Cargo
Rust's package manager and build system, used to manage dependencies and compile Quizlr.

### Client-Side Rendering (CSR)
A web application architecture where the browser renders the UI using JavaScript. Quizlr uses Leptos for CSR.

### Concurrent Users
The number of users actively using the system at the same time, important for performance planning.

### Curriculum
A structured sequence of quizzes and learning materials designed to teach a specific subject or skill.

## D

### Dependency Injection
A design pattern where objects receive their dependencies from external sources rather than creating them internally.

### Difficulty Weight
A numerical value (0.0 to 1.0) assigned to questions indicating their complexity, used in scoring calculations.

### Docker
A containerization platform that can be used to deploy Quizlr in isolated environments.

## E

### Edge Case
An unusual or extreme scenario that tests the limits of the system's functionality.

### Embedding
A numerical representation of text used by AI models to understand and process natural language.

### Extension API
The interface that allows developers to create plugins and add-ons for Quizlr.

## F

### FFI (Foreign Function Interface)
A mechanism that allows Rust code to interact with code written in other languages, used for platform-specific features.

### Fill in the Blank
A question type where users complete sentences by providing missing words or phrases.

### Frontend
The user-facing part of the application, built with Leptos and rendered in the browser.

## G

### Git
Version control system used for Quizlr's source code management.

### Graceful Degradation
The ability of the system to maintain limited functionality when some features fail or are unavailable.

### Graph Visualization
Visual representation of relationships between concepts, used in the curriculum module.

## H

### Hot Reload
Development feature that automatically updates the application when code changes are detected.

### HTTPS
Secure version of HTTP protocol, required for API communications and user data protection.

## I

### Idempotent
Operations that produce the same result regardless of how many times they're performed.

### Interactive Interview
A question type that simulates a conversational interview format with follow-up questions.

### Integration Test
Tests that verify different parts of the system work correctly together.

## J

### JSON (JavaScript Object Notation)
A lightweight data format used for configuration files and data exchange in Quizlr.

### JWT (JSON Web Token)
A secure method for transmitting authentication information between parties.

### Justfile
A command runner configuration file that defines common development tasks and scripts.

## K

### Key-Value Store
A simple database model used for caching and session storage in Quizlr.

### Knowledge Graph
A network representation of concepts and their relationships, used for adaptive learning paths.

## L

### Leptos
A Rust web framework used for building Quizlr's reactive user interface.

### LLM (Large Language Model)
AI models like GPT-4 or Claude that can understand and generate human-like text, used for intelligent quiz features.

### Local Storage
Browser-based storage mechanism for persisting data on the user's device.

## M

### Match Pairs
A question type where users connect related items from two lists.

### mdBook
The documentation system used to build Quizlr's user and developer guides.

### Middleware
Software that sits between different parts of the application, handling cross-cutting concerns like authentication.

### Migration
The process of updating database schemas or data formats when upgrading Quizlr versions.

### Multi-Select
A question type that allows users to choose multiple correct answers from a list.

### Multiple Choice
A question type with several options where only one answer is correct.

## N

### Non-blocking I/O
Input/output operations that don't halt program execution while waiting for completion.

### Normalization
The process of organizing data to reduce redundancy and improve data integrity.

## O

### OAuth
An open standard for authorization, planned for third-party authentication in Quizlr.

### Open Source
Software whose source code is freely available for anyone to use, modify, and distribute.

### Optimization
The process of improving code performance, reducing resource usage, or enhancing user experience.

## P

### Pagination
Dividing large datasets into smaller pages for better performance and usability.

### Performance Scoring
Scoring calculation that considers both correctness and response time.

### Plugin
An extension that adds functionality to Quizlr without modifying the core codebase.

### Progressive Web App (PWA)
Web application that can be installed and work offline like a native app.

## Q

### Question Bank
A collection of reusable questions that can be included in multiple quizzes.

### Question Type
The format of a quiz question (e.g., Multiple Choice, True/False, Fill in the Blank).

### Quiz Engine
The core component that manages quiz logic, scoring, and session handling.

### Quiz Session
An instance of a user taking a quiz, including their responses and progress.

## R

### Rate Limiting
Controlling the frequency of operations to prevent abuse and ensure fair resource usage.

### Reactive Programming
A programming paradigm where the UI automatically updates in response to data changes.

### REST API
Architectural style for web services using standard HTTP methods (GET, POST, PUT, DELETE).

### Rust
A systems programming language focused on safety and performance, used for Quizlr's core.

## S

### Scoring Strategy
An algorithm for calculating quiz scores (Simple, TimeWeighted, DifficultyWeighted, Adaptive).

### Self-Hosting
Running Quizlr on your own servers rather than using a cloud service.

### Semantic Versioning
Version numbering system (MAJOR.MINOR.PATCH) that conveys meaning about code changes.

### Serialization
Converting data structures into a format that can be stored or transmitted.

### Session State
The current status of a quiz session (NotStarted, InProgress, Paused, Completed).

### SQLite
A lightweight, file-based database used as Quizlr's default storage backend.

### SSO (Single Sign-On)
Authentication method allowing users to access multiple applications with one login.

### Storage API
Abstraction layer that allows Quizlr to work with different database backends.

## T

### Tailwind CSS
A utility-first CSS framework used for styling Quizlr's user interface.

### TDD (Test-Driven Development)
Development methodology where tests are written before the implementation code.

### Time Complexity
A measure of how algorithm performance scales with input size.

### Token
In LLM context, a unit of text (roughly 4 characters) used for API pricing and limits.

### Topic Explanation
A question type that requires detailed, essay-style responses about a subject.

### True/False
A simple question type with binary answer choices.

### Type Safety
Programming language feature that prevents type-related errors at compile time.

## U

### Unit Test
Tests that verify individual components or functions work correctly in isolation.

### User Authentication
The process of verifying a user's identity before granting access to the system.

### UUID (Universally Unique Identifier)
A 128-bit identifier used to uniquely identify quiz elements without central coordination.

## V

### Validation
Checking data integrity and correctness before processing or storage.

### Version Control
System for tracking changes to code over time, enabling collaboration and history.

### Virtual DOM
An in-memory representation of the UI that improves rendering performance.

## W

### WASM (WebAssembly)
A binary instruction format that allows Rust code to run in web browsers at near-native speed.

### Web Worker
Browser feature that runs JavaScript in background threads, used for heavy computations.

### Webhook
HTTP callbacks that notify external systems when events occur in Quizlr.

### Workspace
Rust's feature for managing multiple related packages in a single repository.

## X

### XSS (Cross-Site Scripting)
A security vulnerability where malicious scripts are injected into web pages.

## Y

### YAML
A human-readable data serialization format, sometimes used for configuration files.

## Z

### Zero-Copy
Optimization technique that avoids unnecessary data duplication in memory.

### Zero-Downtime Deployment
Updating the application without interrupting service for active users.

---

## Acronyms Quick Reference

- **API**: Application Programming Interface
- **CI/CD**: Continuous Integration/Continuous Deployment
- **CLI**: Command Line Interface
- **CORS**: Cross-Origin Resource Sharing
- **CRUD**: Create, Read, Update, Delete
- **CSS**: Cascading Style Sheets
- **CSV**: Comma-Separated Values
- **DOM**: Document Object Model
- **GDPR**: General Data Protection Regulation
- **GUI**: Graphical User Interface
- **HTML**: HyperText Markup Language
- **HTTP**: HyperText Transfer Protocol
- **JSON**: JavaScript Object Notation
- **JWT**: JSON Web Token
- **LLM**: Large Language Model
- **LMS**: Learning Management System
- **MIT**: Massachusetts Institute of Technology (license)
- **MVP**: Minimum Viable Product
- **ORM**: Object-Relational Mapping
- **QA**: Quality Assurance
- **REST**: Representational State Transfer
- **SDK**: Software Development Kit
- **SQL**: Structured Query Language
- **UI/UX**: User Interface/User Experience
- **URL**: Uniform Resource Locator
- **WASM**: WebAssembly
- **WCAG**: Web Content Accessibility Guidelines
- **XML**: eXtensible Markup Language

---

*This glossary is continuously updated. If you encounter a term that should be included, please submit a pull request or open an issue.*
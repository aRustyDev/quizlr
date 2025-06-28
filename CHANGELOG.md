# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive test suite for quiz engine following TDD principles
  - 60 tests covering all question types, quiz management, sessions, and scoring
  - Tests for edge cases and error conditions
  - Performance and consistency scoring tests

### Fixed
- Corrected scoring calculations for empty sessions
- Fixed difficulty weighted scoring to consider all questions
- Improved adaptive scoring to handle no-response scenarios
- Fixed unused variable and import warnings

### Added
- Initial project structure with workspace configuration
- Core library (`quizlr-core`) with quiz engine implementation
  - Question types: TrueFalse, MultipleChoice, MultiSelect, FillInTheBlank, MatchPairs, InteractiveInterview, TopicExplanation
  - Quiz management with QuizBuilder pattern
  - Quiz session management with pause/resume capabilities
  - Scoring strategies: Simple, TimeWeighted, DifficultyWeighted, Adaptive
  - Placeholder modules for: curriculum, adaptive learning, LLM integration, storage, authentication, graph visualization
- Web frontend (`quizlr-web`) with Leptos CSR framework
  - Basic application structure
  - Tailwind CSS integration
  - WASM compilation setup
- Development documentation in `.context/` directory
  - CHALLENGES.md for tracking development obstacles
  - LESSONSLEARNED.md for capturing insights
- Build script for web application

### Development Notes
- Resolved multiple dependency version compatibility issues
- Configured project for future cross-platform support (iOS/Android)
- Set up foundation for TDD approach going forward

## [0.1.0] - TBD
- Initial release pending

[Unreleased]: https://github.com/yourusername/quizlr/compare/v0.1.0...HEAD
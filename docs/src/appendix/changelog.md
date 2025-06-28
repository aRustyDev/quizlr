# Changelog

All notable changes to Quizlr will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive documentation system using mdBook
  - User Guide with installation, configuration, and usage instructions
  - Developer Guide with architecture, building, and contribution guidelines
  - Reference documentation for all major components
  - API documentation for extensions, LLM integration, and storage
- Test suite following Test-Driven Development (TDD) principles
  - 60+ tests covering quiz engine functionality
  - Question type validation tests
  - Session management tests
  - Scoring strategy tests
  - Edge case and error condition coverage
- Core quiz engine implementation
  - 7 question types: TrueFalse, MultipleChoice, MultiSelect, FillInTheBlank, MatchPairs, InteractiveInterview, TopicExplanation
  - Quiz builder pattern for flexible quiz creation
  - Session management with pause/resume capabilities
  - 4 scoring strategies: Simple, TimeWeighted, DifficultyWeighted, Adaptive
- Web frontend using Leptos CSR framework
  - Responsive design with Tailwind CSS
  - WASM compilation for browser deployment
  - Basic application structure ready for feature implementation
- Development infrastructure
  - Workspace-based Rust project structure
  - Justfile for common development tasks
  - Build scripts for web deployment
  - GitHub Actions CI/CD pipeline ready

### Fixed
- Scoring calculation bugs in edge cases
  - Empty session handling
  - Difficulty weighted scoring with no questions
  - Adaptive scoring with no responses
- Rust compiler warnings
  - Unused imports and variables
  - Dead code warnings in placeholder modules

### Changed
- Project structure reorganized for better modularity
- Documentation moved to dedicated docs/ directory
- Test files separated from implementation files

## [0.1.0] - TBD

**Initial Release** - This will be the first public release of Quizlr.

### Planned Features
- Basic quiz functionality through web interface
- Support for all 7 question types
- User authentication and profile management
- Quiz creation and editing interface
- Basic scoring and results display
- Local storage for quiz data

### Known Limitations
- No LLM integration yet
- Limited to browser local storage
- No multi-user collaboration
- No mobile app support
- No curriculum management features

## Version History

### Pre-release Development

#### Phase 1: Foundation (Current)
- Project setup and structure
- Core quiz engine implementation
- Basic web frontend
- Documentation framework
- Test suite establishment

#### Phase 2: MVP (Upcoming)
- User interface implementation
- Authentication system
- Basic quiz management
- Local storage integration
- Initial release preparation

#### Phase 3: Enhancement (Future)
- LLM integration
- Advanced analytics
- Mobile app development
- Collaborative features
- Cloud storage options

---

For the main project changelog with technical details, see [CHANGELOG.md](../../../CHANGELOG.md) in the repository root.

[Unreleased]: https://github.com/yourusername/quizlr/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/quizlr/releases/tag/v0.1.0
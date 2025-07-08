# Quizlr Project Plan

## Overview
Quizlr is a flashcard-style learning application that can use GitHub repositories as data sources. Built with Rust and Leptos for WebAssembly deployment.

## Architecture
- **Frontend**: Leptos (Rust WASM framework)
- **Core Logic**: Rust library (quizlr-core)
- **Data Sources**: GitHub repositories (future: markdown files)
- **Deployment**: Static site (WASM + HTML/CSS/JS)

## Development Milestones

### v0.1.0 - MVP Core Functionality (Due: July 15, 2025)
**Goal**: Basic quiz functionality with hardcoded questions and working UI

**Features**:
- [x] Basic quiz UI with question display
- [x] Multiple choice answer buttons
- [x] Score tracking
- [x] Quiz completion screen
- [x] Restart functionality
- [ ] Fix Leptos reactive event handlers (#32)
- [ ] Proper state management with signals
- [ ] Basic styling with Tailwind CSS

### v0.2.0 - Data Source Integration (Due: August 1, 2025)
**Goal**: Implement GitHub repository integration to load quiz content from markdown files

**Features**:
- [ ] GitHub API integration
- [ ] Markdown parsing for quiz content
- [ ] Quiz format specification
- [ ] Repository selection UI
- [ ] Local caching of quiz data
- [ ] Error handling for network issues

### v0.3.0 - Enhanced Learning Features (Due: August 15, 2025)
**Goal**: Add spaced repetition, progress tracking, and adaptive learning algorithms

**Features**:
- [ ] Spaced repetition algorithm
- [ ] User progress persistence (localStorage)
- [ ] Difficulty adjustment based on performance
- [ ] Statistics dashboard
- [ ] Multiple quiz modes (practice, test, review)
- [ ] Categories and tags support

### v1.0.0 - Production Ready (Due: September 1, 2025)
**Goal**: Polish UI/UX, add authentication, deployment infrastructure, and comprehensive documentation

**Features**:
- [ ] Professional UI design
- [ ] User authentication (optional)
- [ ] Cloud sync capabilities
- [ ] PWA support (offline mode)
- [ ] Comprehensive documentation
- [ ] CI/CD pipeline
- [ ] Performance optimization
- [ ] Accessibility features

## Technical Debt & Known Issues
1. **Leptos Event Handlers** (#32) - Currently using JavaScript onclick workaround
2. **Build Errors** (#29, #30) - Type mismatches in Leptos view macros
3. **UI Rendering** (#31) - Initial issues with component visibility

## Development Guidelines
1. All new features should include tests
2. Document build errors in GitHub issues
3. Use semantic versioning
4. Keep CLAUDE.md updated with development patterns
5. Prioritize type safety and Rust idioms

## Next Steps
1. Resolve Leptos event handler issues
2. Implement proper state management
3. Design quiz data format (JSON/YAML in markdown)
4. Create GitHub integration prototype
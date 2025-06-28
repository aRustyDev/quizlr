# Architecture Overview

Quizlr is built with a modular, extensible architecture that prioritizes cross-platform compatibility, offline-first functionality, and developer experience. This document provides a comprehensive overview of the system architecture, design decisions, and key components.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        User Interfaces                           │
├─────────────────┬─────────────────┬─────────────────────────────┤
│   Web (WASM)    │   iOS (FFI)     │   Android (FFI)            │
│   quizlr-web    │   quizlr-ios    │   quizlr-android          │
└────────┬────────┴────────┬────────┴────────┬────────────────────┘
         │                 │                 │
         └─────────────────┴─────────────────┘
                           │
                    ┌──────┴──────┐
                    │ quizlr-core │
                    │   (Rust)    │
                    └──────┬──────┘
                           │
    ┌──────────────────────┴──────────────────────┐
    │              Core Modules                    │
    ├──────────────────────────────────────────────┤
    │ • Quiz Engine     • Adaptive Learning        │
    │ • Storage System  • LLM Integration          │
    │ • Auth & Security • Knowledge Graph          │
    │ • Curriculum Mgmt • Progress Tracking        │
    └──────────────────────────────────────────────┘
                           │
    ┌──────────────────────┴──────────────────────┐
    │           External Services                   │
    ├──────────────────────────────────────────────┤
    │ • LLM APIs (OpenAI, Anthropic, etc.)        │
    │ • Cloud Storage (S3, GCS, etc.)             │
    │ • Authentication Providers                   │
    └──────────────────────────────────────────────┘
```

## Core Design Principles

### 1. **Offline-First**
- All core functionality works without an internet connection
- Data synchronization happens in the background when connectivity is available
- Local storage is the primary data store with cloud backup

### 2. **Cross-Platform**
- Core logic written in Rust, compiled to WebAssembly for web
- Native mobile apps via FFI bindings
- Shared business logic across all platforms

### 3. **Privacy-Focused**
- Local data encryption by default
- Optional cloud sync with end-to-end encryption
- User controls all data sharing

### 4. **Extensible**
- Plugin architecture for storage backends
- Modular LLM provider system
- Customizable quiz types and scoring strategies

## Module Architecture

### Quiz Engine (`quiz/`)

The heart of Quizlr, responsible for quiz logic, question management, and scoring.

```rust
// Core quiz structures
pub struct Quiz {
    id: Uuid,
    metadata: QuizMetadata,
    questions: Vec<Question>,
    config: QuizConfig,
}

pub struct Session {
    quiz: Arc<Quiz>,
    answers: HashMap<Uuid, Answer>,
    state: SessionState,
    scorer: Box<dyn ScoringStrategy>,
}
```

**Key Components:**
- `Question`: Polymorphic question types (multiple choice, true/false, etc.)
- `Session`: Manages quiz state and user interactions
- `Scorer`: Pluggable scoring strategies
- `Validator`: Answer validation logic

### Storage System (`storage/`)

Provides abstraction over different storage backends with a unified API.

```rust
#[async_trait]
pub trait StorageBackend: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    async fn put(&self, key: &str, value: Vec<u8>) -> Result<()>;
    async fn delete(&self, key: &str) -> Result<()>;
    async fn list(&self, prefix: &str) -> Result<Vec<String>>;
}
```

**Implementations:**
- `LocalStorage`: Browser localStorage/IndexedDB for web
- `FileStorage`: File system storage for native apps
- `CloudStorage`: S3-compatible cloud storage
- `EncryptedStorage`: Wrapper providing encryption

### Adaptive Learning (`adaptive/`)

Implements spaced repetition and difficulty adjustment algorithms.

```rust
pub struct AdaptiveEngine {
    algorithm: Box<dyn AdaptiveAlgorithm>,
    user_model: UserModel,
    performance_tracker: PerformanceTracker,
}

pub trait AdaptiveAlgorithm {
    fn next_question(&self, history: &History) -> Option<Question>;
    fn update_model(&mut self, result: &QuizResult);
    fn calculate_interval(&self, performance: f32) -> Duration;
}
```

**Algorithms:**
- SM-2 (SuperMemo 2) for spaced repetition
- Item Response Theory (IRT) for difficulty calibration
- Custom algorithms via trait implementation

### LLM Integration (`llm/`)

Provides abstraction for different LLM providers with fallback support.

```rust
#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn generate(&self, prompt: &Prompt) -> Result<Response>;
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    fn name(&self) -> &str;
    fn priority(&self) -> u8;
}

pub struct LLMManager {
    providers: Vec<Box<dyn LLMProvider>>,
    fallback_strategy: FallbackStrategy,
}
```

**Features:**
- Automatic provider fallback
- Rate limiting and retry logic
- Response caching
- Cost tracking

### Knowledge Graph (`graph/`)

Manages relationships between concepts, topics, and learning paths.

```rust
pub struct KnowledgeGraph {
    nodes: HashMap<NodeId, Node>,
    edges: Vec<Edge>,
    index: GraphIndex,
}

pub enum Node {
    Topic(Topic),
    Concept(Concept),
    Skill(Skill),
    LearningObjective(LearningObjective),
}
```

**Capabilities:**
- Prerequisite tracking
- Learning path generation
- Concept mapping
- Progress visualization

## Data Flow Architecture

### Quiz Creation Flow

```
User Input → LLM Provider → Content Parser → Quiz Builder → Storage
     ↓             ↓              ↓              ↓            ↓
  UI Layer    API Request    Validation    Structuring    Persistence
```

### Quiz Session Flow

```
Session Start → Load Quiz → Present Question → Collect Answer → Score
      ↓            ↓             ↓                 ↓             ↓
   Initialize   From Storage  Adaptive Engine  Validation   Update Model
```

### Synchronization Flow

```
Local Change → Change Queue → Sync Manager → Remote Storage
      ↓             ↓              ↓              ↓
   Detection    Buffering    Conflict Resolution  Upload
```

## Security Architecture

### Data Protection

1. **Encryption at Rest**
   - AES-256-GCM for local data
   - Key derivation from user passphrase
   - Optional hardware security module integration

2. **Encryption in Transit**
   - TLS 1.3 for all network communication
   - Certificate pinning for mobile apps
   - End-to-end encryption for sync

### Authentication

```rust
pub struct AuthManager {
    providers: Vec<Box<dyn AuthProvider>>,
    token_store: SecureTokenStore,
    session_manager: SessionManager,
}
```

**Supported Methods:**
- Local authentication (passphrase)
- OAuth2/OIDC providers
- WebAuthn/FIDO2
- Biometric (mobile)

## Performance Considerations

### Memory Management

- Lazy loading of quiz content
- Question pooling and recycling
- Efficient serialization with bincode
- WASM memory optimization

### Caching Strategy

```rust
pub struct CacheManager {
    memory_cache: LruCache<String, CachedItem>,
    disk_cache: DiskCache,
    cache_policy: CachePolicy,
}
```

**Cache Levels:**
1. In-memory LRU cache
2. IndexedDB/SQLite cache
3. Network cache headers

### Concurrency

- Async/await for I/O operations
- Web Workers for heavy computations
- Thread pool for native platforms
- Lock-free data structures where possible

## Extension Points

### Plugin Architecture

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> Version;
    fn initialize(&mut self, context: &PluginContext) -> Result<()>;
    fn capabilities(&self) -> Vec<Capability>;
}
```

**Extension Categories:**
1. Storage backends
2. LLM providers
3. Question types
4. Scoring algorithms
5. Export formats

### Event System

```rust
pub enum QuizlrEvent {
    QuizStarted(QuizId),
    QuestionAnswered(QuestionId, Answer),
    QuizCompleted(QuizResult),
    SyncStarted,
    SyncCompleted(SyncResult),
}

pub trait EventHandler: Send + Sync {
    fn handle(&self, event: &QuizlrEvent) -> Result<()>;
}
```

## Platform-Specific Considerations

### WebAssembly (Web)

- Compiled with `wasm-pack`
- JavaScript/TypeScript bindings
- Web Workers for background tasks
- IndexedDB for storage

### iOS (via FFI)

- Swift bindings through `uniffi`
- Core Data integration
- CloudKit sync support
- Keychain for secure storage

### Android (via FFI)

- Kotlin bindings through JNI
- Room database integration
- Google Drive sync support
- Android Keystore integration

## Future Architecture Considerations

### Planned Enhancements

1. **Microservices Architecture** (for cloud deployment)
   - Quiz service
   - User service
   - Analytics service
   - Content service

2. **Real-time Collaboration**
   - WebRTC for peer-to-peer
   - CRDT for conflict resolution
   - Live quiz sessions

3. **Advanced Analytics**
   - Learning analytics pipeline
   - Performance prediction models
   - Recommendation engine

### Scalability Path

```
Current: Monolithic Core → Modular Core → Microservices → Distributed
         (Local First)     (Plugin Based)   (Cloud Ready)   (Global Scale)
```

## Architecture Decision Records (ADRs)

### ADR-001: Rust as Core Language
**Decision**: Use Rust for core business logic
**Rationale**: Memory safety, performance, WASM support, cross-platform

### ADR-002: Offline-First Design
**Decision**: All features work offline with optional sync
**Rationale**: Privacy, reliability, performance, global accessibility

### ADR-003: Plugin Architecture
**Decision**: Extensible plugin system for providers
**Rationale**: Flexibility, maintainability, vendor independence

### ADR-004: Local Encryption
**Decision**: Encrypt all local data by default
**Rationale**: Privacy, security, compliance

## Testing Architecture

### Test Pyramid

```
         ┌─────┐
        /  E2E  \      5%
       /─────────\
      / Integration\   15%
     /─────────────\
    /   Unit Tests  \  80%
   ┗━━━━━━━━━━━━━━━━┛
```

### Test Infrastructure

- Unit tests: Built-in Rust testing
- Integration tests: Mock providers
- E2E tests: Playwright (web), XCTest (iOS), Espresso (Android)
- Performance tests: Criterion benchmarks

## Monitoring and Observability

### Metrics Collection

```rust
pub struct MetricsCollector {
    backend: Box<dyn MetricsBackend>,
    config: MetricsConfig,
}

pub trait MetricsBackend {
    fn record_counter(&self, name: &str, value: u64, tags: &[Tag]);
    fn record_gauge(&self, name: &str, value: f64, tags: &[Tag]);
    fn record_histogram(&self, name: &str, value: f64, tags: &[Tag]);
}
```

### Key Metrics

- Quiz completion rates
- Question response times
- Sync performance
- Error rates
- Resource usage

## Conclusion

Quizlr's architecture is designed to be flexible, performant, and privacy-focused. The modular design allows for easy extension and customization while maintaining a clean separation of concerns. The offline-first approach ensures reliability and user control over their data.

For implementation details of specific modules, see the corresponding documentation in the reference section.
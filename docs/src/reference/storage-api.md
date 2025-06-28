# Storage API

The Quizlr storage API provides a flexible abstraction layer for persisting quiz data across different backends. It supports local storage, cloud providers, and custom implementations.

## Architecture

The storage system is built around the `Storage` trait and `StorageManager`:

```rust
#[async_trait]
pub trait Storage: Send + Sync {
    async fn save(&self, key: &str, data: &[u8]) -> Result<(), QuizlrError>;
    async fn load(&self, key: &str) -> Result<Vec<u8>, QuizlrError>;
    async fn delete(&self, key: &str) -> Result<(), QuizlrError>;
    async fn list(&self, prefix: &str) -> Result<Vec<String>, QuizlrError>;
}
```

## Storage Backends

### Local Storage

File-based storage for development and single-user scenarios.

```rust
pub struct LocalStorage {
    base_path: PathBuf,
    encryption_key: Option<Vec<u8>>,
}

impl LocalStorage {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
            encryption_key: None,
        }
    }
    
    pub fn with_encryption(mut self, key: Vec<u8>) -> Self {
        self.encryption_key = Some(key);
        self
    }
}
```

**Features**:
- File-based persistence
- Optional encryption
- Atomic writes
- Directory organization

**Example**:
```rust
let storage = LocalStorage::new("./data")
    .with_encryption(encryption_key);

// Save quiz data
let quiz_data = serde_json::to_vec(&quiz)?;
storage.save("quizzes/math-101", &quiz_data).await?;

// Load quiz data
let data = storage.load("quizzes/math-101").await?;
let quiz: Quiz = serde_json::from_slice(&data)?;
```

### GitHub Storage

Uses GitHub as a backend for version-controlled quiz storage.

```rust
pub struct GitHubStorage {
    owner: String,
    repo: String,
    branch: String,
    token: String,
    base_path: String,
}

impl GitHubStorage {
    pub fn new(owner: String, repo: String, token: String) -> Self {
        Self {
            owner,
            repo,
            branch: "main".to_string(),
            token,
            base_path: "quizzes".to_string(),
        }
    }
    
    pub fn with_branch(mut self, branch: String) -> Self {
        self.branch = branch;
        self
    }
    
    pub fn with_base_path(mut self, path: String) -> Self {
        self.base_path = path;
        self
    }
}
```

**Features**:
- Version control
- Collaboration support
- Commit history
- Branch management

**Example**:
```rust
let storage = GitHubStorage::new(
    "myorg".to_string(),
    "quiz-repo".to_string(),
    github_token,
)
.with_branch("develop".to_string())
.with_base_path("content/quizzes".to_string());

// Save with commit message
let commit_msg = format!("Update quiz: {}", quiz.title);
storage.save_with_message("math/algebra-basics", &data, &commit_msg).await?;
```

### S3-Compatible Storage

Cloud object storage for scalable deployments.

```rust
pub struct S3Storage {
    bucket: String,
    region: String,
    access_key: String,
    secret_key: String,
    endpoint: Option<String>,
}

impl S3Storage {
    pub fn new(
        bucket: String,
        region: String,
        access_key: String,
        secret_key: String,
    ) -> Self {
        Self {
            bucket,
            region,
            access_key,
            secret_key,
            endpoint: None,
        }
    }
    
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }
}
```

**Features**:
- Scalable storage
- CDN integration
- Versioning support
- Lifecycle policies

**Example**:
```rust
let storage = S3Storage::new(
    "quizlr-data".to_string(),
    "us-east-1".to_string(),
    aws_access_key,
    aws_secret_key,
);

// Save with metadata
let metadata = HashMap::from([
    ("content-type", "application/json"),
    ("quiz-version", "2.0"),
]);
storage.save_with_metadata("quizzes/science/physics-101", &data, metadata).await?;
```

## Storage Manager

The `StorageManager` provides high-level operations and caching:

```rust
pub struct StorageManager {
    backend: Box<dyn Storage>,
    cache: Option<Cache>,
    serializer: Serializer,
}

impl StorageManager {
    pub fn new(backend: Box<dyn Storage>) -> Self {
        Self {
            backend,
            cache: None,
            serializer: Serializer::Json,
        }
    }
    
    pub fn with_cache(mut self, cache: Cache) -> Self {
        self.cache = Some(cache);
        self
    }
    
    pub fn with_serializer(mut self, serializer: Serializer) -> Self {
        self.serializer = serializer;
        self
    }
}
```

### High-Level Operations

```rust
impl StorageManager {
    // Save typed data
    pub async fn save_quiz(&self, quiz: &Quiz) -> Result<(), QuizlrError> {
        let key = format!("quizzes/{}", quiz.id);
        let data = self.serializer.serialize(quiz)?;
        
        if let Some(cache) = &self.cache {
            cache.set(&key, &data);
        }
        
        self.backend.save(&key, &data).await
    }
    
    // Load typed data
    pub async fn load_quiz(&self, id: Uuid) -> Result<Quiz, QuizlrError> {
        let key = format!("quizzes/{}", id);
        
        // Check cache first
        if let Some(cache) = &self.cache {
            if let Some(data) = cache.get(&key) {
                return self.serializer.deserialize(&data);
            }
        }
        
        // Load from backend
        let data = self.backend.load(&key).await?;
        
        // Update cache
        if let Some(cache) = &self.cache {
            cache.set(&key, &data);
        }
        
        self.serializer.deserialize(&data)
    }
    
    // Batch operations
    pub async fn save_quizzes(&self, quizzes: &[Quiz]) -> Result<(), QuizlrError> {
        let futures = quizzes.iter()
            .map(|quiz| self.save_quiz(quiz))
            .collect::<Vec<_>>();
        
        futures::future::try_join_all(futures).await?;
        Ok(())
    }
}
```

## Data Organization

### Key Structure

Quizlr uses a hierarchical key structure:

```
quizzes/{quiz_id}                    # Quiz definitions
sessions/{user_id}/{session_id}       # Quiz sessions
results/{user_id}/{quiz_id}/{date}    # Quiz results
questions/{topic_id}/{question_id}    # Question bank
users/{user_id}/profile              # User profiles
users/{user_id}/progress             # Learning progress
curricula/{curriculum_id}            # Curriculum definitions
```

### Data Formats

#### JSON (Default)

```json
{
  "version": "1.0",
  "type": "quiz",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "JavaScript Basics",
    "questions": [...]
  },
  "metadata": {
    "created_at": "2024-12-28T10:00:00Z",
    "updated_at": "2024-12-28T10:00:00Z",
    "author": "user123"
  }
}
```

#### MessagePack (Binary)

For performance-critical applications:

```rust
let serializer = Serializer::MessagePack;
let manager = StorageManager::new(storage)
    .with_serializer(serializer);
```

## Caching

### In-Memory Cache

```rust
pub struct MemoryCache {
    capacity: usize,
    ttl: Duration,
    entries: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl MemoryCache {
    pub fn new(capacity: usize, ttl: Duration) -> Self {
        Self {
            capacity,
            ttl,
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
```

### Redis Cache

```rust
pub struct RedisCache {
    client: redis::Client,
    prefix: String,
    ttl: Duration,
}

impl RedisCache {
    pub fn new(redis_url: &str, prefix: String) -> Result<Self, QuizlrError> {
        let client = redis::Client::open(redis_url)?;
        Ok(Self {
            client,
            prefix,
            ttl: Duration::from_secs(3600),
        })
    }
}
```

## Custom Storage Implementation

To implement a custom storage backend:

```rust
pub struct CustomStorage {
    // Your fields
}

#[async_trait]
impl Storage for CustomStorage {
    async fn save(&self, key: &str, data: &[u8]) -> Result<(), QuizlrError> {
        // Your save logic
    }
    
    async fn load(&self, key: &str) -> Result<Vec<u8>, QuizlrError> {
        // Your load logic
    }
    
    async fn delete(&self, key: &str) -> Result<(), QuizlrError> {
        // Your delete logic
    }
    
    async fn list(&self, prefix: &str) -> Result<Vec<String>, QuizlrError> {
        // Your list logic
    }
}
```

## Migration Support

### Data Migration

```rust
pub struct StorageMigrator {
    source: Box<dyn Storage>,
    destination: Box<dyn Storage>,
}

impl StorageMigrator {
    pub async fn migrate_all(&self) -> Result<MigrationReport, QuizlrError> {
        let keys = self.source.list("").await?;
        let mut report = MigrationReport::default();
        
        for key in keys {
            match self.migrate_key(&key).await {
                Ok(_) => report.successful += 1,
                Err(e) => {
                    report.failed += 1;
                    report.errors.push((key, e.to_string()));
                }
            }
        }
        
        Ok(report)
    }
    
    async fn migrate_key(&self, key: &str) -> Result<(), QuizlrError> {
        let data = self.source.load(key).await?;
        self.destination.save(key, &data).await
    }
}
```

### Version Upgrades

```rust
pub struct DataUpgrader {
    version_map: HashMap<String, Box<dyn Fn(&[u8]) -> Result<Vec<u8>, QuizlrError>>>,
}

impl DataUpgrader {
    pub fn upgrade(&self, data: &[u8], from_version: &str, to_version: &str) 
        -> Result<Vec<u8>, QuizlrError> {
        // Apply upgrade functions in sequence
        let mut current_data = data.to_vec();
        let mut current_version = from_version.to_string();
        
        while current_version != to_version {
            let upgrade_fn = self.version_map.get(&current_version)
                .ok_or_else(|| QuizlrError::InvalidInput(
                    format!("No upgrade path from version {}", current_version)
                ))?;
            
            current_data = upgrade_fn(&current_data)?;
            current_version = self.next_version(&current_version);
        }
        
        Ok(current_data)
    }
}
```

## Performance Considerations

### Batch Operations

```rust
// Efficient batch loading
let ids = vec![id1, id2, id3];
let keys: Vec<String> = ids.iter()
    .map(|id| format!("quizzes/{}", id))
    .collect();

let futures = keys.iter()
    .map(|key| storage.load(key))
    .collect::<Vec<_>>();

let results = futures::future::join_all(futures).await;
```

### Streaming Large Data

```rust
pub trait StreamingStorage: Storage {
    async fn save_stream(
        &self, 
        key: &str, 
        stream: impl Stream<Item = Result<Bytes, Error>>
    ) -> Result<(), QuizlrError>;
    
    async fn load_stream(
        &self, 
        key: &str
    ) -> Result<impl Stream<Item = Result<Bytes, Error>>, QuizlrError>;
}
```

### Connection Pooling

```rust
pub struct PooledStorage {
    pool: Arc<Pool<StorageConnection>>,
}

impl PooledStorage {
    pub fn new(config: PoolConfig) -> Result<Self, QuizlrError> {
        let pool = Pool::builder()
            .max_size(config.max_connections)
            .min_idle(config.min_idle)
            .build(StorageConnectionManager::new(config))?;
        
        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}
```

## Error Handling

### Storage Errors

```rust
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Key not found: {0}")]
    NotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Storage quota exceeded")]
    QuotaExceeded,
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
}
```

### Retry Logic

```rust
pub struct RetryStorage {
    inner: Box<dyn Storage>,
    max_retries: u32,
    backoff: ExponentialBackoff,
}

#[async_trait]
impl Storage for RetryStorage {
    async fn save(&self, key: &str, data: &[u8]) -> Result<(), QuizlrError> {
        retry(self.backoff.clone(), || async {
            self.inner.save(key, data).await
                .map_err(|e| match e {
                    QuizlrError::Network(_) => backoff::Error::Transient(e),
                    _ => backoff::Error::Permanent(e),
                })
        }).await
    }
}
```

## Best Practices

### Key Design

1. **Hierarchical**: Use path-like keys for organization
2. **Versioned**: Include version in key for upgrades
3. **Timestamped**: Add timestamps for time-series data
4. **Indexed**: Design keys for efficient listing

### Data Safety

1. **Atomic Writes**: Use temp files and rename
2. **Checksums**: Verify data integrity
3. **Backups**: Regular automated backups
4. **Encryption**: Encrypt sensitive data at rest

### Performance

1. **Caching**: Cache frequently accessed data
2. **Compression**: Compress large datasets
3. **Pagination**: List operations with limits
4. **Concurrent**: Parallel operations when possible

## Examples

### Complete Storage Setup

```rust
use quizlr_core::storage::*;

// Development setup
let storage = LocalStorage::new("./data")
    .with_encryption(generate_key());

// Production setup
let storage = S3Storage::new(
    env::var("S3_BUCKET")?,
    env::var("AWS_REGION")?,
    env::var("AWS_ACCESS_KEY")?,
    env::var("AWS_SECRET_KEY")?,
);

// Create manager with caching
let cache = MemoryCache::new(1000, Duration::from_secs(300));
let manager = StorageManager::new(Box::new(storage))
    .with_cache(cache)
    .with_serializer(Serializer::MessagePack);

// Use the manager
let quiz = create_quiz();
manager.save_quiz(&quiz).await?;

let loaded = manager.load_quiz(quiz.id).await?;
assert_eq!(quiz.id, loaded.id);
```

### Migration Example

```rust
// Migrate from local to cloud
let local = LocalStorage::new("./data");
let cloud = GitHubStorage::new(owner, repo, token);

let migrator = StorageMigrator::new(
    Box::new(local),
    Box::new(cloud),
);

let report = migrator.migrate_all().await?;
println!("Migrated {} items ({} failed)", 
    report.successful, 
    report.failed
);
```
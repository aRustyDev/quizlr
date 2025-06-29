# Storage Backends

Learn how to implement custom storage backends for Quizlr.

## Built-in Backends

Quizlr includes several storage backends:
- **SQLite**: Default local storage
- **PostgreSQL**: For production deployments
- **In-Memory**: For testing

## Implementing Custom Backends

To create a custom storage backend:

1. **Implement the StorageBackend trait**:
   ```rust
   pub trait StorageBackend {
       async fn save_quiz(&self, quiz: &Quiz) -> Result<()>;
       async fn load_quiz(&self, id: Uuid) -> Result<Quiz>;
       // ... other methods
   }
   ```

2. **Handle serialization**:
   - Convert domain objects to storage format
   - Handle schema migrations

3. **Ensure thread safety**:
   - Use appropriate synchronization
   - Consider connection pooling

## Example: Redis Backend

```rust
pub struct RedisBackend {
    client: redis::Client,
}

impl StorageBackend for RedisBackend {
    // Implementation details
}
```

For the complete API reference, see [Storage API](../reference/storage-api.md).
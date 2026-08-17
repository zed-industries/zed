# Design Document

## Overview

The CRC parameters caching system will add a thread-safe, memory-efficient cache to the `CrcParams::new()` method. The cache will store pre-computed folding keys indexed by the input parameters, eliminating redundant key generation for identical parameter sets. The design prioritizes performance, thread safety, and minimal memory overhead while maintaining complete API compatibility.

## Architecture

### Cache Structure

The caching system will use a global, thread-safe cache implemented with:

- **Cache Storage**: `std::collections::HashMap<CrcParamsCacheKey, [u64; 23]>`
- **Thread Safety**: `std::sync::RwLock` for concurrent read access with exclusive write access
- **Cache Key**: Custom struct containing all parameters that affect key generation
- **Lazy Initialization**: `std::sync::OnceLock` to initialize the cache on first use

### Cache Key Design

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CrcParamsCacheKey {
    width: u8,
    poly: u64,
    reflected: bool,
}
```

The cache key includes only the parameters that directly affect key generation (`width`, `poly`, `reflected`), excluding parameters like `name`, `init`, `xorout`, and `check` which don't influence the mathematical key computation.

### Cache Access Pattern

1. **Cache Hit Path**: Read lock → HashMap lookup → Return cached keys
2. **Cache Miss Path**: Read lock → Cache miss → Generate keys → Write lock → Store in cache → Return keys
3. **Concurrent Access**: Multiple readers can access simultaneously; writers get exclusive access

## Components and Interfaces

### Core Components

#### 1. Cache Module (`src/cache.rs`)

```rust
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

static CACHE: OnceLock<RwLock<HashMap<CrcParamsCacheKey, [u64; 23]>>> = OnceLock::new();

pub fn get_or_generate_keys(width: u8, poly: u64, reflected: bool) -> [u64; 23]
pub fn clear_cache()  // For testing and memory management
```

#### 2. Cache Key Structure

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CrcParamsCacheKey {
    width: u8,
    poly: u64,
    reflected: bool,
}
```

#### 3. Modified CrcParams Implementation

The existing `CrcParams::new()` method will be updated to use the cache:

```rust
impl CrcParams {
    pub fn new(
        name: &'static str,
        width: u8,
        poly: u64,
        init: u64,
        reflected: bool,
        xorout: u64,
        check: u64,
    ) -> Self {
        let keys = cache::get_or_generate_keys(width, poly, reflected);
        
        let algorithm = match width {
            32 => CrcAlgorithm::Crc32Custom,
            64 => CrcAlgorithm::Crc64Custom,
            _ => panic!("Unsupported width: {}", width),
        };

        Self {
            algorithm,
            name,
            width,
            poly,
            init,
            refin: reflected,
            refout: reflected,
            xorout,
            check,
            keys,
        }
    }
}
```

### Interface Design

#### Public Interface
- No changes to existing public APIs
- `CrcParams::new()` maintains identical signature and behavior
- Cache operations are completely internal

#### Internal Interface
- `cache::get_or_generate_keys()` - Primary cache interface
- `cache::clear_cache()` - For testing and memory management
- Cache key creation and hashing handled internally

## Data Models

### Cache Key Model
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CrcParamsCacheKey {
    width: u8,      // CRC width (32 or 64)
    poly: u64,      // Polynomial value
    reflected: bool, // Reflection mode
}
```

### Cache Storage Model
```rust
type CacheStorage = HashMap<CrcParamsCacheKey, [u64; 23]>;
type ThreadSafeCache = RwLock<CacheStorage>;
```

### Memory Layout Considerations
- Cache keys: ~17 bytes per entry (8 + 8 + 1 bytes + HashMap overhead)
- Cache values: 184 bytes per entry (23 × 8 bytes)
- Total per entry: ~201 bytes + HashMap overhead
- Expected usage: 1-10 unique parameter sets in typical applications (single parameter set most common)

## Error Handling

### Cache Access Errors
- **RwLock Poisoning**: If a thread panics while holding the write lock, subsequent accesses will fall back to direct key generation
- **Memory Allocation**: HashMap growth failures will be handled by Rust's standard allocation error handling

### Fallback Strategy
```rust
fn get_or_generate_keys(width: u8, poly: u64, reflected: bool) -> [u64; 23] {
    let cache_key = CrcParamsCacheKey { width, poly, reflected };
    
    // Try cache read first
    if let Ok(cache) = get_cache().read() {
        if let Some(keys) = cache.get(&cache_key) {
            return *keys;
        }
    }
    
    // Generate keys outside of write lock to minimize lock hold time
    let keys = generate::keys(width, poly, reflected);
    
    // Try to cache the result (best effort)
    if let Ok(mut cache) = get_cache().write() {
        cache.insert(cache_key, keys);
    }
    
    keys
}
```

### Error Recovery
- Lock poisoning: Continue with direct key generation
- Memory pressure: Cache operations become no-ops, functionality preserved
- Hash collisions: Handled by HashMap implementation

## Testing Strategy

### Unit Tests
1. **Cache Functionality**
   - Verify cache hits return identical keys
   - Verify cache misses generate and store keys
   - Test cache key equality and hashing

2. **Thread Safety**
   - Concurrent read access tests
   - Read-write contention tests
   - Cache consistency under concurrent access

3. **Performance Tests**
   - Benchmark cache hit vs. miss performance
   - Memory usage validation
   - Comparison with uncached implementation

4. **Edge Cases**
   - Empty cache behavior
   - Cache with single entry
   - Maximum realistic cache size
   - Lock poisoning recovery

### Integration Tests
1. **API Compatibility**
   - Existing CrcParams::new() behavior unchanged
   - All existing tests continue to pass
   - Identical results for cached vs. uncached keys

2. **Real-world Usage Patterns**
   - Multiple CrcParams instances with same parameters
   - Mixed usage with different parameters
   - Long-running application simulation

### Performance Benchmarks
1. **Cache Hit Performance**: Measure lookup time vs. key generation time
2. **Cache Miss Performance**: Measure overhead of cache check + generation
3. **Memory Usage**: Track cache memory consumption over time
4. **Concurrent Access**: Measure performance under thread contention

## Implementation Phases

### Phase 1: Core Cache Implementation
- Create cache module with basic HashMap storage
- Implement thread-safe access with RwLock
- Add cache key structure and hashing

### Phase 2: Integration
- Modify CrcParams::new() to use cache
- Add fallback error handling
- Ensure API compatibility

### Phase 3: Testing and Optimization
- Comprehensive test suite
- Performance benchmarking
- Memory usage optimization
- Documentation updates

## Performance Considerations

### Cache Hit Performance
- Expected improvement: 50-100x faster than key generation
- RwLock read access: ~10-20ns overhead
- HashMap lookup: O(1) average case, ~50-100ns

### Cache Miss Performance
- Additional overhead: ~100-200ns for cache check
- Write lock acquisition: ~50-100ns
- HashMap insertion: O(1) average case

### Memory Efficiency
- Cache overhead per entry: ~201 bytes
- Expected cache size: 200 bytes - 2KB for typical applications
- Memory growth: Linear with unique parameter combinations

### Thread Contention
- Read-heavy workload: Excellent scalability
- Write contention: Minimal impact (writes are rare after warmup)
- Lock-free reads: Multiple threads can read simultaneously

## Security Considerations

### Memory Safety
- All cache operations use safe Rust constructs
- No unsafe code in cache implementation
- HashMap provides memory safety guarantees

### Thread Safety
- RwLock prevents data races
- Cache key immutability prevents modification after creation
- Atomic operations for cache initialization

### Resource Management
- Cache growth is bounded by unique parameter combinations
- No automatic eviction policy (acceptable for typical usage)
- Manual cache clearing available for memory management
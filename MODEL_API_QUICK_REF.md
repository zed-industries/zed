# Model API Quick Reference

This is a quick reference guide for the Model API rules defined in [MODEL_API_RULES.md](./MODEL_API_RULES.md).

## Core Concept

When developing an IDE for models (not vice versa), the API must be:
- **Explicit**: No ambiguity in what models can do
- **Safe**: Cannot break builds or compromise security
- **Predictable**: Same inputs produce same outputs
- **Observable**: All operations are logged and monitored

## At a Glance

### Allowed During Build Phase ✅

- **Read** project files and configuration
- **Analyze** code, dependencies, and build artifacts
- **Suggest** optimizations and fixes
- **Report** progress, errors, and metrics

### Prohibited During Build Phase ❌

- **Write** or modify files directly
- **Network** calls without approval
- **Mutate** global state or environment
- **Interfere** with build processes

## Key API Operations

### 1. Code Analysis
```json
{
  "operation": "analyze_code",
  "parameters": {
    "file_path": "/path/to/file.rs",
    "analysis_type": "semantic"
  }
}
```

### 2. Dependency Analysis
```json
{
  "operation": "analyze_dependencies",
  "parameters": {
    "manifest_path": "/path/to/Cargo.toml"
  }
}
```

### 3. Build Optimization
```json
{
  "operation": "suggest_optimizations",
  "parameters": {
    "build_config_path": "/path/to/config"
  }
}
```

## Request Structure

Every request must include:
```json
{
  "api_version": "1.0",
  "operation": "<operation_name>",
  "context": {
    "build_phase": "compilation|testing|analysis",
    "project_root": "/absolute/path",
    "timestamp": "ISO-8601"
  },
  "constraints": {
    "timeout_ms": 5000,
    "max_tokens": 1000,
    "read_only": true
  }
}
```

## Response Structure

Every response must include:
```json
{
  "api_version": "1.0",
  "status": "success|error|partial",
  "operation": "<operation_name>",
  "result": { /* operation-specific */ },
  "metadata": {
    "processing_time_ms": 123,
    "tokens_used": 456
  },
  "errors": [ /* if any */ ]
}
```

## Error Handling

### Error Types
- **Build Errors** (Critical) → Halt build
- **Model Errors** (Non-Critical) → Log and continue
- **System Errors** (Critical) → Halt and report

### Error Response
```json
{
  "status": "error",
  "errors": [{
    "code": "ERROR_CODE",
    "message": "Description",
    "severity": "error|warning|info",
    "recoverable": true
  }]
}
```

## Resource Limits

### Rate Limits
- **100** API calls per build session
- **10** concurrent requests maximum
- **1,000,000** tokens per build maximum

### Operation Limits
- `analyze_code`: 50 calls/build
- `analyze_dependencies`: 10 calls/build
- `suggest_optimizations`: 5 calls/build

### Timeouts
- Default: **5000ms**
- Minimum: **100ms**
- Maximum: **30000ms**

## Quick Implementation

### Basic Setup
```rust
let model_client = ModelApiClient::new(
    api_version: "1.0",
    base_url: config.model_api_url,
    timeout: Duration::from_millis(5000),
);
```

### Making a Call
```rust
let request = ModelApiRequest {
    api_version: "1.0".to_string(),
    operation: "analyze_code".to_string(),
    context: build_context(cx)?,
    parameters: json!({ "file_path": path }),
    constraints: RequestConstraints::default(),
};

let response = model_client.call(request).await?;
```

### Error Handling Pattern
```rust
match model_client.call(request).await {
    Ok(response) if response.status == "success" => {
        // Use results
    },
    Ok(response) => {
        log::warn!("Model returned non-success: {}", response.status);
        // Continue build without model enhancement
    },
    Err(e) => {
        log::error!("Model call failed: {}", e);
        // Build continues
    }
}
```

## Security Rules

1. **No code training** during build
2. **Least privilege** access only
3. **Sandboxed execution** required
4. **All operations logged** for audit
5. **No system files** access

## Testing Requirements

### Must Test
- ✅ Success paths
- ✅ Error handling
- ✅ Timeout behavior
- ✅ Rate limiting
- ✅ Resource constraints
- ✅ Build with model unavailable

### Mock for Tests
```rust
#[cfg(test)]
struct MockModelClient {
    responses: Vec<ModelApiResponse>,
}

impl MockModelClient {
    fn returns_success() -> Self {
        Self {
            responses: vec![success_response()]
        }
    }
}
```

## Configuration Example

```toml
[build.model_integration]
enabled = true
api_url = "http://localhost:8080/api/v1"
timeout_ms = 5000
max_calls_per_build = 100
max_concurrent_calls = 10
```

## Validation Checklist

Before deploying model integration:

- [ ] API version specified and validated
- [ ] All operations in allowed list
- [ ] Timeouts set appropriately
- [ ] Error handling implemented
- [ ] Rate limits configured
- [ ] Resource limits enforced
- [ ] Security sandboxing enabled
- [ ] Logging and monitoring active
- [ ] Tests cover error paths
- [ ] Build works without model

## Common Patterns

### Graceful Degradation
```rust
let analysis = match get_model_analysis(cx).await {
    Ok(a) => Some(a),
    Err(e) => {
        log::warn!("Model unavailable: {}", e);
        None
    }
};
// Build proceeds either way
```

### With Retry
```rust
for attempt in 0..3 {
    match model_client.call(request.clone()).await {
        Ok(response) => return Ok(response),
        Err(e) if is_transient(&e) => {
            delay(100 * 2_u64.pow(attempt)).await;
        }
        Err(e) => return Err(e),
    }
}
```

### With Caching
```rust
let cache_key = format!("{}:{}", operation, file_path);
if let Some(cached) = cache.get(&cache_key) {
    return Ok(cached.clone());
}
let response = model_client.call(request).await?;
cache.insert(cache_key, response.clone());
```

## Performance Expectations

- **Model overhead**: < 10% of total build time
- **Cache hit rate**: > 50% for repeated analyses
- **Success rate**: > 95% under normal conditions
- **P95 latency**: < 10 seconds per operation

## Monitoring Metrics

Track these metrics:
- Total API calls per build
- Success/failure rates
- Average latency
- Token usage
- Cache hit rates
- Error frequencies by type
- Resource utilization

## Getting Help

- Full API specification: [MODEL_API_RULES.md](./MODEL_API_RULES.md)
- Implementation guide: [docs/MODEL_BUILD_INTEGRATION.md](./docs/MODEL_BUILD_INTEGRATION.md)
- Agent documentation: [docs/src/ai/](./docs/src/ai/)
- Coding guidelines: [.rules](./.rules)
- Contributing: [CONTRIBUTING.md](./CONTRIBUTING.md)

## Version

- **API Version**: 1.0
- **Document Version**: 1.0.0
- **Last Updated**: 2024-02-04

# Model Integration Guide for Build Phase

This guide provides practical instructions for integrating AI/LLM models with the Zed IDE build system.

## Overview

This document complements [MODEL_API_RULES.md](../MODEL_API_RULES.md) by providing:
- Practical implementation patterns
- Code examples
- Integration workflows
- Testing strategies

## Quick Start

### 1. Setting Up Model Integration

To integrate a model with the build system:

```rust
use agent::ModelApiClient;
use gpui::App;

// Initialize model client
let model_client = ModelApiClient::new(
    api_version: "1.0",
    base_url: config.model_api_url,
    timeout: Duration::from_millis(5000),
);

// Register with build system
build_system.register_model_integration(model_client);
```

### 2. Making API Calls

All API calls must follow the contract defined in MODEL_API_RULES.md:

```rust
async fn analyze_code_during_build(
    file_path: &Path,
    cx: &mut AsyncApp,
) -> anyhow::Result<AnalysisResult> {
    let request = ModelApiRequest {
        api_version: "1.0".to_string(),
        operation: "analyze_code".to_string(),
        context: build_context(cx)?,
        parameters: json!({
            "file_path": file_path.to_string_lossy(),
            "analysis_type": "semantic",
        }),
        constraints: RequestConstraints::default(),
    };

    let response = cx
        .background_spawn(async move {
            model_api_client.call(request).await
        })
        .await?;

    handle_model_response(response)
}
```

## Integration Points

### Build Phase Integration Points

1. **Pre-Compilation Phase**
   - Static analysis
   - Dependency validation
   - Configuration checking

2. **Compilation Phase**
   - Error detection and suggestion
   - Code optimization hints
   - Type checking assistance

3. **Post-Compilation Phase**
   - Build artifact analysis
   - Performance profiling
   - Test suggestion generation

### Example: Pre-Compilation Hook

```rust
impl BuildPhaseHook for ModelAnalysisHook {
    fn execute(
        &self,
        context: &BuildContext,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        let model_client = self.model_client.clone();
        let files = context.source_files.clone();

        cx.spawn(async move |cx| {
            for file in files {
                let result = analyze_code_during_build(&file, &cx).await;
                
                match result {
                    Ok(analysis) => {
                        log::info!("Analysis complete for {:?}", file);
                        self.emit_diagnostics(analysis, &cx);
                    }
                    Err(e) => {
                        // Non-critical: log and continue
                        log::warn!("Model analysis failed for {:?}: {}", file, e);
                    }
                }
            }
            Ok(())
        })
    }
}
```

## Error Handling Patterns

### Pattern 1: Graceful Degradation

When model APIs fail, build should continue with reduced functionality:

```rust
async fn build_with_model_support(
    project: &Project,
    cx: &mut AsyncApp,
) -> anyhow::Result<BuildResult> {
    // Attempt model-enhanced build
    let model_analysis = match get_model_analysis(project, cx).await {
        Ok(analysis) => Some(analysis),
        Err(e) => {
            log::warn!("Model unavailable, continuing without AI features: {}", e);
            None
        }
    };

    // Build proceeds with or without model
    execute_build(project, model_analysis, cx).await
}
```

### Pattern 2: Timeout Handling

Always set and respect timeouts:

```rust
async fn call_model_with_timeout(
    request: ModelApiRequest,
    timeout: Duration,
) -> anyhow::Result<ModelApiResponse> {
    let future = model_client.call(request);
    
    match timeout(timeout, future).await {
        Ok(Ok(response)) => Ok(response),
        Ok(Err(e)) => Err(anyhow!("Model API error: {}", e)),
        Err(_) => Err(anyhow!("Model API timeout after {:?}", timeout)),
    }
}
```

### Pattern 3: Retry with Backoff

For transient failures, implement retry logic:

```rust
async fn call_model_with_retry(
    request: ModelApiRequest,
    max_retries: u32,
) -> anyhow::Result<ModelApiResponse> {
    let mut delay = Duration::from_millis(100);
    
    for attempt in 0..max_retries {
        match model_client.call(request.clone()).await {
            Ok(response) => return Ok(response),
            Err(e) if is_transient_error(&e) && attempt < max_retries - 1 => {
                log::warn!("Retry attempt {} after error: {}", attempt + 1, e);
                smol::Timer::after(delay).await;
                delay *= 2; // Exponential backoff
            }
            Err(e) => return Err(e),
        }
    }
    
    Err(anyhow!("Max retries exceeded"))
}

fn is_transient_error(error: &anyhow::Error) -> bool {
    // Check if error is temporary (timeout, network, etc.)
    error.to_string().contains("timeout") || 
    error.to_string().contains("network")
}
```

## Resource Management

### Memory Management

```rust
struct ModelApiClient {
    response_cache: Arc<Mutex<LruCache<String, CachedResponse>>>,
    max_cache_size_mb: usize,
}

impl ModelApiClient {
    fn cache_response(&self, key: String, response: ModelApiResponse) {
        let mut cache = self.response_cache.lock();
        
        // Evict old entries if cache is full
        let response_size = estimate_size(&response);
        if cache.total_size() + response_size > self.max_cache_size_mb * 1024 * 1024 {
            cache.evict_lru();
        }
        
        cache.insert(key, CachedResponse {
            response,
            timestamp: Instant::now(),
        });
    }
}
```

### Rate Limiting

```rust
struct RateLimiter {
    calls_per_build: Arc<AtomicU32>,
    max_calls: u32,
    build_id: String,
}

impl RateLimiter {
    fn check_and_increment(&self) -> anyhow::Result<()> {
        let current = self.calls_per_build.fetch_add(1, Ordering::SeqCst);
        
        if current >= self.max_calls {
            Err(anyhow!(
                "Rate limit exceeded: {} calls per build",
                self.max_calls
            ))
        } else {
            Ok(())
        }
    }
}
```

## Testing Strategies

### Unit Testing with Mocks

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct MockModelClient {
        responses: Vec<ModelApiResponse>,
    }

    impl MockModelClient {
        fn new() -> Self {
            Self {
                responses: vec![
                    ModelApiResponse {
                        status: "success".to_string(),
                        result: json!({
                            "findings": []
                        }),
                        ..Default::default()
                    }
                ],
            }
        }
    }

    #[gpui::test]
    async fn test_code_analysis(cx: &mut TestAppContext) {
        let mock_client = MockModelClient::new();
        let result = analyze_code_during_build(
            Path::new("/test/file.rs"),
            &mock_client,
            cx,
        ).await;

        assert!(result.is_ok());
    }
}
```

### Integration Testing

```rust
#[gpui::test]
async fn test_build_with_model_integration(cx: &mut TestAppContext) {
    // Setup test project
    let project = create_test_project(cx);
    
    // Configure model client
    let model_config = ModelConfig {
        enabled: true,
        timeout_ms: 1000,
        max_calls: 10,
    };
    
    // Execute build
    let result = build_with_model_support(&project, model_config, cx).await;
    
    // Verify build succeeded
    assert!(result.is_ok());
    
    // Verify model was called
    assert!(model_metrics.calls_made > 0);
}
```

### Performance Testing

```rust
#[test]
fn test_model_performance_impact() {
    let baseline_duration = measure_build_without_model();
    let with_model_duration = measure_build_with_model();
    
    // Model should add less than 10% overhead
    let overhead_ratio = (with_model_duration - baseline_duration) as f64 
                        / baseline_duration as f64;
    
    assert!(
        overhead_ratio < 0.10,
        "Model overhead too high: {:.2}%",
        overhead_ratio * 100.0
    );
}
```

## Monitoring and Observability

### Metrics Collection

```rust
struct ModelMetrics {
    total_calls: AtomicU64,
    successful_calls: AtomicU64,
    failed_calls: AtomicU64,
    total_latency_ms: AtomicU64,
    total_tokens_used: AtomicU64,
}

impl ModelMetrics {
    fn record_call(&self, result: &Result<ModelApiResponse, anyhow::Error>, latency_ms: u64) {
        self.total_calls.fetch_add(1, Ordering::Relaxed);
        
        match result {
            Ok(response) => {
                self.successful_calls.fetch_add(1, Ordering::Relaxed);
                if let Some(tokens) = response.metadata.tokens_used {
                    self.total_tokens_used.fetch_add(tokens, Ordering::Relaxed);
                }
            }
            Err(_) => {
                self.failed_calls.fetch_add(1, Ordering::Relaxed);
            }
        }
        
        self.total_latency_ms.fetch_add(latency_ms, Ordering::Relaxed);
    }
    
    fn report(&self) {
        let total = self.total_calls.load(Ordering::Relaxed);
        let success = self.successful_calls.load(Ordering::Relaxed);
        let failed = self.failed_calls.load(Ordering::Relaxed);
        let avg_latency = if total > 0 {
            self.total_latency_ms.load(Ordering::Relaxed) / total
        } else {
            0
        };
        
        log::info!(
            "Model API Stats - Total: {}, Success: {}, Failed: {}, Avg Latency: {}ms",
            total, success, failed, avg_latency
        );
    }
}
```

### Logging

```rust
fn log_model_call(
    operation: &str,
    request: &ModelApiRequest,
    response: &Result<ModelApiResponse, anyhow::Error>,
    duration: Duration,
) {
    match response {
        Ok(resp) => {
            log::info!(
                "Model API call succeeded: operation={}, status={}, duration={}ms",
                operation,
                resp.status,
                duration.as_millis()
            );
        }
        Err(e) => {
            log::error!(
                "Model API call failed: operation={}, error={}, duration={}ms",
                operation,
                e,
                duration.as_millis()
            );
        }
    }
}
```

## Configuration

### Build Configuration

Add to `Cargo.toml` or build configuration:

```toml
[build.model_integration]
enabled = true
api_url = "http://localhost:8080/api/v1"
timeout_ms = 5000
max_calls_per_build = 100
max_concurrent_calls = 10

[build.model_integration.operations]
analyze_code = { enabled = true, max_calls = 50 }
analyze_dependencies = { enabled = true, max_calls = 10 }
suggest_optimizations = { enabled = true, max_calls = 5 }
```

### Runtime Configuration

```rust
#[derive(Debug, Clone, Deserialize)]
struct ModelBuildConfig {
    enabled: bool,
    api_url: String,
    timeout_ms: u64,
    max_calls_per_build: u32,
    max_concurrent_calls: u32,
    operations: HashMap<String, OperationConfig>,
}

impl Default for ModelBuildConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            api_url: "http://localhost:8080/api/v1".to_string(),
            timeout_ms: 5000,
            max_calls_per_build: 100,
            max_concurrent_calls: 10,
            operations: Default::default(),
        }
    }
}
```

## Security Considerations

### Sandboxing Model Operations

```rust
fn execute_model_operation_sandboxed(
    operation: ModelOperation,
) -> anyhow::Result<ModelApiResponse> {
    // Create restricted context
    let sandbox = Sandbox::new()
        .restrict_network(true)
        .restrict_filesystem(vec![project_root.clone()])
        .max_memory_mb(512)
        .max_cpu_percent(10)
        .timeout(Duration::from_millis(5000));
    
    // Execute in sandbox
    sandbox.execute(|| {
        operation.execute()
    })
}
```

### Input Validation

```rust
fn validate_model_request(request: &ModelApiRequest) -> anyhow::Result<()> {
    // Validate API version
    if request.api_version != "1.0" {
        return Err(anyhow!("Unsupported API version: {}", request.api_version));
    }
    
    // Validate operation
    if !ALLOWED_OPERATIONS.contains(&request.operation.as_str()) {
        return Err(anyhow!("Invalid operation: {}", request.operation));
    }
    
    // Validate paths are within project
    if let Some(file_path) = request.parameters.get("file_path") {
        let path = Path::new(file_path.as_str().unwrap());
        if !path.starts_with(&request.context.project_root) {
            return Err(anyhow!("File path outside project root"));
        }
    }
    
    // Validate constraints
    if request.constraints.timeout_ms < 100 || request.constraints.timeout_ms > 30000 {
        return Err(anyhow!("Invalid timeout: must be 100-30000ms"));
    }
    
    Ok(())
}
```

## Troubleshooting

### Common Issues

1. **Model API Timeout**
   - Increase timeout in configuration
   - Check network connectivity
   - Verify model service is running

2. **Rate Limit Exceeded**
   - Reduce frequency of calls
   - Implement caching
   - Increase rate limit if appropriate

3. **High Resource Usage**
   - Enable response caching
   - Reduce max_concurrent_calls
   - Lower timeout values

### Debug Mode

Enable debug logging:

```rust
env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
    .init();

log::debug!("Model API request: {:?}", request);
log::debug!("Model API response: {:?}", response);
```

## Migration Guide

### Migrating from Non-Model Builds

1. **Phase 1: Add Configuration**
   - Add model integration config with `enabled = false`
   - Deploy configuration to all environments

2. **Phase 2: Enable in Development**
   - Enable in dev/test environments
   - Monitor performance and errors
   - Gather feedback

3. **Phase 3: Gradual Rollout**
   - Enable for percentage of builds
   - Monitor metrics closely
   - Scale up gradually

4. **Phase 4: Full Deployment**
   - Enable for all builds
   - Continue monitoring
   - Optimize based on data

## Best Practices Summary

1. **Always handle failures gracefully** - Build should work without model
2. **Set appropriate timeouts** - Balance speed and accuracy
3. **Implement caching** - Reduce redundant API calls
4. **Monitor metrics** - Track performance and errors
5. **Test thoroughly** - Unit, integration, and performance tests
6. **Validate inputs** - Ensure all requests are safe
7. **Log comprehensively** - Enable debugging and auditing
8. **Follow API contract** - Adhere to MODEL_API_RULES.md
9. **Resource limits** - Respect memory and CPU constraints
10. **Security first** - Sandbox operations and validate paths

## References

- [MODEL_API_RULES.md](../MODEL_API_RULES.md) - API contract and rules
- [Zed Agent Documentation](../docs/src/ai/agent-panel.md)
- [GPUI Guidelines](../.rules) - Concurrency and state management
- [Testing Guidelines](../CONTRIBUTING.md) - Testing best practices

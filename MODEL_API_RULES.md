# Model API Rules for Building Phase

This document defines the API contract, requirements, and best practices for AI/LLM model integration during the Zed IDE building phase. These rules ensure consistent, reliable, and maintainable model interactions.

## Purpose

When developing an IDE for models (rather than models for the IDE), it is imperative that:
- The API contract is explicit and unambiguous
- Models understand exactly what they can and cannot do during build operations
- Error handling is predictable and consistent
- All operations are safe and do not introduce breaking changes

## Core Principles

### 1. API Contract Clarity

All model interactions during the building phase must adhere to a strict contract:

- **Deterministic Behavior**: Model API calls must produce predictable results for the same inputs
- **Version Stability**: API endpoints and data structures must maintain backward compatibility
- **Explicit Capabilities**: Models must declare their capabilities upfront
- **Clear Boundaries**: Define what models can access and modify during build phase

### 2. Build Phase Operations

The building phase includes:
- Code compilation
- Dependency resolution
- Asset bundling
- Test execution
- Static analysis
- Code generation

During these operations, models must follow these rules:

#### Allowed Operations

1. **Read-Only File Access**
   - Models can read project files
   - Models can access build configuration
   - Models can read dependency manifests

2. **Analysis Operations**
   - Code parsing and syntax analysis
   - Dependency graph analysis
   - Static code analysis
   - Type checking assistance

3. **Suggestion Generation**
   - Code completion suggestions
   - Build optimization recommendations
   - Error explanation and fix suggestions

4. **Reporting**
   - Build progress reporting
   - Error and warning reporting
   - Performance metrics collection

#### Prohibited Operations

1. **Direct File Modification**
   - Models cannot directly modify source files during build
   - All modifications must go through approved tooling APIs

2. **Network Access**
   - No unrestricted network calls during build
   - External API calls must be explicitly approved and rate-limited

3. **State Mutation**
   - Cannot modify global build state
   - Cannot change environment variables

4. **Build Process Interference**
   - Cannot stop or restart build processes
   - Cannot modify build tool configurations

## API Structure

### Request Format

All model API requests during build phase must follow this structure:

```json
{
  "api_version": "1.0",
  "operation": "operation_name",
  "context": {
    "build_phase": "compilation|testing|analysis",
    "project_root": "/absolute/path/to/project",
    "timestamp": "ISO-8601 timestamp"
  },
  "parameters": {
    // Operation-specific parameters
  },
  "constraints": {
    "timeout_ms": 5000,
    "max_tokens": 1000,
    "read_only": true
  }
}
```

### Response Format

All model API responses must follow this structure:

```json
{
  "api_version": "1.0",
  "status": "success|error|partial",
  "operation": "operation_name",
  "result": {
    // Operation-specific result data
  },
  "metadata": {
    "processing_time_ms": 123,
    "tokens_used": 456,
    "cache_hit": false
  },
  "errors": [
    {
      "code": "ERROR_CODE",
      "message": "Human-readable error message",
      "severity": "error|warning|info"
    }
  ]
}
```

## Error Handling

### Error Categories

1. **Build Errors** (Critical)
   - Compilation failures
   - Missing dependencies
   - Configuration errors

2. **Model Errors** (Non-Critical)
   - API timeout
   - Invalid model response
   - Token limit exceeded

3. **System Errors** (Critical)
   - Out of memory
   - File system errors
   - Permission errors

### Error Propagation Rules

1. **Build Errors**: Must halt the build process
2. **Model Errors**: Should log and continue with degraded functionality
3. **System Errors**: Must halt and report to user

### Error Response Example

```json
{
  "api_version": "1.0",
  "status": "error",
  "operation": "code_analysis",
  "errors": [
    {
      "code": "TIMEOUT",
      "message": "Model response timeout after 5000ms",
      "severity": "error",
      "recoverable": true,
      "suggested_action": "Retry with reduced scope"
    }
  ]
}
```

## Request/Response Validation

### Request Validation Rules

1. **Required Fields**
   - All requests must include: `api_version`, `operation`, `context`
   - `context` must include: `build_phase`, `project_root`, `timestamp`

2. **Field Constraints**
   - `timeout_ms`: Must be between 100 and 30000
   - `max_tokens`: Must be between 1 and 100000
   - `project_root`: Must be an absolute path

3. **Operation Validation**
   - `operation` must be from allowed operations list
   - Parameters must match operation schema

### Response Validation Rules

1. **Required Fields**
   - All responses must include: `api_version`, `status`, `operation`
   - Error responses must include: `errors` array

2. **Field Constraints**
   - `status`: Must be one of: `success`, `error`, `partial`
   - `errors`: Must be an array (can be empty for success)
   - `processing_time_ms`: Must be a positive number

3. **Type Safety**
   - All fields must match declared types
   - No additional undeclared fields in strict mode

## Allowed Operations

### 1. Code Analysis

**Operation**: `analyze_code`

**Purpose**: Analyze code for errors, warnings, and suggestions

**Request Parameters**:
```json
{
  "file_path": "/absolute/path/to/file.rs",
  "analysis_type": "syntax|semantic|style",
  "include_suggestions": true
}
```

**Response Result**:
```json
{
  "findings": [
    {
      "type": "error|warning|info",
      "line": 42,
      "column": 10,
      "message": "Description of issue",
      "suggestion": "Suggested fix"
    }
  ]
}
```

### 2. Dependency Analysis

**Operation**: `analyze_dependencies`

**Purpose**: Analyze project dependencies

**Request Parameters**:
```json
{
  "manifest_path": "/absolute/path/to/Cargo.toml",
  "include_vulnerabilities": true
}
```

**Response Result**:
```json
{
  "dependencies": [
    {
      "name": "dependency_name",
      "version": "1.0.0",
      "status": "ok|outdated|vulnerable"
    }
  ]
}
```

### 3. Build Optimization

**Operation**: `suggest_optimizations`

**Purpose**: Suggest build performance improvements

**Request Parameters**:
```json
{
  "build_config_path": "/absolute/path/to/config",
  "current_build_time_ms": 5000
}
```

**Response Result**:
```json
{
  "suggestions": [
    {
      "category": "compilation|linking|caching",
      "description": "Description of optimization",
      "estimated_improvement_ms": 1000,
      "implementation_complexity": "low|medium|high"
    }
  ]
}
```

## Rate Limiting and Resource Constraints

### API Rate Limits

1. **Per-Build Limits**
   - Maximum 100 API calls per build session
   - Maximum 10 concurrent requests
   - Maximum 1,000,000 tokens per build

2. **Per-Operation Limits**
   - `analyze_code`: 50 calls per build
   - `analyze_dependencies`: 10 calls per build
   - `suggest_optimizations`: 5 calls per build

3. **Timeout Constraints**
   - Default timeout: 5000ms
   - Maximum timeout: 30000ms
   - Minimum timeout: 100ms

### Resource Usage

1. **Memory Limits**
   - Maximum response size: 10MB
   - Maximum request size: 1MB

2. **CPU Limits**
   - Model operations should not exceed 10% CPU usage
   - Background operations must yield to build tasks

## Security Requirements

### 1. Data Privacy

- No model training on user code during build
- No external transmission of code without explicit consent
- All API calls must be logged for audit

### 2. Access Control

- Models operate with least-privilege principle
- File system access limited to project directory
- No access to system files or environment secrets

### 3. Sandboxing

- Model operations run in isolated environment
- No direct system calls
- All I/O goes through approved APIs

## Versioning and Compatibility

### API Versioning

- Current version: `1.0`
- Version format: `MAJOR.MINOR`
- Breaking changes require MAJOR version bump
- Backward-compatible additions require MINOR version bump

### Compatibility Promise

1. **Backward Compatibility**
   - API v1.x will maintain compatibility
   - Deprecated features will be supported for 2 major versions

2. **Forward Compatibility**
   - Clients must ignore unknown fields
   - Servers may add optional fields without version bump

3. **Migration Path**
   - Clear migration guides for breaking changes
   - Deprecation warnings 6 months before removal

## Best Practices

### For Model Developers

1. **Always validate inputs**: Check all request parameters before processing
2. **Handle timeouts gracefully**: Implement progressive timeout handling
3. **Use caching**: Cache repeated analysis results
4. **Fail fast**: Return early errors rather than partial corrupt data
5. **Log comprehensively**: Log all operations for debugging

### For IDE Developers

1. **Set appropriate timeouts**: Balance responsiveness and accuracy
2. **Handle failures gracefully**: Build should succeed even if model fails
3. **Monitor resource usage**: Track and limit model resource consumption
4. **Provide fallbacks**: Have non-model alternatives for critical operations
5. **Test error paths**: Ensure build works when model is unavailable

## Usage Examples

### Example 1: Code Analysis During Build

```rust
// Request code analysis
let request = ModelApiRequest {
    api_version: "1.0".to_string(),
    operation: "analyze_code".to_string(),
    context: RequestContext {
        build_phase: "compilation".to_string(),
        project_root: "/path/to/project".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    },
    parameters: json!({
        "file_path": "/path/to/project/src/main.rs",
        "analysis_type": "semantic",
        "include_suggestions": true
    }),
    constraints: RequestConstraints {
        timeout_ms: 5000,
        max_tokens: 1000,
        read_only: true,
    },
};

// Handle response
match model_api.call(request).await {
    Ok(response) => {
        if response.status == "success" {
            // Process findings
            for finding in response.result.findings {
                log::info!("Found issue: {}", finding.message);
            }
        }
    }
    Err(e) => {
        // Build continues, but log the error
        log::warn!("Model analysis failed: {}", e);
    }
}
```

### Example 2: Dependency Vulnerability Check

```rust
let request = ModelApiRequest {
    api_version: "1.0".to_string(),
    operation: "analyze_dependencies".to_string(),
    context: RequestContext {
        build_phase: "analysis".to_string(),
        project_root: project_root.clone(),
        timestamp: Utc::now().to_rfc3339(),
    },
    parameters: json!({
        "manifest_path": manifest_path,
        "include_vulnerabilities": true
    }),
    constraints: RequestConstraints {
        timeout_ms: 10000,
        max_tokens: 5000,
        read_only: true,
    },
};

match model_api.call(request).await {
    Ok(response) => {
        for dep in response.result.dependencies {
            if dep.status == "vulnerable" {
                log::error!("Vulnerable dependency: {} {}", dep.name, dep.version);
            }
        }
    }
    Err(e) => {
        log::warn!("Dependency analysis unavailable: {}", e);
    }
}
```

## Testing Requirements

### Unit Testing

1. **Mock Model Responses**: All model API calls must be mockable
2. **Test Error Paths**: Test all error conditions
3. **Test Timeouts**: Verify timeout handling
4. **Test Rate Limits**: Verify rate limiting works

### Integration Testing

1. **End-to-End Build**: Test full build with model integration
2. **Failure Scenarios**: Test build with model unavailable
3. **Performance**: Measure model API impact on build time
4. **Resource Usage**: Monitor memory and CPU usage

## Compliance and Monitoring

### Logging Requirements

All model API calls must log:
- Timestamp
- Operation name
- Parameters (sanitized)
- Response status
- Processing time
- Token usage

### Metrics Collection

Track and report:
- API call success rate
- Average response time
- Token usage per build
- Error frequency by type
- Resource utilization

### Audit Trail

Maintain audit logs for:
- All API calls
- All errors and failures
- Configuration changes
- Access control violations

## Updates and Evolution

This document is versioned and maintained alongside the Zed codebase.

- **Version**: 1.0.0
- **Last Updated**: 2024-02-04
- **Status**: Active

### Change Process

1. Propose changes via RFC
2. Review by core team
3. Update documentation
4. Implement with feature flag
5. Test in staging
6. Release with migration guide

## References

- [Zed Agent Documentation](./docs/src/ai/agent-panel.md)
- [LLM Providers](./docs/src/ai/llm-providers.md)
- [Agent Tools](./docs/src/ai/tools.md)
- [Privacy and Security](./docs/src/ai/privacy-and-security.md)
- [Rust Coding Guidelines](./.rules)
- [Agent Guidelines](./AGENTS.md)

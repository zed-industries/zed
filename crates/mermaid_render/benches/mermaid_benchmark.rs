use criterion::{criterion_group, criterion_main, Criterion};
use mermaid_render::MermaidTheme;

const LARGE_FLOWCHART: &str = r#"flowchart TD
    Start([System Boot Sequence]) --> InitCheck{Hardware Diagnostics}
    InitCheck -->|Pass| LoadConfig[Load Configuration Files]
    InitCheck -->|Fail| HWError([Hardware Failure Detected])
    HWError --> AlertOps[Alert Operations Team]
    AlertOps -.-> LogIncident[(Log Incident to Database)]

    LoadConfig --> ValidateConfig{Config Schema Valid?}
    ValidateConfig -->|Yes| ParseEnv[Parse Environment Variables]
    ValidateConfig -->|No| FallbackConfig[Use Fallback Configuration]
    FallbackConfig -.-> WarnLog[Emit Configuration Warning]
    WarnLog -.-> LogIncident

    ParseEnv --> InitDB[(Initialize Database Pool)]
    InitDB --> MigrateDB{Pending Migrations?}
    MigrateDB -->|Yes| RunMigrations[Execute Database Migrations]
    MigrateDB -->|No| DBReady([Database Ready])
    RunMigrations --> MigrationCheck{Migration Successful?}
    MigrationCheck -->|Yes| DBReady
    MigrationCheck -->|No| RollbackMigration[Rollback Failed Migration]
    RollbackMigration --> AlertOps

    subgraph AuthSubsystem [Authentication Subsystem]
        AuthInit[Initialize Auth Provider] --> LoadKeys[(Load Signing Keys)]
        LoadKeys --> ValidateKeys{Keys Valid and Not Expired?}
        ValidateKeys -->|Yes| AuthReady([Auth Service Ready])
        ValidateKeys -->|No| RotateKeys[Rotate Expired Keys]
        RotateKeys --> NotifyAdmins[Notify Security Administrators]
        NotifyAdmins -.-> LogIncident
        RotateKeys --> LoadKeys
        AuthReady --> TokenCache[Initialize Token Cache]
        TokenCache --> SessionStore[(Setup Session Store)]
        SessionStore --> RateLimiter[Configure Rate Limiter]
        RateLimiter --> AuthComplete([Authentication Subsystem Online])
    end

    subgraph CacheLayer [Distributed Cache Layer]
        CacheInit[Initialize Cache Nodes] --> DiscoverPeers([Discover Peer Nodes])
        DiscoverPeers --> HealthCheck{All Peers Healthy?}
        HealthCheck -->|Yes| SyncState[Synchronize Shared State]
        HealthCheck -->|No| EvictUnhealthy[Evict Unhealthy Nodes]
        EvictUnhealthy -.-> AlertOps
        SyncState --> WarmCache[Pre-warm Frequently Accessed Data]
        WarmCache --> SetTTL[Configure TTL Policies]
        SetTTL --> CacheReady([Cache Layer Operational])
        CacheReady --> InvalidationQueue[(Setup Invalidation Queue)]
        InvalidationQueue --> PubSubChannel[Open Pub/Sub Channel]
    end

    subgraph WorkerPool [Background Worker Pool]
        SpawnWorkers[Spawn Worker Threads] --> RegisterHandlers[Register Task Handlers]
        RegisterHandlers --> PriorityQueues{Multiple Priority Levels?}
        PriorityQueues -->|Yes| HighPriority([High Priority Queue])
        PriorityQueues -->|Yes| MedPriority([Medium Priority Queue])
        PriorityQueues -->|Yes| LowPriority([Low Priority Queue])
        PriorityQueues -->|No| SingleQueue([Default Task Queue])
        HighPriority --> Scheduler[Task Scheduler with Backpressure]
        MedPriority --> Scheduler
        LowPriority --> Scheduler
        SingleQueue --> Scheduler
        Scheduler --> DeadLetterQueue[(Dead Letter Queue for Failed Tasks)]
        DeadLetterQueue -.-> AlertOps
        Scheduler --> RetryPolicy[Configure Exponential Retry Policy]
        RetryPolicy --> WorkersReady([Worker Pool Active])
    end

    subgraph APIGateway [API Gateway and Routing]
        LoadRoutes[Load Route Definitions] --> Middleware[Apply Middleware Stack]
        Middleware --> CORSPolicy[Configure CORS Policy]
        CORSPolicy --> CompressionLayer[Enable Response Compression]
        CompressionLayer --> RequestValidation[Setup Request Validation]
        RequestValidation --> CircuitBreaker{Circuit Breaker Enabled?}
        CircuitBreaker -->|Yes| CBConfig[Configure Failure Thresholds]
        CircuitBreaker -->|No| DirectProxy[Direct Proxy Mode]
        CBConfig --> GatewayReady([API Gateway Listening])
        DirectProxy --> GatewayReady
    end

    DBReady ==> AuthInit
    DBReady ==> CacheInit
    AuthComplete ==> SpawnWorkers
    CacheReady ==> SpawnWorkers
    WorkersReady ==> LoadRoutes
    AuthComplete --> LoadRoutes

    GatewayReady --> HealthEndpoint[Register Health Check Endpoint]
    HealthEndpoint --> MetricsExporter[Start Prometheus Metrics Exporter]
    MetricsExporter --> ReadinessProbe{Readiness Probe Passing?}
    ReadinessProbe -->|Yes| AcceptTraffic([Begin Accepting Traffic])
    ReadinessProbe -->|No| DiagnosticDump[Generate Diagnostic Report]
    DiagnosticDump -.-> LogIncident
    DiagnosticDump --> ReadinessProbe

    AcceptTraffic --> Operational([System Fully Operational])
    Operational -.-> PeriodicHealthCheck[Periodic Health Monitoring]
    PeriodicHealthCheck -.-> MetricsExporter
"#;

fn bench_mermaid_rendering(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("mermaid");

    let theme = MermaidTheme::default();

    group.bench_function("merman/render_large_flowchart", |b| {
        b.iter(|| mermaid_render::render_to_svg(LARGE_FLOWCHART, &theme));
    });

    group.finish();
}

criterion_group!(benches, bench_mermaid_rendering);
criterion_main!(benches);

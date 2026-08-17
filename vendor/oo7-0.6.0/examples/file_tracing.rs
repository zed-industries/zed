//! Performance testing example for the file backend.
//!
//! This example demonstrates how to use tracing spans to measure performance
//! characteristics of keyring operations. It simulates realistic workloads
//! and measures timing across different scenarios.
//!
//! Run with: cargo run --example file_tracing --features "tokio tracing"
//! --release

use std::time::Instant;

use oo7::file::UnlockedKeyring;
use tempfile::tempdir;
use tracing::info;
use tracing_subscriber::{
    EnvFilter,
    fmt::{format::FmtSpan, time::SystemTime},
    prelude::*,
};

#[tokio::main]
async fn main() -> oo7::Result<()> {
    // Set up tracing subscriber to capture timing information
    let subscriber = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(SystemTime)
                .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
                .with_file(true)
                .with_line_number(true)
                .with_target(true),
        )
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("oo7=debug,file_tracing=info")),
        );

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    info!("Starting oo7 file backend performance tests");

    // Test scenarios
    test_keyring_lifecycle().await?;
    test_bulk_operations().await?;
    test_scaling_behavior().await?;
    test_search_performance().await?;

    info!("Performance tests completed");
    Ok(())
}

/// Test basic keyring lifecycle operations
async fn test_keyring_lifecycle() -> oo7::Result<()> {
    info!("=== Testing Keyring Lifecycle ===");

    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("lifecycle_test.keyring");
    let secret = oo7::Secret::from("test-secret-key-that-is-long-enough".as_bytes());

    // Measure keyring creation
    let start = Instant::now();
    let keyring = UnlockedKeyring::load(&keyring_path, secret.clone()).await?;
    let create_time = start.elapsed();
    info!("Fresh keyring creation: {:?}", create_time);

    // Measure item creation
    let start = Instant::now();
    keyring
        .create_item(
            "Test Item",
            &[("app", "test"), ("user", "alice")],
            "my-secret-password",
            false,
        )
        .await?;
    let item_create_time = start.elapsed();
    info!("Single item creation: {:?}", item_create_time);

    // Measure keyring reload (with existing data)
    drop(keyring);
    let start = Instant::now();
    let keyring = UnlockedKeyring::load(&keyring_path, secret).await?;
    let reload_time = start.elapsed();
    info!("Keyring reload with 1 item: {:?}", reload_time);

    // Measure search
    let start = Instant::now();
    let items = keyring.search_items(&[("app", "test")]).await?;
    let search_time = start.elapsed();
    info!(
        "Single item search: {:?} (found {} items)",
        search_time,
        items.len()
    );

    Ok(())
}

/// Test bulk operations performance
async fn test_bulk_operations() -> oo7::Result<()> {
    info!("=== Testing Bulk Operations ===");

    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("bulk_test.keyring");
    let secret = oo7::Secret::from("test-secret-key-that-is-long-enough".as_bytes());
    let keyring = UnlockedKeyring::load(&keyring_path, secret).await?;

    // Test creating multiple items individually
    let item_counts = [10, 50, 100];

    for count in item_counts {
        info!("Testing {} individual item creations", count);

        let start = Instant::now();
        for i in 0..count {
            keyring
                .create_item(
                    &format!("Item {}", i),
                    &[
                        ("app", "bulk_test"),
                        ("index", &i.to_string()),
                        ("batch", "individual"),
                    ],
                    format!("secret-{}", i),
                    false,
                )
                .await?;
        }
        let total_time = start.elapsed();
        let avg_time = total_time / count;

        info!(
            "{} items created individually: total={:?}, avg={:?}",
            count, total_time, avg_time
        );

        // Test search performance with more items
        let start = Instant::now();
        let items = keyring.search_items(&[("app", "bulk_test")]).await?;
        let search_time = start.elapsed();

        info!(
            "Search with {} total items: {:?} (found {} items)",
            keyring.n_items().await,
            search_time,
            items.len()
        );
    }

    Ok(())
}

/// Test how performance scales with keyring size
async fn test_scaling_behavior() -> oo7::Result<()> {
    info!("=== Testing Scaling Behavior ===");

    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("scaling_test.keyring");
    let secret = oo7::Secret::from("test-secret-key-that-is-long-enough".as_bytes());

    // Create progressively larger keyrings and measure operations
    let sizes = [0, 100, 500, 1000];

    for &size in &sizes {
        info!("Testing with keyring size: {}", size);

        // Create fresh keyring with 'size' items
        std::fs::remove_file(&keyring_path).ok(); // Remove if exists
        let keyring = UnlockedKeyring::load(&keyring_path, secret.clone()).await?;

        // Populate keyring
        if size > 0 {
            let start = Instant::now();
            for i in 0..size {
                keyring
                    .create_item(
                        &format!("Scale Item {}", i),
                        &[("app", "scaling_test"), ("index", &i.to_string())],
                        format!("scaling-secret-{}", i),
                        false,
                    )
                    .await?;
            }
            let populate_time = start.elapsed();
            info!("Populated {} items in: {:?}", size, populate_time);
        }

        // Test reload performance
        drop(keyring);
        let start = Instant::now();
        let keyring = UnlockedKeyring::load(&keyring_path, secret.clone()).await?;
        let reload_time = start.elapsed();
        info!("Reload with {} items: {:?}", size, reload_time);

        // Test adding one more item
        let start = Instant::now();
        keyring
            .create_item(
                "New Item",
                &[("app", "scaling_test"), ("type", "new")],
                "new-secret",
                false,
            )
            .await?;
        let add_time = start.elapsed();
        info!("Add item to keyring with {} items: {:?}", size, add_time);

        // Test search performance
        let start = Instant::now();
        let items = keyring.search_items(&[("app", "scaling_test")]).await?;
        let search_time = start.elapsed();
        info!(
            "Search keyring with {} items: {:?} (found {})",
            size + 1,
            search_time,
            items.len()
        );
    }

    Ok(())
}

/// Test search performance with different query patterns
async fn test_search_performance() -> oo7::Result<()> {
    info!("=== Testing Search Performance ===");

    let temp_dir = tempdir().unwrap();
    let keyring_path = temp_dir.path().join("search_test.keyring");
    let secret = oo7::Secret::from("test-secret-key-that-is-long-enough".as_bytes());
    let keyring = UnlockedKeyring::load(&keyring_path, secret).await?;

    // Create diverse set of items for search testing
    let apps = ["browser", "email", "social", "development", "finance"];
    let users = ["alice", "bob", "charlie", "diana", "eve"];

    info!("Creating test dataset...");
    let start = Instant::now();
    for (i, app) in apps.iter().enumerate() {
        for (j, user) in users.iter().enumerate() {
            keyring
                .create_item(
                    &format!("{} - {}", app, user),
                    &[
                        ("app", *app),
                        ("user", *user),
                        ("index", &(i * users.len() + j).to_string()),
                    ],
                    format!("{}-{}-secret", app, user),
                    false,
                )
                .await?;
        }
    }
    let create_time = start.elapsed();
    let total_items = apps.len() * users.len();
    info!("Created {} items in: {:?}", total_items, create_time);

    // Test different search patterns
    info!("Testing search patterns:");

    // 1. Exact match (should find 1 item)
    let start = Instant::now();
    let items = keyring
        .search_items(&[("app", "browser"), ("user", "alice")])
        .await?;
    let exact_time = start.elapsed();
    info!(
        "Exact match search: {:?} (found {} items)",
        exact_time,
        items.len()
    );

    // 2. Single attribute match (should find multiple items)
    let start = Instant::now();
    let items = keyring.search_items(&[("app", "browser")]).await?;
    let single_attr_time = start.elapsed();
    info!(
        "Single attribute search: {:?} (found {} items)",
        single_attr_time,
        items.len()
    );

    // 3. No match
    let start = Instant::now();
    let items = keyring.search_items(&[("app", "nonexistent")]).await?;
    let no_match_time = start.elapsed();
    info!(
        "No match search: {:?} (found {} items)",
        no_match_time,
        items.len()
    );

    // 4. Large result set
    let start = Instant::now();
    let items = keyring.items().await?;
    let all_items_time = start.elapsed();
    let valid_items = items.iter().filter(|r| r.is_ok()).count();
    info!(
        "Get all items: {:?} (found {} valid items)",
        all_items_time, valid_items
    );

    Ok(())
}

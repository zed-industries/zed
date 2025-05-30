//! # UIDE Basic Demo
//! 
//! This example demonstrates the core capabilities of the Unified Intelligent Data Engine:
//! - Storing different data types (structured, documents, vectors)
//! - Universal querying across all data types
//! - Intelligent indexing and search
//! - Real-time statistics

use serde::{Deserialize, Serialize};
use tempfile;
use tracing_subscriber;
use uide::{
    engine::UnifiedDataEngine,
    query::UniversalQuery,
    universal::{DataType, UniversalContent, UniversalRecord, VectorEncoding, StructuredBuilder},
};

#[derive(Debug, Serialize, Deserialize)]
struct KnowledgeItem {
    title: String,
    content: String,
    category: String,
    importance: f64,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CodeSnippet {
    language: String,
    code: String,
    description: String,
    performance_score: f64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing for better output
    tracing_subscriber::fmt::init();

    println!("🚀 UIDE (Unified Intelligent Data Engine) Demo");
    println!("{}", "=".repeat(50));

    // Create a temporary directory for the demo
    let temp_dir = tempfile::tempdir()?;
    let data_path = temp_dir.path().to_string_lossy();
    println!("📁 Using temporary database at: {}", data_path);

    // Initialize UIDE engine
    let engine = UnifiedDataEngine::new(data_path).await?;
    println!("✅ UIDE engine initialized successfully!");

    println!("\n📝 Demo 1: Storing Different Data Types");
    println!("{}", "-".repeat(40));

    // Store structured knowledge items
    let knowledge1 = KnowledgeItem {
        title: "Rust Memory Safety".to_string(),
        content: "Rust prevents memory bugs through ownership system".to_string(),
        category: "Programming".to_string(),
        importance: 9.5,
        tags: vec!["rust".to_string(), "memory".to_string(), "safety".to_string()],
    };

    let knowledge2 = KnowledgeItem {
        title: "AI Model Architecture".to_string(),
        content: "Transformer models use attention mechanisms for better performance".to_string(),
        category: "AI".to_string(),
        importance: 8.7,
        tags: vec!["ai".to_string(), "transformer".to_string(), "attention".to_string()],
    };

    let id1 = engine.store(&knowledge1).await?;
    let id2 = engine.store(&knowledge2).await?;
    println!("✅ Stored knowledge items: {} and {}", id1, id2);

    // Store code snippets
    let code1 = CodeSnippet {
        language: "rust".to_string(),
        code: "fn main() { println!(\"Hello, world!\"); }".to_string(),
        description: "Simple Hello World in Rust".to_string(),
        performance_score: 10.0,
    };

    let code_id = engine.store(&code1).await?;
    println!("✅ Stored code snippet: {}", code_id);

    // Store document content directly
    let doc_record = UniversalRecord::new(
        DataType::Document,
        UniversalContent::Document {
            text: "UIDE is a revolutionary storage engine that unifies all data types into one intelligent system".to_string(),
            tokens: None,
            language: Some("en".to_string()),
        },
    );
    let doc_id = engine.store_record(doc_record).await?;
    println!("✅ Stored document: {}", doc_id);

    // Store vector embeddings (simulated)
    let vector_record = UniversalRecord::new(
        DataType::Vector,
        UniversalContent::Vector {
            dimensions: 5,
            values: vec![0.8, 0.2, 0.9, 0.1, 0.7], // Simulated embedding for "rust programming"
            encoding: VectorEncoding::Float32,
        },
    );
    let vector_id = engine.store_record(vector_record).await?;
    println!("✅ Stored vector embedding: {}", vector_id);

    // Store complex structured data using builder
    let complex_record = UniversalRecord::new(
        DataType::Structured,
        StructuredBuilder::new()
            .text_field("title", "Learning Progress Tracker")
            .text_field("subject", "Machine Learning")
            .number_field("progress_percentage", 75.5)
            .bool_field("completed", false)
            .build(),
    );
    let complex_id = engine.store_record(complex_record).await?;
    println!("✅ Stored complex structured data: {}", complex_id);

    println!("\n🔍 Demo 2: Universal Search Capabilities");
    println!("{}", "-".repeat(40));

    // Text search across all stored data
    println!("\n🔎 Text Search for 'rust':");
    let text_query = UniversalQuery::text_search("rust");
    let results = engine.search(text_query).await?;
    println!("   Found {} results in {}ms", results.results.len(), results.query_time_ms);
    for (i, result) in results.results.iter().enumerate() {
        println!("   {}. Score: {:.3} - Type: {:?} - ID: {}", 
                 i + 1, result.score, result.record.data_type, result.record.id);
    }

    // Vector similarity search
    println!("\n🧮 Vector Similarity Search:");
    let similar_vector = vec![0.7, 0.3, 0.8, 0.2, 0.6]; // Similar to our stored vector
    let vector_query = UniversalQuery::vector_search(similar_vector, 0.5);
    let vector_results = engine.search(vector_query).await?;
    println!("   Found {} similar vectors in {}ms", vector_results.results.len(), vector_results.query_time_ms);
    for (i, result) in vector_results.results.iter().enumerate() {
        println!("   {}. Similarity: {:.3} - ID: {}", i + 1, result.score, result.record.id);
    }

    // Search by data type
    println!("\n📊 Search by Data Type (Documents):");
    let type_query = UniversalQuery::by_type(DataType::Document);
    let type_results = engine.search(type_query).await?;
    println!("   Found {} documents", type_results.results.len());

    // Complex query with filters
    println!("\n🔧 Complex Query with Filters:");
    let complex_query = UniversalQuery::builder()
        .text("learning")
        .filter_recent(chrono::Duration::hours(1)) // Recent items
        .limit(5)
        .sort_by_relevance()
        .build()?;
    let complex_results = engine.search(complex_query).await?;
    println!("   Found {} recent items with 'learning' in {}ms", 
             complex_results.results.len(), complex_results.query_time_ms);

    println!("\n📈 Demo 3: Engine Statistics");
    println!("{}", "-".repeat(40));

    let stats = engine.stats().await?;
    println!("📊 Storage Statistics:");
    println!("   - Total records: {}", stats.storage.record_count);
    println!("   - Relationships: {}", stats.storage.relationship_count);
    println!("   - Database size: {} bytes", stats.storage.database_size);
    println!("📊 Index Statistics:");
    println!("   - Data types indexed: {}", stats.index.types_count);
    println!("   - Tags indexed: {}", stats.index.tags_count);
    println!("   - Text terms indexed: {}", stats.index.text_terms_count);

    println!("\n🔄 Demo 4: Data Retrieval and Conversion");
    println!("{}", "-".repeat(40));

    // Retrieve and convert back to original type
    let retrieved_knowledge: Option<KnowledgeItem> = engine.retrieve(id1).await?;
    if let Some(knowledge) = retrieved_knowledge {
        println!("📖 Retrieved Knowledge Item:");
        println!("   Title: {}", knowledge.title);
        println!("   Category: {}", knowledge.category);
        println!("   Importance: {}", knowledge.importance);
        println!("   Tags: {:?}", knowledge.tags);
    }

    // Retrieve raw universal record
    if let Some(raw_record) = engine.get_record(doc_id).await? {
        println!("📄 Raw Document Record:");
        println!("   ID: {}", raw_record.id);
        println!("   Type: {:?}", raw_record.data_type);
        println!("   Timestamp: {}", raw_record.timestamp);
        if let Some(text) = raw_record.content.searchable_text() {
            println!("   Content: {}", text);
        }
    }

    println!("\n✨ Demo Complete!");
    println!("🎯 Key Features Demonstrated:");
    println!("   ✅ Universal data storage (structured, documents, vectors)");
    println!("   ✅ Intelligent querying (text, vector, type-based)");
    println!("   ✅ Automatic indexing and optimization");
    println!("   ✅ Type-safe data retrieval");
    println!("   ✅ Real-time statistics and monitoring");
    println!("\n🚀 UIDE successfully handles all data types in one unified engine!");

    Ok(())
} 
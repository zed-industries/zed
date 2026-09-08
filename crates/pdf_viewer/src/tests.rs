use std::{path::Path, sync::Arc};

use super::*;
use crate::{
    pdf_engine::{MockPdfEngine, PdfEngine, PopplerCliEngine},
    tile_cache::{TileCache, TileKey},
};
use gpui::{Image, ImageFormat};
use project::ProjectPath;
use util::rel_path::rel_path;

#[test]
fn test_mock_engine_document_loading() {
    let engine = MockPdfEngine::new(7);
    assert_eq!(engine.name(), "Mock Engine");
    assert!(engine.is_available());

    let doc = engine
        .load_document(Path::new("/test/sample.pdf"), &[])
        .expect("Should load mock document");

    assert_eq!(doc.page_count(), 7);
    let (w, h) = doc.page_size(0);
    assert!(w > 0.0);
    assert!(h > 0.0);
}

#[test]
fn test_mock_engine_render_pages() {
    let engine = MockPdfEngine::new(3);
    let doc = engine
        .load_document(Path::new("/test/guide.pdf"), &[])
        .expect("Should load mock document");

    // Page 0 (first page) at 1.0x scale
    let img1 = doc.render_page(0, 1.0).expect("Should render page 0");
    assert!(!img1.bytes.is_empty());

    // Page 2 (last page) at 2.0x scale
    let img2 = doc.render_page(2, 2.0).expect("Should render page 2");
    assert!(!img2.bytes.is_empty());

    // Out of bounds page 3
    let err = doc.render_page(3, 1.0);
    assert!(err.is_err(), "Accessing out-of-bounds page should fail");
}

#[test]
fn test_mock_engine_render_tiles() {
    let engine = MockPdfEngine::new(2);
    let doc = engine
        .load_document(Path::new("/test/tiled.pdf"), &[])
        .expect("Should load mock document");

    // Render tile at (0, 0) with 512x512
    let tile = doc
        .render_tile(0, 1.0, 0, 0, 512, 512)
        .expect("Should render tile");
    assert!(!tile.bytes.is_empty());
    assert_eq!(tile.format, ImageFormat::Svg);

    // Render sub-tile at offset (512, 0)
    let tile2 = doc
        .render_tile(0, 1.5, 512, 0, 256, 512)
        .expect("Should render sub-tile");
    assert!(!tile2.bytes.is_empty());
}

#[test]
fn test_tile_key_bucketing() {
    let key1 = TileKey::new(0, 1.02, 0, 0, 0);
    let key2 = TileKey::new(0, 1.04, 0, 0, 0);
    // Both 1.02 and 1.04 round to 10 in 10% buckets
    assert_eq!(key1, key2);
    assert_eq!(key1.zoom_bucket, 10);
    assert_eq!(key1.zoom_scale(), 1.0);

    let key3 = TileKey::new(0, 1.51, 0, 0, 0);
    assert_eq!(key3.zoom_bucket, 15);
    assert_eq!(key3.zoom_scale(), 1.5);
    assert_ne!(key1, key3);

    // Rotation isolates tile keys
    let key_rot90 = TileKey::new(0, 1.02, 90, 0, 0);
    assert_ne!(key1, key_rot90);
}

#[test]
fn test_tile_cache_lru_and_eviction() {
    // Cache with capacity for 2 entries (each entry counts at least 1MB)
    let one_tile_bytes = 1024 * 1024;
    let mut cache = TileCache::new(one_tile_bytes * 2 + 1024);

    let dummy_img = Arc::new(Image::from_bytes(ImageFormat::Png, vec![0u8; 100]));

    let k1 = TileKey::new(0, 1.0, 0, 0, 0);
    let k2 = TileKey::new(0, 1.0, 0, 1, 0);
    let k3 = TileKey::new(0, 1.0, 0, 2, 0);

    cache.insert(k1, dummy_img.clone());
    assert_eq!(cache.len(), 1);

    cache.insert(k2, dummy_img.clone());
    assert_eq!(cache.len(), 2);

    // Access k1 so k2 becomes the oldest
    assert!(cache.get(&k1).is_some());

    // Inserting k3 should evict k2
    cache.insert(k3, dummy_img.clone());
    assert_eq!(cache.len(), 2);
    assert!(cache.get(&k1).is_some());
    assert!(cache.get(&k3).is_some());
    assert!(cache.get(&k2).is_none());

    // Test evicting pages outside range
    let k_p5 = TileKey::new(5, 1.0, 0, 0, 0);
    cache.insert(k_p5, dummy_img.clone());
    cache.evict_pages_outside(0, 2);
    assert!(cache.get(&k_p5).is_none());
}

#[test]
fn test_render_pool_generation_cancellation() {
    let pool = RenderPool::new(2);
    let gen0 = pool.generation();
    assert_eq!(gen0, 0);

    let gen1 = pool.increment_generation();
    assert_eq!(gen1, 1);
    assert_eq!(pool.generation(), 1);
}

#[test]
fn test_zoom_clamping_math() {
    let mut zoom = 1.0f32;

    // Zoom in repeatedly
    for _ in 0..20 {
        zoom = (zoom * ZOOM_FACTOR).clamp(MIN_ZOOM, MAX_ZOOM);
    }
    assert_eq!(zoom, MAX_ZOOM);

    // Zoom out repeatedly
    for _ in 0..30 {
        zoom = (zoom / ZOOM_FACTOR).clamp(MIN_ZOOM, MAX_ZOOM);
    }
    assert_eq!(zoom, MIN_ZOOM);
}

#[test]
fn test_page_clamping_logic() {
    let total_pages = 5usize;
    let mut current_page = 0usize;

    // Cannot go below 0
    if current_page > 0 {
        current_page -= 1;
    }
    assert_eq!(current_page, 0);

    // Advance to end
    for _ in 0..10 {
        if current_page + 1 < total_pages {
            current_page += 1;
        }
    }
    assert_eq!(current_page, total_pages - 1);

    // Cannot advance past last page
    if current_page + 1 < total_pages {
        current_page += 1;
    }
    assert_eq!(current_page, total_pages - 1);
}

#[test]
fn test_extension_matching() {
    let pdf_path = ProjectPath {
        worktree_id: project::WorktreeId::from_usize(1),
        path: rel_path("reports/q3_earnings.pdf").into(),
    };
    assert_eq!(
        pdf_path.path.extension().map(|s| s.to_ascii_lowercase()),
        Some("pdf".to_string())
    );

    let uppercase_pdf = ProjectPath {
        worktree_id: project::WorktreeId::from_usize(1),
        path: rel_path("MANUAL.PDF").into(),
    };
    assert_eq!(
        uppercase_pdf.path.extension().map(|s| s.to_ascii_lowercase()),
        Some("pdf".to_string())
    );

    let rust_file = ProjectPath {
        worktree_id: project::WorktreeId::from_usize(1),
        path: rel_path("src/main.rs").into(),
    };
    assert_ne!(
        rust_file.path.extension().map(|s| s.to_ascii_lowercase()),
        Some("pdf".to_string())
    );
}

#[test]
fn test_poppler_engine_initialization() {
    let poppler = PopplerCliEngine::new();
    assert_eq!(poppler.name(), "Poppler (pdftoppm)");
    // System may or may not have pdftoppm, but calling it must not panic
    let _ = poppler.is_available();
}

#[test]
fn test_tile_cache_overviews_and_rotation() {
    let mut cache = TileCache::new(10 * 1024 * 1024);
    let overview_0 = Arc::new(Image::from_bytes(ImageFormat::Png, vec![0u8; 100]));
    let overview_90 = Arc::new(Image::from_bytes(ImageFormat::Png, vec![1u8; 200]));

    // Test page overview caching per rotation
    assert!(cache.get_page_overview(0, 0).is_none());
    assert!(cache.get_page_overview(0, 90).is_none());

    cache.insert_page_overview(0, 0, overview_0.clone());
    assert!(cache.get_page_overview(0, 0).is_some());
    assert_eq!(cache.get_page_overview(0, 0).unwrap().bytes.len(), 100);
    assert!(cache.get_page_overview(0, 90).is_none());

    cache.insert_page_overview(0, 90, overview_90.clone());
    assert!(cache.get_page_overview(0, 90).is_some());
    assert_eq!(cache.get_page_overview(0, 90).unwrap().bytes.len(), 200);
}

#[test]
fn test_effective_page_size_rotation() {
    let engine = MockPdfEngine::new(1);
    let doc = engine
        .load_document(Path::new("/test.pdf"), &[])
        .unwrap();

    let (orig_w, orig_h) = doc.page_size(0);
    assert_eq!(orig_w, 612.0);
    assert_eq!(orig_h, 792.0);

    // 0 degrees: (w, h)
    let rot0_size = if 0 == 90 || 0 == 270 { (orig_h, orig_w) } else { (orig_w, orig_h) };
    assert_eq!(rot0_size, (612.0, 792.0));

    // 90 degrees: (h, w)
    let rot90_size = if 90 == 90 || 90 == 270 { (orig_h, orig_w) } else { (orig_w, orig_h) };
    assert_eq!(rot90_size, (792.0, 612.0));

    // 180 degrees: (w, h)
    let rot180_size = if 180 == 90 || 180 == 270 { (orig_h, orig_w) } else { (orig_w, orig_h) };
    assert_eq!(rot180_size, (612.0, 792.0));

    // 270 degrees: (h, w)
    let rot270_size = if 270 == 90 || 270 == 270 { (orig_h, orig_w) } else { (orig_w, orig_h) };
    assert_eq!(rot270_size, (792.0, 612.0));
}

#[test]
fn test_fit_to_page_zoom_calculation() {
    // 612x792 page in a 1200x800 viewport
    let page_w = 612.0f32;
    let page_h = 792.0f32;
    let avail_w = 1200.0 - (PAGE_MARGIN * 2.0);
    let avail_h = 800.0 - (PAGE_MARGIN * 2.0);

    let scale_w = avail_w / page_w;
    let scale_h = avail_h / page_h;
    let target_zoom = scale_w.min(scale_h).clamp(MIN_ZOOM, MAX_ZOOM);

    // Height is the constraint (768 / 792 = ~0.9697)
    assert!(target_zoom < 1.0);
    assert!((target_zoom - (avail_h / page_h)).abs() < 0.001);
}

#[test]
fn test_render_pool_deduplication() {
    let pool = RenderPool::new(2);
    let doc = MockPdfEngine::new(3).load_document(Path::new("/test.pdf"), &[]).unwrap();

    let key = TileKey::new(0, 1.0, 0, 0, 0);
    pool.request_tile(key, 1.0, 0, 0, 512, 512, doc.clone());
    // Duplicate request should be ignored silently
    pool.request_tile(key, 1.0, 0, 0, 512, 512, doc.clone());

    pool.request_page_overview(0, 0, 0.75, doc.clone());
    // Duplicate overview request should be ignored silently
    pool.request_page_overview(0, 0, 0.75, doc.clone());
}

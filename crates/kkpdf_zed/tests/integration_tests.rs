use kkpdf_zed::{PageLayoutMode, PdfToolbarState, PdfView, PdfViewerSettings};
use std::time::Duration;

const MINIMAL_PDF: &[u8] = b"%PDF-1.7\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>\nendobj\nxref\n0 4\n0000000000 65535 f \n0000000010 00000 n \n0000000060 00000 n \n0000000117 00000 n \ntrailer\n<< /Size 4 /Root 1 0 R >>\nstartxref\n185\n%%EOF\n";

#[test]
fn test_end_to_end_pdf_view_lifecycle() {
    let settings = PdfViewerSettings {
        cache_budget_mb: 16,
        ..Default::default()
    };

    let mut view = PdfView::new(settings);
    let sample_pdf = MINIMAL_PDF;

    view.load_from_bytes(sample_pdf.to_vec(), None)
        .expect("Load sample PDF bytes");

    assert_eq!(view.total_pages(), 1);
    assert_eq!(view.display_page_number(), 1);

    // Initial container layout
    view.set_container_size(1024.0, 768.0);
    assert!(view.zoom_percentage() > 0);

    // Zoom manipulations
    view.set_zoom(1.5, Some((512.0, 384.0)));
    assert_eq!(view.zoom_percentage(), 150);

    view.zoom_in(None);
    assert!(view.zoom_percentage() > 150);

    view.reset_zoom();
    assert_eq!(view.zoom_percentage(), 100);

    // Rendering page
    let rendered = view.get_or_render_page(0).expect("Render page 0");
    assert!(rendered.width > 0);
    assert!(rendered.height > 0);
    assert!(rendered.dark_mode); // smart dark mode default

    // Dark mode toggle
    view.toggle_dark_mode();
    let rendered_light = view.get_or_render_page(0).expect("Render light mode");
    assert!(!rendered_light.dark_mode);
}

#[test]
fn test_reload_preserving_exact_viewport_and_zoom() {
    let temp_dir = tempfile::tempdir().expect("tempdir");
    let pdf_path = temp_dir.path().join("document.pdf");

    // Write initial PDF
    {
        std::fs::write(&pdf_path, MINIMAL_PDF).expect("write");
    }

    let mut view = PdfView::new(PdfViewerSettings::default());
    view.open_file(&pdf_path).expect("open file");

    // Simulate user zooming and panning
    view.set_zoom(2.5, None);
    view.handle_mouse_down((100.0, 100.0));
    view.handle_mouse_move((150.0, 200.0));
    view.handle_mouse_up();

    let initial_zoom = view.zoom_percentage();
    let snapshot = view.save_state_snapshot();
    assert_eq!(initial_zoom, 250);
    assert_eq!(snapshot.pan_x, 50.0);
    assert_eq!(snapshot.pan_y, 100.0);

    // Sleep to ensure timestamp difference
    std::thread::sleep(Duration::from_millis(25));

    // Compiler rebuilds the PDF
    {
        std::fs::write(&pdf_path, MINIMAL_PDF).expect("rebuild");
    }

    // Debounced hot reload trigger
    let reloaded = view.reload_preserving_state().expect("hot reload");
    assert!(reloaded, "Reload should succeed");

    // Verify viewport state was strictly preserved
    assert_eq!(view.zoom_percentage(), initial_zoom);
    let new_snapshot = view.save_state_snapshot();
    assert_eq!(new_snapshot.pan_x, 50.0);
    assert_eq!(new_snapshot.pan_y, 100.0);
}

#[test]
fn test_toolbar_synchronization_with_view_state() {
    let view = PdfView::new(PdfViewerSettings::default());
    let toolbar_state = PdfToolbarState::new(
        view.display_page_number(),
        view.total_pages(),
        view.zoom_percentage(),
        view.dark_mode(),
        view.layout_mode(),
    );

    assert_eq!(toolbar_state.current_page, 1);
    assert_eq!(toolbar_state.zoom_percentage, 100);
    assert!(toolbar_state.dark_mode);
    assert_eq!(toolbar_state.layout_mode, PageLayoutMode::Continuous);
}

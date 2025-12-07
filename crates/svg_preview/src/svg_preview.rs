use std::sync::Arc;

use gpui::{App, Context, Window, actions};
use language::LanguageRegistry;
use workspace::{ItemHandle, PreviewFactory, Workspace, register_preview_factory};

pub mod svg_preview_view;

actions!(
    svg,
    [
        /// Opens an SVG preview for the current file.
        OpenPreview,
        /// Opens an SVG preview in a split pane.
        OpenPreviewToTheSide,
        /// Opens a following SVG preview that syncs with the editor.
        OpenFollowingPreview
    ]
);

pub fn init(cx: &mut App) {
    // Register the preview factory
    register_preview_factory(Arc::new(SvgPreviewFactory), cx);

    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        crate::svg_preview_view::SvgPreviewView::register(workspace, window, cx);
    })
    .detach();
}

struct SvgPreviewFactory;

impl PreviewFactory for SvgPreviewFactory {
    fn can_preview_extension(&self, extension: &str) -> bool {
        extension.eq_ignore_ascii_case("svg")
    }

    fn can_preview(&self, item: &dyn ItemHandle, cx: &App) -> bool {
        // Check if the item has an SVG file extension
        let mut can_preview = false;
        item.for_each_project_item(cx, &mut |_, project_item| {
            if let Some(path) = project_item.project_path(cx) {
                if let Some(extension) = path.path.extension() {
                    can_preview = extension.eq_ignore_ascii_case("svg");
                }
            }
        });
        can_preview
    }

    fn create_preview(
        &self,
        item: Box<dyn ItemHandle>,
        _language_registry: Arc<language::LanguageRegistry>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Box<dyn ItemHandle> {
        use svg_preview_view::{SvgPreviewMode, SvgPreviewView};

        // Get the MultiBuffer from the item using act_as
        // This works because Editor implements act_as for MultiBuffer
        let buffer = item
            .act_as::<multi_buffer::MultiBuffer>(cx)
            .expect("Item should be able to provide a MultiBuffer");

        let workspace = cx.entity().downgrade();

        let preview = SvgPreviewView::new(SvgPreviewMode::Default, buffer, workspace, window, cx);

        Box::new(preview)
    }
}

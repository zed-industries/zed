use std::sync::Arc;

use gpui::{App, Context, Window, actions};
use workspace::{
    ItemHandle, PreviewFactory, PreviewSourceExtractor, Workspace, register_preview_factory,
    register_preview_source_extractor,
};

pub mod markdown_elements;
mod markdown_minifier;
pub mod markdown_parser;
pub mod markdown_preview_view;
pub mod markdown_renderer;

actions!(
    markdown,
    [
        /// Scrolls up by one page in the markdown preview.
        MovePageUp,
        /// Scrolls down by one page in the markdown preview.
        MovePageDown,
        /// Opens a markdown preview for the current file.
        OpenPreview,
        /// Opens a markdown preview in a split pane.
        OpenPreviewToTheSide,
        /// Opens a following markdown preview that syncs with the editor.
        OpenFollowingPreview
    ]
);

pub fn init(cx: &mut App) {
    // Register the preview factory
    register_preview_factory(Arc::new(MarkdownPreviewFactory), cx);

    // Register the source extractor for OpenEditor action
    register_preview_source_extractor(Arc::new(MarkdownSourceExtractor), cx);

    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        markdown_preview_view::MarkdownPreviewView::register(workspace, window, cx);
    })
    .detach();
}

struct MarkdownPreviewFactory;

impl PreviewFactory for MarkdownPreviewFactory {
    fn can_preview_extension(&self, extension: &str) -> bool {
        matches!(extension.to_lowercase().as_str(), "md" | "markdown")
    }

    fn can_preview(&self, item: &dyn ItemHandle, cx: &App) -> bool {
        // Check if the item has a markdown file extension
        let mut can_preview = false;
        item.for_each_project_item(cx, &mut |_, project_item| {
            if let Some(path) = project_item.project_path(cx) {
                if let Some(extension) = path.path.extension() {
                    can_preview = matches!(extension.to_lowercase().as_str(), "md" | "markdown");
                }
            }
        });
        can_preview
    }

    fn create_preview(
        &self,
        item: Box<dyn ItemHandle>,
        language_registry: Arc<language::LanguageRegistry>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Box<dyn ItemHandle> {
        use markdown_preview_view::{MarkdownPreviewMode, MarkdownPreviewView};

        // Downcast to Editor to get the entity
        let editor = item.to_any_view().downcast::<editor::Editor>().unwrap();

        let workspace = cx.entity().downgrade();

        let preview = MarkdownPreviewView::new(
            MarkdownPreviewMode::Default,
            editor,
            workspace,
            language_registry,
            window,
            cx,
        );

        Box::new(preview)
    }
}

struct MarkdownSourceExtractor;

impl PreviewSourceExtractor for MarkdownSourceExtractor {
    fn extract_source(
        &self,
        item: &dyn ItemHandle,
        _window: &mut gpui::Window,
        cx: &mut gpui::App,
    ) -> Option<Box<dyn ItemHandle>> {
        // Try to downcast to MarkdownPreviewView
        let preview = item
            .to_any_view()
            .downcast::<markdown_preview_view::MarkdownPreviewView>()
            .ok()?;

        // Get the active editor from the preview
        let editor = preview.read(cx).active_editor()?;

        Some(Box::new(editor))
    }
}

use gpui::{App, actions};
use workspace::Workspace;

pub mod markdown_preview_settings;
pub mod markdown_preview_view;

pub use zed_actions::preview::markdown::{OpenPreview, OpenPreviewToTheSide};

use crate::markdown_preview_view::MarkdownPreviewView;

actions!(
    markdown,
    [
        /// Scrolls up by one page in the markdown preview.
        #[action(deprecated_aliases = ["markdown::MovePageUp"])]
        ScrollPageUp,
        /// Scrolls down by one page in the markdown preview.
        #[action(deprecated_aliases = ["markdown::MovePageDown"])]
        ScrollPageDown,
        /// Scrolls up by approximately one visual line.
        ScrollUp,
        /// Scrolls down by approximately one visual line.
        ScrollDown,
        /// Scrolls up by one markdown element in the markdown preview
        ScrollUpByItem,
        /// Scrolls down by one markdown element in the markdown preview
        ScrollDownByItem,
        /// Scrolls to the top of the markdown preview.
        ScrollToTop,
        /// Scrolls to the bottom of the markdown preview.
        ScrollToBottom,
        /// Opens a following markdown preview that syncs with the editor.
        OpenFollowingPreview,
        /// Closes the markdown preview and returns focus to the source editor.
        CloseAndReturnToEditor
    ]
);

pub fn init(cx: &mut App) {
    workspace::register_serializable_item::<MarkdownPreviewView>(cx);

    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        markdown_preview_view::MarkdownPreviewView::register(workspace, window, cx);
    })
    .detach();
}

/// Headless markdown specification for autonomous agent documentation generation
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkdownArtifactSpec {
    pub title: String,
    pub content: String,
    pub section_count: usize,
}

/// Parameters for markdown rendering
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HeadlessMarkdownRenderParams {
    /// Markdown content to render
    pub markdown: String,
}

/// Headless renderer for converting markdown syntax to plain-text / AST structures for external agents
#[derive(Clone, Debug, Default)]
pub struct HeadlessMarkdownRenderer;

impl HeadlessMarkdownRenderer {
    pub fn parse_sections(raw_markdown: &str) -> Vec<MarkdownArtifactSpec> {
        let mut sections = Vec::new();
        let mut current_title = String::from("Overview");
        let mut current_content = String::new();
        let mut count = 0;

        for line in raw_markdown.lines() {
            if line.starts_with('#') {
                if !current_content.is_empty() {
                    sections.push(MarkdownArtifactSpec {
                        title: current_title.clone(),
                        content: current_content.trim().to_string(),
                        section_count: count,
                    });
                    current_content.clear();
                    count += 1;
                }
                current_title = line.trim_start_matches('#').trim().to_string();
            } else {
                current_content.push_str(line);
                current_content.push('\n');
            }
        }

        if !current_content.is_empty() {
            sections.push(MarkdownArtifactSpec {
                title: current_title,
                content: current_content.trim().to_string(),
                section_count: count,
            });
        }

        sections
    }
}

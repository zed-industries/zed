use gpui::{App, actions};
use settings::{MarkdownPreviewLinkClickBehavior, RegisterSetting, Settings};
use workspace::Workspace;

#[derive(RegisterSetting)]
pub struct MarkdownPreviewSettings {
    pub link_click_behavior: MarkdownPreviewLinkClickBehavior,
}

impl Settings for MarkdownPreviewSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let markdown_preview = content.markdown_preview.as_ref();
        Self {
            link_click_behavior: markdown_preview
                .and_then(|mp| mp.link_click_behavior)
                .unwrap_or_default(),
        }
    }
}

pub mod markdown_elements;
mod markdown_minifier;
pub mod markdown_parser;
pub mod markdown_preview_view;
pub mod markdown_renderer;

pub use zed_actions::preview::markdown::{OpenPreview, OpenPreviewToTheSide};

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
        /// Opens a following markdown preview that syncs with the editor.
        OpenFollowingPreview
    ]
);

pub fn init(cx: &mut App) {
    MarkdownPreviewSettings::register(cx);
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        markdown_preview_view::MarkdownPreviewView::register(workspace, window, cx);
    })
    .detach();
}

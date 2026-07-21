use csv_preview::{CsvPreviewView, TabularDataPreviewFeatureFlag};
use editor::{Editor, MultiBuffer};
use feature_flags::FeatureFlagAppExt as _;
use gpui::{AnyElement, Entity, Modifiers};
use markdown_preview::markdown_preview_view::MarkdownPreviewView;
use svg_preview::svg_preview_view::SvgPreviewView;
use ui::{Tooltip, prelude::*, text_for_keystroke};

use super::QuickActionBar;

enum PreviewTarget {
    Markdown(Entity<Editor>),
    Svg(Entity<MultiBuffer>),
    Csv(Entity<Editor>),
}

impl QuickActionBar {
    pub fn render_preview_button(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        // Resolve against this toolbar's own pane item rather than the
        // workspace's focused item, so each pane's button reflects and
        // targets the content of the pane it belongs to.
        let active_item = self.active_item.as_ref()?;
        let editor = active_item.act_as::<Editor>(cx);

        let preview_target = if let Some(editor) = &editor
            && MarkdownPreviewView::is_markdown_file(editor, cx)
        {
            PreviewTarget::Markdown(editor.clone())
        } else if let Some(buffer) = active_item.act_as::<MultiBuffer>(cx)
            && SvgPreviewView::is_svg_file(&buffer, cx)
        {
            PreviewTarget::Svg(buffer)
        } else if let Some(editor) = editor
            && cx.has_flag::<TabularDataPreviewFeatureFlag>()
            && CsvPreviewView::is_csv_file(&editor, cx)
        {
            PreviewTarget::Csv(editor)
        } else {
            return None;
        };

        let (button_id, tooltip_text, open_action_for_tooltip) = match &preview_target {
            PreviewTarget::Markdown(_) => (
                "toggle-markdown-preview",
                "Preview Markdown",
                &markdown_preview::OpenPreview as &dyn gpui::Action,
            ),
            PreviewTarget::Svg(_) => (
                "toggle-svg-preview",
                "Preview SVG",
                &svg_preview::OpenPreview as &dyn gpui::Action,
            ),
            PreviewTarget::Csv(_) => (
                "toggle-csv-preview",
                "Preview CSV",
                &csv_preview::OpenPreview as &dyn gpui::Action,
            ),
        };

        let alt_click = gpui::Keystroke {
            key: "click".into(),
            modifiers: Modifiers::alt(),
            ..Default::default()
        };

        let button = IconButton::new(button_id, IconName::Eye)
            .icon_size(IconSize::Small)
            .style(ButtonStyle::Subtle)
            .tooltip(move |_window, cx| {
                Tooltip::with_meta(
                    tooltip_text,
                    Some(open_action_for_tooltip),
                    format!(
                        "{} to open in a split",
                        text_for_keystroke(&alt_click.modifiers, &alt_click.key, cx)
                    ),
                    cx,
                )
            })
            .on_click({
                let workspace_handle = self.workspace.clone();
                let active_item = active_item.boxed_clone();
                move |_, window, cx| {
                    let Some(workspace) = workspace_handle.upgrade() else {
                        return;
                    };
                    workspace.update(cx, |workspace, cx| {
                        let Some(pane) = workspace.pane_for(active_item.as_ref()) else {
                            return;
                        };
                        let open_to_the_side = window.modifiers().alt;
                        match &preview_target {
                            PreviewTarget::Markdown(editor) => {
                                let editor = editor.clone();
                                if open_to_the_side {
                                    MarkdownPreviewView::open_preview_to_the_side_of_pane(
                                        workspace, editor, pane, window, cx,
                                    );
                                } else {
                                    MarkdownPreviewView::open_preview_in_pane(
                                        workspace, editor, pane, window, cx,
                                    );
                                }
                            }
                            PreviewTarget::Svg(buffer) => {
                                let buffer = buffer.clone();
                                if open_to_the_side {
                                    SvgPreviewView::open_preview_to_the_side_of_pane(
                                        workspace, buffer, pane, window, cx,
                                    );
                                } else {
                                    SvgPreviewView::open_preview_in_pane(
                                        workspace, buffer, pane, window, cx,
                                    );
                                }
                            }
                            PreviewTarget::Csv(editor) => {
                                let editor = editor.clone();
                                if open_to_the_side {
                                    CsvPreviewView::open_preview_to_the_side_of_pane(
                                        workspace, editor, pane, window, cx,
                                    );
                                } else {
                                    CsvPreviewView::open_preview_in_pane(editor, pane, window, cx);
                                }
                            }
                        }
                    });
                }
            });

        Some(button.into_any_element())
    }
}

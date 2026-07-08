use csv_preview::{CsvPreviewView, TabularDataPreviewFeatureFlag};
use editor::{Editor, MultiBuffer};
use feature_flags::FeatureFlagAppExt as _;
use gpui::{Action as _, AnyElement, Entity, Modifiers};
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
    pub fn render_open_source_button(&self, _cx: &mut Context<Self>) -> Option<AnyElement> {
        let item = self.active_item.as_ref()?;
        let (button_id, tooltip_text) = if item.downcast::<MarkdownPreviewView>().is_some() {
            ("edit-markdown-source", "Edit Markdown")
        } else if item.downcast::<SvgPreviewView>().is_some() {
            ("edit-svg-source", "Edit SVG")
        } else if item.downcast::<CsvPreviewView>().is_some() {
            ("edit-csv-source", "Edit CSV")
        } else {
            return None;
        };

        let button = IconButton::new(button_id, IconName::Pencil)
            .icon_size(IconSize::Small)
            .style(ButtonStyle::Subtle)
            .tooltip(move |_window, cx| {
                Tooltip::for_action(tooltip_text, &zed_actions::preview::Toggle::default(), cx)
            })
            .on_click(move |_, window, cx| {
                window.dispatch_action(zed_actions::preview::Toggle::default().boxed_clone(), cx);
            });

        Some(button.into_any_element())
    }

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

        let (button_id, tooltip_text) = match &preview_target {
            PreviewTarget::Markdown(_) => ("toggle-markdown-preview", "Preview Markdown"),
            PreviewTarget::Svg(_) => ("toggle-svg-preview", "Preview SVG"),
            PreviewTarget::Csv(_) => ("toggle-csv-preview", "Preview CSV"),
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
                    Some(&zed_actions::preview::Toggle::default()),
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
                    if !window.modifiers().alt {
                        window.dispatch_action(
                            zed_actions::preview::Toggle::default().boxed_clone(),
                            cx,
                        );
                        return;
                    }
                    let Some(workspace) = workspace_handle.upgrade() else {
                        return;
                    };
                    workspace.update(cx, |workspace, cx| {
                        let Some(pane) = workspace.pane_for(active_item.as_ref()) else {
                            return;
                        };
                        match &preview_target {
                            PreviewTarget::Markdown(editor) => {
                                MarkdownPreviewView::open_preview_to_the_side_of_pane(
                                    workspace,
                                    editor.clone(),
                                    pane,
                                    window,
                                    cx,
                                );
                            }
                            PreviewTarget::Svg(buffer) => {
                                SvgPreviewView::open_preview_to_the_side_of_pane(
                                    workspace,
                                    buffer.clone(),
                                    pane,
                                    window,
                                    cx,
                                );
                            }
                            PreviewTarget::Csv(editor) => {
                                CsvPreviewView::open_preview_to_the_side_of_pane(
                                    workspace,
                                    editor.clone(),
                                    pane,
                                    window,
                                    cx,
                                );
                            }
                        }
                    });
                }
            });

        Some(button.into_any_element())
    }
}

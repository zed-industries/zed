use csv_preview::{
    CsvPreviewView, OpenPreviewToTheSide as CsvOpenPreviewToTheSide, TabularDataPreviewFeatureFlag,
};
use feature_flags::FeatureFlagAppExt as _;
use gpui::{Action as _, AnyElement, Modifiers, WeakEntity};
use markdown_preview::{
    OpenPreviewToTheSide as MarkdownOpenPreviewToTheSide,
    markdown_preview_view::MarkdownPreviewView,
};
use svg_preview::{
    OpenPreviewToTheSide as SvgOpenPreviewToTheSide, svg_preview_view::SvgPreviewView,
};
use ui::{Tooltip, prelude::*, text_for_keystroke};
use workspace::Workspace;

use super::QuickActionBar;

#[derive(Clone, Copy)]
enum PreviewType {
    Markdown,
    Svg,
    Csv,
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

    pub fn render_preview_button(
        &self,
        workspace_handle: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let mut preview_type = None;

        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                if MarkdownPreviewView::resolve_active_item_as_markdown_editor(workspace, cx)
                    .is_some()
                {
                    preview_type = Some(PreviewType::Markdown);
                } else if SvgPreviewView::resolve_active_item_as_svg_buffer(workspace, cx).is_some()
                {
                    preview_type = Some(PreviewType::Svg);
                } else if cx.has_flag::<TabularDataPreviewFeatureFlag>()
                    && CsvPreviewView::resolve_active_item_as_csv_editor(workspace, cx).is_some()
                {
                    preview_type = Some(PreviewType::Csv);
                }
            });
        }

        let preview_type = preview_type?;

        let (button_id, tooltip_text, open_to_side_action) = match preview_type {
            PreviewType::Markdown => (
                "toggle-markdown-preview",
                "Preview Markdown",
                Box::new(MarkdownOpenPreviewToTheSide) as Box<dyn gpui::Action>,
            ),
            PreviewType::Svg => (
                "toggle-svg-preview",
                "Preview SVG",
                Box::new(SvgOpenPreviewToTheSide) as Box<dyn gpui::Action>,
            ),
            PreviewType::Csv => (
                "toggle-csv-preview",
                "Preview CSV",
                Box::new(CsvOpenPreviewToTheSide) as Box<dyn gpui::Action>,
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
                    Some(&zed_actions::preview::Toggle::default()),
                    format!(
                        "{} to open in a split",
                        text_for_keystroke(&alt_click.modifiers, &alt_click.key, cx)
                    ),
                    cx,
                )
            })
            .on_click(move |_, window, cx| {
                if let Some(workspace) = workspace_handle.upgrade() {
                    workspace.update(cx, |_, cx| {
                        if window.modifiers().alt {
                            window.dispatch_action(open_to_side_action.boxed_clone(), cx);
                        } else {
                            window.dispatch_action(
                                zed_actions::preview::Toggle::default().boxed_clone(),
                                cx,
                            );
                        }
                    });
                }
            });

        Some(button.into_any_element())
    }
}

use editor::{Editor, MultiBuffer};
use gpui::{Action, App, Entity, Modifiers, WeakEntity, Window};
use markdown_preview::markdown_preview_view::MarkdownPreviewView;
use svg_preview::svg_preview_view::SvgPreviewView;
use tabular_data_preview::TabularDataPreviewPane;
use ui::{IconName, text_for_keystroke};
use workspace::{Workspace, item::ItemHandle};

use super::{
    QuickActionBarItem, QuickActionButton, QuickActionElement, QuickActionTarget, VisibilityTrigger,
};

#[derive(Clone, PartialEq)]
enum PreviewTarget {
    Markdown(Entity<Editor>),
    Svg(Entity<MultiBuffer>),
    TabularData(Entity<Editor>),
}

pub(super) struct PreviewContext {
    target: PreviewTarget,
    item: Box<dyn ItemHandle>,
    workspace: WeakEntity<Workspace>,
}

impl PartialEq for PreviewContext {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target
            && self.item.item_id() == other.item.item_id()
            && self.workspace == other.workspace
    }
}

pub(super) struct PreviewButton;

fn preview_target(target: &QuickActionTarget, cx: &App) -> Option<PreviewTarget> {
    // Resolve against this toolbar's own pane item rather than the
    // workspace's focused item, so each pane's button reflects and
    // targets the content of the pane it belongs to.
    let editor = target.editor();

    if let Some(editor) = editor
        && MarkdownPreviewView::is_markdown_file(editor, cx)
    {
        Some(PreviewTarget::Markdown(editor.clone()))
    } else if let Some(buffer) = target.item().act_as::<MultiBuffer>(cx)
        && SvgPreviewView::is_svg_file(&buffer, cx)
    {
        Some(PreviewTarget::Svg(buffer))
    } else if let Some(editor) = editor
        && TabularDataPreviewPane::is_tabular_data_file(editor, cx)
    {
        Some(PreviewTarget::TabularData(editor.clone()))
    } else {
        None
    }
}

impl QuickActionBarItem for PreviewButton {
    type Context = PreviewContext;

    const ID: &'static str = "toggle-preview";

    // The buffer's language may only be detected after the item becomes active.
    const TRIGGERS: &'static [VisibilityTrigger] = &[VisibilityTrigger::Editor];

    fn context(&self, target: &QuickActionTarget, cx: &mut App) -> Option<Self::Context> {
        let preview_target = preview_target(target, cx)?;
        Some(PreviewContext {
            target: preview_target,
            item: target.item().boxed_clone(),
            workspace: target.workspace().clone(),
        })
    }

    fn render(
        &self,
        context: &Self::Context,
        _window: &mut Window,
        cx: &mut App,
    ) -> QuickActionElement {
        let (tooltip_text, open_action): (&'static str, Box<dyn Action>) = match context.target {
            PreviewTarget::Markdown(_) => {
                ("Preview Markdown", Box::new(markdown_preview::OpenPreview))
            }
            PreviewTarget::Svg(_) => ("Preview SVG", Box::new(svg_preview::OpenPreview)),
            PreviewTarget::TabularData(_) => (
                "Preview Tabular Data",
                Box::new(tabular_data_preview::OpenPreview),
            ),
        };

        let alt_click = gpui::Keystroke {
            key: "click".into(),
            modifiers: Modifiers::alt(),
            ..Default::default()
        };

        let preview_target = context.target.clone();
        let item = context.item.boxed_clone();
        let workspace = context.workspace.clone();

        QuickActionElement::Button(
            QuickActionButton::new(IconName::Eye, tooltip_text, move |window, cx| {
                let Some(workspace) = workspace.upgrade() else {
                    return;
                };
                workspace.update(cx, |workspace, cx| {
                    let Some(pane) = workspace.pane_for(item.as_ref()) else {
                        return;
                    };
                    let open_to_the_side = window.modifiers().alt;
                    match preview_target.clone() {
                        PreviewTarget::Markdown(editor) => {
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
                        PreviewTarget::TabularData(editor) => {
                            if open_to_the_side {
                                TabularDataPreviewPane::open_preview_to_the_side_of_pane(
                                    workspace, editor, pane, window, cx,
                                );
                            } else {
                                TabularDataPreviewPane::open_preview_in_pane(
                                    editor, pane, window, cx,
                                );
                            }
                        }
                    }
                });
            })
            .action(open_action)
            .tooltip_meta(format!(
                "{} to open in a split",
                text_for_keystroke(&alt_click.modifiers, &alt_click.key, cx)
            )),
        )
    }
}

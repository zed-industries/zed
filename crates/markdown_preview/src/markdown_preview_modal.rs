use client::{Client, ZED_SERVER_URL};
use editor::{display_map::ToDisplayPoint, scroll::Autoscroll, Editor};
use futures::AsyncReadExt;
use gpui::{
    div, px, rems, AnyElement, AppContext, CursorStyle, DismissEvent, EventEmitter, FocusHandle,
    FocusableView, Hsla, InteractiveElement, IntoElement, Model, MouseButton, ParentElement,
    Pixels, PromptLevel, Render, SharedString, Size, StatefulInteractiveElement, Styled, Task,
    View, ViewContext, WeakView,
};
use isahc::Request;
use language::{Buffer, LanguageRegistry};
use project::{Project, ProjectPath};
use std::{ops::RangeInclusive, path::PathBuf, sync::Arc, time::Duration};
use ui::{prelude::*, Button, ButtonStyle};
use workspace::{ModalView, Workspace};

use crate::OpenPreview;

pub struct MarkdownPreviewModal {
    focus_handle: FocusHandle,
    languages: Arc<LanguageRegistry>,
    active_editor: View<Editor>,
}

impl FocusableView for MarkdownPreviewModal {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
impl EventEmitter<DismissEvent> for MarkdownPreviewModal {}

impl ModalView for MarkdownPreviewModal {
    fn on_before_dismiss(&mut self, cx: &mut ViewContext<Self>) -> bool {
        true
    }
}

impl MarkdownPreviewModal {
    pub fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        let languages = workspace.app_state().languages.clone();

        let _handle = cx.view().downgrade();
        workspace.register_action(move |workspace, _: &OpenPreview, cx| {
            let project = workspace.project().clone();

            let languages = languages.clone();
            let editor = workspace.active_item_as::<Editor>(cx).unwrap();

            cx.spawn(|workspace, mut cx| async move {
                workspace.update(&mut cx, |workspace, cx| {
                    workspace.toggle_modal(cx, move |cx| {
                        MarkdownPreviewModal::new(editor, languages, cx)
                    });
                })?;

                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        });
    }

    pub fn new(
        active_editor: View<Editor>,
        languages: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        Self {
            languages,
            active_editor,
            focus_handle,
        }
    }

    fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent)
    }
}

impl Render for MarkdownPreviewModal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut container = div()
            .elevation_3(cx)
            .key_context("MarkdownPreview")
            .min_w(rems(40.))
            .max_w(rems(96.))
            .p_4()
            .gap_4();

        let close_button = Button::new("done_markdown_preview", "Done")
            .style(ButtonStyle::Filled)
            .color(Color::Default)
            // .width(rem(12.))
            // .py_2()
            .on_click(cx.listener(move |_, _, cx| {
                cx.spawn(|this, mut cx| async move {
                    this.update(&mut cx, |_, cx| cx.emit(DismissEvent)).ok();
                })
                .detach();
            }));

        let editor = self.active_editor.read(cx);
        let contents = editor.buffer().read(cx).snapshot(cx).text();

        let mentions = vec![];
        let text = rich_text::render_markdown(contents, &mentions, &self.languages, None);
        // let text: &dyn InteractiveElement = text.element("body".into(), cx).into_element();

        let md_container = div()
            // TODO: Why do I need `.id` in order to use overflow?
            .id("markdown_preview_container")
            .gap_2()
            .overflow_y_scroll()
            .child(text.element("body".into(), cx));

        // TODO: Allow scroll
        // TODO: Pin the button to the bottom

        container.child(md_container).child(close_button)
    }
}

// TODO: Testing

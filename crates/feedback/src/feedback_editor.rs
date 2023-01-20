use std::{ops::Range, sync::Arc};

use anyhow::bail;
use client::{Client, ZED_SECRET_CLIENT_TOKEN};
use editor::Editor;
use futures::AsyncReadExt;
use gpui::{
    actions,
    elements::{ChildView, Flex, Label, MouseEventHandler, ParentElement, Stack, Text},
    serde_json, AnyViewHandle, CursorStyle, Element, ElementBox, Entity, ModelHandle, MouseButton,
    MutableAppContext, PromptLevel, RenderContext, Task, View, ViewContext, ViewHandle,
    WeakViewHandle,
};
use isahc::Request;
use language::{Language, LanguageConfig};
use postage::prelude::Stream;

use lazy_static::lazy_static;
use project::{Project, ProjectEntryId, ProjectPath};
use serde::Serialize;
use settings::Settings;
use smallvec::SmallVec;
use workspace::{
    item::{Item, ItemHandle},
    StatusItemView, Workspace,
};

use crate::system_specs::SystemSpecs;

lazy_static! {
    pub static ref ZED_SERVER_URL: String =
        std::env::var("ZED_SERVER_URL").unwrap_or_else(|_| "https://zed.dev".to_string());
}

// TODO FEEDBACK: In the future, it would be nice to use this is some sort of live-rendering character counter thing
// Currently, we are just checking on submit that the the text exceeds the `start` value in this range
const FEEDBACK_CHAR_COUNT_RANGE: Range<usize> = Range {
    start: 5,
    end: 1000,
};

actions!(feedback, [SubmitFeedback, GiveFeedback, DeployFeedback]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(FeedbackEditor::deploy);
}

pub struct FeedbackButton;

impl Entity for FeedbackButton {
    type Event = ();
}

impl View for FeedbackButton {
    fn ui_name() -> &'static str {
        "FeedbackButton"
    }

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        Stack::new()
            .with_child(
                MouseEventHandler::<Self>::new(0, cx, |state, cx| {
                    let theme = &cx.global::<Settings>().theme;
                    let theme = &theme.workspace.status_bar.feedback;

                    Text::new(
                        "Give Feedback".to_string(),
                        theme.style_for(state, true).clone(),
                    )
                    .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, |_, cx| cx.dispatch_action(GiveFeedback))
                .boxed(),
            )
            .boxed()
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, _: &mut ViewContext<Self>) {}

    fn focus_out(&mut self, _: gpui::AnyViewHandle, _: &mut ViewContext<Self>) {}

    fn key_down(&mut self, _: &gpui::KeyDownEvent, _: &mut ViewContext<Self>) -> bool {
        false
    }

    fn key_up(&mut self, _: &gpui::KeyUpEvent, _: &mut ViewContext<Self>) -> bool {
        false
    }

    fn modifiers_changed(
        &mut self,
        _: &gpui::ModifiersChangedEvent,
        _: &mut ViewContext<Self>,
    ) -> bool {
        false
    }

    fn keymap_context(&self, _: &gpui::AppContext) -> gpui::keymap_matcher::KeymapContext {
        Self::default_keymap_context()
    }

    fn default_keymap_context() -> gpui::keymap_matcher::KeymapContext {
        let mut cx = gpui::keymap_matcher::KeymapContext::default();
        cx.set.insert(Self::ui_name().into());
        cx
    }

    fn debug_json(&self, _: &gpui::AppContext) -> gpui::serde_json::Value {
        gpui::serde_json::Value::Null
    }

    fn text_for_range(&self, _: Range<usize>, _: &gpui::AppContext) -> Option<String> {
        None
    }

    fn selected_text_range(&self, _: &gpui::AppContext) -> Option<Range<usize>> {
        None
    }

    fn marked_text_range(&self, _: &gpui::AppContext) -> Option<Range<usize>> {
        None
    }

    fn unmark_text(&mut self, _: &mut ViewContext<Self>) {}

    fn replace_text_in_range(
        &mut self,
        _: Option<Range<usize>>,
        _: &str,
        _: &mut ViewContext<Self>,
    ) {
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        _: Option<Range<usize>>,
        _: &str,
        _: Option<Range<usize>>,
        _: &mut ViewContext<Self>,
    ) {
    }
}

impl StatusItemView for FeedbackButton {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn ItemHandle>,
        _: &mut gpui::ViewContext<Self>,
    ) {
    }
}

#[derive(Serialize)]
struct FeedbackRequestBody<'a> {
    feedback_text: &'a str,
    metrics_id: Option<Arc<str>>,
    system_specs: SystemSpecs,
    token: &'a str,
}

#[derive(Clone)]
struct FeedbackEditor {
    editor: ViewHandle<Editor>,
}

impl FeedbackEditor {
    fn new(
        project_handle: ModelHandle<Project>,
        _: WeakViewHandle<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        // TODO FEEDBACK: This doesn't work like I expected it would
        // let markdown_language = Arc::new(Language::new(
        //     LanguageConfig::default(),
        //     Some(tree_sitter_markdown::language()),
        // ));

        let markdown_language = project_handle
            .read(cx)
            .languages()
            .get_language("Markdown")
            .unwrap();

        let buffer = project_handle
            .update(cx, |project, cx| {
                project.create_buffer("", Some(markdown_language), cx)
            })
            .expect("creating buffers on a local workspace always succeeds");

        const FEDBACK_PLACEHOLDER_TEXT: &str = "Thanks for spending time with Zed. Enter your feedback here in the form of Markdown. Save the tab to submit your feedback.";

        let editor = cx.add_view(|cx| {
            let mut editor = Editor::for_buffer(buffer, Some(project_handle.clone()), cx);
            editor.set_vertical_scroll_margin(5, cx);
            editor.set_placeholder_text(FEDBACK_PLACEHOLDER_TEXT, cx);
            editor
        });

        let this = Self { editor };
        this
    }

    fn handle_save(
        &mut self,
        _: gpui::ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        // TODO FEEDBACK: These don't look right
        let feedback_text_length = self.editor.read(cx).buffer().read(cx).len(cx);

        if feedback_text_length <= FEEDBACK_CHAR_COUNT_RANGE.start {
            cx.prompt(
                PromptLevel::Critical,
                &format!(
                    "Feedback must be longer than {} characters",
                    FEEDBACK_CHAR_COUNT_RANGE.start
                ),
                &["OK"],
            );

            return Task::ready(Ok(()));
        }

        let mut answer = cx.prompt(
            PromptLevel::Warning,
            "Ready to submit your feedback?",
            &["Yes, Submit!", "No"],
        );

        let this = cx.handle();
        cx.spawn(|_, mut cx| async move {
            let answer = answer.recv().await;

            if answer == Some(0) {
                cx.update(|cx| {
                    this.update(cx, |this, cx| match this.submit_feedback(cx) {
                        // TODO FEEDBACK
                        Ok(_) => {
                            // Close file after feedback sent successfully
                            // workspace
                            //     .update(cx, |workspace, cx| {
                            //         Pane::close_active_item(workspace, &Default::default(), cx)
                            //             .unwrap()
                            //     })
                            //     .await
                            //     .unwrap();
                        }
                        Err(error) => {
                            cx.prompt(PromptLevel::Critical, &error.to_string(), &["OK"]);
                            // Prompt that something failed (and to check the log for the exact error? and to try again?)
                        }
                    })
                })
            }
        })
        .detach();

        Task::ready(Ok(()))
    }

    fn submit_feedback(&mut self, cx: &mut ViewContext<'_, Self>) -> anyhow::Result<()> {
        let feedback_text = self.editor.read(cx).text(cx);
        let zed_client = cx.global::<Arc<Client>>();
        let system_specs = SystemSpecs::new(cx);
        let feedback_endpoint = format!("{}/api/feedback", *ZED_SERVER_URL);

        let metrics_id = zed_client.metrics_id();
        let http_client = zed_client.http_client();

        // TODO FEEDBACK: how to get error out of the thread

        let this = cx.handle();

        cx.spawn(|_, async_cx| {
            async move {
                let request = FeedbackRequestBody {
                    feedback_text: &feedback_text,
                    metrics_id,
                    system_specs,
                    token: ZED_SECRET_CLIENT_TOKEN,
                };

                let json_bytes = serde_json::to_vec(&request)?;

                let request = Request::post(feedback_endpoint)
                    .header("content-type", "application/json")
                    .body(json_bytes.into())?;

                let mut response = http_client.send(request).await?;
                let mut body = String::new();
                response.body_mut().read_to_string(&mut body).await?;

                let response_status = response.status();

                if !response_status.is_success() {
                    bail!("Feedback API failed with: {}", response_status)
                }

                this.read_with(&async_cx, |this, cx| -> anyhow::Result<()> {
                    bail!("Error")
                })?;

                // TODO FEEDBACK: Use or remove
                // Will need to handle error cases
                // async_cx.update(|cx| {
                //     this.update(cx, |this, cx| {
                //         this.handle_error(error);
                //         cx.notify();
                //         cx.dispatch_action(ShowErrorPopover);
                //         this.error_text = "Embedding failed"
                //     })
                // });

                Ok(())
            }
        })
        .detach();

        Ok(())
    }
}

impl FeedbackEditor {
    pub fn deploy(workspace: &mut Workspace, _: &GiveFeedback, cx: &mut ViewContext<Workspace>) {
        // if let Some(existing) = workspace.item_of_type::<FeedbackEditor>(cx) {
        //     workspace.activate_item(&existing, cx);
        // } else {
        let workspace_handle = cx.weak_handle();
        let feedback_editor = cx
            .add_view(|cx| FeedbackEditor::new(workspace.project().clone(), workspace_handle, cx));
        workspace.add_item(Box::new(feedback_editor), cx);
        // }
    }
}

impl View for FeedbackEditor {
    fn ui_name() -> &'static str {
        "FeedbackEditor"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        ChildView::new(&self.editor, cx).boxed()
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(&self.editor);
        }
    }
}

impl Entity for FeedbackEditor {
    type Event = ();
}

impl Item for FeedbackEditor {
    fn tab_content(
        &self,
        _: Option<usize>,
        style: &theme::Tab,
        _: &gpui::AppContext,
    ) -> ElementBox {
        Flex::row()
            .with_child(
                Label::new("Feedback".to_string(), style.label.clone())
                    .aligned()
                    .contained()
                    .boxed(),
            )
            .boxed()
    }

    fn to_item_events(_: &Self::Event) -> Vec<workspace::item::ItemEvent> {
        Vec::new()
    }

    fn project_path(&self, _: &gpui::AppContext) -> Option<ProjectPath> {
        None
    }

    fn project_entry_ids(&self, _: &gpui::AppContext) -> SmallVec<[ProjectEntryId; 3]> {
        SmallVec::new()
    }

    fn is_singleton(&self, _: &gpui::AppContext) -> bool {
        true
    }

    fn set_nav_history(&mut self, _: workspace::ItemNavHistory, _: &mut ViewContext<Self>) {}

    fn can_save(&self, _: &gpui::AppContext) -> bool {
        true
    }

    fn save(
        &mut self,
        project_handle: gpui::ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.handle_save(project_handle, cx)
    }

    fn save_as(
        &mut self,
        project_handle: gpui::ModelHandle<Project>,
        _: std::path::PathBuf,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.handle_save(project_handle, cx)
    }

    fn reload(
        &mut self,
        _: gpui::ModelHandle<Project>,
        _: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        unreachable!("reload should not have been called")
    }

    fn clone_on_split(
        &self,
        _workspace_id: workspace::WorkspaceId,
        cx: &mut ViewContext<Self>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        // TODO FEEDBACK: split is busted
        // Some(self.clone())
        None
    }

    fn serialized_item_kind() -> Option<&'static str> {
        None
    }

    fn deserialize(
        _: gpui::ModelHandle<Project>,
        _: gpui::WeakViewHandle<Workspace>,
        _: workspace::WorkspaceId,
        _: workspace::ItemId,
        _: &mut ViewContext<workspace::Pane>,
    ) -> Task<anyhow::Result<ViewHandle<Self>>> {
        unreachable!()
    }
}

// TODO FEEDBACK: search buffer?
// TODO FEEDBACK: warnings

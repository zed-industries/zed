use std::{ops::Range, sync::Arc};

use anyhow::bail;
use client::{Client, ZED_SECRET_CLIENT_TOKEN};
use editor::{Editor, MultiBuffer};
use futures::AsyncReadExt;
use gpui::{
    actions,
    elements::{ChildView, Flex, Label, MouseEventHandler, ParentElement, Stack, Text},
    serde_json, CursorStyle, Element, ElementBox, Entity, ModelHandle, MouseButton,
    MutableAppContext, RenderContext, Task, View, ViewContext, ViewHandle, WeakViewHandle,
};
use isahc::Request;

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

// TODO FEEDBACK: Rename this file to feedback editor?
// TODO FEEDBACK: Where is the backend code for air table?

lazy_static! {
    pub static ref ZED_SERVER_URL: String =
        std::env::var("ZED_SERVER_URL").unwrap_or_else(|_| "https://zed.dev".to_string());
}

const FEEDBACK_CHAR_COUNT_RANGE: Range<usize> = Range {
    start: 5,
    end: 1000,
};

actions!(feedback, [SubmitFeedback, GiveFeedback, DeployFeedback]);

pub fn init(cx: &mut MutableAppContext) {
    // cx.add_action(FeedbackView::submit_feedback);
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
        // N/A
    }
}

// impl Entity for FeedbackView {
//     type Event = ();
// }

#[derive(Serialize)]
struct FeedbackRequestBody<'a> {
    feedback_text: &'a str,
    metrics_id: Option<Arc<str>>,
    system_specs: SystemSpecs,
    token: &'a str,
}

struct FeedbackEditor {
    editor: ViewHandle<Editor>,
}

impl FeedbackEditor {
    fn new(
        project_handle: ModelHandle<Project>,
        _: WeakViewHandle<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        // TODO FEEDBACK: Get rid of this expect
        let buffer = project_handle
            .update(cx, |project, cx| project.create_buffer("", None, cx))
            .expect("Could not open feedback window");

        let editor = cx.add_view(|cx| {
            let mut editor = Editor::for_buffer(buffer, Some(project_handle.clone()), cx);
            editor.set_vertical_scroll_margin(5, cx);
            editor.set_placeholder_text("Enter your feedback here, save to submit feedback", cx);
            editor
        });

        let this = Self { editor };
        this
    }

    fn submit_feedback(&mut self, cx: &mut ViewContext<'_, Self>) {
        let feedback_text = self.editor.read(cx).text(cx);
        let zed_client = cx.global::<Arc<Client>>();
        let system_specs = SystemSpecs::new(cx);
        let feedback_endpoint = format!("{}/api/feedback", *ZED_SERVER_URL);

        let metrics_id = zed_client.metrics_id();
        let http_client = zed_client.http_client();

        cx.spawn(|_, _| {
            async move {
                // TODO FEEDBACK: Use or remove
                // this.read_with(&async_cx, |this, cx| {
                //     // Now we have a &self and a &AppContext
                // });

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

                dbg!(response_status);

                if !response_status.is_success() {
                    // TODO FEEDBACK: Do some sort of error reporting here for if store fails
                    bail!("Error")
                }

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
    }
    // }
}

// struct FeedbackView {
//     editor: Editor,
// }

impl View for FeedbackEditor {
    fn ui_name() -> &'static str {
        "Feedback"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        // let theme = cx.global::<Settings>().theme.clone();
        // let submit_feedback_text_button_height = 20.0;

        ChildView::new(&self.editor, cx).boxed()
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
        _: gpui::ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        cx.prompt(
            gpui::PromptLevel::Info,
            &format!("You are trying to to submit this feedbac"),
            &["OK"],
        );

        self.submit_feedback(cx);
        Task::ready(Ok(()))
    }

    fn save_as(
        &mut self,
        _: gpui::ModelHandle<Project>,
        _: std::path::PathBuf,
        _: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        unreachable!("save_as should not have been called");
    }

    fn reload(
        &mut self,
        _: gpui::ModelHandle<Project>,
        _: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        unreachable!("should not have been called")
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

// TODO FEEDBACK: Add placeholder text
// TODO FEEDBACK: act_as_type (max mentionedt this)
// TODO FEEDBACK: focus
// TODO FEEDBACK: markdown highlighting
// TODO FEEDBACK: save prompts and accepting closes
// TODO FEEDBACK: multiple tabs?

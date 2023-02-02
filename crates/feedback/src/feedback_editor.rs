use std::{
    any::TypeId,
    ops::{Range, RangeInclusive},
    sync::Arc,
};

use anyhow::bail;
use client::{Client, ZED_SECRET_CLIENT_TOKEN, ZED_SERVER_URL};
use editor::{Anchor, Editor};
use futures::AsyncReadExt;
use gpui::{
    actions,
    elements::{ChildView, Flex, Label, MouseEventHandler, ParentElement, Stack, Text},
    serde_json, AnyViewHandle, AppContext, CursorStyle, Element, ElementBox, Entity, ModelHandle,
    MouseButton, MutableAppContext, PromptLevel, RenderContext, Task, View, ViewContext,
    ViewHandle, WeakViewHandle,
};
use isahc::Request;
use language::Buffer;
use postage::prelude::Stream;

use project::Project;
use serde::Serialize;
use settings::Settings;
use workspace::{
    item::{Item, ItemHandle},
    searchable::{SearchableItem, SearchableItemHandle},
    AppState, StatusItemView, Workspace,
};

use crate::system_specs::SystemSpecs;

const FEEDBACK_CHAR_LIMIT: RangeInclusive<usize> = 10..=5000;
const FEEDBACK_PLACEHOLDER_TEXT: &str = "Save to submit feedback as Markdown.";
const FEEDBACK_SUBMISSION_ERROR_TEXT: &str =
    "Feedback failed to submit, see error log for details.";

actions!(feedback, [SubmitFeedback, GiveFeedback, DeployFeedback]);

pub fn init(system_specs: SystemSpecs, app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    cx.add_action({
        move |workspace: &mut Workspace, _: &GiveFeedback, cx: &mut ViewContext<Workspace>| {
            FeedbackEditor::deploy(system_specs.clone(), workspace, app_state.clone(), cx);
        }
    });
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
}

impl StatusItemView for FeedbackButton {
    fn set_active_pane_item(&mut self, _: Option<&dyn ItemHandle>, _: &mut ViewContext<Self>) {}
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
    system_specs: SystemSpecs,
    editor: ViewHandle<Editor>,
    project: ModelHandle<Project>,
}

impl FeedbackEditor {
    fn new(
        system_specs: SystemSpecs,
        project: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let editor = cx.add_view(|cx| {
            let mut editor = Editor::for_buffer(buffer, Some(project.clone()), cx);
            editor.set_vertical_scroll_margin(5, cx);
            editor.set_placeholder_text(FEEDBACK_PLACEHOLDER_TEXT, cx);
            editor
        });

        cx.subscribe(&editor, |_, _, e, cx| cx.emit(e.clone()))
            .detach();

        Self {
            system_specs: system_specs.clone(),
            editor,
            project,
        }
    }

    fn handle_save(
        &mut self,
        _: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        let feedback_text = self.editor.read(cx).text(cx);
        let feedback_char_count = feedback_text.chars().count();
        let feedback_text = feedback_text.trim().to_string();

        let error = if feedback_char_count < *FEEDBACK_CHAR_LIMIT.start() {
            Some(format!(
                "Feedback can't be shorter than {} characters.",
                FEEDBACK_CHAR_LIMIT.start()
            ))
        } else if feedback_char_count > *FEEDBACK_CHAR_LIMIT.end() {
            Some(format!(
                "Feedback can't be longer than {} characters.",
                FEEDBACK_CHAR_LIMIT.end()
            ))
        } else {
            None
        };

        if let Some(error) = error {
            cx.prompt(PromptLevel::Critical, &error, &["OK"]);
            return Task::ready(Ok(()));
        }

        let mut answer = cx.prompt(
            PromptLevel::Info,
            "Ready to submit your feedback?",
            &["Yes, Submit!", "No"],
        );

        let this = cx.handle();
        let client = cx.global::<Arc<Client>>().clone();
        let specs = self.system_specs.clone();

        cx.spawn(|_, mut cx| async move {
            let answer = answer.recv().await;

            if answer == Some(0) {
                match FeedbackEditor::submit_feedback(&feedback_text, client, specs).await {
                    Ok(_) => {
                        cx.update(|cx| {
                            this.update(cx, |_, cx| {
                                cx.dispatch_action(workspace::CloseActiveItem);
                            })
                        });
                    }
                    Err(error) => {
                        log::error!("{}", error);

                        cx.update(|cx| {
                            this.update(cx, |_, cx| {
                                cx.prompt(
                                    PromptLevel::Critical,
                                    FEEDBACK_SUBMISSION_ERROR_TEXT,
                                    &["OK"],
                                );
                            })
                        });
                    }
                }
            }
        })
        .detach();

        Task::ready(Ok(()))
    }

    async fn submit_feedback(
        feedback_text: &str,
        zed_client: Arc<Client>,
        system_specs: SystemSpecs,
    ) -> anyhow::Result<()> {
        let feedback_endpoint = format!("{}/api/feedback", *ZED_SERVER_URL);

        let metrics_id = zed_client.metrics_id();
        let http_client = zed_client.http_client();

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
            bail!("Feedback API failed with error: {}", response_status)
        }

        Ok(())
    }
}

impl FeedbackEditor {
    pub fn deploy(
        system_specs: SystemSpecs,
        workspace: &mut Workspace,
        app_state: Arc<AppState>,
        cx: &mut ViewContext<Workspace>,
    ) {
        workspace
            .with_local_workspace(&app_state, cx, |workspace, cx| {
                let project = workspace.project().clone();
                let markdown_language = project.read(cx).languages().language_for_name("Markdown");
                let buffer = project
                    .update(cx, |project, cx| {
                        project.create_buffer("", markdown_language, cx)
                    })
                    .expect("creating buffers on a local workspace always succeeds");
                let feedback_editor =
                    cx.add_view(|cx| FeedbackEditor::new(system_specs, project, buffer, cx));
                workspace.add_item(Box::new(feedback_editor), cx);
            })
            .detach();
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
    type Event = editor::Event;
}

impl Item for FeedbackEditor {
    fn tab_content(&self, _: Option<usize>, style: &theme::Tab, _: &AppContext) -> ElementBox {
        Flex::row()
            .with_child(
                Label::new("Feedback".to_string(), style.label.clone())
                    .aligned()
                    .contained()
                    .boxed(),
            )
            .boxed()
    }

    fn for_each_project_item(&self, cx: &AppContext, f: &mut dyn FnMut(usize, &dyn project::Item)) {
        self.editor.for_each_project_item(cx, f)
    }

    fn to_item_events(_: &Self::Event) -> Vec<workspace::item::ItemEvent> {
        Vec::new()
    }

    fn is_singleton(&self, _: &AppContext) -> bool {
        true
    }

    fn set_nav_history(&mut self, _: workspace::ItemNavHistory, _: &mut ViewContext<Self>) {}

    fn can_save(&self, _: &AppContext) -> bool {
        true
    }

    fn save(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.handle_save(project, cx)
    }

    fn save_as(
        &mut self,
        project: ModelHandle<Project>,
        _: std::path::PathBuf,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.handle_save(project, cx)
    }

    fn reload(
        &mut self,
        _: ModelHandle<Project>,
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
        let buffer = self
            .editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("Feedback buffer is only ever singleton");

        Some(Self::new(
            self.system_specs.clone(),
            self.project.clone(),
            buffer.clone(),
            cx,
        ))
    }

    fn serialized_item_kind() -> Option<&'static str> {
        None
    }

    fn deserialize(
        _: ModelHandle<Project>,
        _: WeakViewHandle<Workspace>,
        _: workspace::WorkspaceId,
        _: workspace::ItemId,
        _: &mut ViewContext<workspace::Pane>,
    ) -> Task<anyhow::Result<ViewHandle<Self>>> {
        unreachable!()
    }

    fn as_searchable(&self, handle: &ViewHandle<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }

    fn act_as_type(
        &self,
        type_id: TypeId,
        self_handle: &ViewHandle<Self>,
        _: &AppContext,
    ) -> Option<AnyViewHandle> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.into())
        } else if type_id == TypeId::of::<Editor>() {
            Some((&self.editor).into())
        } else {
            None
        }
    }
}

impl SearchableItem for FeedbackEditor {
    type Match = Range<Anchor>;

    fn to_search_event(event: &Self::Event) -> Option<workspace::searchable::SearchEvent> {
        Editor::to_search_event(event)
    }

    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.clear_matches(cx))
    }

    fn update_matches(&mut self, matches: Vec<Self::Match>, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.update_matches(matches, cx))
    }

    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String {
        self.editor
            .update(cx, |editor, cx| editor.query_suggestion(cx))
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: Vec<Self::Match>,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor
            .update(cx, |editor, cx| editor.activate_match(index, matches, cx))
    }

    fn find_matches(
        &mut self,
        query: project::search::SearchQuery,
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Self::Match>> {
        self.editor
            .update(cx, |editor, cx| editor.find_matches(query, cx))
    }

    fn active_match_index(
        &mut self,
        matches: Vec<Self::Match>,
        cx: &mut ViewContext<Self>,
    ) -> Option<usize> {
        self.editor
            .update(cx, |editor, cx| editor.active_match_index(matches, cx))
    }
}

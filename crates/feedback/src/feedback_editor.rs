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
    elements::{ChildView, Flex, Label, ParentElement},
    serde_json, AnyViewHandle, AppContext, Element, ElementBox, Entity, ModelHandle,
    MutableAppContext, PromptLevel, RenderContext, Task, View, ViewContext, ViewHandle,
};
use isahc::Request;
use language::Buffer;
use postage::prelude::Stream;

use project::Project;
use serde::Serialize;
use util::ResultExt;
use workspace::{
    item::{Item, ItemHandle},
    searchable::{SearchableItem, SearchableItemHandle},
    AppState, Workspace,
};

use crate::{submit_feedback_button::SubmitFeedbackButton, system_specs::SystemSpecs};

const FEEDBACK_CHAR_LIMIT: RangeInclusive<usize> = 10..=5000;
const FEEDBACK_SUBMISSION_ERROR_TEXT: &str =
    "Feedback failed to submit, see error log for details.";

actions!(feedback, [GiveFeedback, SubmitFeedback]);

pub fn init(system_specs: SystemSpecs, app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    cx.add_action({
        move |workspace: &mut Workspace, _: &GiveFeedback, cx: &mut ViewContext<Workspace>| {
            FeedbackEditor::deploy(system_specs.clone(), workspace, app_state.clone(), cx);
        }
    });

    cx.add_async_action(
        |submit_feedback_button: &mut SubmitFeedbackButton, _: &SubmitFeedback, cx| {
            if let Some(active_item) = submit_feedback_button.active_item.as_ref() {
                Some(active_item.update(cx, |feedback_editor, cx| feedback_editor.handle_save(cx)))
            } else {
                None
            }
        },
    );
}

#[derive(Serialize)]
struct FeedbackRequestBody<'a> {
    feedback_text: &'a str,
    metrics_id: Option<Arc<str>>,
    system_specs: SystemSpecs,
    is_staff: bool,
    token: &'a str,
}

#[derive(Clone)]
pub(crate) struct FeedbackEditor {
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

    fn handle_save(&mut self, cx: &mut ViewContext<Self>) -> Task<anyhow::Result<()>> {
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
        let is_staff = zed_client.is_staff();
        let http_client = zed_client.http_client();

        let request = FeedbackRequestBody {
            feedback_text: &feedback_text,
            metrics_id,
            system_specs,
            is_staff: is_staff.unwrap_or(false),
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
        _: &mut Workspace,
        app_state: Arc<AppState>,
        cx: &mut ViewContext<Workspace>,
    ) {
        let markdown = app_state.languages.language_for_name("Markdown");
        cx.spawn(|workspace, mut cx| async move {
            let markdown = markdown.await.log_err();
            workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.with_local_workspace(&app_state, cx, |workspace, cx| {
                        let project = workspace.project().clone();
                        let buffer = project
                            .update(cx, |project, cx| project.create_buffer("", markdown, cx))
                            .expect("creating buffers on a local workspace always succeeds");
                        let feedback_editor = cx
                            .add_view(|cx| FeedbackEditor::new(system_specs, project, buffer, cx));
                        workspace.add_item(Box::new(feedback_editor), cx);
                    })
                })
                .await;
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
                Label::new("Feedback", style.label.clone())
                    .aligned()
                    .contained()
                    .boxed(),
            )
            .boxed()
    }

    fn for_each_project_item(&self, cx: &AppContext, f: &mut dyn FnMut(usize, &dyn project::Item)) {
        self.editor.for_each_project_item(cx, f)
    }

    fn is_singleton(&self, _: &AppContext) -> bool {
        true
    }

    fn can_save(&self, _: &AppContext) -> bool {
        true
    }

    fn save(
        &mut self,
        _: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.handle_save(cx)
    }

    fn save_as(
        &mut self,
        _: ModelHandle<Project>,
        _: std::path::PathBuf,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.handle_save(cx)
    }

    fn reload(
        &mut self,
        _: ModelHandle<Project>,
        _: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        Task::Ready(Some(Ok(())))
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

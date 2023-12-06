use crate::system_specs::SystemSpecs;
use anyhow::bail;
use client::{Client, ZED_SECRET_CLIENT_TOKEN, ZED_SERVER_URL};
use editor::{Anchor, Editor, EditorEvent};
use futures::AsyncReadExt;
use gpui::{
    actions, serde_json, AnyElement, AnyView, AppContext, Div, EntityId, EventEmitter,
    FocusableView, Model, PromptLevel, Task, View, ViewContext, WindowContext,
};
use isahc::Request;
use language::{Buffer, Event};
use project::{search::SearchQuery, Project};
use regex::Regex;
use serde::Serialize;
use std::{
    any::TypeId,
    ops::{Range, RangeInclusive},
    sync::Arc,
};
use ui::{prelude::*, Icon, IconElement, Label};
use util::ResultExt;
use workspace::{
    item::{Item, ItemEvent, ItemHandle},
    searchable::{SearchEvent, SearchableItem, SearchableItemHandle},
    Workspace,
};

const FEEDBACK_CHAR_LIMIT: RangeInclusive<usize> = 10..=5000;
const FEEDBACK_SUBMISSION_ERROR_TEXT: &str =
    "Feedback failed to submit, see error log for details.";

actions!(GiveFeedback, SubmitFeedback);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &GiveFeedback, cx| {
            FeedbackEditor::deploy(workspace, cx);
        });
    })
    .detach();
}

#[derive(Serialize)]
struct FeedbackRequestBody<'a> {
    feedback_text: &'a str,
    email: Option<String>,
    metrics_id: Option<Arc<str>>,
    installation_id: Option<Arc<str>>,
    system_specs: SystemSpecs,
    is_staff: bool,
    token: &'a str,
}

#[derive(Clone)]
pub(crate) struct FeedbackEditor {
    system_specs: SystemSpecs,
    editor: View<Editor>,
    project: Model<Project>,
    pub allow_submission: bool,
}

impl EventEmitter<Event> for FeedbackEditor {}
impl EventEmitter<EditorEvent> for FeedbackEditor {}

impl FeedbackEditor {
    fn new(
        system_specs: SystemSpecs,
        project: Model<Project>,
        buffer: Model<Buffer>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let editor = cx.build_view(|cx| {
            let mut editor = Editor::for_buffer(buffer, Some(project.clone()), cx);
            editor.set_vertical_scroll_margin(5, cx);
            editor
        });

        cx.subscribe(
            &editor,
            |&mut _, _, e: &EditorEvent, cx: &mut ViewContext<_>| cx.emit(e.clone()),
        )
        .detach();

        Self {
            system_specs: system_specs.clone(),
            editor,
            project,
            allow_submission: true,
        }
    }

    pub fn submit(&mut self, cx: &mut ViewContext<Self>) -> Task<anyhow::Result<()>> {
        if !self.allow_submission {
            return Task::ready(Ok(()));
        }

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
            let prompt = cx.prompt(PromptLevel::Critical, &error, &["OK"]);
            cx.spawn(|_, _cx| async move {
                prompt.await.ok();
            })
            .detach();
            return Task::ready(Ok(()));
        }

        let answer = cx.prompt(
            PromptLevel::Info,
            "Ready to submit your feedback?",
            &["Yes, Submit!", "No"],
        );

        let client = cx.global::<Arc<Client>>().clone();
        let specs = self.system_specs.clone();

        cx.spawn(|this, mut cx| async move {
            let answer = answer.await.ok();

            if answer == Some(0) {
                this.update(&mut cx, |feedback_editor, cx| {
                    feedback_editor.set_allow_submission(false, cx);
                })
                .log_err();

                match FeedbackEditor::submit_feedback(&feedback_text, client, specs).await {
                    Ok(_) => {
                        this.update(&mut cx, |_, cx| cx.emit(Event::Closed))
                            .log_err();
                    }

                    Err(error) => {
                        log::error!("{}", error);
                        this.update(&mut cx, |feedback_editor, cx| {
                            let prompt = cx.prompt(
                                PromptLevel::Critical,
                                FEEDBACK_SUBMISSION_ERROR_TEXT,
                                &["OK"],
                            );
                            cx.spawn(|_, _cx| async move {
                                prompt.await.ok();
                            })
                            .detach();
                            feedback_editor.set_allow_submission(true, cx);
                        })
                        .log_err();
                    }
                }
            }
        })
        .detach();

        Task::ready(Ok(()))
    }

    fn set_allow_submission(&mut self, allow_submission: bool, cx: &mut ViewContext<Self>) {
        self.allow_submission = allow_submission;
        cx.notify();
    }

    async fn submit_feedback(
        feedback_text: &str,
        zed_client: Arc<Client>,
        system_specs: SystemSpecs,
    ) -> anyhow::Result<()> {
        let feedback_endpoint = format!("{}/api/feedback", *ZED_SERVER_URL);

        let telemetry = zed_client.telemetry();
        let metrics_id = telemetry.metrics_id();
        let installation_id = telemetry.installation_id();
        let is_staff = telemetry.is_staff();
        let http_client = zed_client.http_client();

        let re = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();

        let emails: Vec<&str> = re
            .captures_iter(feedback_text)
            .map(|capture| capture.get(0).unwrap().as_str())
            .collect();

        let email = emails.first().map(|e| e.to_string());

        let request = FeedbackRequestBody {
            feedback_text: &feedback_text,
            email,
            metrics_id,
            installation_id,
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
    pub fn deploy(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        let markdown = workspace
            .app_state()
            .languages
            .language_for_name("Markdown");
        cx.spawn(|workspace, mut cx| async move {
            let markdown = markdown.await.log_err();
            workspace
                .update(&mut cx, |workspace, cx| {
                    workspace.with_local_workspace(cx, |workspace, cx| {
                        let project = workspace.project().clone();
                        let buffer = project
                            .update(cx, |project, cx| project.create_buffer("", markdown, cx))
                            .expect("creating buffers on a local workspace always succeeds");
                        let system_specs = SystemSpecs::new(cx);
                        let feedback_editor = cx.build_view(|cx| {
                            FeedbackEditor::new(system_specs, project, buffer, cx)
                        });
                        workspace.add_item(Box::new(feedback_editor), cx);
                    })
                })?
                .await
        })
        .detach_and_log_err(cx);
    }
}

// TODO
impl Render for FeedbackEditor {
    type Element = Div;

    fn render(&mut self, _: &mut ViewContext<Self>) -> Self::Element {
        div().size_full().child(self.editor.clone())
    }
}

impl EventEmitter<ItemEvent> for FeedbackEditor {}

impl FocusableView for FeedbackEditor {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for FeedbackEditor {
    fn tab_tooltip_text(&self, _: &AppContext) -> Option<SharedString> {
        Some("Send Feedback".into())
    }

    fn tab_content(&self, detail: Option<usize>, cx: &WindowContext) -> AnyElement {
        h_stack()
            .gap_1()
            .child(IconElement::new(Icon::Envelope).color(Color::Accent))
            .child(Label::new("Send Feedback".to_string()))
            .into_any_element()
    }

    fn for_each_project_item(
        &self,
        cx: &AppContext,
        f: &mut dyn FnMut(EntityId, &dyn project::Item),
    ) {
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
        _project: Model<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.submit(cx)
    }

    fn save_as(
        &mut self,
        _: Model<Project>,
        _: std::path::PathBuf,
        cx: &mut ViewContext<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.submit(cx)
    }

    fn reload(&mut self, _: Model<Project>, _: &mut ViewContext<Self>) -> Task<anyhow::Result<()>> {
        Task::Ready(Some(Ok(())))
    }

    fn clone_on_split(
        &self,
        _workspace_id: workspace::WorkspaceId,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>>
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

        Some(cx.build_view(|cx| {
            Self::new(
                self.system_specs.clone(),
                self.project.clone(),
                buffer.clone(),
                cx,
            )
        }))
    }

    fn as_searchable(&self, handle: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a View<Self>,
        cx: &'a AppContext,
    ) -> Option<AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.to_any())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.to_any())
        } else {
            None
        }
    }

    fn deactivated(&mut self, _: &mut ViewContext<Self>) {}

    fn workspace_deactivated(&mut self, _: &mut ViewContext<Self>) {}

    fn navigate(&mut self, _: Box<dyn std::any::Any>, _: &mut ViewContext<Self>) -> bool {
        false
    }

    fn tab_description(&self, _: usize, _: &AppContext) -> Option<ui::prelude::SharedString> {
        None
    }

    fn set_nav_history(&mut self, _: workspace::ItemNavHistory, _: &mut ViewContext<Self>) {}

    fn is_dirty(&self, _: &AppContext) -> bool {
        false
    }

    fn has_conflict(&self, _: &AppContext) -> bool {
        false
    }

    fn breadcrumb_location(&self) -> workspace::ToolbarItemLocation {
        workspace::ToolbarItemLocation::Hidden
    }

    fn breadcrumbs(
        &self,
        _theme: &theme::Theme,
        _cx: &AppContext,
    ) -> Option<Vec<workspace::item::BreadcrumbText>> {
        None
    }

    fn added_to_workspace(&mut self, _workspace: &mut Workspace, _cx: &mut ViewContext<Self>) {}

    fn serialized_item_kind() -> Option<&'static str> {
        Some("feedback")
    }

    fn deserialize(
        _project: gpui::Model<Project>,
        _workspace: gpui::WeakView<Workspace>,
        _workspace_id: workspace::WorkspaceId,
        _item_id: workspace::ItemId,
        _cx: &mut ViewContext<workspace::Pane>,
    ) -> Task<anyhow::Result<View<Self>>> {
        unimplemented!(
            "deserialize() must be implemented if serialized_item_kind() returns Some(_)"
        )
    }

    fn show_toolbar(&self) -> bool {
        true
    }

    fn pixel_position_of_cursor(&self, _: &AppContext) -> Option<gpui::Point<gpui::Pixels>> {
        None
    }
}

impl EventEmitter<SearchEvent> for FeedbackEditor {}

impl SearchableItem for FeedbackEditor {
    type Match = Range<Anchor>;

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

    fn select_matches(&mut self, matches: Vec<Self::Match>, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |e, cx| e.select_matches(matches, cx))
    }
    fn replace(&mut self, matches: &Self::Match, query: &SearchQuery, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |e, cx| e.replace(matches, query, cx));
    }
    fn find_matches(
        &mut self,
        query: Arc<project::search::SearchQuery>,
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

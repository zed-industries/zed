use std::path::Path;

use collections::HashSet;
use feature_flags::FeatureFlagAppExt;
use gpui::{
    App, Empty, Entity, EventEmitter, FocusHandle, Focusable, Global, ListAlignment, ListState,
    SharedString, Subscription, Window, actions, list, prelude::*,
};
use release_channel::ReleaseChannel;
use settings::Settings;
use ui::prelude::*;
use workspace::{Item, Workspace, WorkspaceId, item::ItemEvent};

use super::edit_action::EditAction;

actions!(debug, [EditTool]);

pub fn init(cx: &mut App) {
    if cx.is_staff() || ReleaseChannel::global(cx) == ReleaseChannel::Dev {
        // Track events even before opening the log
        EditToolLog::global(cx);
    }

    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &EditTool, window, cx| {
            let viewer = cx.new(EditToolLogViewer::new);
            workspace.add_item_to_active_pane(Box::new(viewer), None, true, window, cx)
        });
    })
    .detach();
}

pub struct GlobalEditToolLog(Entity<EditToolLog>);

impl Global for GlobalEditToolLog {}

#[derive(Default)]
pub struct EditToolLog {
    requests: Vec<EditToolRequest>,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct EditToolRequestId(u32);

impl EditToolLog {
    pub fn global(cx: &mut App) -> Entity<Self> {
        match Self::try_global(cx) {
            Some(entity) => entity,
            None => {
                let entity = cx.new(|_cx| Self::default());
                cx.set_global(GlobalEditToolLog(entity.clone()));
                entity
            }
        }
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalEditToolLog>()
            .map(|log| log.0.clone())
    }

    pub fn new_request(
        &mut self,
        instructions: String,
        cx: &mut Context<Self>,
    ) -> EditToolRequestId {
        let id = EditToolRequestId(self.requests.len() as u32);
        self.requests.push(EditToolRequest {
            id,
            instructions,
            editor_response: None,
            tool_output: None,
            parsed_edits: Vec::new(),
        });
        cx.emit(EditToolLogEvent::Inserted);
        id
    }

    pub fn push_editor_response_chunk(
        &mut self,
        id: EditToolRequestId,
        chunk: &str,
        new_actions: &[(EditAction, String)],
        cx: &mut Context<Self>,
    ) {
        if let Some(request) = self.requests.get_mut(id.0 as usize) {
            match &mut request.editor_response {
                None => {
                    request.editor_response = Some(chunk.to_string());
                }
                Some(response) => {
                    response.push_str(chunk);
                }
            }
            request
                .parsed_edits
                .extend(new_actions.iter().cloned().map(|(action, _)| action));

            cx.emit(EditToolLogEvent::Updated);
        }
    }

    pub fn set_tool_output(
        &mut self,
        id: EditToolRequestId,
        tool_output: Result<String, String>,
        cx: &mut Context<Self>,
    ) {
        if let Some(request) = self.requests.get_mut(id.0 as usize) {
            request.tool_output = Some(tool_output);
            cx.emit(EditToolLogEvent::Updated);
        }
    }
}

enum EditToolLogEvent {
    Inserted,
    Updated,
}

impl EventEmitter<EditToolLogEvent> for EditToolLog {}

pub struct EditToolRequest {
    id: EditToolRequestId,
    instructions: String,
    // we don't use a result here because the error might have occurred after we got a response
    editor_response: Option<String>,
    parsed_edits: Vec<EditAction>,
    tool_output: Option<Result<String, String>>,
}

pub struct EditToolLogViewer {
    focus_handle: FocusHandle,
    log: Entity<EditToolLog>,
    list_state: ListState,
    expanded_edits: HashSet<(EditToolRequestId, usize)>,
    _subscription: Subscription,
}

impl EditToolLogViewer {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let log = EditToolLog::global(cx);

        let subscription = cx.subscribe(&log, Self::handle_log_event);

        Self {
            focus_handle: cx.focus_handle(),
            log: log.clone(),
            list_state: ListState::new(
                log.read(cx).requests.len(),
                ListAlignment::Bottom,
                px(1024.),
                {
                    let this = cx.entity().downgrade();
                    move |ix, window: &mut Window, cx: &mut App| {
                        this.update(cx, |this, cx| this.render_request(ix, window, cx))
                            .unwrap()
                    }
                },
            ),
            expanded_edits: HashSet::default(),
            _subscription: subscription,
        }
    }

    fn handle_log_event(
        &mut self,
        _: Entity<EditToolLog>,
        event: &EditToolLogEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            EditToolLogEvent::Inserted => {
                let count = self.list_state.item_count();
                self.list_state.splice(count..count, 1);
            }
            EditToolLogEvent::Updated => {}
        }

        cx.notify();
    }

    fn render_request(
        &self,
        index: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let requests = &self.log.read(cx).requests;
        let request = &requests[index];

        v_flex()
            .gap_3()
            .child(Self::render_section(IconName::ArrowRight, "Tool Input"))
            .child(request.instructions.clone())
            .py_5()
            .when(index + 1 < requests.len(), |element| {
                element
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
            })
            .map(|parent| match &request.editor_response {
                None => {
                    if request.tool_output.is_none() {
                        parent.child("...")
                    } else {
                        parent
                    }
                }
                Some(response) => parent
                    .child(Self::render_section(
                        IconName::ZedAssistant,
                        "Editor Response",
                    ))
                    .child(Label::new(response.clone()).buffer_font(cx)),
            })
            .when(!request.parsed_edits.is_empty(), |parent| {
                parent
                    .child(Self::render_section(IconName::Microscope, "Parsed Edits"))
                    .child(
                        v_flex()
                            .gap_2()
                            .children(request.parsed_edits.iter().enumerate().map(
                                |(index, edit)| {
                                    self.render_edit_action(edit, request.id, index, cx)
                                },
                            )),
                    )
            })
            .when_some(request.tool_output.as_ref(), |parent, output| {
                parent
                    .child(Self::render_section(IconName::ArrowLeft, "Tool Output"))
                    .child(match output {
                        Ok(output) => Label::new(output.clone()).color(Color::Success),
                        Err(error) => Label::new(error.clone()).color(Color::Error),
                    })
            })
            .into_any()
    }

    fn render_section(icon: IconName, title: &'static str) -> AnyElement {
        h_flex()
            .gap_1()
            .child(Icon::new(icon).color(Color::Muted))
            .child(Label::new(title).size(LabelSize::Small).color(Color::Muted))
            .into_any()
    }

    fn render_edit_action(
        &self,
        edit_action: &EditAction,
        request_id: EditToolRequestId,
        index: usize,
        cx: &Context<Self>,
    ) -> AnyElement {
        let expanded_id = (request_id, index);

        match edit_action {
            EditAction::Replace {
                file_path,
                old,
                new,
            } => self
                .render_edit_action_container(
                    expanded_id,
                    &file_path,
                    [
                        Self::render_block(IconName::MagnifyingGlass, "Search", old.clone(), cx)
                            .border_r_1()
                            .border_color(cx.theme().colors().border)
                            .into_any(),
                        Self::render_block(IconName::Replace, "Replace", new.clone(), cx)
                            .into_any(),
                    ],
                    cx,
                )
                .into_any(),
            EditAction::Write { file_path, content } => self
                .render_edit_action_container(
                    expanded_id,
                    &file_path,
                    [
                        Self::render_block(IconName::Pencil, "Write", content.clone(), cx)
                            .into_any(),
                    ],
                    cx,
                )
                .into_any(),
        }
    }

    fn render_edit_action_container(
        &self,
        expanded_id: (EditToolRequestId, usize),
        file_path: &Path,
        content: impl IntoIterator<Item = AnyElement>,
        cx: &Context<Self>,
    ) -> AnyElement {
        let is_expanded = self.expanded_edits.contains(&expanded_id);

        v_flex()
            .child(
                h_flex()
                    .bg(cx.theme().colors().element_background)
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .rounded_t_md()
                    .when(!is_expanded, |el| el.rounded_b_md())
                    .py_1()
                    .px_2()
                    .gap_1()
                    .child(
                        ui::Disclosure::new(ElementId::Integer(expanded_id.1), is_expanded)
                            .on_click(cx.listener(move |this, _ev, _window, cx| {
                                if is_expanded {
                                    this.expanded_edits.remove(&expanded_id);
                                } else {
                                    this.expanded_edits.insert(expanded_id);
                                }

                                cx.notify();
                            })),
                    )
                    .child(Label::new(file_path.display().to_string()).size(LabelSize::Small)),
            )
            .child(if is_expanded {
                h_flex()
                    .border_1()
                    .border_t_0()
                    .border_color(cx.theme().colors().border)
                    .rounded_b_md()
                    .children(content)
                    .into_any()
            } else {
                Empty.into_any()
            })
            .into_any()
    }

    fn render_block(icon: IconName, title: &'static str, content: String, cx: &App) -> Div {
        v_flex()
            .p_1()
            .gap_1()
            .flex_1()
            .h_full()
            .child(
                h_flex()
                    .gap_1()
                    .child(Icon::new(icon).color(Color::Muted))
                    .child(Label::new(title).size(LabelSize::Small).color(Color::Muted)),
            )
            .font(theme::ThemeSettings::get_global(cx).buffer_font.clone())
            .text_sm()
            .child(content)
            .child(div().flex_1())
    }
}

impl EventEmitter<()> for EditToolLogViewer {}

impl Focusable for EditToolLogViewer {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for EditToolLogViewer {
    type Event = ();

    fn to_item_events(_: &Self::Event, _: impl FnMut(ItemEvent)) {}

    fn tab_content_text(&self, _window: &Window, _cx: &App) -> Option<SharedString> {
        Some("Edit Tool Log".into())
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>>
    where
        Self: Sized,
    {
        Some(cx.new(Self::new))
    }
}

impl Render for EditToolLogViewer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.list_state.item_count() == 0 {
            return v_flex()
                .justify_center()
                .size_full()
                .gap_1()
                .bg(cx.theme().colors().editor_background)
                .text_center()
                .text_lg()
                .child("No requests yet")
                .child(
                    div()
                        .text_ui(cx)
                        .child("Go ask the assistant to perform some edits"),
                );
        }

        v_flex()
            .p_4()
            .bg(cx.theme().colors().editor_background)
            .size_full()
            .child(list(self.list_state.clone()).flex_grow())
    }
}

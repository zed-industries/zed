use feature_flags::FeatureFlagAppExt;
use gpui::{
    actions, prelude::*, App, Entity, EventEmitter, FocusHandle, Focusable, Global, SharedString,
    Subscription, Window,
};
use release_channel::ReleaseChannel;
use ui::prelude::*;
use workspace::{item::ItemEvent, Item, Workspace, WorkspaceId};

actions!(debug, [OpenAssistantEditToolLog]);

pub fn init(cx: &mut App) {
    if cx.is_staff() || ReleaseChannel::global(cx) == ReleaseChannel::Dev {
        // Track events even before opening the log
        EditToolLog::global(cx);
    }

    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &OpenAssistantEditToolLog, window, cx| {
            let viewer = cx.new(|cx| EditToolLogViewer::new(cx));
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

#[derive(Clone, Copy)]
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

    pub fn insert_request(
        &mut self,
        instructions: String,
        cx: &mut Context<Self>,
    ) -> EditToolRequestId {
        let id = EditToolRequestId(self.requests.len() as u32);
        self.requests.push(EditToolRequest {
            instructions,
            response: None,
            error: None,
        });
        cx.emit(());
        id
    }

    pub fn push_response_chunk(
        &mut self,
        id: EditToolRequestId,
        chunk: &str,
        cx: &mut Context<Self>,
    ) {
        if let Some(request) = self.requests.get_mut(id.0 as usize) {
            match &mut request.response {
                None => {
                    request.response = Some(chunk.to_string());
                }
                Some(response) => {
                    response.push_str(chunk);
                }
            }
            cx.emit(());
        }
    }

    pub fn set_request_error(
        &mut self,
        id: EditToolRequestId,
        error: String,
        cx: &mut Context<Self>,
    ) {
        if let Some(request) = self.requests.get_mut(id.0 as usize) {
            request.error = Some(error);
            cx.emit(());
        }
    }
}

impl EventEmitter<()> for EditToolLog {}

pub struct EditToolRequest {
    instructions: String,
    // we don't use a result here because the error might have occurred after we got a response
    response: Option<String>,
    error: Option<String>,
}

pub struct EditToolLogViewer {
    focus_handle: FocusHandle,
    log: Entity<EditToolLog>,
    _subscription: Subscription,
}

impl EditToolLogViewer {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let log = EditToolLog::global(cx);
        let subscription = cx.subscribe(&log, |_, _, _, cx| {
            cx.notify();
        });

        Self {
            focus_handle: cx.focus_handle(),
            log,
            _subscription: subscription,
        }
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
        Some(cx.new(|cx| Self::new(cx)))
    }
}

impl Render for EditToolLogViewer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("edit-tool-log")
            .bg(cx.theme().colors().editor_background)
            .h_full()
            .p_2()
            .gap_3()
            .overflow_y_scroll()
            .children(self.log.read(cx).requests.iter().map(|request| {
                v_flex()
                    .gap_3()
                    .child(
                        Label::new("Instructions")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(request.instructions.clone())
                    .map(|parent| match &request.response {
                        None => {
                            if request.error.is_none() {
                                parent.child("...")
                            } else {
                                parent
                            }
                        }
                        Some(response) => parent
                            .child(
                                Label::new("Response")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(response.clone()),
                    })
                    .when_some(request.error.as_ref(), |parent, error| {
                        parent
                            .child(
                                Label::new("Error")
                                    .size(LabelSize::Small)
                                    .color(Color::Error),
                            )
                            .child(Label::new(error.clone()).color(Color::Error))
                    })
                    .child(ui::Divider::horizontal())
            }))
    }
}

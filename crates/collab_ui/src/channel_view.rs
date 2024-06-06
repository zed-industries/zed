use anyhow::Result;
use call::report_call_event_for_channel;
use channel::{Channel, ChannelBuffer, ChannelBufferEvent, ChannelStore};
use client::{
    proto::{self, PeerId},
    ChannelId, Collaborator, ParticipantIndex,
};
use collections::HashMap;
use editor::{
    display_map::ToDisplayPoint, scroll::Autoscroll, CollaborationHub, DisplayPoint, Editor,
    EditorEvent,
};
use gpui::{
    actions, AnyElement, AnyView, AppContext, ClipboardItem, Entity as _, EventEmitter,
    FocusableView, IntoElement as _, Model, Pixels, Point, Render, Subscription, Task, View,
    ViewContext, VisualContext as _, WeakView, WindowContext,
};
use project::Project;
use std::{
    any::{Any, TypeId},
    sync::Arc,
};
use ui::{prelude::*, Label};
use util::ResultExt;
use workspace::notifications::NotificationId;
use workspace::{
    item::{FollowableItem, Item, ItemEvent, ItemHandle, TabContentParams},
    register_followable_item,
    searchable::SearchableItemHandle,
    ItemNavHistory, Pane, SaveIntent, Toast, ViewId, Workspace, WorkspaceId,
};

actions!(collab, [CopyLink]);

pub fn init(cx: &mut AppContext) {
    register_followable_item::<ChannelView>(cx)
}

pub struct ChannelView {
    pub editor: View<Editor>,
    workspace: WeakView<Workspace>,
    project: Model<Project>,
    channel_store: Model<ChannelStore>,
    channel_buffer: Model<ChannelBuffer>,
    remote_id: Option<ViewId>,
    _editor_event_subscription: Subscription,
    _reparse_subscription: Option<Subscription>,
}

impl ChannelView {
    pub fn open(
        channel_id: ChannelId,
        link_position: Option<String>,
        workspace: View<Workspace>,
        cx: &mut WindowContext,
    ) -> Task<Result<View<Self>>> {
        let pane = workspace.read(cx).active_pane().clone();
        let channel_view = Self::open_in_pane(
            channel_id,
            link_position,
            pane.clone(),
            workspace.clone(),
            cx,
        );
        cx.spawn(|mut cx| async move {
            let channel_view = channel_view.await?;
            pane.update(&mut cx, |pane, cx| {
                report_call_event_for_channel(
                    "open channel notes",
                    channel_id,
                    &workspace.read(cx).app_state().client,
                    cx,
                );
                pane.add_item(Box::new(channel_view.clone()), true, true, None, cx);
            })?;
            anyhow::Ok(channel_view)
        })
    }

    pub fn open_in_pane(
        channel_id: ChannelId,
        link_position: Option<String>,
        pane: View<Pane>,
        workspace: View<Workspace>,
        cx: &mut WindowContext,
    ) -> Task<Result<View<Self>>> {
        let weak_workspace = workspace.downgrade();
        let workspace = workspace.read(cx);
        let project = workspace.project().to_owned();
        let channel_store = ChannelStore::global(cx);
        let language_registry = workspace.app_state().languages.clone();
        let markdown = language_registry.language_for_name("Markdown");
        let channel_buffer =
            channel_store.update(cx, |store, cx| store.open_channel_buffer(channel_id, cx));

        cx.spawn(|mut cx| async move {
            let channel_buffer = channel_buffer.await?;
            let markdown = markdown.await.log_err();

            channel_buffer.update(&mut cx, |channel_buffer, cx| {
                channel_buffer.buffer().update(cx, |buffer, cx| {
                    buffer.set_language_registry(language_registry);
                    let Some(markdown) = markdown else {
                        return;
                    };
                    buffer.set_language(Some(markdown), cx);
                })
            })?;

            pane.update(&mut cx, |pane, cx| {
                let buffer_id = channel_buffer.read(cx).remote_id(cx);

                let existing_view = pane
                    .items_of_type::<Self>()
                    .find(|view| view.read(cx).channel_buffer.read(cx).remote_id(cx) == buffer_id);

                // If this channel buffer is already open in this pane, just return it.
                if let Some(existing_view) = existing_view.clone() {
                    if existing_view.read(cx).channel_buffer == channel_buffer {
                        if let Some(link_position) = link_position {
                            existing_view.update(cx, |channel_view, cx| {
                                channel_view.focus_position_from_link(link_position, true, cx)
                            });
                        }
                        return existing_view;
                    }
                }

                let view = cx.new_view(|cx| {
                    let mut this =
                        Self::new(project, weak_workspace, channel_store, channel_buffer, cx);
                    this.acknowledge_buffer_version(cx);
                    this
                });

                // If the pane contained a disconnected view for this channel buffer,
                // replace that.
                if let Some(existing_item) = existing_view {
                    if let Some(ix) = pane.index_for_item(&existing_item) {
                        pane.close_item_by_id(existing_item.entity_id(), SaveIntent::Skip, cx)
                            .detach();
                        pane.add_item(Box::new(view.clone()), true, true, Some(ix), cx);
                    }
                }

                if let Some(link_position) = link_position {
                    view.update(cx, |channel_view, cx| {
                        channel_view.focus_position_from_link(link_position, true, cx)
                    });
                }

                view
            })
        })
    }

    pub fn new(
        project: Model<Project>,
        workspace: WeakView<Workspace>,
        channel_store: Model<ChannelStore>,
        channel_buffer: Model<ChannelBuffer>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = channel_buffer.read(cx).buffer();
        let this = cx.view().downgrade();
        let editor = cx.new_view(|cx| {
            let mut editor = Editor::for_buffer(buffer, None, cx);
            editor.set_collaboration_hub(Box::new(ChannelBufferCollaborationHub(
                channel_buffer.clone(),
            )));
            editor.set_custom_context_menu(move |_, position, cx| {
                let this = this.clone();
                Some(ui::ContextMenu::build(cx, move |menu, _| {
                    menu.entry("Copy link to section", None, move |cx| {
                        this.update(cx, |this, cx| this.copy_link_for_position(position, cx))
                            .ok();
                    })
                }))
            });
            editor
        });
        let _editor_event_subscription =
            cx.subscribe(&editor, |_, _, e: &EditorEvent, cx| cx.emit(e.clone()));

        cx.subscribe(&channel_buffer, Self::handle_channel_buffer_event)
            .detach();

        Self {
            editor,
            workspace,
            project,
            channel_store,
            channel_buffer,
            remote_id: None,
            _editor_event_subscription,
            _reparse_subscription: None,
        }
    }

    fn focus_position_from_link(
        &mut self,
        position: String,
        first_attempt: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let position = Channel::slug(&position).to_lowercase();
        let snapshot = self.editor.update(cx, |editor, cx| editor.snapshot(cx));

        if let Some(outline) = snapshot.buffer_snapshot.outline(None) {
            if let Some(item) = outline
                .items
                .iter()
                .find(|item| &Channel::slug(&item.text).to_lowercase() == &position)
            {
                self.editor.update(cx, |editor, cx| {
                    editor.change_selections(Some(Autoscroll::focused()), cx, |s| {
                        s.replace_cursors_with(|map| vec![item.range.start.to_display_point(&map)])
                    })
                });
                return;
            }
        }

        if !first_attempt {
            return;
        }
        self._reparse_subscription = Some(cx.subscribe(
            &self.editor,
            move |this, _, e: &EditorEvent, cx| {
                match e {
                    EditorEvent::Reparsed => {
                        this.focus_position_from_link(position.clone(), false, cx);
                        this._reparse_subscription.take();
                    }
                    EditorEvent::Edited | EditorEvent::SelectionsChanged { local: true } => {
                        this._reparse_subscription.take();
                    }
                    _ => {}
                };
            },
        ));
    }

    fn copy_link(&mut self, _: &CopyLink, cx: &mut ViewContext<Self>) {
        let position = self
            .editor
            .update(cx, |editor, cx| editor.selections.newest_display(cx).start);
        self.copy_link_for_position(position, cx)
    }

    fn copy_link_for_position(&self, position: DisplayPoint, cx: &mut ViewContext<Self>) {
        let snapshot = self.editor.update(cx, |editor, cx| editor.snapshot(cx));

        let mut closest_heading = None;

        if let Some(outline) = snapshot.buffer_snapshot.outline(None) {
            for item in outline.items {
                if item.range.start.to_display_point(&snapshot) > position {
                    break;
                }
                closest_heading = Some(item);
            }
        }

        let Some(channel) = self.channel(cx) else {
            return;
        };

        let link = channel.notes_link(closest_heading.map(|heading| heading.text), cx);
        cx.write_to_clipboard(ClipboardItem::new(link));
        self.workspace
            .update(cx, |workspace, cx| {
                struct CopyLinkForPositionToast;

                workspace.show_toast(
                    Toast::new(
                        NotificationId::unique::<CopyLinkForPositionToast>(),
                        "Link copied to clipboard",
                    ),
                    cx,
                );
            })
            .ok();
    }

    pub fn channel(&self, cx: &AppContext) -> Option<Arc<Channel>> {
        self.channel_buffer.read(cx).channel(cx)
    }

    fn handle_channel_buffer_event(
        &mut self,
        _: Model<ChannelBuffer>,
        event: &ChannelBufferEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ChannelBufferEvent::Disconnected => self.editor.update(cx, |editor, cx| {
                editor.set_read_only(true);
                cx.notify();
            }),
            ChannelBufferEvent::ChannelChanged => {
                self.editor.update(cx, |_, cx| {
                    cx.emit(editor::EditorEvent::TitleChanged);
                    cx.notify()
                });
            }
            ChannelBufferEvent::BufferEdited => {
                if self.editor.read(cx).is_focused(cx) {
                    self.acknowledge_buffer_version(cx);
                } else {
                    self.channel_store.update(cx, |store, cx| {
                        let channel_buffer = self.channel_buffer.read(cx);
                        store.update_latest_notes_version(
                            channel_buffer.channel_id,
                            channel_buffer.epoch(),
                            &channel_buffer.buffer().read(cx).version(),
                            cx,
                        )
                    });
                }
            }
            ChannelBufferEvent::CollaboratorsChanged => {}
        }
    }

    fn acknowledge_buffer_version(&mut self, cx: &mut ViewContext<ChannelView>) {
        self.channel_store.update(cx, |store, cx| {
            let channel_buffer = self.channel_buffer.read(cx);
            store.acknowledge_notes_version(
                channel_buffer.channel_id,
                channel_buffer.epoch(),
                &channel_buffer.buffer().read(cx).version(),
                cx,
            )
        });
        self.channel_buffer.update(cx, |buffer, cx| {
            buffer.acknowledge_buffer_version(cx);
        });
    }
}

impl EventEmitter<EditorEvent> for ChannelView {}

impl Render for ChannelView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .on_action(cx.listener(Self::copy_link))
            .child(self.editor.clone())
    }
}

impl FocusableView for ChannelView {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.editor.read(cx).focus_handle(cx)
    }
}

impl Item for ChannelView {
    type Event = EditorEvent;

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a View<Self>,
        _: &'a AppContext,
    ) -> Option<AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.to_any())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.to_any())
        } else {
            None
        }
    }

    fn tab_content(&self, params: TabContentParams, cx: &WindowContext) -> AnyElement {
        let label = if let Some(channel) = self.channel(cx) {
            match (
                self.channel_buffer.read(cx).buffer().read(cx).read_only(),
                self.channel_buffer.read(cx).is_connected(),
            ) {
                (false, true) => format!("#{}", channel.name),
                (true, true) => format!("#{} (read-only)", channel.name),
                (_, false) => format!("#{} (disconnected)", channel.name),
            }
        } else {
            "channel notes (disconnected)".to_string()
        };
        Label::new(label)
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn clone_on_split(
        &self,
        _: Option<WorkspaceId>,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>> {
        Some(cx.new_view(|cx| {
            Self::new(
                self.project.clone(),
                self.workspace.clone(),
                self.channel_store.clone(),
                self.channel_buffer.clone(),
                cx,
            )
        }))
    }

    fn is_singleton(&self, _cx: &AppContext) -> bool {
        false
    }

    fn navigate(&mut self, data: Box<dyn Any>, cx: &mut ViewContext<Self>) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, cx))
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| Item::deactivated(editor, cx))
    }

    fn set_nav_history(&mut self, history: ItemNavHistory, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| Item::set_nav_history(editor, history, cx))
    }

    fn as_searchable(&self, _: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn show_toolbar(&self) -> bool {
        true
    }

    fn pixel_position_of_cursor(&self, cx: &AppContext) -> Option<Point<Pixels>> {
        self.editor.read(cx).pixel_position_of_cursor(cx)
    }

    fn to_item_events(event: &EditorEvent, f: impl FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }
}

impl FollowableItem for ChannelView {
    fn remote_id(&self) -> Option<workspace::ViewId> {
        self.remote_id
    }

    fn to_state_proto(&self, cx: &WindowContext) -> Option<proto::view::Variant> {
        let channel_buffer = self.channel_buffer.read(cx);
        if !channel_buffer.is_connected() {
            return None;
        }

        Some(proto::view::Variant::ChannelView(
            proto::view::ChannelView {
                channel_id: channel_buffer.channel_id.0,
                editor: if let Some(proto::view::Variant::Editor(proto)) =
                    self.editor.read(cx).to_state_proto(cx)
                {
                    Some(proto)
                } else {
                    None
                },
            },
        ))
    }

    fn from_state_proto(
        pane: View<workspace::Pane>,
        workspace: View<workspace::Workspace>,
        remote_id: workspace::ViewId,
        state: &mut Option<proto::view::Variant>,
        cx: &mut WindowContext,
    ) -> Option<gpui::Task<anyhow::Result<View<Self>>>> {
        let Some(proto::view::Variant::ChannelView(_)) = state else {
            return None;
        };
        let Some(proto::view::Variant::ChannelView(state)) = state.take() else {
            unreachable!()
        };

        let open =
            ChannelView::open_in_pane(ChannelId(state.channel_id), None, pane, workspace, cx);

        Some(cx.spawn(|mut cx| async move {
            let this = open.await?;

            let task = this.update(&mut cx, |this, cx| {
                this.remote_id = Some(remote_id);

                if let Some(state) = state.editor {
                    Some(this.editor.update(cx, |editor, cx| {
                        editor.apply_update_proto(
                            &this.project,
                            proto::update_view::Variant::Editor(proto::update_view::Editor {
                                selections: state.selections,
                                pending_selection: state.pending_selection,
                                scroll_top_anchor: state.scroll_top_anchor,
                                scroll_x: state.scroll_x,
                                scroll_y: state.scroll_y,
                                ..Default::default()
                            }),
                            cx,
                        )
                    }))
                } else {
                    None
                }
            })?;

            if let Some(task) = task {
                task.await?;
            }

            Ok(this)
        }))
    }

    fn add_event_to_update_proto(
        &self,
        event: &EditorEvent,
        update: &mut Option<proto::update_view::Variant>,
        cx: &WindowContext,
    ) -> bool {
        self.editor
            .read(cx)
            .add_event_to_update_proto(event, update, cx)
    }

    fn apply_update_proto(
        &mut self,
        project: &Model<Project>,
        message: proto::update_view::Variant,
        cx: &mut ViewContext<Self>,
    ) -> gpui::Task<anyhow::Result<()>> {
        self.editor.update(cx, |editor, cx| {
            editor.apply_update_proto(project, message, cx)
        })
    }

    fn set_leader_peer_id(&mut self, leader_peer_id: Option<PeerId>, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.set_leader_peer_id(leader_peer_id, cx)
        })
    }

    fn is_project_item(&self, _cx: &WindowContext) -> bool {
        false
    }

    fn to_follow_event(event: &Self::Event) -> Option<workspace::item::FollowEvent> {
        Editor::to_follow_event(event)
    }
}

struct ChannelBufferCollaborationHub(Model<ChannelBuffer>);

impl CollaborationHub for ChannelBufferCollaborationHub {
    fn collaborators<'a>(&self, cx: &'a AppContext) -> &'a HashMap<PeerId, Collaborator> {
        self.0.read(cx).collaborators()
    }

    fn user_participant_indices<'a>(
        &self,
        cx: &'a AppContext,
    ) -> &'a HashMap<u64, ParticipantIndex> {
        self.0.read(cx).user_store().read(cx).participant_indices()
    }

    fn user_names(&self, cx: &AppContext) -> HashMap<u64, SharedString> {
        let user_ids = self.collaborators(cx).values().map(|c| c.user_id);
        self.0
            .read(cx)
            .user_store()
            .read(cx)
            .participant_names(user_ids, cx)
    }
}

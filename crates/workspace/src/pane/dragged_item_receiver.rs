use super::DraggedItem;
use crate::{Pane, SplitDirection, Workspace};
use gpui::{
    color::Color,
    elements::{Canvas, MouseEventHandler, ParentComponent, Stack},
    geometry::{rect::RectF, vector::Vector2F},
    platform::MouseButton,
    scene::MouseUp,
    AppContext, Element, EventContext, MouseState, Quad, ViewContext, WeakViewHandle,
};
use project2::ProjectEntryId;

pub fn dragged_item_receiver<Tag, D, F>(
    pane: &Pane,
    region_id: usize,
    drop_index: usize,
    allow_same_pane: bool,
    split_margin: Option<f32>,
    cx: &mut ViewContext<Pane>,
    render_child: F,
) -> MouseEventHandler<Pane>
where
    Tag: 'static,
    D: Element<Pane>,
    F: FnOnce(&mut MouseState, &mut ViewContext<Pane>) -> D,
{
    let drag_and_drop = cx.global::<DragAndDrop<Workspace>>();
    let drag_position = if (pane.can_drop)(drag_and_drop, cx) {
        drag_and_drop
            .currently_dragged::<DraggedItem>(cx.window())
            .map(|(drag_position, _)| drag_position)
            .or_else(|| {
                drag_and_drop
                    .currently_dragged::<ProjectEntryId>(cx.window())
                    .map(|(drag_position, _)| drag_position)
            })
    } else {
        None
    };

    let mut handler = MouseEventHandler::above::<Tag, _>(region_id, cx, |state, cx| {
        // Observing hovered will cause a render when the mouse enters regardless
        // of if mouse position was accessed before
        let drag_position = if state.dragging() {
            drag_position
        } else {
            None
        };
        Stack::new()
            .with_child(render_child(state, cx))
            .with_children(drag_position.map(|drag_position| {
                Canvas::new(move |bounds, _, _, cx| {
                    if bounds.contains_point(drag_position) {
                        let overlay_region = split_margin
                            .and_then(|split_margin| {
                                drop_split_direction(drag_position, bounds, split_margin)
                                    .map(|dir| (dir, split_margin))
                            })
                            .map(|(dir, margin)| dir.along_edge(bounds, margin))
                            .unwrap_or(bounds);

                        cx.scene().push_stacking_context(None, None);
                        let background = overlay_color(cx);
                        cx.scene().push_quad(Quad {
                            bounds: overlay_region,
                            background: Some(background),
                            border: Default::default(),
                            corner_radii: Default::default(),
                        });
                        cx.scene().pop_stacking_context();
                    }
                })
            }))
    });

    if drag_position.is_some() {
        handler = handler
            .on_up(MouseButton::Left, {
                move |event, pane, cx| {
                    let workspace = pane.workspace.clone();
                    let pane = cx.weak_handle();
                    handle_dropped_item(
                        event,
                        workspace,
                        &pane,
                        drop_index,
                        allow_same_pane,
                        split_margin,
                        cx,
                    );
                    cx.notify();
                }
            })
            .on_move(|_, _, cx| {
                let drag_and_drop = cx.global::<DragAndDrop<Workspace>>();

                if drag_and_drop
                    .currently_dragged::<DraggedItem>(cx.window())
                    .is_some()
                    || drag_and_drop
                        .currently_dragged::<ProjectEntryId>(cx.window())
                        .is_some()
                {
                    cx.notify();
                } else {
                    cx.propagate_event();
                }
            })
    }

    handler
}

pub fn handle_dropped_item<V: 'static>(
    event: MouseUp,
    workspace: WeakViewHandle<Workspace>,
    pane: &WeakViewHandle<Pane>,
    index: usize,
    allow_same_pane: bool,
    split_margin: Option<f32>,
    cx: &mut EventContext<V>,
) {
    enum Action {
        Move(WeakViewHandle<Pane>, usize),
        Open(ProjectEntryId),
    }
    let drag_and_drop = cx.global::<DragAndDrop<Workspace>>();
    let action = if let Some((_, dragged_item)) =
        drag_and_drop.currently_dragged::<DraggedItem>(cx.window())
    {
        Action::Move(dragged_item.pane.clone(), dragged_item.handle.id())
    } else if let Some((_, project_entry)) =
        drag_and_drop.currently_dragged::<ProjectEntryId>(cx.window())
    {
        Action::Open(*project_entry)
    } else {
        cx.propagate_event();
        return;
    };

    if let Some(split_direction) =
        split_margin.and_then(|margin| drop_split_direction(event.position, event.region, margin))
    {
        let pane_to_split = pane.clone();
        match action {
            Action::Move(from, item_id_to_move) => {
                cx.window_context().defer(move |cx| {
                    if let Some(workspace) = workspace.upgrade(cx) {
                        workspace.update(cx, |workspace, cx| {
                            workspace.split_pane_with_item(
                                pane_to_split,
                                split_direction,
                                from,
                                item_id_to_move,
                                cx,
                            );
                        })
                    }
                });
            }
            Action::Open(project_entry) => {
                cx.window_context().defer(move |cx| {
                    if let Some(workspace) = workspace.upgrade(cx) {
                        workspace.update(cx, |workspace, cx| {
                            if let Some(task) = workspace.split_pane_with_project_entry(
                                pane_to_split,
                                split_direction,
                                project_entry,
                                cx,
                            ) {
                                task.detach_and_log_err(cx);
                            }
                        })
                    }
                });
            }
        };
    } else {
        match action {
            Action::Move(from, item_id) => {
                if pane != &from || allow_same_pane {
                    let pane = pane.clone();
                    cx.window_context().defer(move |cx| {
                        if let Some(((workspace, from), to)) = workspace
                            .upgrade(cx)
                            .zip(from.upgrade(cx))
                            .zip(pane.upgrade(cx))
                        {
                            workspace.update(cx, |workspace, cx| {
                                workspace.move_item(from, to, item_id, index, cx);
                            })
                        }
                    });
                } else {
                    cx.propagate_event();
                }
            }
            Action::Open(project_entry) => {
                let pane = pane.clone();
                cx.window_context().defer(move |cx| {
                    if let Some(workspace) = workspace.upgrade(cx) {
                        workspace.update(cx, |workspace, cx| {
                            if let Some(path) =
                                workspace.project.read(cx).path_for_entry(project_entry, cx)
                            {
                                workspace
                                    .open_path(path, Some(pane), true, cx)
                                    .detach_and_log_err(cx);
                            }
                        });
                    }
                });
            }
        }
    }
}

fn drop_split_direction(
    position: Vector2F,
    region: RectF,
    split_margin: f32,
) -> Option<SplitDirection> {
    let mut min_direction = None;
    let mut min_distance = split_margin;
    for direction in SplitDirection::all() {
        let edge_distance = (direction.edge(region) - direction.axis().component(position)).abs();

        if edge_distance < min_distance {
            min_direction = Some(direction);
            min_distance = edge_distance;
        }
    }

    min_direction
}

fn overlay_color(cx: &AppContext) -> Color {
    theme2::current(cx).workspace.drop_target_overlay_color
}

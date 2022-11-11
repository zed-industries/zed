use drag_and_drop::DragAndDrop;
use gpui::{
    color::Color,
    elements::{Canvas, MouseEventHandler, ParentElement, Stack},
    geometry::{rect::RectF, vector::Vector2F},
    scene::MouseUp,
    AppContext, Element, ElementBox, EventContext, MouseButton, MouseState, Quad, RenderContext,
    WeakViewHandle,
};
use project::ProjectEntryId;
use settings::Settings;

use crate::{
    MoveItem, OpenProjectEntryInPane, Pane, SplitDirection, SplitWithItem, SplitWithProjectEntry,
    Workspace,
};

use super::DraggedItem;

pub fn dragged_item_receiver<Tag, F>(
    region_id: usize,
    drop_index: usize,
    allow_same_pane: bool,
    split_margin: Option<f32>,
    cx: &mut RenderContext<Pane>,
    render_child: F,
) -> MouseEventHandler<Tag>
where
    Tag: 'static,
    F: FnOnce(&mut MouseState, &mut RenderContext<Pane>) -> ElementBox,
{
    MouseEventHandler::<Tag>::above(region_id, cx, |state, cx| {
        // Observing hovered will cause a render when the mouse enters regardless
        // of if mouse position was accessed before
        let drag_position = if state.hovered() {
            cx.global::<DragAndDrop<Workspace>>()
                .currently_dragged::<DraggedItem>(cx.window_id())
                .map(|(drag_position, _)| drag_position)
                .or_else(|| {
                    cx.global::<DragAndDrop<Workspace>>()
                        .currently_dragged::<ProjectEntryId>(cx.window_id())
                        .map(|(drag_position, _)| drag_position)
                })
        } else {
            None
        };

        Stack::new()
            .with_child(render_child(state, cx))
            .with_children(drag_position.map(|drag_position| {
                Canvas::new(move |bounds, _, cx| {
                    if bounds.contains_point(drag_position) {
                        let overlay_region = split_margin
                            .and_then(|split_margin| {
                                drop_split_direction(drag_position, bounds, split_margin)
                                    .map(|dir| (dir, split_margin))
                            })
                            .map(|(dir, margin)| dir.along_edge(bounds, margin))
                            .unwrap_or(bounds);

                        cx.paint_stacking_context(None, None, |cx| {
                            cx.scene.push_quad(Quad {
                                bounds: overlay_region,
                                background: Some(overlay_color(cx)),
                                border: Default::default(),
                                corner_radius: 0.,
                            });
                        });
                    }
                })
                .boxed()
            }))
            .boxed()
    })
    .on_up(MouseButton::Left, {
        let pane = cx.handle();
        move |event, cx| {
            handle_dropped_item(event, &pane, drop_index, allow_same_pane, split_margin, cx);
            cx.notify();
        }
    })
    .on_move(|_, cx| {
        let drag_and_drop = cx.global::<DragAndDrop<Workspace>>();

        if drag_and_drop
            .currently_dragged::<DraggedItem>(cx.window_id())
            .is_some()
            || drag_and_drop
                .currently_dragged::<ProjectEntryId>(cx.window_id())
                .is_some()
        {
            cx.notify();
        } else {
            cx.propagate_event();
        }
    })
}

pub fn handle_dropped_item(
    event: MouseUp,
    pane: &WeakViewHandle<Pane>,
    index: usize,
    allow_same_pane: bool,
    split_margin: Option<f32>,
    cx: &mut EventContext,
) {
    enum Action {
        Move(WeakViewHandle<Pane>, usize),
        Open(ProjectEntryId),
    }
    let drag_and_drop = cx.global::<DragAndDrop<Workspace>>();
    let action = if let Some((_, dragged_item)) =
        drag_and_drop.currently_dragged::<DraggedItem>(cx.window_id)
    {
        Action::Move(dragged_item.pane.clone(), dragged_item.item.id())
    } else if let Some((_, project_entry)) =
        drag_and_drop.currently_dragged::<ProjectEntryId>(cx.window_id)
    {
        Action::Open(*project_entry)
    } else {
        return;
    };

    if let Some(split_direction) =
        split_margin.and_then(|margin| drop_split_direction(event.position, event.region, margin))
    {
        let pane_to_split = pane.clone();
        match action {
            Action::Move(from, item_id_to_move) => cx.dispatch_action(SplitWithItem {
                from,
                item_id_to_move,
                pane_to_split,
                split_direction,
            }),
            Action::Open(project_entry) => cx.dispatch_action(SplitWithProjectEntry {
                pane_to_split,
                split_direction,
                project_entry,
            }),
        };
    } else {
        match action {
            Action::Move(from, item_id) => {
                if pane != &from || allow_same_pane {
                    cx.dispatch_action(MoveItem {
                        item_id,
                        from,
                        to: pane.clone(),
                        destination_index: index,
                    })
                } else {
                    cx.propagate_event();
                }
            }
            Action::Open(project_entry) => cx.dispatch_action(OpenProjectEntryInPane {
                pane: pane.clone(),
                project_entry,
            }),
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
    cx.global::<Settings>()
        .theme
        .workspace
        .drop_target_overlay_color
}

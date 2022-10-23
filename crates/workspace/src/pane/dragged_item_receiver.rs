use drag_and_drop::DragAndDrop;
use gpui::{
    color::Color,
    elements::{Canvas, MouseEventHandler, ParentElement, Stack},
    geometry::{rect::RectF, vector::Vector2F},
    scene::MouseUp,
    AppContext, Element, ElementBox, EventContext, MouseButton, MouseState, Quad, RenderContext,
    WeakViewHandle,
};
use settings::Settings;

use crate::{MoveItem, Pane, SplitDirection, SplitWithItem, Workspace};

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
        let hovered = state.hovered();
        let drag_position = cx
            .global::<DragAndDrop<Workspace>>()
            .currently_dragged::<DraggedItem>(cx.window_id())
            .filter(|_| hovered)
            .map(|(drag_position, _)| drag_position);

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

                        cx.paint_stacking_context(None, |cx| {
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
        if cx
            .global::<DragAndDrop<Workspace>>()
            .currently_dragged::<DraggedItem>(cx.window_id())
            .is_some()
        {
            cx.notify();
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
    if let Some((_, dragged_item)) = cx
        .global::<DragAndDrop<Workspace>>()
        .currently_dragged::<DraggedItem>(cx.window_id)
    {
        if let Some(split_direction) = split_margin
            .and_then(|margin| drop_split_direction(event.position, event.region, margin))
        {
            cx.dispatch_action(SplitWithItem {
                from: dragged_item.pane.clone(),
                item_id_to_move: dragged_item.item.id(),
                pane_to_split: pane.clone(),
                split_direction,
            });
        } else if pane != &dragged_item.pane || allow_same_pane {
            // If no split margin or not close enough to the edge, just move the item
            cx.dispatch_action(MoveItem {
                item_id: dragged_item.item.id(),
                from: dragged_item.pane.clone(),
                to: pane.clone(),
                destination_index: index,
            })
        }
    } else {
        cx.propagate_event();
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

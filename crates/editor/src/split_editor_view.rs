use gpui::{
    AnyElement, App, AppContext, Context, DragMoveEvent, Entity, Hsla, IntoElement, Pixels,
    Window, div, px,
};
use theme::ActiveTheme;
use ui::{prelude::*, h_flex};

use crate::{EditorStyle, element::{EditorElement, SplitSide}, split::SplittableEditor};

const RESIZE_HANDLE_WIDTH: f32 = 8.0;

#[derive(Debug, Clone)]
struct DraggedSplitHandle;

pub struct SplitEditorState {
    left_ratio: f32,
    visible_left_ratio: f32,
    cached_width: Pixels,
}

impl SplitEditorState {
    pub fn new(_cx: &mut App) -> Self {
        Self {
            left_ratio: 0.5,
            visible_left_ratio: 0.5,
            cached_width: px(0.),
        }
    }

    pub fn left_ratio(&self) -> f32 {
        self.visible_left_ratio
    }

    pub fn right_ratio(&self) -> f32 {
        1.0 - self.visible_left_ratio
    }

    fn on_drag_move(
        &mut self,
        drag_event: &DragMoveEvent<DraggedSplitHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        let drag_position = drag_event.event.position;
        let bounds = drag_event.bounds;
        let bounds_width = bounds.right() - bounds.left();

        if bounds_width > px(0.) {
            self.cached_width = bounds_width;
        }

        let min_ratio = 0.1;
        let max_ratio = 0.9;

        let new_ratio = (drag_position.x - bounds.left()) / bounds_width;
        self.visible_left_ratio = new_ratio.clamp(min_ratio, max_ratio);
    }

    fn commit_ratio(&mut self) {
        self.left_ratio = self.visible_left_ratio;
    }

    fn on_double_click(&mut self) {
        self.left_ratio = 0.5;
        self.visible_left_ratio = 0.5;
    }
}

#[derive(IntoElement)]
pub struct SplitEditorView {
    splittable_editor: Entity<SplittableEditor>,
    style: EditorStyle,
    split_state: Entity<SplitEditorState>,
}

impl SplitEditorView {
    pub fn new(
        splittable_editor: Entity<SplittableEditor>,
        style: EditorStyle,
        split_state: Entity<SplitEditorState>,
    ) -> Self {
        Self {
            splittable_editor,
            style,
            split_state,
        }
    }
}

fn render_resize_handle(
    state: &Entity<SplitEditorState>,
    handle_color: Hsla,
    handle_hover_color: Hsla,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let state_for_click = state.clone();

    let hovered = window.use_state(cx, |_, _| false);
    let is_hovered = *hovered.read(cx);

    let divider_color = if is_hovered {
        handle_hover_color
    } else {
        handle_color
    };

    div()
        .id("split-resize-container")
        .relative()
        .h_full()
        .w(px(1.))
        .bg(divider_color)
        .child(
            div()
                .id("split-resize-handle")
                .absolute()
                .left(px(-RESIZE_HANDLE_WIDTH / 2.0))
                .w(px(RESIZE_HANDLE_WIDTH))
                .h_full()
                .cursor_col_resize()
                .on_hover(move |&was_hovered, _, cx| hovered.write(cx, was_hovered))
                .on_click(move |event, _, cx| {
                    if event.click_count() >= 2 {
                        state_for_click.update(cx, |state, _| {
                            state.on_double_click();
                        });
                    }
                    cx.stop_propagation();
                })
                .on_drag(DraggedSplitHandle, |_, _, _, cx| {
                    cx.new(|_| gpui::Empty)
                }),
        )
        .into_any_element()
}

impl RenderOnce for SplitEditorView {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let splittable_editor = self.splittable_editor.read(cx);

        let lhs_editor = splittable_editor
            .secondary_editor()
            .expect("SplitEditorView requires a secondary editor to be present")
            .clone();

        let rhs_editor = splittable_editor.primary_editor().clone();

        let mut lhs = EditorElement::new(&lhs_editor, self.style.clone());
        let mut rhs = EditorElement::new(&rhs_editor, self.style);

        lhs.set_split_side(SplitSide::Left);
        rhs.set_split_side(SplitSide::Right);

        let left_ratio = self.split_state.read(cx).left_ratio();
        let right_ratio = self.split_state.read(cx).right_ratio();

        let handle_color = cx.theme().colors().border;
        let handle_hover_color = cx.theme().colors().border_focused;

        let resize_handle = render_resize_handle(
            &self.split_state,
            handle_color,
            handle_hover_color,
            window,
            cx,
        );

        let state_for_drag = self.split_state.downgrade();
        let state_for_drop = self.split_state.downgrade();

        h_flex()
            .id("split-editor-view")
            .size_full()
            .on_drag_move::<DraggedSplitHandle>(move |event, window, cx| {
                state_for_drag
                    .update(cx, |state, cx| {
                        state.on_drag_move(event, window, cx);
                    })
                    .ok();
            })
            .on_drop::<DraggedSplitHandle>(move |_, _, cx| {
                state_for_drop
                    .update(cx, |state, _| {
                        state.commit_ratio();
                    })
                    .ok();
            })
            .child(
                div()
                    .id("split-editor-left")
                    .flex_shrink_0()
                    .h_full()
                    .w(DefiniteLength::Fraction(left_ratio))
                    .overflow_hidden()
                    .child(lhs),
            )
            .child(resize_handle)
            .child(
                div()
                    .id("split-editor-right")
                    .flex_shrink_0()
                    .h_full()
                    .w(DefiniteLength::Fraction(right_ratio))
                    .overflow_hidden()
                    .child(rhs),
            )
    }
}
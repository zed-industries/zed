use super::{
    CursorLayout, EditorElement, EditorLayout, EditorPaintPhase, EditorRequestLayoutState,
};
use crate::{Editor, EditorSettings};
use gpui::{
    AnyElement, App, AppContext as _, Bounds, ContentMask, Context, Element, ElementId, Empty,
    Entity, EntityId, GlobalElementId, InspectorElementId, InteractiveElement, IntoElement,
    LayoutId, ParentElement, Pixels, Point, Render, Style, StyleRefinement, Styled, Subscription,
    TextStyleRefinement, View, ViewElement, WeakEntity, Window, canvas, div, relative,
};
use settings::Settings;
use smallvec::SmallVec;
use std::{mem, time::Instant};

pub(crate) struct EditorContent {
    editor: WeakEntity<Editor>,
    cursor_layer: Entity<CursorLayer>,
    layout: Option<(GlobalElementId, EditorLayout)>,
    _subscription: Subscription,
    #[cfg(test)]
    pub(crate) render_counts: [usize; 3],
}

impl EditorContent {
    pub(crate) fn new(editor: &Entity<Editor>, cx: &mut Context<Self>) -> Self {
        Self {
            editor: editor.downgrade(),
            cursor_layer: cx.new(|_| CursorLayer {
                editor: editor.downgrade(),
                cursors: Vec::new(),
                origin: Point::default(),
                mask: ContentMask::default(),
                rem_size: None,
                text_style: TextStyleRefinement::default(),
                #[cfg(test)]
                paint_count: 0,
                #[cfg(test)]
                cursor_bounds: Vec::new(),
            }),
            layout: None,
            _subscription: cx.observe(editor, |_, _, cx| cx.notify()),
            #[cfg(test)]
            render_counts: [0; 3],
        }
    }

    pub(crate) fn cursor_layer_id(&self) -> EntityId {
        self.cursor_layer.entity_id()
    }

    #[cfg(test)]
    pub(crate) fn cursor_paint_count(&self, cx: &App) -> usize {
        self.cursor_layer.read(cx).paint_count
    }

    #[cfg(test)]
    pub(crate) fn cursor_bounds(&self, cx: &App) -> Vec<Bounds<Pixels>> {
        self.cursor_layer.read(cx).cursor_bounds.clone()
    }

    pub(crate) fn render_layers(content: &Entity<Self>, cx: &App) -> AnyElement {
        let layer = |phase| {
            ViewElement::new(EditorContentView {
                content: content.clone(),
                phase,
            })
            .cached_independently(StyleRefinement::default().size_full())
        };
        div()
            .relative()
            .size_full()
            .child(
                div()
                    .id("editor-background")
                    .size_full()
                    .child(layer(EditorPaintPhase::BeforeCursor)),
            )
            .child(content.read(cx).cursor_layer.clone())
            .child(
                div()
                    .id("editor-foreground")
                    .absolute()
                    .inset_0()
                    .size_full()
                    .child(layer(EditorPaintPhase::AfterCursor)),
            )
            .into_any_element()
    }
}

struct EditorContentView {
    content: Entity<EditorContent>,
    phase: EditorPaintPhase,
}

impl View for EditorContentView {
    fn entity_id(&self) -> Option<EntityId> {
        Some(self.content.entity_id())
    }

    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(editor) = self.content.read(cx).editor.upgrade() else {
            return Empty.into_any_element();
        };
        #[cfg(test)]
        self.content
            .update(cx, |content, _| content.render_counts[0] += 1);
        let mut element = EditorElement::new(&editor, editor.read(cx).create_style(cx));
        element.paint_phase = self.phase;
        EditorLayerElement {
            content: self.content,
            element,
        }
        .into_any_element()
    }
}

struct EditorLayerElement {
    content: Entity<EditorContent>,
    element: EditorElement,
}

impl IntoElement for EditorLayerElement {
    type Element = Self;

    fn into_element(self) -> Self {
        self
    }
}

impl Element for EditorLayerElement {
    type RequestLayoutState = EditorRequestLayoutState;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(ElementId::from("editor-content"))
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        if self.element.paint_phase == EditorPaintPhase::BeforeCursor {
            self.element.request_layout(id, inspector_id, window, cx)
        } else {
            let mut style = Style::default();
            style.size.width = relative(1.0).into();
            style.size.height = relative(1.0).into();
            (
                window.request_layout(style, None, cx),
                EditorRequestLayoutState::default(),
            )
        }
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if self.element.paint_phase == EditorPaintPhase::AfterCursor {
            return;
        }
        let mut layout =
            self.element
                .prepaint(id, inspector_id, bounds, request_layout, window, cx);
        let mut mask = ContentMask {
            bounds: layout.position_map.text_hitbox.bounds.intersect(&bounds),
        };
        if let Some(sticky_mask) = layout.below_sticky_headers_mask(bounds) {
            mask = mask.intersect(&sticky_mask);
        }
        let rem_size = self.element.rem_size(cx);
        self.content.update(cx, |content, cx| {
            #[cfg(test)]
            {
                content.render_counts[1] += 1;
            }
            content.cursor_layer.update(cx, |cursor_layer, _| {
                cursor_layer.cursors = mem::take(&mut layout.visible_cursors);
                cursor_layer.origin = layout.content_origin;
                cursor_layer.mask = mask;
                cursor_layer.rem_size = rem_size;
                cursor_layer.text_style = TextStyleRefinement {
                    font_size: Some(self.element.style.text.font_size),
                    line_height: Some(self.element.style.text.line_height),
                    ..TextStyleRefinement::default()
                };
            });
            content.layout = id.cloned().map(|id| (id, layout));
        });
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.content.update(cx, |content, cx| {
            #[cfg(test)]
            {
                content.render_counts[2] += 1;
            }
            if let Some((layout_id, layout)) = &mut content.layout {
                window.with_global_element_id(layout_id, |window| {
                    self.element.paint(
                        id,
                        inspector_id,
                        bounds,
                        request_layout,
                        layout,
                        window,
                        cx,
                    );
                });
            }
            if self.element.paint_phase == EditorPaintPhase::AfterCursor {
                content.layout = None;
            }
        });
    }
}

struct CursorLayer {
    editor: WeakEntity<Editor>,
    cursors: Vec<CursorLayout>,
    origin: Point<Pixels>,
    mask: ContentMask<Pixels>,
    rem_size: Option<Pixels>,
    text_style: TextStyleRefinement,
    #[cfg(test)]
    paint_count: usize,
    #[cfg(test)]
    cursor_bounds: Vec<Bounds<Pixels>>,
}

impl Render for CursorLayer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let layer = cx.entity();
        canvas::<(SmallVec<[CursorLayout; 1]>, _, _, _, _)>(
            move |_, window, cx| {
                layer.update(cx, |layer, cx| {
                    let origin = layer.origin;
                    let editor = layer.editor.upgrade();
                    let show_local_cursors = editor.as_ref().is_some_and(|editor| {
                        editor.update(cx, |editor, cx| editor.show_local_cursors(window, cx))
                    });
                    let mut cursors = layer
                        .cursors
                        .iter()
                        .filter(|cursor| !cursor.is_local || show_local_cursors)
                        .map(|cursor| CursorLayout {
                            origin: cursor.origin,
                            block_width: cursor.block_width,
                            line_height: cursor.line_height,
                            color: cursor.color,
                            shape: cursor.shape,
                            block_text: cursor.block_text.clone(),
                            cursor_name: None,
                            animated_corners: None,
                            animation_target: cursor.animation_target,
                            name: cursor.name.clone(),
                            is_local: cursor.is_local,
                        })
                        .collect::<SmallVec<[_; 1]>>();
                    let animate = EditorSettings::get_global(cx).cursor_animation.enabled
                        && !cx.reduce_motion();
                    let mut active = false;
                    if let Some(editor) = editor {
                        editor.update(cx, |editor, _| {
                            let now = Instant::now();
                            for cursor in &mut cursors {
                                if animate
                                    && let Some((selection_id, position, viewport)) =
                                        cursor.animation_target
                                {
                                    cursor.animated_corners = editor.cursor_animations.update(
                                        selection_id,
                                        position,
                                        window.pixel_snap_bounds(cursor.bounds(origin)),
                                        viewport,
                                        now,
                                    );
                                    active |= cursor.animated_corners.is_some();
                                }
                            }
                            editor.cursor_animations.capture_newest_state();
                            if !show_local_cursors {
                                editor.cursor_animations.retain(|_| false);
                            }
                        });
                    }
                    window.with_rem_size(layer.rem_size, |window| {
                        window.with_text_style(Some(layer.text_style.clone()), |window| {
                            window.with_content_mask(Some(layer.mask), |window| {
                                for cursor in &mut cursors {
                                    cursor.layout(origin, cursor.name.clone(), window, cx);
                                }
                            });
                        });
                    });
                    if active {
                        window.request_animation_frame();
                    }
                    (
                        cursors,
                        origin,
                        layer.mask,
                        layer.rem_size,
                        layer.text_style.clone(),
                    )
                })
            },
            {
                #[cfg(test)]
                let layer = cx.entity();
                move |_, (mut cursors, origin, mask, rem_size, text_style), window, cx| {
                    window.with_rem_size(rem_size, |window| {
                        window.with_text_style(Some(text_style), |window| {
                            window.with_content_mask(Some(mask), |window| {
                                for cursor in &mut cursors {
                                    cursor.paint(origin, window, cx);
                                }
                            });
                        });
                    });
                    #[cfg(test)]
                    layer.update(cx, |layer, _| {
                        layer.paint_count += 1;
                        layer.cursor_bounds = cursors
                            .iter()
                            .map(|cursor| window.pixel_snap_bounds(cursor.bounds(origin)))
                            .collect();
                    });
                }
            },
        )
        .absolute()
        .inset_0()
        .size_full()
    }
}

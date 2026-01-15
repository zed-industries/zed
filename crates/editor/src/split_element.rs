use gpui::{
    AnyElement, App, AvailableSpace, Background, Bounds, Context, Corners, CursorStyle, Edges,
    Element, Entity, Hitbox, HitboxBehavior, IntoElement, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, PaintQuad, Pixels, Point, Rgba, Size, Style, hsla, px, relative, rgba,
};
use theme::ActiveTheme;

use crate::{
    Editor, EditorElement, SplittableEditor,
    element::{SplitInfo, SplitSide},
};

pub struct SplitEditorElement {
    editor: Entity<SplittableEditor>,
    lhs: Entity<Editor>,
    rhs: Entity<Editor>,
}

const SEPARATOR_WIDTH: Pixels = px(1.0);
const SEPARATOR_HITBOX_WIDTH: Pixels = px(8.0);

impl SplitEditorElement {
    pub fn new(
        rhs: &Entity<Editor>,
        lhs: &Entity<Editor>,
        cx: &mut Context<SplittableEditor>,
    ) -> Self {
        Self {
            editor: cx.entity(),
            lhs: lhs.clone(),
            rhs: rhs.clone(),
        }
    }

    fn lhs_bounds(&self, lhs_width: Pixels, bounds: Bounds<Pixels>) -> Bounds<Pixels> {
        Bounds {
            origin: bounds.origin,
            size: Size {
                width: lhs_width,
                height: bounds.size.height,
            },
        }
    }

    fn rhs_bounds(&self, lhs_width: Pixels, bounds: Bounds<Pixels>) -> Bounds<Pixels> {
        Bounds {
            origin: Point {
                x: bounds.origin.x + lhs_width + SEPARATOR_WIDTH,
                y: bounds.origin.y,
            },
            size: Size {
                width: bounds.size.width - lhs_width - SEPARATOR_WIDTH,
                height: bounds.size.height,
            },
        }
    }

    fn separator_bounds(&self, lhs_width: Pixels, bounds: Bounds<Pixels>) -> Bounds<Pixels> {
        Bounds {
            origin: Point {
                x: bounds.origin.x + lhs_width,
                y: bounds.origin.y,
            },
            size: Size {
                width: SEPARATOR_WIDTH,
                height: bounds.size.height,
            },
        }
    }

    fn separator_hitbox_bounds(&self, lhs_width: Pixels, bounds: Bounds<Pixels>) -> Bounds<Pixels> {
        let hitbox_left =
            bounds.origin.x + lhs_width - (SEPARATOR_HITBOX_WIDTH - SEPARATOR_WIDTH) / 2.0;
        Bounds {
            origin: Point {
                x: hitbox_left,
                y: bounds.origin.y,
            },
            size: Size {
                width: SEPARATOR_HITBOX_WIDTH,
                height: bounds.size.height,
            },
        }
    }
}

pub struct SplitEditorRequestLayoutState {
    lhs_bounds: Bounds<Pixels>,
    rhs_bounds: Bounds<Pixels>,
    separator_bounds: Bounds<Pixels>,
    separator_hitbox_bounds: Bounds<Pixels>,
}

pub struct SplitEditorPrepaintState {
    lhs_element: AnyElement,
    rhs_element: AnyElement,
    separator_bounds: Bounds<Pixels>,
    separator_hitbox: Hitbox,
}

impl IntoElement for SplitEditorElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for SplitEditorElement {
    type RequestLayoutState = SplitEditorRequestLayoutState;

    type PrepaintState = SplitEditorPrepaintState;

    fn id(&self) -> Option<ui::ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut ui::Window,
        cx: &mut ui::App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = relative(1.).into();

        let id = window.request_layout(style, [], cx);
        let state = SplitEditorRequestLayoutState {
            lhs_bounds: Bounds::default(),
            rhs_bounds: Bounds::default(),
            separator_bounds: Bounds::default(),
            separator_hitbox_bounds: Bounds::default(),
        };

        (id, state)
    }

    fn prepaint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: gpui::Bounds<ui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut ui::Window,
        cx: &mut ui::App,
    ) -> Self::PrepaintState {
        let split_ratio = self.editor.read(cx).split_ratio();
        let lhs_width = (bounds.size.width - SEPARATOR_WIDTH) * split_ratio;

        let lhs_bounds = self.lhs_bounds(lhs_width, bounds);
        let rhs_bounds = self.rhs_bounds(lhs_width, bounds);
        let separator_bounds = self.separator_bounds(lhs_width, bounds);
        let separator_hitbox_bounds = self.separator_hitbox_bounds(lhs_width, bounds);

        request_layout.lhs_bounds = lhs_bounds;
        request_layout.rhs_bounds = rhs_bounds;
        request_layout.separator_bounds = separator_bounds;
        request_layout.separator_hitbox_bounds = separator_hitbox_bounds;

        let separator_hitbox =
            window.insert_hitbox(separator_hitbox_bounds, HitboxBehavior::Normal);

        let lhs_style = self.lhs.update(cx, |editor, cx| editor.style(cx).clone());
        let rhs_style = self.rhs.update(cx, |editor, cx| editor.style(cx).clone());

        let mut lhs_element = EditorElement::new(&self.lhs, lhs_style);
        let mut rhs_element = EditorElement::new(&self.rhs, rhs_style);

        lhs_element.set_split_info(SplitInfo {
            bounds,
            side: SplitSide::Left,
        });
        rhs_element.set_split_info(SplitInfo {
            bounds,
            side: SplitSide::Right,
        });

        let mut lhs_any = lhs_element.into_any_element();
        let lhs_available_space = Size {
            width: AvailableSpace::Definite(lhs_bounds.size.width),
            height: AvailableSpace::Definite(lhs_bounds.size.height),
        };
        lhs_any.layout_as_root(lhs_available_space, window, cx);
        lhs_any.prepaint_at(lhs_bounds.origin, window, cx);

        let mut rhs_any = rhs_element.into_any_element();
        let rhs_available_space = Size {
            width: AvailableSpace::Definite(rhs_bounds.size.width),
            height: AvailableSpace::Definite(rhs_bounds.size.height),
        };
        rhs_any.layout_as_root(rhs_available_space, window, cx);
        rhs_any.prepaint_at(rhs_bounds.origin, window, cx);

        SplitEditorPrepaintState {
            lhs_element: lhs_any,
            rhs_element: rhs_any,
            separator_bounds,
            separator_hitbox,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: gpui::Bounds<ui::Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut ui::Window,
        cx: &mut ui::App,
    ) {
        prepaint.lhs_element.paint(window, cx);

        window.paint_quad(PaintQuad {
            background: Background::from(cx.theme().colors().border),
            border_color: hsla(0.0, 0.0, 0.0, 0.0),
            border_style: gpui::BorderStyle::Solid,
            border_widths: gpui::Edges::default(),
            bounds: prepaint.separator_bounds,
            corner_radii: Corners::default(),
        });

        prepaint.rhs_element.paint(window, cx);

        window.set_cursor_style(CursorStyle::ResizeLeftRight, &prepaint.separator_hitbox);

        let editor = self.editor.clone();
        window.on_mouse_event({
            let separator_hitbox = prepaint.separator_hitbox.clone();
            move |_event: &MouseDownEvent, phase, window, cx| {
                if phase.bubble() && separator_hitbox.is_hovered(window) {
                    editor.update(cx, |editor, _cx| {
                        editor.set_dragging_divider(true);
                    });
                    cx.stop_propagation();
                }
            }
        });

        window.on_mouse_event({
            let editor = self.editor.clone();
            move |_event: &MouseUpEvent, phase, _window, cx| {
                if phase.bubble() {
                    editor.update(cx, |editor, _cx| {
                        editor.set_dragging_divider(false);
                    });
                }
            }
        });

        window.on_mouse_event({
            let editor = self.editor.clone();
            move |event: &MouseMoveEvent, phase, _window, cx| {
                if phase.bubble() {
                    editor.update(cx, |editor, cx| {
                        if editor.is_dragging_divider() {
                            let bounds_width: f32 = bounds.size.width.into();
                            if bounds_width <= 0.0 {
                                return;
                            }
                            let bounds_left: f32 = bounds.origin.x.into();
                            let mouse_x: f32 = event.position.x.into();
                            let relative_x = mouse_x - bounds_left;
                            let new_ratio = (relative_x / bounds_width).clamp(0.1, 0.9);
                            editor.set_split_ratio(new_ratio, cx);
                        }
                    });
                }
            }
        });
    }
}

pub struct CheckerboardElement {
    width: Pixels,
    height: Pixels,
    light: Rgba,
    dark: Rgba,
}

impl Default for CheckerboardElement {
    fn default() -> Self {
        Self {
            width: px(10.),
            height: px(10.),
            light: rgba(0xFFFFFF10),
            dark: rgba(0x00000000),
        }
    }
}

impl CheckerboardElement {
    pub fn new(size: Pixels) -> Self {
        Self {
            width: size,
            height: size,
            ..Default::default()
        }
    }
}

impl IntoElement for CheckerboardElement {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for CheckerboardElement {
    type RequestLayoutState = ();

    type PrepaintState = ();

    fn id(&self) -> Option<ui::ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut ui::Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = relative(1.).into();

        let id = window.request_layout(style, [], cx);
        (id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut ui::Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut ui::Window,
        _cx: &mut App,
    ) {
        let columns = (bounds.size.width / self.width).ceil() as u32;
        let rows = (bounds.size.height / self.height).ceil() as u32;

        for i in 0..columns {
            for j in 0..rows {
                let color = if (i + j) % 2 == 0 {
                    self.light
                } else {
                    self.dark
                };

                let x = bounds.origin.x + (self.width * i as f32);
                let y = bounds.origin.y + (self.height * j as f32);

                let max_width = bounds.origin.x + bounds.size.width - x;
                let max_height = bounds.origin.y + bounds.size.height - y;

                let square_bounds = Bounds {
                    origin: Point { x, y },
                    size: Size {
                        width: std::cmp::min(self.width, max_width),
                        height: std::cmp::min(self.height, max_height),
                    },
                };
                window.paint_quad(gpui::PaintQuad {
                    bounds: square_bounds,
                    background: color.into(),
                    border_color: hsla(0.0, 0.0, 0.0, 0.0),
                    border_style: gpui::BorderStyle::Solid,
                    border_widths: Edges::default(),
                    corner_radii: Corners::default(),
                });
            }
        }
    }
}
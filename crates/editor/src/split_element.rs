use gpui::{
    App, Background, Bounds, ContentMask, Context, Corners, Edges, Element, ElementInputHandler,
    Entity, IntoElement, Length, PaintQuad, Pixels, Point, Rgba, Size, Style, colors::Colors, hsla,
    px, relative, rgb, rgba,
};
use multi_buffer::Anchor;
use sum_tree::Dimension;
use theme::ThemeColors;

use crate::{Editor, EditorElement, EditorMode, EditorStyle, SplittableEditor, element::SplitSide};

pub struct SplitEditorElement {
    editor: Entity<SplittableEditor>,
    lhs: EditorElement,
    rhs: EditorElement,
    style: EditorStyle, // maybe redundant?
    lhs_width: Pixels,
}

/// Dummy value used in `lhs_width` before first call to prepaint
const BEFORE_FIRST_PREPAINT: Pixels = px(-1.0);

// todo! extra large so it's more obvious when debugging
const SEPARATOR_WIDTH: Pixels = px(1.0);

impl SplitEditorElement {
    pub fn new(
        rhs: &Entity<Editor>,
        lhs: &Entity<Editor>,
        style: EditorStyle,
        cx: &mut Context<SplittableEditor>,
    ) -> Self {
        let mut lhs = EditorElement::new(lhs, style.clone());
        let mut rhs = EditorElement::new(rhs, style.clone());

        lhs.set_split_side(SplitSide::Left);
        rhs.set_split_side(SplitSide::Right);

        Self {
            editor: cx.entity(),
            lhs,
            rhs,
            style,
            lhs_width: BEFORE_FIRST_PREPAINT,
        }
    }

    fn rhs_editor(&self, cx: &App) -> Entity<Editor> {
        self.editor.read(cx).primary_editor().clone()
    }

    fn lhs_editor(&self, cx: &App) -> Entity<Editor> {
        self.editor.read(cx).secondary_editor().unwrap().clone()
    }

    fn lhs_bounds(&self, width: Pixels, bounds: Bounds<Pixels>) -> Bounds<Pixels> {
        debug_assert_ne!(self.lhs_width, BEFORE_FIRST_PREPAINT);
        Bounds {
            origin: bounds.origin,
            size: Size {
                width: width,
                height: bounds.size.height,
            },
        }
    }

    fn rhs_bounds(&self, width: Pixels, bounds: Bounds<Pixels>) -> Bounds<Pixels> {
        debug_assert_ne!(self.lhs_width, BEFORE_FIRST_PREPAINT);

        Bounds {
            origin: Point {
                x: bounds.origin.x + width + SEPARATOR_WIDTH,
                y: bounds.origin.y,
            },
            size: Size {
                width: width,
                height: bounds.size.height,
            },
        }
    }
}

pub struct SplitEditorRequestLayoutState {
    lhs: <EditorElement as Element>::RequestLayoutState,
    rhs: <EditorElement as Element>::RequestLayoutState,
}

pub struct SplitEditorPrepaintState {
    lhs: <EditorElement as Element>::PrepaintState,
    rhs: <EditorElement as Element>::PrepaintState,
    lhs_bounds: Bounds<Pixels>,
    rhs_bounds: Bounds<Pixels>,
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
        id: Option<&gpui::GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut ui::Window,
        cx: &mut ui::App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        // `EditorElement::request_layout` will later apply these styles to the
        // underlying `Editor`s
        self.lhs.set_style(self.style.clone());
        self.rhs.set_style(self.style.clone());

        let (lhs_id, lhs) = self.lhs.request_layout(id, inspector_id, window, cx);
        let (rhs_id, rhs) = self.rhs.request_layout(id, inspector_id, window, cx);

        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = relative(1.).into();

        let id = window.request_layout(style, [lhs_id, rhs_id], cx);
        let state = SplitEditorRequestLayoutState { lhs, rhs };

        (id, state)
    }

    fn prepaint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: gpui::Bounds<ui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut ui::Window,
        cx: &mut ui::App,
    ) -> Self::PrepaintState {
        if self.lhs_width == BEFORE_FIRST_PREPAINT {
            self.lhs_width = (bounds.size.width - SEPARATOR_WIDTH) / 2.0;
        }
        let lhs_width = self.lhs_width;
        let rhs_width = bounds.size.width - (SEPARATOR_WIDTH + lhs_width);

        let lhs_bounds = self.lhs_bounds(lhs_width, bounds);
        let rhs_bounds = self.rhs_bounds(rhs_width, bounds);

        // todo! id, inspector_id?
        let lhs = self.lhs.prepaint(
            id,
            inspector_id,
            lhs_bounds,
            &mut request_layout.lhs,
            window,
            cx,
        );
        self.rhs.set_split_bounds(bounds);
        let rhs = self.rhs.prepaint(
            id,
            inspector_id,
            rhs_bounds,
            &mut request_layout.rhs,
            window,
            cx,
        );

        SplitEditorPrepaintState {
            lhs,
            rhs,
            lhs_bounds,
            rhs_bounds,
        }
    }

    fn paint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: gpui::Bounds<ui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut ui::Window,
        cx: &mut ui::App,
    ) {
        // todo! id, inspector_id?
        self.lhs.paint(
            id,
            inspector_id,
            prepaint.lhs_bounds,
            &mut request_layout.lhs,
            &mut prepaint.lhs,
            window,
            cx,
        );
        self.rhs.paint(
            id,
            inspector_id,
            prepaint.rhs_bounds,
            &mut request_layout.rhs,
            &mut prepaint.rhs,
            window,
            cx,
        );

        window.paint_quad(PaintQuad {
            background: Background::from(Colors::for_appearance(window).text),
            border_color: hsla(0.0, 0.0, 0.0, 0.0),
            border_style: gpui::BorderStyle::Solid,
            border_widths: gpui::Edges::default(),
            bounds: Bounds {
                origin: Point {
                    x: bounds.origin.x + self.lhs_width,
                    y: bounds.origin.y,
                },
                size: Size {
                    width: SEPARATOR_WIDTH,
                    height: bounds.size.height,
                },
            },
            corner_radii: Corners::default(),
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
        id: Option<&gpui::GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
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
        id: Option<&gpui::GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut ui::Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut ui::Window,
        cx: &mut App,
    ) {
        // number of squares in each dimension (including partial)
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

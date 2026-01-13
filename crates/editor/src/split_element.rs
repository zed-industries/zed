use gpui::{
    App, Background, Bounds, Context, Corners, Edges, Element, ElementInputHandler, Entity, IntoElement, Length, PaintQuad, Pixels, Point, Size, Style, hsla, px, relative, rgb
};
use multi_buffer::Anchor;

use crate::{Editor, EditorElement, EditorMode, EditorStyle, SplittableEditor};

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
const SEPARATOR_WIDTH: Pixels = px(50.0);

impl SplitEditorElement {
    pub fn new(
        rhs: &Entity<Editor>,
        lhs: &Entity<Editor>,
        style: EditorStyle,
        cx: &mut Context<SplittableEditor>,
    ) -> Self {
        let mut lhs = EditorElement::new(lhs, style.clone());
        let mut rhs = EditorElement::new(rhs, style.clone());

        lhs.set_in_split();
        rhs.set_in_split();

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
        let rhs = self.rhs.prepaint(
            id,
            inspector_id,
            rhs_bounds,
            &mut request_layout.rhs,
            window,
            cx,
        );

        SplitEditorPrepaintState { lhs, rhs, lhs_bounds, rhs_bounds }
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
            background: Background::from(rgb(0x0000FF)),
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

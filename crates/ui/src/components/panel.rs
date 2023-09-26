use std::marker::PhantomData;

use gpui2::geometry::AbsoluteLength;

use crate::prelude::*;
use crate::{theme, token, v_stack};

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub enum PanelSide {
    #[default]
    Left,
    Right,
    Bottom,
}

use std::collections::HashSet;

#[derive(Element)]
pub struct Panel<V: 'static> {
    view_type: PhantomData<V>,
    scroll_state: ScrollState,
    current_side: PanelSide,
    /// Defaults to PanelSide:Left & PanelSide::Right
    allowed_sides: HashSet<PanelSide>,
    initial_width: AbsoluteLength,
    width: Option<AbsoluteLength>,
}

impl<V: 'static> Panel<V> {
    pub fn new(scroll_state: ScrollState) -> Self {
        let token = token();

        let mut allowed_sides = HashSet::new();
        allowed_sides.insert(PanelSide::Left);
        allowed_sides.insert(PanelSide::Right);

        Self {
            view_type: PhantomData,
            scroll_state,
            current_side: PanelSide::default(),
            allowed_sides,
            initial_width: token.default_panel_size,
            width: None,
        }
    }

    pub fn initial_width(&mut self, initial_width: AbsoluteLength) {
        self.initial_width = initial_width;
    }

    pub fn width(&mut self, width: AbsoluteLength) {
        self.width = Some(width);
    }

    pub fn allowed_sides(&mut self, allowed_sides: HashSet<PanelSide>) {
        self.allowed_sides = allowed_sides;
    }

    pub fn side(&mut self, side: PanelSide) {
        if self.allowed_sides.contains(&side) {
            self.current_side = side;
        } else {
            panic!(
                "The panel side {:?} was not added as allowed before it was set.",
                side
            );
        }
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let token = token();
        let theme = theme(cx);

        let panel_base;
        let current_width = if let Some(width) = self.width {
            width
        } else {
            self.initial_width
        };

        match self.current_side {
            PanelSide::Left => {
                panel_base = v_stack()
                    .overflow_y_scroll(self.scroll_state.clone())
                    .h_full()
                    .w(current_width)
                    .fill(theme.middle.base.default.background)
                    .border_r()
                    .border_color(theme.middle.base.default.border);
            }
            PanelSide::Right => {
                panel_base = v_stack()
                    .overflow_y_scroll(self.scroll_state.clone())
                    .h_full()
                    .w(current_width)
                    .fill(theme.middle.base.default.background)
                    .border_r()
                    .border_color(theme.middle.base.default.border);
            }
            PanelSide::Bottom => {
                panel_base = v_stack()
                    .overflow_y_scroll(self.scroll_state.clone())
                    .w_full()
                    .h(current_width)
                    .fill(theme.middle.base.default.background)
                    .border_r()
                    .border_color(theme.middle.base.default.border);
            }
        }

        panel_base
    }
}

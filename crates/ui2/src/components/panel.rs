use gpui2::{AbsoluteLength, AnyElement};
use smallvec::SmallVec;

use crate::prelude::*;
use crate::settings::user_settings;
use crate::v_stack;

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PanelAllowedSides {
    LeftOnly,
    RightOnly,
    BottomOnly,
    #[default]
    LeftAndRight,
    All,
}

impl PanelAllowedSides {
    /// Return a `HashSet` that contains the allowable `PanelSide`s.
    pub fn allowed_sides(&self) -> HashSet<PanelSide> {
        match self {
            Self::LeftOnly => HashSet::from_iter([PanelSide::Left]),
            Self::RightOnly => HashSet::from_iter([PanelSide::Right]),
            Self::BottomOnly => HashSet::from_iter([PanelSide::Bottom]),
            Self::LeftAndRight => HashSet::from_iter([PanelSide::Left, PanelSide::Right]),
            Self::All => HashSet::from_iter([PanelSide::Left, PanelSide::Right, PanelSide::Bottom]),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PanelSide {
    #[default]
    Left,
    Right,
    Bottom,
}

use std::collections::HashSet;

#[derive(Component)]
pub struct Panel<V: 'static> {
    id: ElementId,
    current_side: PanelSide,
    /// Defaults to PanelAllowedSides::LeftAndRight
    allowed_sides: PanelAllowedSides,
    initial_width: AbsoluteLength,
    width: Option<AbsoluteLength>,
    children: SmallVec<[AnyElement<V>; 2]>,
}

impl<V: 'static> Panel<V> {
    pub fn new(id: impl Into<ElementId>, cx: &mut WindowContext) -> Self {
        let settings = user_settings(cx);

        Self {
            id: id.into(),
            current_side: PanelSide::default(),
            allowed_sides: PanelAllowedSides::default(),
            initial_width: *settings.default_panel_size,
            width: None,
            children: SmallVec::new(),
        }
    }

    pub fn initial_width(mut self, initial_width: AbsoluteLength) -> Self {
        self.initial_width = initial_width;
        self
    }

    pub fn width(mut self, width: AbsoluteLength) -> Self {
        self.width = Some(width);
        self
    }

    pub fn allowed_sides(mut self, allowed_sides: PanelAllowedSides) -> Self {
        self.allowed_sides = allowed_sides;
        self
    }

    pub fn side(mut self, side: PanelSide) -> Self {
        let allowed_sides = self.allowed_sides.allowed_sides();

        if allowed_sides.contains(&side) {
            self.current_side = side;
        } else {
            panic!(
                "The panel side {:?} was not added as allowed before it was set.",
                side
            );
        }
        self
    }

    fn render(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        let theme = old_theme(cx);

        let current_size = self.width.unwrap_or(self.initial_width);

        v_stack()
            .id(self.id.clone())
            .flex_initial()
            .when(
                self.current_side == PanelSide::Left || self.current_side == PanelSide::Right,
                |this| this.h_full().w(current_size),
            )
            .when(self.current_side == PanelSide::Left, |this| this.border_r())
            .when(self.current_side == PanelSide::Right, |this| {
                this.border_l()
            })
            .when(self.current_side == PanelSide::Bottom, |this| {
                this.border_b().w_full().h(current_size)
            })
            .bg(theme.surface)
            .border_color(theme.border)
            .children(self.children)
    }
}

impl<V: 'static> ParentElement<V> for Panel<V> {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement<V>; 2]> {
        &mut self.children
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::{Label, Story};
    use gpui2::{Div, Render};

    pub struct PanelStory;

    impl Render for PanelStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, Panel<Self>>(cx))
                .child(Story::label(cx, "Default"))
                .child(
                    Panel::new("panel", cx).child(
                        div()
                            .id("panel-contents")
                            .overflow_y_scroll()
                            .children((0..100).map(|ix| Label::new(format!("Item {}", ix + 1)))),
                    ),
                )
        }
    }
}

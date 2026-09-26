use gpui::{App, Div, Hsla, Pixels, div, px};
use settings::Settings as _;
use theme::ActiveTheme as _;
use ui::{ElevationIndex, prelude::*};

use crate::WorkspaceSettings;

/// The geometry of the "floating" workspace layout, in which every dock and the
/// editor area is drawn as a rounded, elevated card, separated from its
/// neighbours by a gap of bare window background.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct FloatingLayout {
    /// The space left between two neighbouring cards, and between a card and
    /// the edge of the window.
    pub gap: Pixels,
    /// The corner radius of every card.
    pub radius: Pixels,
}

impl FloatingLayout {
    /// The floating layout currently in effect, or `None` when the workspace is
    /// using the classic edge-to-edge layout.
    pub fn get(cx: &App) -> Option<Self> {
        let settings = WorkspaceSettings::get_global(cx);
        match settings.ui_layout {
            settings::UiLayout::Classic => None,
            settings::UiLayout::Floating => Some(Self {
                gap: px(settings.ui_card_gap.max(0.)),
                radius: px(settings.ui_card_radius.max(0.)),
            }),
        }
    }

    /// The color of the surface the cards sit on, visible in the gaps between
    /// them. Themes generally give `background`, `panel_background` and
    /// `editor_background` near-identical values, so the cards would be
    /// indistinguishable from the space around them without pushing this one
    /// step away from the card backgrounds.
    pub fn canvas_background(&self, cx: &App) -> Hsla {
        cx.theme()
            .darken(cx.theme().colors().background, 0.04, 0.02)
    }

    /// Styles `card` as a floating card: rounded, elevated and drawn on top of
    /// the canvas. The caller is responsible for the card's own background, as
    /// docks and the editor area use different ones.
    pub fn style_card<E: Styled>(&self, card: E, cx: &App) -> E {
        card.rounded(self.radius)
            .shadow(ElevationIndex::ElevatedSurface.shadow(cx))
    }

    /// The overlay that gives a card its rounded corners, to be added as the
    /// last child of the card itself.
    ///
    /// GPUI masks content to rectangular bounds only, so a card's rounded
    /// corners are painted over by any descendant that fills its own background
    /// — which most panels do. This is a ring in the canvas color whose inner
    /// edge is a rounded rectangle of exactly [`Self::radius`]: a quad with
    /// corner radius `2 * radius` and a border `radius` wide has an inner edge
    /// of radius `radius`, so placing it inset by `-radius` leaves the corners
    /// covered and the rest of the card untouched. The card clips the ring's
    /// outer half away, so it never bleeds into the gap.
    pub fn corner_mask(&self, cx: &App) -> Div {
        let inset = -self.radius;
        div()
            .absolute()
            .top(inset)
            .left(inset)
            .right(inset)
            .bottom(inset)
            .rounded(self.radius * 2.)
            .border(self.radius)
            .border_color(self.canvas_background(cx))
    }

    /// The hairline outline drawn along a card's rounded edge, to be added after
    /// [`Self::corner_mask`] so it is not covered by it.
    pub fn outline(&self, cx: &App) -> Div {
        div()
            .absolute()
            .inset_0()
            .rounded(self.radius)
            .border_1()
            .border_color(cx.theme().colors().border)
    }
}

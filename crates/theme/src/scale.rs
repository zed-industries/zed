#![allow(missing_docs)]
use gpui::{App, Hsla, SharedString};

use crate::{ActiveTheme, Appearance};

/// A collection of colors that are used to style the UI.
///
/// Each step has a semantic meaning, and is used to style different parts of the UI.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ColorScaleStep(usize);

impl ColorScaleStep {
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
    pub const THREE: Self = Self(3);
    pub const FOUR: Self = Self(4);
    pub const FIVE: Self = Self(5);
    pub const SIX: Self = Self(6);
    pub const SEVEN: Self = Self(7);
    pub const EIGHT: Self = Self(8);
    pub const NINE: Self = Self(9);
    pub const TEN: Self = Self(10);
    pub const ELEVEN: Self = Self(11);
    pub const TWELVE: Self = Self(12);

    /// All of the steps in a [`ColorScale`].
    pub const ALL: [ColorScaleStep; 12] = [
        Self::ONE,
        Self::TWO,
        Self::THREE,
        Self::FOUR,
        Self::FIVE,
        Self::SIX,
        Self::SEVEN,
        Self::EIGHT,
        Self::NINE,
        Self::TEN,
        Self::ELEVEN,
        Self::TWELVE,
    ];
}

/// A scale of colors for a given [`ColorScaleSet`].
///
/// Each [`ColorScale`] contains exactly 12 colors. Refer to
/// [`ColorScaleStep`] for a reference of what each step is used for.
pub struct ColorScale(Vec<Hsla>);

impl FromIterator<Hsla> for ColorScale {
    fn from_iter<T: IntoIterator<Item = Hsla>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl ColorScale {
    /// Returns the specified step in the [`ColorScale`].
    #[inline]
    pub fn step(&self, step: ColorScaleStep) -> Hsla {
        // Steps are one-based, so we need convert to the zero-based vec index.
        self.0[step.0 - 1]
    }

    /// `Step 1` - Used for main application backgrounds.
    ///
    /// This step provides a neutral base for any overlaying components, ideal for applications' main backdrop or empty spaces such as canvas areas.
    ///
    #[inline]
    pub fn step_1(&self) -> Hsla {
        self.step(ColorScaleStep::ONE)
    }

    /// `Step 2` - Used for both main application backgrounds and subtle component backgrounds.
    ///
    /// Like `Step 1`, this step allows variations in background styles, from striped tables, sidebar backgrounds, to card backgrounds.
    #[inline]
    pub fn step_2(&self) -> Hsla {
        self.step(ColorScaleStep::TWO)
    }

    /// `Step 3` - Used for UI component backgrounds in their normal states.
    ///
    /// This step maintains accessibility by guaranteeing a contrast ratio of 4.5:1 with steps 11 and 12 for text. It could also suit hover states for transparent components.
    #[inline]
    pub fn step_3(&self) -> Hsla {
        self.step(ColorScaleStep::THREE)
    }

    /// `Step 4` - Used for UI component backgrounds in their hover states.
    ///
    /// Also suited for pressed or selected states of components with a transparent background.
    #[inline]
    pub fn step_4(&self) -> Hsla {
        self.step(ColorScaleStep::FOUR)
    }

    /// `Step 5` - Used for UI component backgrounds in their pressed or selected states.
    #[inline]
    pub fn step_5(&self) -> Hsla {
        self.step(ColorScaleStep::FIVE)
    }

    /// `Step 6` - Used for subtle borders on non-interactive components.
    ///
    /// Its usage spans from sidebars' borders, headers' dividers, cards' outlines, to alerts' edges and separators.
    #[inline]
    pub fn step_6(&self) -> Hsla {
        self.step(ColorScaleStep::SIX)
    }

    /// `Step 7` - Used for subtle borders on interactive components.
    ///
    /// This step subtly delineates the boundary of elements users interact with.
    #[inline]
    pub fn step_7(&self) -> Hsla {
        self.step(ColorScaleStep::SEVEN)
    }

    /// `Step 8` - Used for stronger borders on interactive components and focus rings.
    ///
    /// It strengthens the visibility and accessibility of active elements and their focus states.
    #[inline]
    pub fn step_8(&self) -> Hsla {
        self.step(ColorScaleStep::EIGHT)
    }

    /// `Step 9` - Used for solid backgrounds.
    ///
    /// `Step 9` is the most saturated step, having the least mix of white or black.
    ///
    /// Due to its high chroma, `Step 9` is versatile and particularly useful for semantic colors such as
    /// error, warning, and success indicators.
    #[inline]
    pub fn step_9(&self) -> Hsla {
        self.step(ColorScaleStep::NINE)
    }

    /// `Step 10` - Used for hovered or active solid backgrounds, particularly when `Step 9` is their normal state.
    ///
    /// May also be used for extremely low contrast text. This should be used sparingly, as it may be difficult to read.
    #[inline]
    pub fn step_10(&self) -> Hsla {
        self.step(ColorScaleStep::TEN)
    }

    /// `Step 11` - Used for text and icons requiring low contrast or less emphasis.
    #[inline]
    pub fn step_11(&self) -> Hsla {
        self.step(ColorScaleStep::ELEVEN)
    }

    /// `Step 12` - Used for text and icons requiring high contrast or prominence.
    #[inline]
    pub fn step_12(&self) -> Hsla {
        self.step(ColorScaleStep::TWELVE)
    }
}

pub struct ColorScales {
    pub gray: ColorScaleSet,
    pub mauve: ColorScaleSet,
    pub slate: ColorScaleSet,
    pub sage: ColorScaleSet,
    pub olive: ColorScaleSet,
    pub sand: ColorScaleSet,
    pub gold: ColorScaleSet,
    pub bronze: ColorScaleSet,
    pub brown: ColorScaleSet,
    pub yellow: ColorScaleSet,
    pub amber: ColorScaleSet,
    pub orange: ColorScaleSet,
    pub tomato: ColorScaleSet,
    pub red: ColorScaleSet,
    pub ruby: ColorScaleSet,
    pub crimson: ColorScaleSet,
    pub pink: ColorScaleSet,
    pub plum: ColorScaleSet,
    pub purple: ColorScaleSet,
    pub violet: ColorScaleSet,
    pub iris: ColorScaleSet,
    pub indigo: ColorScaleSet,
    pub blue: ColorScaleSet,
    pub cyan: ColorScaleSet,
    pub teal: ColorScaleSet,
    pub jade: ColorScaleSet,
    pub green: ColorScaleSet,
    pub grass: ColorScaleSet,
    pub lime: ColorScaleSet,
    pub mint: ColorScaleSet,
    pub sky: ColorScaleSet,
    pub black: ColorScaleSet,
    pub white: ColorScaleSet,
}

impl IntoIterator for ColorScales {
    type Item = ColorScaleSet;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.gray,
            self.mauve,
            self.slate,
            self.sage,
            self.olive,
            self.sand,
            self.gold,
            self.bronze,
            self.brown,
            self.yellow,
            self.amber,
            self.orange,
            self.tomato,
            self.red,
            self.ruby,
            self.crimson,
            self.pink,
            self.plum,
            self.purple,
            self.violet,
            self.iris,
            self.indigo,
            self.blue,
            self.cyan,
            self.teal,
            self.jade,
            self.green,
            self.grass,
            self.lime,
            self.mint,
            self.sky,
            self.black,
            self.white,
        ]
        .into_iter()
    }
}

/// Provides groups of [`ColorScale`]s for light and dark themes, as well as transparent versions of each scale.
pub struct ColorScaleSet {
    name: SharedString,
    light: ColorScale,
    dark: ColorScale,
    light_alpha: ColorScale,
    dark_alpha: ColorScale,
}

impl ColorScaleSet {
    pub fn new(
        name: impl Into<SharedString>,
        light: ColorScale,
        light_alpha: ColorScale,
        dark: ColorScale,
        dark_alpha: ColorScale,
    ) -> Self {
        Self {
            name: name.into(),
            light,
            light_alpha,
            dark,
            dark_alpha,
        }
    }

    pub fn name(&self) -> &SharedString {
        &self.name
    }

    pub fn light(&self) -> &ColorScale {
        &self.light
    }

    pub fn light_alpha(&self) -> &ColorScale {
        &self.light_alpha
    }

    pub fn dark(&self) -> &ColorScale {
        &self.dark
    }

    pub fn dark_alpha(&self) -> &ColorScale {
        &self.dark_alpha
    }

    pub fn step(&self, cx: &App, step: ColorScaleStep) -> Hsla {
        match cx.theme().appearance {
            Appearance::Light => self.light().step(step),
            Appearance::Dark => self.dark().step(step),
        }
    }

    pub fn step_alpha(&self, cx: &App, step: ColorScaleStep) -> Hsla {
        match cx.theme().appearance {
            Appearance::Light => self.light_alpha.step(step),
            Appearance::Dark => self.dark_alpha.step(step),
        }
    }
}

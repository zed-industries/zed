use gpui::{AppContext, Hsla, SharedString};

use crate::{ActiveTheme, Appearance};

/// A one-based step in a [`ColorScale`].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ColorScaleStep(usize);

impl ColorScaleStep {
    /// The first step in a [`ColorScale`].
    pub const ONE: Self = Self(1);

    /// The second step in a [`ColorScale`].
    pub const TWO: Self = Self(2);

    /// The third step in a [`ColorScale`].
    pub const THREE: Self = Self(3);

    /// The fourth step in a [`ColorScale`].
    pub const FOUR: Self = Self(4);

    /// The fifth step in a [`ColorScale`].
    pub const FIVE: Self = Self(5);

    /// The sixth step in a [`ColorScale`].
    pub const SIX: Self = Self(6);

    /// The seventh step in a [`ColorScale`].
    pub const SEVEN: Self = Self(7);

    /// The eighth step in a [`ColorScale`].
    pub const EIGHT: Self = Self(8);

    /// The ninth step in a [`ColorScale`].
    pub const NINE: Self = Self(9);

    /// The tenth step in a [`ColorScale`].
    pub const TEN: Self = Self(10);

    /// The eleventh step in a [`ColorScale`].
    pub const ELEVEN: Self = Self(11);

    /// The twelfth step in a [`ColorScale`].
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

    /// Returns the first step in the [`ColorScale`].
    #[inline]
    pub fn step_1(&self) -> Hsla {
        self.step(ColorScaleStep::ONE)
    }

    /// Returns the second step in the [`ColorScale`].
    #[inline]
    pub fn step_2(&self) -> Hsla {
        self.step(ColorScaleStep::TWO)
    }

    /// Returns the third step in the [`ColorScale`].
    #[inline]
    pub fn step_3(&self) -> Hsla {
        self.step(ColorScaleStep::THREE)
    }

    /// Returns the fourth step in the [`ColorScale`].
    #[inline]
    pub fn step_4(&self) -> Hsla {
        self.step(ColorScaleStep::FOUR)
    }

    /// Returns the fifth step in the [`ColorScale`].
    #[inline]
    pub fn step_5(&self) -> Hsla {
        self.step(ColorScaleStep::FIVE)
    }

    /// Returns the sixth step in the [`ColorScale`].
    #[inline]
    pub fn step_6(&self) -> Hsla {
        self.step(ColorScaleStep::SIX)
    }

    /// Returns the seventh step in the [`ColorScale`].
    #[inline]
    pub fn step_7(&self) -> Hsla {
        self.step(ColorScaleStep::SEVEN)
    }

    /// Returns the eighth step in the [`ColorScale`].
    #[inline]
    pub fn step_8(&self) -> Hsla {
        self.step(ColorScaleStep::EIGHT)
    }

    /// Returns the ninth step in the [`ColorScale`].
    #[inline]
    pub fn step_9(&self) -> Hsla {
        self.step(ColorScaleStep::NINE)
    }

    /// Returns the tenth step in the [`ColorScale`].
    #[inline]
    pub fn step_10(&self) -> Hsla {
        self.step(ColorScaleStep::TEN)
    }

    /// Returns the eleventh step in the [`ColorScale`].
    #[inline]
    pub fn step_11(&self) -> Hsla {
        self.step(ColorScaleStep::ELEVEN)
    }

    /// Returns the twelfth step in the [`ColorScale`].
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

    pub fn step(&self, cx: &AppContext, step: ColorScaleStep) -> Hsla {
        match cx.theme().appearance {
            Appearance::Light => self.light().step(step),
            Appearance::Dark => self.dark().step(step),
        }
    }

    pub fn step_alpha(&self, cx: &AppContext, step: ColorScaleStep) -> Hsla {
        match cx.theme().appearance {
            Appearance::Light => self.light_alpha.step(step),
            Appearance::Dark => self.dark_alpha.step(step),
        }
    }
}

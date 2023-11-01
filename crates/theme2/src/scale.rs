use gpui2::{AppContext, Hsla, SharedString};

use crate::{old_theme, Appearance};

pub type ColorScale = [Hsla; 12];

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

/// A one-based step in a [`ColorScale`].
pub type ColorScaleStep = usize;

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

    pub fn name(&self) -> SharedString {
        self.name.clone()
    }

    pub fn light(&self, step: ColorScaleStep) -> Hsla {
        self.light[step - 1]
    }

    pub fn light_alpha(&self, step: ColorScaleStep) -> Hsla {
        self.light_alpha[step - 1]
    }

    pub fn dark(&self, step: ColorScaleStep) -> Hsla {
        self.dark[step - 1]
    }

    pub fn dark_alpha(&self, step: ColorScaleStep) -> Hsla {
        self.dark_alpha[step - 1]
    }

    fn current_appearance(cx: &AppContext) -> Appearance {
        let theme = old_theme(cx);
        if theme.metadata.is_light {
            Appearance::Light
        } else {
            Appearance::Dark
        }
    }

    pub fn step(&self, cx: &AppContext, step: ColorScaleStep) -> Hsla {
        let appearance = Self::current_appearance(cx);

        match appearance {
            Appearance::Light => self.light(step),
            Appearance::Dark => self.dark(step),
        }
    }

    pub fn step_alpha(&self, cx: &AppContext, step: ColorScaleStep) -> Hsla {
        let appearance = Self::current_appearance(cx);
        match appearance {
            Appearance::Light => self.light_alpha(step),
            Appearance::Dark => self.dark_alpha(step),
        }
    }
}

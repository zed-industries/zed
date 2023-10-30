use std::collections::HashMap;

use gpui2::{AppContext, Hsla};

use crate::{theme, Appearance};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ColorScaleName {
    Gray,
    Mauve,
    Slate,
    Sage,
    Olive,
    Sand,
    Gold,
    Bronze,
    Brown,
    Yellow,
    Amber,
    Orange,
    Tomato,
    Red,
    Ruby,
    Crimson,
    Pink,
    Plum,
    Purple,
    Violet,
    Iris,
    Indigo,
    Blue,
    Cyan,
    Teal,
    Jade,
    Green,
    Grass,
    Lime,
    Mint,
    Sky,
    Black,
    White,
}

impl std::fmt::Display for ColorScaleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Gray => "Gray",
                Self::Mauve => "Mauve",
                Self::Slate => "Slate",
                Self::Sage => "Sage",
                Self::Olive => "Olive",
                Self::Sand => "Sand",
                Self::Gold => "Gold",
                Self::Bronze => "Bronze",
                Self::Brown => "Brown",
                Self::Yellow => "Yellow",
                Self::Amber => "Amber",
                Self::Orange => "Orange",
                Self::Tomato => "Tomato",
                Self::Red => "Red",
                Self::Ruby => "Ruby",
                Self::Crimson => "Crimson",
                Self::Pink => "Pink",
                Self::Plum => "Plum",
                Self::Purple => "Purple",
                Self::Violet => "Violet",
                Self::Iris => "Iris",
                Self::Indigo => "Indigo",
                Self::Blue => "Blue",
                Self::Cyan => "Cyan",
                Self::Teal => "Teal",
                Self::Jade => "Jade",
                Self::Green => "Green",
                Self::Grass => "Grass",
                Self::Lime => "Lime",
                Self::Mint => "Mint",
                Self::Sky => "Sky",
                Self::Black => "Black",
                Self::White => "White",
            }
        )
    }
}

pub type ColorScale = [Hsla; 12];

pub type ColorScales = HashMap<ColorScaleName, ColorScaleSet>;

pub struct ColorScaleSet {
    name: ColorScaleName,
    light: ColorScale,
    dark: ColorScale,
    light_alpha: ColorScale,
    dark_alpha: ColorScale,
}

impl ColorScaleSet {
    pub fn new(
        name: ColorScaleName,
        light: ColorScale,
        light_alpha: ColorScale,
        dark: ColorScale,
        dark_alpha: ColorScale,
    ) -> Self {
        Self {
            name,
            light,
            light_alpha,
            dark,
            dark_alpha,
        }
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn light(&self, step: usize) -> Hsla {
        self.light[step - 1]
    }

    pub fn light_alpha(&self, step: usize) -> Hsla {
        self.light_alpha[step - 1]
    }

    pub fn dark(&self, step: usize) -> Hsla {
        self.dark[step - 1]
    }

    pub fn dark_alpha(&self, step: usize) -> Hsla {
        self.dark[step - 1]
    }

    fn current_appearance(cx: &AppContext) -> Appearance {
        let theme = theme(cx);
        if theme.metadata.is_light {
            Appearance::Light
        } else {
            Appearance::Dark
        }
    }

    /// Returns the one-based step in the scale.
    ///
    /// We usually reference steps as 1-12 instead of 0-11, so we
    /// automatically subtract 1 from the index.
    pub fn step(self, cx: &AppContext, index: usize) -> Hsla {
        let appearance = Self::current_appearance(cx);

        match appearance {
            Appearance::Light => self.light(index),
            Appearance::Dark => self.dark(index),
        }
    }

    pub fn step_alpha(self, cx: &AppContext, index: usize) -> Hsla {
        let appearance = Self::current_appearance(cx);
        match appearance {
            Appearance::Light => self.light_alpha(index),
            Appearance::Dark => self.dark_alpha(index),
        }
    }
}

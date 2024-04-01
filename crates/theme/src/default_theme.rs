use std::sync::Arc;

use gpui::WindowBackgroundAppearance;

use crate::prelude::*;

use crate::{
    default_color_scales,
    one_themes::{one_dark, one_family},
    Appearance, PlayerColors, StatusColors, SyntaxTheme, SystemColors, Theme, ThemeColors,
    ThemeFamily, ThemeStyles,
};

fn zed_pro_daylight() -> Theme {
    Theme {
        id: "zed_pro_daylight".to_string(),
        name: "Zed Pro Daylight".into(),
        appearance: Appearance::Light,
        styles: ThemeStyles {
            window_background_appearance: WindowBackgroundAppearance::Opaque,
            system: SystemColors::default(),
            colors: ThemeColors::light(),
            status: StatusColors::light(),
            player: PlayerColors::light(),
            syntax: Arc::new(SyntaxTheme::light()),
            accents: vec![
                blue().light().step_9(),
                orange().light().step_9(),
                pink().light().step_9(),
                lime().light().step_9(),
                purple().light().step_9(),
                amber().light().step_9(),
                jade().light().step_9(),
                tomato().light().step_9(),
                cyan().light().step_9(),
                gold().light().step_9(),
                grass().light().step_9(),
                indigo().light().step_9(),
                iris().light().step_9(),
            ],
        },
    }
}

pub(crate) fn zed_pro_moonlight() -> Theme {
    Theme {
        id: "zed_pro_moonlight".to_string(),
        name: "Zed Pro Moonlight".into(),
        appearance: Appearance::Dark,
        styles: ThemeStyles {
            window_background_appearance: WindowBackgroundAppearance::Opaque,
            system: SystemColors::default(),
            colors: ThemeColors::dark(),
            status: StatusColors::dark(),
            player: PlayerColors::dark(),
            syntax: Arc::new(SyntaxTheme::dark()),
            accents: vec![
                blue().dark().step_9(),
                orange().dark().step_9(),
                pink().dark().step_9(),
                lime().dark().step_9(),
                purple().dark().step_9(),
                amber().dark().step_9(),
                jade().dark().step_9(),
                tomato().dark().step_9(),
                cyan().dark().step_9(),
                gold().dark().step_9(),
                grass().dark().step_9(),
                indigo().dark().step_9(),
                iris().dark().step_9(),
            ],
        },
    }
}

pub fn zed_pro_family() -> ThemeFamily {
    ThemeFamily {
        id: "zed_pro".to_string(),
        name: "Zed Pro".into(),
        author: "Zed Team".into(),
        themes: vec![zed_pro_daylight(), zed_pro_moonlight()],
        scales: default_color_scales(),
    }
}

impl Default for ThemeFamily {
    fn default() -> Self {
        one_family()
    }
}

impl Default for Theme {
    fn default() -> Self {
        one_dark()
    }
}

use std::sync::Arc;

use crate::{
    colors::{StatusColors, SystemColors, ThemeColors, ThemeStyles},
    default_color_scales, Appearance, PlayerColors, SyntaxTheme, Theme, ThemeFamily,
};

fn zed_pro_daylight() -> Theme {
    Theme {
        id: "zed_pro_daylight".to_string(),
        name: "Zed Pro Daylight".into(),
        appearance: Appearance::Light,
        styles: ThemeStyles {
            system: SystemColors::default(),
            colors: ThemeColors::default_light(),
            status: StatusColors::default(),
            player: PlayerColors::default_light(),
            syntax: Arc::new(SyntaxTheme::default_light()),
        },
    }
}

pub(crate) fn zed_pro_moonlight() -> Theme {
    Theme {
        id: "zed_pro_moonlight".to_string(),
        name: "Zed Pro Moonlight".into(),
        appearance: Appearance::Dark,
        styles: ThemeStyles {
            system: SystemColors::default(),
            colors: ThemeColors::default_dark(),
            status: StatusColors::default(),
            player: PlayerColors::default(),
            syntax: Arc::new(SyntaxTheme::default_dark()),
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
        zed_pro_family()
    }
}

impl Default for Theme {
    fn default() -> Self {
        zed_pro_daylight()
    }
}

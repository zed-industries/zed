use crate::{
    colors::{GitStatusColors, PlayerColors, StatusColors, SystemColors, ThemeColors, ThemeStyle},
    default_color_scales, Appearance, SyntaxStyles, ThemeFamily, ThemeVariant,
};

fn zed_pro_daylight() -> ThemeVariant {
    ThemeVariant {
        id: "zed_pro_daylight".to_string(),
        name: "Zed Pro Daylight".into(),
        appearance: Appearance::Light,
        styles: ThemeStyle {
            system: SystemColors::default(),
            colors: ThemeColors::default_light(),
            status: StatusColors::default(),
            git: GitStatusColors::default(),
            player: PlayerColors::default(),
            syntax: SyntaxStyles::default_light(),
        },
    }
}

pub(crate) fn zed_pro_moonlight() -> ThemeVariant {
    ThemeVariant {
        id: "zed_pro_moonlight".to_string(),
        name: "Zed Pro Moonlight".into(),
        appearance: Appearance::Dark,
        styles: ThemeStyle {
            system: SystemColors::default(),
            colors: ThemeColors::default_dark(),
            status: StatusColors::default(),
            git: GitStatusColors::default(),
            player: PlayerColors::default(),
            syntax: SyntaxStyles::default_dark(),
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

impl Default for ThemeVariant {
    fn default() -> Self {
        zed_pro_daylight()
    }
}

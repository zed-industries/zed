use crate::{
    colors::{GitStatusColors, PlayerColors, StatusColors, SystemColors, ThemeColors, ThemeStyle},
    family::{ThemeFamily, ThemeVariant},
    Appearance, ColorScales, SyntaxStyles,
};

fn zed_pro_daylight() -> ThemeVariant {
    ThemeVariant {
        name: "Zed Pro Daylight".to_string(),
        appearance: Appearance::Light,
        styles: ThemeStyle {
            system: SystemColors::default(),
            color: ThemeColors::default_light(),
            status: StatusColors::default(),
            git: GitStatusColors::default(),
            player: PlayerColors::default(),
            syntax: SyntaxStyles::default_light(),
        },
    }
}

fn zed_pro_moonlight() -> ThemeVariant {
    ThemeVariant {
        name: "Zed Pro Moonlight".to_string(),
        appearance: Appearance::Light,
        styles: ThemeStyle {
            system: SystemColors::default(),
            color: ThemeColors::default_dark(),
            status: StatusColors::default(),
            git: GitStatusColors::default(),
            player: PlayerColors::default(),
            syntax: SyntaxStyles::default_dark(),
        },
    }
}

pub fn zed_pro_family() -> ThemeFamily {
    ThemeFamily {
        name: "Zed Pro".to_string(),
        author: "Zed Team".to_string(),
        themes: vec![zed_pro_daylight(), zed_pro_moonlight()],
        scales: ColorScales::default(),
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

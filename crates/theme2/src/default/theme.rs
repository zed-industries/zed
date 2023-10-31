use crate::{
    colors::{GitStatusColors, PlayerColors, StatusColors, SystemColors, ThemeColors, ThemeStyle},
    family::{ThemeFamily, ThemeVariant},
    Appearance, ColorScales,
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
        },
    }
}

fn zed_pro_moonlight() -> ThemeVariant {
    ThemeVariant {
        name: "Zed Pro Moonlight".to_string(),
        appearance: Appearance::Light,
        styles: ThemeStyle {
            system: SystemColors::default(),
            color: ThemeColors::default_light(),
            status: StatusColors::default(),
            git: GitStatusColors::default(),
            player: PlayerColors::default(),
        },
    }
}

fn zed_pro_midnight() -> ThemeVariant {
    ThemeVariant {
        name: "Zed Pro Midnight".to_string(),
        appearance: Appearance::Light,
        styles: ThemeStyle {
            system: SystemColors::default(),
            color: ThemeColors::default_light(),
            status: StatusColors::default(),
            git: GitStatusColors::default(),
            player: PlayerColors::default(),
        },
    }
}

pub fn zed_pro_family() -> ThemeFamily {
    ThemeFamily {
        name: "Zed Pro".to_string(),
        author: "Zed Team".to_string(),
        themes: vec![zed_pro_daylight(), zed_pro_moonlight(), zed_pro_midnight()],
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

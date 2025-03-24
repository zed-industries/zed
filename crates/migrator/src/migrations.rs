pub(crate) mod m_2025_01_02 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_01_29 {
    mod keymap;
    mod settings;

    pub(crate) use keymap::KEYMAP_PATTERNS;
    pub(crate) use settings::{replace_edit_prediction_provider_setting, SETTINGS_PATTERNS};
}

pub(crate) mod m_2025_01_30 {
    mod keymap;
    mod settings;

    pub(crate) use keymap::KEYMAP_PATTERNS;
    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_03_03 {
    mod keymap;

    pub(crate) use keymap::KEYMAP_PATTERNS;
}

pub(crate) mod m_2025_03_06 {
    mod keymap;

    pub(crate) use keymap::KEYMAP_PATTERNS;
}

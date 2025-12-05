pub(crate) mod m_2025_01_02 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_01_29 {
    mod keymap;
    mod settings;

    pub(crate) use keymap::KEYMAP_PATTERNS;
    pub(crate) use settings::{SETTINGS_PATTERNS, replace_edit_prediction_provider_setting};
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

pub(crate) mod m_2025_03_29 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_04_15 {
    mod keymap;
    mod settings;

    pub(crate) use keymap::KEYMAP_PATTERNS;
    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_04_21 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_04_23 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_05_05 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_05_08 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_05_29 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_06_16 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_06_25 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_06_27 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_07_08 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_10_01 {
    mod settings;

    pub(crate) use settings::flatten_code_actions_formatters;
}

pub(crate) mod m_2025_10_02 {
    mod settings;

    pub(crate) use settings::remove_formatters_on_save;
}

pub(crate) mod m_2025_10_03 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_10_16 {
    mod settings;

    pub(crate) use settings::restore_code_actions_on_format;
}

pub(crate) mod m_2025_10_17 {
    mod settings;

    pub(crate) use settings::make_file_finder_include_ignored_an_enum;
}

pub(crate) mod m_2025_10_21 {
    mod settings;

    pub(crate) use settings::make_relative_line_numbers_an_enum;
}

pub(crate) mod m_2025_11_12 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_11_20 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

pub(crate) mod m_2025_11_25 {
    mod settings;

    pub(crate) use settings::remove_context_server_source;
}

pub(crate) mod m_2025_12_01 {
    mod settings;

    pub(crate) use settings::SETTINGS_PATTERNS;
}

use std::{num::NonZeroU32, str::FromStr};

use collections::{hash_map, HashMap};
use gpui::SharedString;

use anyhow::Context as _;
use ini::Ini;

#[derive(Debug, Clone)]
pub struct Editorconfig {
    main_section: Section,
    glob_sections: HashMap<SharedString, Section>,
    glob_sections_order: Vec<SharedString>,
    is_root: bool,
}

impl Editorconfig {
    pub fn merge_with(&mut self, parent_editorconfig: &Editorconfig) {
        debug_assert!(
            !self.is_root,
            "Should not merge root editorconfig with more properties"
        );

        self.main_section
            .merge_with(&parent_editorconfig.main_section);
        for (parent_pattern, parent_section) in &parent_editorconfig.glob_sections {
            match self.glob_sections.entry(parent_pattern.clone()) {
                hash_map::Entry::Vacant(e) => {
                    self.glob_sections_order.push(parent_pattern.clone());
                    e.insert(*parent_section);
                }
                hash_map::Entry::Occupied(mut e) => {
                    e.get_mut().merge_with(parent_section);
                }
            }
        }
    }

    pub fn is_root(&self) -> bool {
        self.is_root
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Section {
    indent_style: Option<IndentStyle>,
    indent_size: Option<IndentSize>,
    tab_width: Option<NonZeroU32>,
    trim_trailing_whitespace: Option<bool>,
    insert_final_newline: Option<bool>,
    max_line_length: Option<MaxLineLength>,
    // Currently not supported by Zed
    // end_of_line: Option<bool>,
    // charset: Option<u32>,
    // spelling_language: Option<String>,
}
impl Section {
    fn merge_with(&mut self, parent_section: &Section) {
        if self.indent_style.is_none() {
            self.indent_style = parent_section.indent_style;
        }
        if self.indent_size.is_none() {
            self.indent_size = parent_section.indent_size;
        }
        if self.tab_width.is_none() {
            self.tab_width = parent_section.tab_width;
        }
        if self.trim_trailing_whitespace.is_none() {
            self.trim_trailing_whitespace = parent_section.trim_trailing_whitespace;
        }
        if self.insert_final_newline.is_none() {
            self.insert_final_newline = parent_section.insert_final_newline;
        }
        if self.max_line_length.is_none() {
            self.max_line_length = parent_section.max_line_length;
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum IndentStyle {
    Tab,
    Space,
}

#[derive(Debug, Copy, Clone)]
enum IndentSize {
    Tab,
    Value(NonZeroU32),
}

#[derive(Debug, Copy, Clone)]
enum MaxLineLength {
    Off,
    Value(NonZeroU32),
}

impl FromStr for Editorconfig {
    type Err = anyhow::Error;

    // TODO kb be more lenient and allow (omit) partially incorrect fields?
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let editorconfig_ini =
            Ini::load_from_str(s).context("parsing editorconfig string as ini")?;
        let mut is_root = false;
        let mut main_section = Section::default();
        let mut glob_sections = HashMap::default();
        let mut glob_sections_order = Vec::new();
        for (ini_section, ini_properties) in editorconfig_ini {
            let (has_section, section_to_fill) = match ini_section {
                Some(ini_section) => {
                    let ini_section = SharedString::from(ini_section);
                    let section_to_fill = match glob_sections.entry(ini_section.clone()) {
                        hash_map::Entry::Occupied(mut o) => {
                            o.insert(Section::default());
                            o.into_mut()
                        }
                        hash_map::Entry::Vacant(v) => {
                            glob_sections_order.push(ini_section);
                            v.insert(Section::default())
                        }
                    };
                    (true, section_to_fill)
                }
                None => (false, &mut main_section),
            };

            for (mut ini_property, mut ini_value) in ini_properties {
                ini_property.make_ascii_lowercase();
                ini_value.make_ascii_lowercase();
                let ini_property = ini_property.as_str();
                let ini_value = ini_value.as_str();
                match ini_property {
                    "indent_style" => match ini_value {
                        "tab" => section_to_fill.indent_style = Some(IndentStyle::Tab),
                        "space" => section_to_fill.indent_style = Some(IndentStyle::Space),
                        _unknown => {}
                    },
                    "indent_size" => match ini_value {
                        "tab" => section_to_fill.indent_size = Some(IndentSize::Tab),
                        value => {
                            section_to_fill.indent_size = Some(IndentSize::Value(
                                NonZeroU32::from_str(value).context("parsing indent_size")?,
                            ))
                        }
                    },
                    "tab_width" => {
                        section_to_fill.tab_width =
                            Some(NonZeroU32::from_str(ini_value).context("parsing tab_width")?)
                    }
                    "trim_trailing_whitespace" => {
                        section_to_fill.trim_trailing_whitespace = Some(
                            bool::from_str(ini_value)
                                .context("parsing trim_trailing_whitespace")?,
                        )
                    }
                    "insert_final_newline" => {
                        section_to_fill.insert_final_newline = Some(
                            bool::from_str(ini_value).context("parsing insert_final_newline")?,
                        )
                    }
                    "max_line_length" => match ini_value {
                        "off" => section_to_fill.max_line_length = Some(MaxLineLength::Off),
                        value => {
                            section_to_fill.max_line_length = Some(MaxLineLength::Value(
                                NonZeroU32::from_str(value).context("parsing max_line_length")?,
                            ))
                        }
                    },
                    "root" if !has_section => {
                        is_root = bool::from_str(ini_value).context("parsing root")?;
                    }
                    // unsupported
                    "end_of_line" => {}
                    "charset" => {}
                    "spelling_language" => {}
                    _unknown => {}
                }
            }
        }

        Ok(Self {
            main_section,
            is_root,
            glob_sections,
            glob_sections_order,
        })
    }
}

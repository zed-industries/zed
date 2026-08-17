//! Generate the ISA-specific settings.

use crate::cdsl::camel_case;
use crate::cdsl::settings::{
    BoolSetting, Predicate, Preset, Setting, SettingGroup, SpecificSetting,
};
use crate::constant_hash::generate_table;
use crate::unique_table::UniqueSeqTable;
use cranelift_codegen_shared::constant_hash::simple_hash;
use cranelift_srcgen::{Formatter, Language, Match, error, fmtln};
use std::collections::HashMap;

pub(crate) enum ParentGroup {
    None,
    Shared,
}

/// Emits the constructor of the Flags structure.
fn gen_constructor(group: &SettingGroup, parent: ParentGroup, fmt: &mut Formatter) {
    let args = match parent {
        ParentGroup::None => "builder: Builder",
        ParentGroup::Shared => "shared: &settings::Flags, builder: &Builder",
    };
    fmt.add_block("impl Flags", |fmt| {
        fmt.doc_comment(format!("Create flags {} settings group.", group.name));
        fmtln!(
            fmt,
            "#[allow(unused_variables, reason = \"generated code\")]"
        );
        fmt.add_block(&format!("pub fn new({args}) -> Self"), |fmt| {
            fmtln!(fmt, "let bvec = builder.state_for(\"{}\");", group.name);
            fmtln!(
                fmt,
                "let mut {} = Self {{ bytes: [0; {}] }};",
                group.name,
                group.byte_size()
            );
            fmtln!(
                fmt,
                "debug_assert_eq!(bvec.len(), {});",
                group.settings_size
            );
            fmtln!(
                fmt,
                "{}.bytes[0..{}].copy_from_slice(&bvec);",
                group.name,
                group.settings_size
            );

            // Now compute the predicates.
            for p in &group.predicates {
                fmt.comment(format!("Precompute #{}.", p.number));
                fmt.add_block(&format!("if {}", p.render(group)), |fmt| {
                    fmtln!(
                        fmt,
                        "{}.bytes[{}] |= 1 << {};",
                        group.name,
                        group.bool_start_byte_offset + p.number / 8,
                        p.number % 8
                    );
                });
            }

            fmtln!(fmt, "{}", group.name);
        });
    });
}

/// Generates the `iter` function.
fn gen_iterator(group: &SettingGroup, fmt: &mut Formatter) {
    fmt.add_block("impl Flags",|fmt| {
        fmt.doc_comment("Iterates the setting values.");
        fmt.add_block("pub fn iter(&self) -> impl Iterator<Item = Value> + use<>",|fmt| {
            fmtln!(fmt, "let mut bytes = [0; {}];", group.settings_size);
            fmtln!(fmt, "bytes.copy_from_slice(&self.bytes[0..{}]);", group.settings_size);
            fmt.add_block("DESCRIPTORS.iter().filter_map(move |d|", |fmt| {
                fmt.add_block("let values = match &d.detail", |fmt| {
                    fmtln!(fmt, "detail::Detail::Preset => return None,");
                    fmtln!(fmt, "detail::Detail::Enum {{ last, enumerators }} => Some(TEMPLATE.enums(*last, *enumerators)),");
                    fmtln!(fmt, "_ => None");
                });
                fmtln!(fmt, ";");
                fmtln!(fmt, "Some(Value {{ name: d.name, detail: d.detail, values, value: bytes[d.offset as usize] }})");
            });
            fmtln!(fmt, ")");
        });
    });
}

/// Generates a `all()` function with all options for this enum
fn gen_enum_all(name: &str, values: &[&'static str], fmt: &mut Formatter) {
    fmtln!(
        fmt,
        "/// Returns a slice with all possible [{}] values.",
        name
    );
    fmt.add_block(&format!("pub fn all() -> &'static [{name}]"), |fmt| {
        fmtln!(fmt, "&[");
        fmt.indent(|fmt| {
            for v in values.iter() {
                fmtln!(fmt, "Self::{},", camel_case(v));
            }
        });
        fmtln!(fmt, "]");
    });
}

/// Emit Display and FromStr implementations for enum settings.
fn gen_to_and_from_str(name: &str, values: &[&'static str], fmt: &mut Formatter) {
    fmt.add_block(&format!("impl fmt::Display for {name}"), |fmt| {
        fmt.add_block(
            "fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result",
            |fmt| {
                fmt.add_block("f.write_str(match *self", |fmt| {
                    for v in values.iter() {
                        fmtln!(fmt, "Self::{} => \"{}\",", camel_case(v), v);
                    }
                });
                fmtln!(fmt, ")");
            },
        );
    });

    fmt.add_block(&format!("impl core::str::FromStr for {name}"), |fmt| {
        fmtln!(fmt, "type Err = ();");
        fmt.add_block("fn from_str(s: &str) -> Result<Self, Self::Err>", |fmt| {
            fmt.add_block("match s", |fmt| {
                for v in values.iter() {
                    fmtln!(fmt, "\"{}\" => Ok(Self::{}),", v, camel_case(v));
                }
                fmtln!(fmt, "_ => Err(()),");
            });
        });
    });
}

/// Emit real enum for the Enum settings.
fn gen_enum_types(group: &SettingGroup, fmt: &mut Formatter) {
    for setting in group.settings.iter() {
        let values = match setting.specific {
            SpecificSetting::Bool(_) | SpecificSetting::Num(_) => continue,
            SpecificSetting::Enum(ref values) => values,
        };
        let name = camel_case(setting.name);

        fmt.doc_comment(format!("Values for `{}.{}`.", group.name, setting.name));
        fmtln!(fmt, "#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]");
        fmt.add_block(&format!("pub enum {name}"), |fmt| {
            for v in values.iter() {
                fmt.doc_comment(format!("`{v}`."));
                fmtln!(fmt, "{},", camel_case(v));
            }
        });

        fmt.add_block(&format!("impl {name}"), |fmt| {
            gen_enum_all(&name, values, fmt);
        });

        gen_to_and_from_str(&name, values, fmt);
    }
}

/// Emit a getter function for `setting`.
fn gen_getter(setting: &Setting, fmt: &mut Formatter) {
    fmt.doc_comment(format!("{}\n{}", setting.description, setting.comment));
    match setting.specific {
        SpecificSetting::Bool(BoolSetting {
            predicate_number, ..
        }) => {
            fmt.add_block(&format!("pub fn {}(&self) -> bool", setting.name), |fmt| {
                fmtln!(fmt, "self.numbered_predicate({})", predicate_number);
            });
        }
        SpecificSetting::Enum(ref values) => {
            let ty = camel_case(setting.name);
            fmt.add_block(
                &format!("pub fn {}(&self) -> {}", setting.name, ty),
                |fmt| {
                    let mut m = Match::new(format!("self.bytes[{}]", setting.byte_offset));
                    for (i, v) in values.iter().enumerate() {
                        m.arm_no_fields(format!("{i}"), format!("{}::{}", ty, camel_case(v)));
                    }
                    m.arm_no_fields("_", "panic!(\"Invalid enum value\")");
                    fmt.add_match(m);
                },
            );
        }
        SpecificSetting::Num(_) => {
            fmt.add_block(&format!("pub fn {}(&self) -> u8", setting.name), |fmt| {
                fmtln!(fmt, "self.bytes[{}]", setting.byte_offset);
            });
        }
    }
}

fn gen_pred_getter(predicate: &Predicate, group: &SettingGroup, fmt: &mut Formatter) {
    fmt.doc_comment(format!("Computed predicate `{}`.", predicate.render(group)));
    fmt.add_block(
        &format!("pub fn {}(&self) -> bool", predicate.name),
        |fmt| {
            fmtln!(fmt, "self.numbered_predicate({})", predicate.number);
        },
    );
}

/// Emits getters for each setting value.
fn gen_getters(group: &SettingGroup, fmt: &mut Formatter) {
    fmt.doc_comment("User-defined settings.");
    fmtln!(fmt, "#[allow(dead_code, reason = \"generated code\")]");
    fmt.add_block("impl Flags", |fmt| {
        if !group.settings.is_empty() {
            fmt.doc_comment("Dynamic numbered predicate getter.");
            fmt.add_block("fn numbered_predicate(&self, p: usize) -> bool", |fmt| {
                fmtln!(
                    fmt,
                    "self.bytes[{} + p / 8] & (1 << (p % 8)) != 0",
                    group.bool_start_byte_offset
                );
            });
        }

        for setting in &group.settings {
            gen_getter(setting, fmt);
        }
        for predicate in &group.predicates {
            gen_pred_getter(predicate, group, fmt);
        }
    });
}

#[derive(Hash, PartialEq, Eq)]
enum SettingOrPreset<'a> {
    Setting(&'a Setting),
    Preset(&'a Preset),
}

impl<'a> SettingOrPreset<'a> {
    fn name(&self) -> &str {
        match *self {
            SettingOrPreset::Setting(s) => s.name,
            SettingOrPreset::Preset(p) => p.name,
        }
    }
}

/// Emits DESCRIPTORS, ENUMERATORS, HASH_TABLE and PRESETS.
fn gen_descriptors(group: &SettingGroup, fmt: &mut Formatter) {
    let mut enum_table = UniqueSeqTable::new();

    let mut descriptor_index_map: HashMap<SettingOrPreset, usize> = HashMap::new();

    // Generate descriptors.
    fmtln!(
        fmt,
        "static DESCRIPTORS: [detail::Descriptor; {}] = [",
        group.settings.len() + group.presets.len()
    );
    fmt.indent(|fmt| {
        for (idx, setting) in group.settings.iter().enumerate() {
            fmt.add_block("detail::Descriptor", |fmt| {
                fmtln!(fmt, "name: \"{}\",", setting.name);
                fmtln!(fmt, "description: \"{}\",", setting.description);
                fmtln!(fmt, "offset: {},", setting.byte_offset);
                match setting.specific {
                    SpecificSetting::Bool(BoolSetting { bit_offset, .. }) => {
                        fmtln!(
                            fmt,
                            "detail: detail::Detail::Bool {{ bit: {} }},",
                            bit_offset
                        );
                    }
                    SpecificSetting::Enum(ref values) => {
                        let offset = enum_table.add(values);
                        fmtln!(
                            fmt,
                            "detail: detail::Detail::Enum {{ last: {}, enumerators: {} }},",
                            values.len() - 1,
                            offset
                        );
                    }
                    SpecificSetting::Num(_) => {
                        fmtln!(fmt, "detail: detail::Detail::Num,");
                    }
                }

                descriptor_index_map.insert(SettingOrPreset::Setting(setting), idx);
            });
            fmtln!(fmt, ",");
        }

        for (idx, preset) in group.presets.iter().enumerate() {
            fmt.add_block("detail::Descriptor", |fmt| {
                fmtln!(fmt, "name: \"{}\",", preset.name);
                fmtln!(fmt, "description: \"{}\",", preset.description);
                fmtln!(fmt, "offset: {},", (idx as u8) * group.settings_size);
                fmtln!(fmt, "detail: detail::Detail::Preset,");
            });
            fmtln!(fmt, ",");

            let whole_idx = idx + group.settings.len();
            descriptor_index_map.insert(SettingOrPreset::Preset(preset), whole_idx);
        }
    });
    fmtln!(fmt, "];");

    // Generate enumerators.
    fmtln!(fmt, "static ENUMERATORS: [&str; {}] = [", enum_table.len());
    fmt.indent(|fmt| {
        for enum_val in enum_table.iter() {
            fmtln!(fmt, "\"{}\",", enum_val);
        }
    });
    fmtln!(fmt, "];");

    // Generate hash table.
    let mut hash_entries: Vec<SettingOrPreset> = Vec::new();
    hash_entries.extend(group.settings.iter().map(SettingOrPreset::Setting));
    hash_entries.extend(group.presets.iter().map(SettingOrPreset::Preset));

    let hash_table = generate_table(hash_entries.iter(), hash_entries.len(), |entry| {
        simple_hash(entry.name())
    });
    fmtln!(fmt, "static HASH_TABLE: [u16; {}] = [", hash_table.len());
    fmt.indent(|fmt| {
        for h in &hash_table {
            match *h {
                Some(setting_or_preset) => fmtln!(
                    fmt,
                    "{},",
                    &descriptor_index_map
                        .get(setting_or_preset)
                        .unwrap()
                        .to_string()
                ),
                None => fmtln!(fmt, "0xffff,"),
            }
        }
    });
    fmtln!(fmt, "];");

    // Generate presets.
    fmtln!(
        fmt,
        "static PRESETS: [(u8, u8); {}] = [",
        group.presets.len() * (group.settings_size as usize)
    );
    fmt.indent(|fmt| {
        for preset in &group.presets {
            fmt.comment(format!(
                "{}: {}",
                preset.name,
                preset.setting_names(group).collect::<Vec<_>>().join(", ")
            ));
            for (mask, value) in preset.layout(group) {
                fmtln!(fmt, "(0b{:08b}, 0b{:08b}),", mask, value);
            }
        }
    });
    fmtln!(fmt, "];");
}

fn gen_template(group: &SettingGroup, fmt: &mut Formatter) {
    let mut default_bytes: Vec<u8> = vec![0; group.settings_size as usize];
    for setting in &group.settings {
        *default_bytes.get_mut(setting.byte_offset as usize).unwrap() |= setting.default_byte();
    }

    let default_bytes: Vec<String> = default_bytes.iter().map(|x| format!("{x:#04x}")).collect();
    let default_bytes_str = default_bytes.join(", ");

    fmt.add_block(
        "static TEMPLATE: detail::Template = detail::Template",
        |fmt| {
            fmtln!(fmt, "name: \"{}\",", group.name);
            fmtln!(fmt, "descriptors: &DESCRIPTORS,");
            fmtln!(fmt, "enumerators: &ENUMERATORS,");
            fmtln!(fmt, "hash_table: &HASH_TABLE,");
            fmtln!(fmt, "defaults: &[{}],", default_bytes_str);
            fmtln!(fmt, "presets: &PRESETS,");
        },
    );
    fmtln!(fmt, ";");

    fmt.doc_comment(format!(
        "Create a `settings::Builder` for the {} settings group.",
        group.name
    ));
    fmt.add_block("pub fn builder() -> Builder", |fmt| {
        fmtln!(fmt, "Builder::new(&TEMPLATE)");
    });
}

fn gen_display(group: &SettingGroup, fmt: &mut Formatter) {
    fmt.add_block("impl fmt::Display for Flags", |fmt| {
        fmt.add_block(
            "fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result",
            |fmt| {
                fmtln!(fmt, "writeln!(f, \"[{}]\")?;", group.name);
                fmt.add_block("for d in &DESCRIPTORS", |fmt| {
                    fmt.add_block("if !d.detail.is_preset()", |fmt| {
                        fmtln!(fmt, "write!(f, \"{{}} = \", d.name)?;");
                        fmtln!(
                        fmt,
                        "TEMPLATE.format_toml_value(d.detail, self.bytes[d.offset as usize], f)?;",
                    );
                        fmtln!(fmt, "writeln!(f)?;");
                    });
                });
                fmtln!(fmt, "Ok(())");
            },
        );
    });
}

fn gen_hash_key(fmt: &mut Formatter) {
    fmt.add_block("impl Flags", |fmt| {
        fmt.doc_comment("Get the flag values as raw bytes for hashing.");
        fmt.add_block("pub fn hash_key(&self) -> &[u8]", |fmt| {
            fmtln!(fmt, "&self.bytes");
        });
    });
}

fn gen_group(group: &SettingGroup, parent: ParentGroup, fmt: &mut Formatter) {
    // Generate struct.
    fmtln!(fmt, "#[derive(Clone, PartialEq, Hash)]");
    fmt.doc_comment(format!("Flags group `{}`.", group.name));
    fmt.add_block("pub struct Flags", |fmt| {
        fmtln!(fmt, "bytes: [u8; {}],", group.byte_size());
    });

    gen_constructor(group, parent, fmt);
    gen_iterator(group, fmt);
    gen_enum_types(group, fmt);
    gen_getters(group, fmt);
    gen_descriptors(group, fmt);
    gen_template(group, fmt);
    gen_display(group, fmt);
    gen_hash_key(fmt);
}

pub(crate) fn generate(
    settings: &SettingGroup,
    parent_group: ParentGroup,
    filename: &str,
    out_dir: &std::path::Path,
) -> Result<(), error::Error> {
    let mut fmt = Formatter::new(Language::Rust);
    gen_group(settings, parent_group, &mut fmt);
    fmt.write(filename, out_dir)?;
    Ok(())
}

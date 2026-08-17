//! Shared settings module.
//!
//! This module defines data structures to access the settings defined in the meta language.
//!
//! Each settings group is translated to a `Flags` struct either in this module or in its
//! ISA-specific `settings` module. The struct provides individual getter methods for all of the
//! settings as well as computed predicate flags.
//!
//! The `Flags` struct is immutable once it has been created. A `Builder` instance is used to
//! create it.
//!
//! # Example
//! ```
//! use cranelift_codegen::settings::{self, Configurable};
//!
//! let mut b = settings::builder();
//! b.set("opt_level", "speed_and_size");
//!
//! let f = settings::Flags::new(b);
//! assert_eq!(f.opt_level(), settings::OptLevel::SpeedAndSize);
//! ```

use crate::constant_hash::{probe, simple_hash};
use crate::isa::TargetIsa;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt;
use core::str;

/// A string-based configurator for settings groups.
///
/// The `Configurable` protocol allows settings to be modified by name before a finished `Flags`
/// struct is created.
pub trait Configurable {
    /// Set the string value of any setting by name.
    ///
    /// This can set any type of setting whether it is numeric, boolean, or enumerated.
    fn set(&mut self, name: &str, value: &str) -> SetResult<()>;

    /// Enable a boolean setting or apply a preset.
    ///
    /// If the identified setting isn't a boolean or a preset, a `BadType` error is returned.
    fn enable(&mut self, name: &str) -> SetResult<()>;
}

/// Represents the kind of setting.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SettingKind {
    /// The setting is an enumeration.
    Enum,
    /// The setting is a number.
    Num,
    /// The setting is a boolean.
    Bool,
    /// The setting is a preset.
    Preset,
}

/// Represents an available builder setting.
///
/// This is used for iterating settings in a builder.
#[derive(Clone, Copy, Debug)]
pub struct Setting {
    /// The name of the setting.
    pub name: &'static str,
    /// The description of the setting.
    pub description: &'static str,
    /// The kind of the setting.
    pub kind: SettingKind,
    /// The supported values of the setting (for enum values).
    pub values: Option<&'static [&'static str]>,
}

/// Represents a setting value.
///
/// This is used for iterating values in `Flags`.
pub struct Value {
    /// The name of the setting associated with this value.
    pub name: &'static str,
    pub(crate) detail: detail::Detail,
    pub(crate) values: Option<&'static [&'static str]>,
    pub(crate) value: u8,
}

impl Value {
    /// Gets the kind of setting.
    pub fn kind(&self) -> SettingKind {
        match &self.detail {
            detail::Detail::Enum { .. } => SettingKind::Enum,
            detail::Detail::Num => SettingKind::Num,
            detail::Detail::Bool { .. } => SettingKind::Bool,
            detail::Detail::Preset => unreachable!(),
        }
    }

    /// Gets the enum value if the value is from an enum setting.
    pub fn as_enum(&self) -> Option<&'static str> {
        self.values.map(|v| v[self.value as usize])
    }

    /// Gets the numerical value if the value is from a num setting.
    pub fn as_num(&self) -> Option<u8> {
        match &self.detail {
            detail::Detail::Num => Some(self.value),
            _ => None,
        }
    }

    /// Gets the boolean value if the value is from a boolean setting.
    pub fn as_bool(&self) -> Option<bool> {
        match &self.detail {
            detail::Detail::Bool { bit } => Some(self.value & (1 << bit) != 0),
            _ => None,
        }
    }

    /// Builds a string from the current value
    pub fn value_string(&self) -> String {
        match self.kind() {
            SettingKind::Enum => self.as_enum().map(|b| b.to_string()),
            SettingKind::Num => self.as_num().map(|b| b.to_string()),
            SettingKind::Bool => self.as_bool().map(|b| b.to_string()),
            SettingKind::Preset => unreachable!(),
        }
        .unwrap()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(enum_variant) = self.as_enum() {
            write!(f, "{}={}", self.name, enum_variant)
        } else if let Some(num) = self.as_num() {
            write!(f, "{}={}", self.name, num)
        } else if let Some(b) = self.as_bool() {
            if b {
                write!(f, "{}=1", self.name)
            } else {
                write!(f, "{}=0", self.name)
            }
        } else {
            unreachable!()
        }
    }
}

/// Collect settings values based on a template.
#[derive(Clone, Hash)]
pub struct Builder {
    template: &'static detail::Template,
    bytes: Box<[u8]>,
}

impl Builder {
    /// Create a new builder with defaults and names from the given template.
    pub fn new(tmpl: &'static detail::Template) -> Self {
        Self {
            template: tmpl,
            bytes: tmpl.defaults.into(),
        }
    }

    /// Extract contents of builder once everything is configured.
    pub fn state_for(&self, name: &str) -> &[u8] {
        assert_eq!(name, self.template.name);
        &self.bytes
    }

    /// Iterates the available settings in the builder.
    pub fn iter(&self) -> impl Iterator<Item = Setting> + use<> {
        let template = self.template;

        template.descriptors.iter().map(move |d| {
            let (kind, values) = match d.detail {
                detail::Detail::Enum { last, enumerators } => {
                    let values = template.enums(last, enumerators);
                    (SettingKind::Enum, Some(values))
                }
                detail::Detail::Num => (SettingKind::Num, None),
                detail::Detail::Bool { .. } => (SettingKind::Bool, None),
                detail::Detail::Preset => (SettingKind::Preset, None),
            };

            Setting {
                name: d.name,
                description: d.description,
                kind,
                values,
            }
        })
    }

    /// Set the value of a single bit.
    fn set_bit(&mut self, offset: usize, bit: u8, value: bool) {
        let byte = &mut self.bytes[offset];
        let mask = 1 << bit;
        if value {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }

    /// Apply a preset. The argument is a slice of (mask, value) bytes.
    fn apply_preset(&mut self, values: &[(u8, u8)]) {
        for (byte, &(mask, value)) in self.bytes.iter_mut().zip(values) {
            *byte = (*byte & !mask) | value;
        }
    }

    /// Look up a descriptor by name.
    fn lookup(&self, name: &str) -> SetResult<(usize, detail::Detail)> {
        match probe(self.template, name, simple_hash(name)) {
            Err(_) => Err(SetError::BadName(name.to_string())),
            Ok(entry) => {
                let d = &self.template.descriptors[self.template.hash_table[entry] as usize];
                Ok((d.offset as usize, d.detail))
            }
        }
    }
}

fn parse_bool_value(value: &str) -> SetResult<bool> {
    match value {
        "true" | "on" | "yes" | "1" => Ok(true),
        "false" | "off" | "no" | "0" => Ok(false),
        _ => Err(SetError::BadValue("bool".to_string())),
    }
}

fn parse_enum_value(value: &str, choices: &[&str]) -> SetResult<u8> {
    match choices.iter().position(|&tag| tag == value) {
        Some(idx) => Ok(idx as u8),
        None => Err(SetError::BadValue(format!(
            "any among {}",
            choices.join(", ")
        ))),
    }
}

impl Configurable for Builder {
    fn enable(&mut self, name: &str) -> SetResult<()> {
        use self::detail::Detail;
        let (offset, detail) = self.lookup(name)?;
        match detail {
            Detail::Bool { bit } => {
                self.set_bit(offset, bit, true);
                Ok(())
            }
            Detail::Preset => {
                self.apply_preset(&self.template.presets[offset..]);
                Ok(())
            }
            _ => Err(SetError::BadType),
        }
    }

    fn set(&mut self, name: &str, value: &str) -> SetResult<()> {
        use self::detail::Detail;
        let (offset, detail) = self.lookup(name)?;
        match detail {
            Detail::Bool { bit } => {
                self.set_bit(offset, bit, parse_bool_value(value)?);
            }
            Detail::Num => {
                self.bytes[offset] = value
                    .parse()
                    .map_err(|_| SetError::BadValue("number".to_string()))?;
            }
            Detail::Enum { last, enumerators } => {
                self.bytes[offset] =
                    parse_enum_value(value, self.template.enums(last, enumerators))?;
            }
            Detail::Preset => return Err(SetError::BadName(name.to_string())),
        }
        Ok(())
    }
}

/// An error produced when changing a setting.
#[derive(Debug, PartialEq, Eq)]
pub enum SetError {
    /// No setting by this name exists.
    BadName(String),

    /// Type mismatch for setting (e.g., setting an enum setting as a bool).
    BadType,

    /// This is not a valid value for this setting.
    BadValue(String),
}

impl std::error::Error for SetError {}

impl fmt::Display for SetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SetError::BadName(name) => write!(f, "No existing setting named '{name}'"),
            SetError::BadType => {
                write!(f, "Trying to set a setting with the wrong type")
            }
            SetError::BadValue(value) => {
                write!(f, "Unexpected value for a setting, expected {value}")
            }
        }
    }
}

/// A result returned when changing a setting.
pub type SetResult<T> = Result<T, SetError>;

/// Implementation details for generated code.
///
/// This module holds definitions that need to be public so the can be instantiated by generated
/// code in other modules.
pub mod detail {
    use crate::constant_hash;
    use core::fmt;
    use core::hash::Hash;

    /// An instruction group template.
    #[derive(Hash)]
    pub struct Template {
        /// Name of the instruction group.
        pub name: &'static str,
        /// List of setting descriptors.
        pub descriptors: &'static [Descriptor],
        /// Union of all enumerators.
        pub enumerators: &'static [&'static str],
        /// Hash table of settings.
        pub hash_table: &'static [u16],
        /// Default values.
        pub defaults: &'static [u8],
        /// Pairs of (mask, value) for presets.
        pub presets: &'static [(u8, u8)],
    }

    impl Template {
        /// Get enumerators corresponding to a `Details::Enum`.
        pub fn enums(&self, last: u8, enumerators: u16) -> &[&'static str] {
            let from = enumerators as usize;
            let len = usize::from(last) + 1;
            &self.enumerators[from..from + len]
        }

        /// Format a setting value as a TOML string. This is mostly for use by the generated
        /// `Display` implementation.
        pub fn format_toml_value(
            &self,
            detail: Detail,
            byte: u8,
            f: &mut fmt::Formatter,
        ) -> fmt::Result {
            match detail {
                Detail::Bool { bit } => write!(f, "{}", (byte & (1 << bit)) != 0),
                Detail::Num => write!(f, "{byte}"),
                Detail::Enum { last, enumerators } => {
                    if byte <= last {
                        let tags = self.enums(last, enumerators);
                        write!(f, "\"{}\"", tags[usize::from(byte)])
                    } else {
                        write!(f, "{byte}")
                    }
                }
                // Presets aren't printed. They are reflected in the other settings.
                Detail::Preset { .. } => Ok(()),
            }
        }
    }

    /// The template contains a hash table for by-name lookup.
    impl<'a> constant_hash::Table<&'a str> for Template {
        fn len(&self) -> usize {
            self.hash_table.len()
        }

        fn key(&self, idx: usize) -> Option<&'a str> {
            let e = self.hash_table[idx] as usize;
            if e < self.descriptors.len() {
                Some(self.descriptors[e].name)
            } else {
                None
            }
        }
    }

    /// A setting descriptor holds the information needed to generically set and print a setting.
    ///
    /// Each settings group will be represented as a constant DESCRIPTORS array.
    #[derive(Hash)]
    pub struct Descriptor {
        /// Lower snake-case name of setting as defined in meta.
        pub name: &'static str,

        /// The description of the setting.
        pub description: &'static str,

        /// Offset of byte containing this setting.
        pub offset: u32,

        /// Additional details, depending on the kind of setting.
        pub detail: Detail,
    }

    /// The different kind of settings along with descriptor bits that depend on the kind.
    #[derive(Clone, Copy, Hash)]
    pub enum Detail {
        /// A boolean setting only uses one bit, numbered from LSB.
        Bool {
            /// 0-7.
            bit: u8,
        },

        /// A numerical setting uses the whole byte.
        Num,

        /// An Enum setting uses a range of enumerators.
        Enum {
            /// Numerical value of last enumerator, allowing for 1-256 enumerators.
            last: u8,

            /// First enumerator in the ENUMERATORS table.
            enumerators: u16,
        },

        /// A preset is not an individual setting, it is a collection of settings applied at once.
        ///
        /// The `Descriptor::offset` field refers to the `PRESETS` table.
        Preset,
    }

    impl Detail {
        /// Check if a detail is a Detail::Preset. Useful because the Descriptor
        /// offset field has a different meaning when the detail is a preset.
        pub fn is_preset(self) -> bool {
            match self {
                Self::Preset => true,
                _ => false,
            }
        }
    }
}

// Include code generated by `meta/gen_settings.rs`. This file contains a public `Flags` struct
// with an implementation for all of the settings defined in
// `cranelift-codegen/meta/src/shared/settings.rs`.
include!(concat!(env!("OUT_DIR"), "/settings.rs"));

/// Wrapper containing flags and optionally a `TargetIsa` trait object.
///
/// A few passes need to access the flags but only optionally a target ISA. The `FlagsOrIsa`
/// wrapper can be used to pass either, and extract the flags so they are always accessible.
#[derive(Clone, Copy)]
pub struct FlagsOrIsa<'a> {
    /// Flags are always present.
    pub flags: &'a Flags,

    /// The ISA may not be present.
    pub isa: Option<&'a dyn TargetIsa>,
}

impl<'a> From<&'a Flags> for FlagsOrIsa<'a> {
    fn from(flags: &'a Flags) -> FlagsOrIsa<'a> {
        FlagsOrIsa { flags, isa: None }
    }
}

impl<'a> From<&'a dyn TargetIsa> for FlagsOrIsa<'a> {
    fn from(isa: &'a dyn TargetIsa) -> FlagsOrIsa<'a> {
        FlagsOrIsa {
            flags: isa.flags(),
            isa: Some(isa),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Configurable;
    use super::SetError::*;
    use super::{Flags, builder};
    use alloc::string::ToString;

    #[test]
    fn display_default() {
        let b = builder();
        let f = Flags::new(b);
        let actual = f.to_string();
        let expected = r#"[shared]
regalloc_algorithm = "backtracking"
opt_level = "none"
tls_model = "none"
stack_switch_model = "none"
libcall_call_conv = "isa_default"
probestack_size_log2 = 12
probestack_strategy = "outline"
bb_padding_log2_minus_one = 0
log2_min_function_alignment = 0
regalloc_checker = false
regalloc_verbose_logs = false
enable_alias_analysis = true
enable_verifier = true
enable_pcc = false
is_pic = false
use_colocated_libcalls = false
enable_float = true
enable_nan_canonicalization = false
enable_pinned_reg = false
enable_atomics = true
enable_safepoints = false
enable_llvm_abi_extensions = false
enable_multi_ret_implicit_sret = false
unwind_info = true
preserve_frame_pointers = false
machine_code_cfg_info = false
enable_probestack = false
enable_jump_tables = true
enable_heap_access_spectre_mitigation = true
enable_table_access_spectre_mitigation = true
enable_incremental_compilation_cache_checks = false
"#;
        if actual != expected {
            panic!(
                "Default settings do not match expectations:\n\n{}",
                similar::TextDiff::from_lines(expected, &actual)
                    .unified_diff()
                    .header("expected", "actual")
            );
        }
        assert_eq!(f.opt_level(), super::OptLevel::None);
    }

    #[test]
    fn modify_bool() {
        let mut b = builder();
        assert_eq!(b.enable("not_there"), Err(BadName("not_there".to_string())));
        assert_eq!(b.enable("enable_atomics"), Ok(()));
        assert_eq!(b.set("enable_atomics", "false"), Ok(()));

        let f = Flags::new(b);
        assert_eq!(f.enable_atomics(), false);
    }

    #[test]
    fn modify_string() {
        let mut b = builder();
        assert_eq!(
            b.set("not_there", "true"),
            Err(BadName("not_there".to_string()))
        );
        assert_eq!(
            b.set("enable_atomics", ""),
            Err(BadValue("bool".to_string()))
        );
        assert_eq!(
            b.set("enable_atomics", "best"),
            Err(BadValue("bool".to_string()))
        );
        assert_eq!(
            b.set("opt_level", "true"),
            Err(BadValue(
                "any among none, speed, speed_and_size".to_string()
            ))
        );
        assert_eq!(b.set("opt_level", "speed"), Ok(()));
        assert_eq!(b.set("enable_atomics", "0"), Ok(()));

        let f = Flags::new(b);
        assert_eq!(f.enable_atomics(), false);
        assert_eq!(f.opt_level(), super::OptLevel::Speed);
    }
}

use std::iter;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) struct BoolSettingIndex(usize);

#[derive(Hash, PartialEq, Eq)]
pub(crate) struct BoolSetting {
    pub default: bool,
    pub bit_offset: u8,
    pub predicate_number: u8,
}

#[derive(Hash, PartialEq, Eq)]
pub(crate) enum SpecificSetting {
    Bool(BoolSetting),
    Enum(Vec<&'static str>),
    Num(u8),
}

#[derive(Hash, PartialEq, Eq)]
pub(crate) struct Setting {
    pub name: &'static str,
    pub description: &'static str,
    pub comment: &'static str,
    pub specific: SpecificSetting,
    pub byte_offset: u8,
}

impl Setting {
    pub fn default_byte(&self) -> u8 {
        match self.specific {
            SpecificSetting::Bool(BoolSetting {
                default,
                bit_offset,
                ..
            }) => {
                if default {
                    1 << bit_offset
                } else {
                    0
                }
            }
            SpecificSetting::Enum(_) => 0,
            SpecificSetting::Num(default) => default,
        }
    }

    fn byte_for_value(&self, v: bool) -> u8 {
        match self.specific {
            SpecificSetting::Bool(BoolSetting { bit_offset, .. }) => {
                if v {
                    1 << bit_offset
                } else {
                    0
                }
            }
            _ => panic!("byte_for_value shouldn't be used for non-boolean settings."),
        }
    }

    fn byte_mask(&self) -> u8 {
        match self.specific {
            SpecificSetting::Bool(BoolSetting { bit_offset, .. }) => 1 << bit_offset,
            _ => panic!("byte_for_value shouldn't be used for non-boolean settings."),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub(crate) struct PresetIndex(usize);

#[derive(Hash, PartialEq, Eq)]
pub(crate) enum PresetType {
    BoolSetting(BoolSettingIndex),
    OtherPreset(PresetIndex),
}

impl From<BoolSettingIndex> for PresetType {
    fn from(bool_setting_index: BoolSettingIndex) -> Self {
        PresetType::BoolSetting(bool_setting_index)
    }
}
impl From<PresetIndex> for PresetType {
    fn from(value: PresetIndex) -> Self {
        PresetType::OtherPreset(value)
    }
}

#[derive(Hash, PartialEq, Eq)]
pub(crate) struct Preset {
    pub name: &'static str,
    pub description: &'static str,
    values: Vec<BoolSettingIndex>,
}

impl Preset {
    pub fn layout(&self, group: &SettingGroup) -> Vec<(u8, u8)> {
        let mut layout: Vec<(u8, u8)> = iter::repeat((0, 0))
            .take(group.settings_size as usize)
            .collect();
        for bool_index in &self.values {
            let setting = &group.settings[bool_index.0];
            let mask = setting.byte_mask();
            let val = setting.byte_for_value(true);
            assert!((val & !mask) == 0);
            let (ref mut l_mask, ref mut l_val) =
                *layout.get_mut(setting.byte_offset as usize).unwrap();
            *l_mask |= mask;
            *l_val = (*l_val & !mask) | val;
        }
        layout
    }

    pub fn setting_names<'a>(
        &'a self,
        group: &'a SettingGroup,
    ) -> impl Iterator<Item = &'static str> + 'a {
        self.values
            .iter()
            .map(|bool_index| group.settings[bool_index.0].name)
    }
}

pub(crate) struct SettingGroup {
    pub name: &'static str,
    pub settings: Vec<Setting>,
    pub bool_start_byte_offset: u8,
    pub settings_size: u8,
    pub presets: Vec<Preset>,
    pub predicates: Vec<Predicate>,
}

impl SettingGroup {
    fn num_bool_settings(&self) -> u8 {
        self.settings
            .iter()
            .filter(|s| matches!(s.specific, SpecificSetting::Bool(_)))
            .count() as u8
    }

    pub fn byte_size(&self) -> u8 {
        let num_predicates = self.num_bool_settings() + (self.predicates.len() as u8);
        self.bool_start_byte_offset + (num_predicates + 7) / 8
    }
}

/// This is the basic information needed to track the specific parts of a setting when building
/// them.
pub(crate) enum ProtoSpecificSetting {
    Bool(bool),
    Enum(Vec<&'static str>),
    Num(u8),
}

/// This is the information provided during building for a setting.
struct ProtoSetting {
    name: &'static str,
    description: &'static str,
    comment: &'static str,
    specific: ProtoSpecificSetting,
}

#[derive(Hash, PartialEq, Eq)]
pub(crate) enum PredicateNode {
    OwnedBool(BoolSettingIndex),
    SharedBool(&'static str, &'static str),
    And(Box<PredicateNode>, Box<PredicateNode>),
}

impl From<BoolSettingIndex> for PredicateNode {
    fn from(bool_setting_index: BoolSettingIndex) -> Self {
        PredicateNode::OwnedBool(bool_setting_index)
    }
}

impl<'a> From<(BoolSettingIndex, &'a SettingGroup)> for PredicateNode {
    fn from(val: (BoolSettingIndex, &'a SettingGroup)) -> Self {
        let (index, group) = (val.0, val.1);
        let setting = &group.settings[index.0];
        PredicateNode::SharedBool(group.name, setting.name)
    }
}

impl PredicateNode {
    fn render(&self, group: &SettingGroup) -> String {
        match *self {
            PredicateNode::OwnedBool(bool_setting_index) => format!(
                "{}.{}()",
                group.name, group.settings[bool_setting_index.0].name
            ),
            PredicateNode::SharedBool(ref group_name, ref bool_name) => {
                format!("{group_name}.{bool_name}()")
            }
            PredicateNode::And(ref lhs, ref rhs) => {
                format!("{} && {}", lhs.render(group), rhs.render(group))
            }
        }
    }
}

struct ProtoPredicate {
    pub name: &'static str,
    node: PredicateNode,
}

pub(crate) type SettingPredicateNumber = u8;

pub(crate) struct Predicate {
    pub name: &'static str,
    node: PredicateNode,
    pub number: SettingPredicateNumber,
}

impl Predicate {
    pub fn render(&self, group: &SettingGroup) -> String {
        self.node.render(group)
    }
}

pub(crate) struct SettingGroupBuilder {
    name: &'static str,
    settings: Vec<ProtoSetting>,
    presets: Vec<Preset>,
    predicates: Vec<ProtoPredicate>,
}

impl SettingGroupBuilder {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            settings: Vec::new(),
            presets: Vec::new(),
            predicates: Vec::new(),
        }
    }

    fn add_setting(
        &mut self,
        name: &'static str,
        description: &'static str,
        comment: &'static str,
        specific: ProtoSpecificSetting,
    ) {
        self.settings.push(ProtoSetting {
            name,
            description,
            comment,
            specific,
        })
    }

    pub fn add_bool(
        &mut self,
        name: &'static str,
        description: &'static str,
        comment: &'static str,
        default: bool,
    ) -> BoolSettingIndex {
        assert!(
            self.predicates.is_empty(),
            "predicates must be added after the boolean settings"
        );
        self.add_setting(
            name,
            description,
            comment,
            ProtoSpecificSetting::Bool(default),
        );
        BoolSettingIndex(self.settings.len() - 1)
    }

    pub fn add_enum(
        &mut self,
        name: &'static str,
        description: &'static str,
        comment: &'static str,
        values: Vec<&'static str>,
    ) {
        self.add_setting(
            name,
            description,
            comment,
            ProtoSpecificSetting::Enum(values),
        );
    }

    pub fn add_num(
        &mut self,
        name: &'static str,
        description: &'static str,
        comment: &'static str,
        default: u8,
    ) {
        self.add_setting(
            name,
            description,
            comment,
            ProtoSpecificSetting::Num(default),
        );
    }

    pub fn add_predicate(&mut self, name: &'static str, node: PredicateNode) {
        self.predicates.push(ProtoPredicate { name, node });
    }

    pub fn add_preset(
        &mut self,
        name: &'static str,
        description: &'static str,
        args: Vec<PresetType>,
    ) -> PresetIndex {
        let mut values = Vec::new();
        for arg in args {
            match arg {
                PresetType::OtherPreset(index) => {
                    values.extend(self.presets[index.0].values.iter());
                }
                PresetType::BoolSetting(index) => values.push(index),
            }
        }
        self.presets.push(Preset {
            name,
            description,
            values,
        });
        PresetIndex(self.presets.len() - 1)
    }

    /// Compute the layout of the byte vector used to represent this settings
    /// group.
    ///
    /// The byte vector contains the following entries in order:
    ///
    /// 1. Byte-sized settings like `NumSetting` and `EnumSetting`.
    /// 2. `BoolSetting` settings.
    /// 3. Precomputed named predicates.
    /// 4. Other numbered predicates, including parent predicates that need to be accessible by
    ///    number.
    ///
    /// Set `self.settings_size` to the length of the byte vector prefix that
    /// contains the settings. All bytes after that are computed, not
    /// configured.
    ///
    /// Set `self.boolean_offset` to the beginning of the numbered predicates,
    /// 2. in the list above.
    ///
    /// Assign `byte_offset` and `bit_offset` fields in all settings.
    pub fn build(self) -> SettingGroup {
        let mut group = SettingGroup {
            name: self.name,
            settings: Vec::new(),
            bool_start_byte_offset: 0,
            settings_size: 0,
            presets: Vec::new(),
            predicates: Vec::new(),
        };

        let mut byte_offset = 0;

        // Assign the non-boolean settings first.
        for s in &self.settings {
            let specific = match s.specific {
                ProtoSpecificSetting::Bool(..) => continue,
                ProtoSpecificSetting::Enum(ref values) => SpecificSetting::Enum(values.clone()),
                ProtoSpecificSetting::Num(default) => SpecificSetting::Num(default),
            };

            group.settings.push(Setting {
                name: s.name,
                description: s.description,
                comment: s.comment,
                byte_offset,
                specific,
            });

            byte_offset += 1;
        }

        group.bool_start_byte_offset = byte_offset;

        let mut predicate_number = 0;

        // Then the boolean settings.
        for s in &self.settings {
            let default = match s.specific {
                ProtoSpecificSetting::Bool(default) => default,
                ProtoSpecificSetting::Enum(_) | ProtoSpecificSetting::Num(_) => continue,
            };
            group.settings.push(Setting {
                name: s.name,
                description: s.description,
                comment: s.comment,
                byte_offset: byte_offset + predicate_number / 8,
                specific: SpecificSetting::Bool(BoolSetting {
                    default,
                    bit_offset: predicate_number % 8,
                    predicate_number,
                }),
            });
            predicate_number += 1;
        }

        assert!(
            group.predicates.is_empty(),
            "settings_size is the byte size before adding predicates"
        );
        group.settings_size = group.byte_size();

        // Sort predicates by name to ensure the same order as the Python code.
        let mut predicates = self.predicates;
        predicates.sort_by_key(|predicate| predicate.name);

        group
            .predicates
            .extend(predicates.into_iter().map(|predicate| {
                let number = predicate_number;
                predicate_number += 1;
                Predicate {
                    name: predicate.name,
                    node: predicate.node,
                    number,
                }
            }));

        group.presets.extend(self.presets);

        group
    }
}

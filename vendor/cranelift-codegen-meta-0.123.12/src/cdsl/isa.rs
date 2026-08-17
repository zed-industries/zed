use crate::cdsl::settings::SettingGroup;

pub(crate) struct TargetIsa {
    pub name: &'static str,
    pub settings: SettingGroup,
}

impl TargetIsa {
    pub fn new(name: &'static str, settings: SettingGroup) -> Self {
        Self { name, settings }
    }
}

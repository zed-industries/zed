use crate::{
    PlatformKeyboardLayout, PlatformKeyboardMapper, ScanCode, is_alphabetic_key, is_immutable_key,
};

pub(crate) struct LinuxKeyboardLayout {
    id: String,
}

impl PlatformKeyboardLayout for LinuxKeyboardLayout {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.id
    }
}

impl LinuxKeyboardLayout {
    pub(crate) fn new(id: String) -> Self {
        Self { id }
    }
}

pub(crate) struct LinuxKeyboardMapper;

impl PlatformKeyboardMapper for LinuxKeyboardMapper {
    fn scan_code_to_key(&self, scan_code: ScanCode) -> anyhow::Result<String> {
        // todo(linux)
        Ok(scan_code.to_key().to_string())
    }

    fn get_shifted_key(&self, key: &str) -> anyhow::Result<String> {
        if is_immutable_key(key) {
            return Ok(key.to_string());
        }
        if is_alphabetic_key(key) {
            return Ok(key.to_uppercase());
        }
        // todo(linux)
        Ok(key.to_string())
    }
}

impl LinuxKeyboardMapper {
    pub(crate) fn new() -> Self {
        Self
    }
}

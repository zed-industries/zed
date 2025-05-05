use crate::{PlatformKeyboardLayout, PlatformKeyboardMapper, ScanCode, is_alphabetic_key};

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

    fn get_shifted_key(&self, key: &str) -> anyhow::Result<Option<String>> {
        if key.chars().count() != 1 {
            return Ok(None);
        }
        if is_alphabetic_key(key) {
            return Ok(Some(key.to_uppercase()));
        }
        // todo(linux)
        Ok(Some(key.to_string()))
    }
}

impl LinuxKeyboardMapper {
    pub(crate) fn new() -> Self {
        Self
    }
}

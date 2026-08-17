use crate::cached_size::CachedSize;
use crate::UnknownFields;

/// Special fields included in each generated message.
#[derive(Default, Eq, PartialEq, Clone, Debug, Hash)]
pub struct SpecialFields {
    unknown_fields: UnknownFields,
    cached_size: CachedSize,
}

impl SpecialFields {
    /// Defaults.
    pub const fn new() -> SpecialFields {
        SpecialFields {
            unknown_fields: UnknownFields::new(),
            cached_size: CachedSize::new(),
        }
    }

    /// Clear.
    pub fn clear(&mut self) {
        self.unknown_fields.clear();
        // No need to clear `cached_size`.
    }

    /// Getter.
    pub fn cached_size(&self) -> &CachedSize {
        &self.cached_size
    }

    /// Getter.
    pub fn unknown_fields(&self) -> &UnknownFields {
        &self.unknown_fields
    }

    /// Setter.
    pub fn mut_unknown_fields(&mut self) -> &mut UnknownFields {
        &mut self.unknown_fields
    }
}

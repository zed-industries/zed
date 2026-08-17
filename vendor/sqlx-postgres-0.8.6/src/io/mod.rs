mod buf_mut;

pub use buf_mut::PgBufMutExt;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::{NonZeroU32, Saturating};

pub(crate) use sqlx_core::io::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct StatementId(IdInner);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct PortalId(IdInner);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct IdInner(Option<NonZeroU32>);

pub(crate) struct DisplayId {
    prefix: &'static str,
    id: NonZeroU32,
}

impl StatementId {
    #[allow(dead_code)]
    pub const UNNAMED: Self = Self(IdInner::UNNAMED);

    pub const NAMED_START: Self = Self(IdInner::NAMED_START);

    #[cfg(test)]
    pub const TEST_VAL: Self = Self(IdInner::TEST_VAL);

    const NAME_PREFIX: &'static str = "sqlx_s_";

    pub fn next(&self) -> Self {
        Self(self.0.next())
    }

    pub fn name_len(&self) -> Saturating<usize> {
        self.0.name_len(Self::NAME_PREFIX)
    }

    /// Get a type to format this statement ID with [`Display`].
    ///
    /// Returns `None` if this is the unnamed statement.
    #[inline(always)]
    pub fn display(&self) -> Option<DisplayId> {
        self.0.display(Self::NAME_PREFIX)
    }

    pub fn put_name_with_nul(&self, buf: &mut Vec<u8>) {
        self.0.put_name_with_nul(Self::NAME_PREFIX, buf)
    }
}

impl Display for DisplayId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.prefix, self.id)
    }
}

#[allow(dead_code)]
impl PortalId {
    // None selects the unnamed portal
    pub const UNNAMED: Self = PortalId(IdInner::UNNAMED);

    pub const NAMED_START: Self = PortalId(IdInner::NAMED_START);

    #[cfg(test)]
    pub const TEST_VAL: Self = Self(IdInner::TEST_VAL);

    const NAME_PREFIX: &'static str = "sqlx_p_";

    /// If ID represents a named portal, return the next ID, wrapping on overflow.
    ///
    /// If this ID represents the unnamed portal, return the same.
    pub fn next(&self) -> Self {
        Self(self.0.next())
    }

    /// Calculate the number of bytes that will be written by [`Self::put_name_with_nul()`].
    pub fn name_len(&self) -> Saturating<usize> {
        self.0.name_len(Self::NAME_PREFIX)
    }

    pub fn put_name_with_nul(&self, buf: &mut Vec<u8>) {
        self.0.put_name_with_nul(Self::NAME_PREFIX, buf)
    }
}

impl IdInner {
    const UNNAMED: Self = Self(None);

    const NAMED_START: Self = Self(Some(NonZeroU32::MIN));

    #[cfg(test)]
    pub const TEST_VAL: Self = Self(NonZeroU32::new(1234567890));

    #[inline(always)]
    fn next(&self) -> Self {
        Self(
            self.0
                .map(|id| id.checked_add(1).unwrap_or(NonZeroU32::MIN)),
        )
    }

    #[inline(always)]
    fn display(&self, prefix: &'static str) -> Option<DisplayId> {
        self.0.map(|id| DisplayId { prefix, id })
    }

    #[inline(always)]
    fn name_len(&self, name_prefix: &str) -> Saturating<usize> {
        let mut len = Saturating(0);

        if let Some(id) = self.0 {
            len += name_prefix.len();
            // estimate the length of the ID in decimal
            // `.ilog10()` can't panic since the value is never zero
            len += id.get().ilog10() as usize;
            // add one to compensate for `ilog10()` rounding down.
            len += 1;
        }

        // count the NUL terminator
        len += 1;

        len
    }

    #[inline(always)]
    fn put_name_with_nul(&self, name_prefix: &str, buf: &mut Vec<u8>) {
        if let Some(id) = self.0 {
            buf.extend_from_slice(name_prefix.as_bytes());
            buf.extend_from_slice(itoa::Buffer::new().format(id.get()).as_bytes());
        }

        buf.push(0);
    }
}

#[test]
fn statement_id_display_matches_encoding() {
    const EXPECTED_STR: &str = "sqlx_s_1234567890";
    const EXPECTED_BYTES: &[u8] = b"sqlx_s_1234567890\0";

    let mut bytes = Vec::new();

    StatementId::TEST_VAL.put_name_with_nul(&mut bytes);

    assert_eq!(bytes, EXPECTED_BYTES);

    let str = StatementId::TEST_VAL.display().unwrap().to_string();

    assert_eq!(str, EXPECTED_STR);
}

// Several implementations of the internals of `Token` depending on the size of `usize`

use std::convert::TryInto;

#[cfg(target_pointer_width = "64")]
const BITS_VERSION: usize = 16;
#[cfg(target_pointer_width = "64")]
const BITS_SUBID: usize = 16;

#[cfg(target_pointer_width = "32")]
const BITS_VERSION: usize = 8;
#[cfg(target_pointer_width = "32")]
const BITS_SUBID: usize = 8;

#[cfg(target_pointer_width = "16")]
const BITS_VERSION: usize = 4;
#[cfg(target_pointer_width = "16")]
const BITS_SUBID: usize = 4;

const MASK_VERSION: usize = (1 << BITS_VERSION) - 1;
const MASK_SUBID: usize = (1 << BITS_SUBID) - 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct TokenInner {
    id: u32,
    version: u16,
    sub_id: u16,
}

impl TokenInner {
    pub(crate) fn new(id: usize) -> Result<TokenInner, ()> {
        Ok(TokenInner {
            id: id.try_into().map_err(|_| ())?,
            version: 0,
            sub_id: 0,
        })
    }

    pub(crate) fn get_id(self) -> usize {
        self.id as usize
    }

    pub(crate) fn same_source_as(self, other: TokenInner) -> bool {
        self.id == other.id && self.version == other.version
    }

    pub(crate) fn increment_version(self) -> TokenInner {
        TokenInner {
            id: self.id,
            version: self.version.wrapping_add(1) & (MASK_VERSION as u16),
            sub_id: 0,
        }
    }

    pub(crate) fn increment_sub_id(self) -> TokenInner {
        let sub_id = match self.sub_id.checked_add(1) {
            Some(sid) if sid <= (MASK_SUBID as u16) => sid,
            _ => panic!("Maximum number of sub-ids reached for source #{}", self.id),
        };

        TokenInner {
            id: self.id,
            version: self.version,
            sub_id,
        }
    }

    pub(crate) fn forget_sub_id(self) -> TokenInner {
        TokenInner {
            id: self.id,
            version: self.version,
            sub_id: 0,
        }
    }
}

impl From<usize> for TokenInner {
    fn from(value: usize) -> Self {
        let sub_id = (value & MASK_SUBID) as u16;
        let version = ((value >> BITS_SUBID) & MASK_VERSION) as u16;
        let id = (value >> (BITS_SUBID + BITS_VERSION)) as u32;
        TokenInner {
            id,
            version,
            sub_id,
        }
    }
}

impl From<TokenInner> for usize {
    fn from(token: TokenInner) -> Self {
        ((token.id as usize) << (BITS_SUBID + BITS_VERSION))
            + ((token.version as usize) << BITS_SUBID)
            + (token.sub_id as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn overflow_subid() {
        let token = TokenInner {
            id: 0,
            version: 0,
            sub_id: MASK_SUBID as u16,
        };
        token.increment_sub_id();
    }
}

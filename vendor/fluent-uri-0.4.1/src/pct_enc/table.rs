//! Byte pattern tables from RFC 3986 and RFC 3987.
//!
//! The predefined table constants in this module are documented with
//! the ABNF notation of [RFC 5234].
//!
//! [RFC 5234]: https://datatracker.ietf.org/doc/html/rfc5234

use crate::utf8;

const TABLE_LEN: usize = 256 + 3;
const INDEX_PCT_ENCODED: usize = 256;
const INDEX_UCSCHAR: usize = 256 + 1;
const INDEX_IPRIVATE: usize = 256 + 2;

const fn is_ucschar(x: u32) -> bool {
    matches!(x, 0xa0..=0xd7ff | 0xf900..=0xfdcf | 0xfdf0..=0xffef)
        || (x >= 0x10000 && x <= 0xdffff && (x & 0xffff) <= 0xfffd)
        || (x >= 0xe1000 && x <= 0xefffd)
}

const fn is_iprivate(x: u32) -> bool {
    (x >= 0xe000 && x <= 0xf8ff) || (x >= 0xf0000 && (x & 0xffff) <= 0xfffd)
}

/// A table specifying the byte patterns allowed in a string.
#[derive(Clone, Copy, Debug)]
pub struct Table {
    table: [bool; TABLE_LEN],
}

impl Table {
    /// Creates a table that only allows the given unencoded bytes.
    ///
    /// # Panics
    ///
    /// Panics if any of the bytes is not ASCII or equals `b'%'`.
    #[must_use]
    pub const fn new(mut bytes: &[u8]) -> Self {
        let mut table = [false; TABLE_LEN];
        while let [cur, rem @ ..] = bytes {
            assert!(
                cur.is_ascii() && *cur != b'%',
                "cannot allow non-ASCII byte or %"
            );
            table[*cur as usize] = true;
            bytes = rem;
        }
        Self { table }
    }

    /// Combines two tables into one.
    ///
    /// Returns a new table that allows all the byte patterns allowed
    /// by `self` or by `other`.
    #[must_use]
    pub const fn or(mut self, other: &Self) -> Self {
        let mut i = 0;
        while i < TABLE_LEN {
            self.table[i] |= other.table[i];
            i += 1;
        }
        self
    }

    /// Marks this table as allowing percent-encoded octets.
    #[must_use]
    pub const fn or_pct_encoded(mut self) -> Self {
        self.table[INDEX_PCT_ENCODED] = true;
        self
    }

    /// Marks this table as allowing characters matching the [`ucschar`]
    /// ABNF rule from RFC 3987.
    ///
    /// [`ucschar`]: https://datatracker.ietf.org/doc/html/rfc3987#section-2.2
    #[must_use]
    pub const fn or_ucschar(mut self) -> Self {
        self.table[INDEX_UCSCHAR] = true;
        self
    }

    /// Marks this table as allowing characters matching the [`iprivate`]
    /// ABNF rule from RFC 3987.
    ///
    /// [`iprivate`]: https://datatracker.ietf.org/doc/html/rfc3987#section-2.2
    #[must_use]
    pub const fn or_iprivate(mut self) -> Self {
        self.table[INDEX_IPRIVATE] = true;
        self
    }

    /// Subtracts from this table.
    ///
    /// Returns a new table that allows all the byte patterns allowed
    /// by `self` but not allowed by `other`.
    #[must_use]
    pub const fn sub(mut self, other: &Self) -> Self {
        let mut i = 0;
        while i < TABLE_LEN {
            self.table[i] &= !other.table[i];
            i += 1;
        }
        self
    }

    /// Checks whether the table is a subset of another, i.e., `other`
    /// allows at least all the byte patterns allowed by `self`.
    #[must_use]
    pub const fn is_subset(&self, other: &Self) -> bool {
        let mut i = 0;
        while i < TABLE_LEN {
            if self.table[i] & !other.table[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    #[inline]
    pub(crate) const fn allows_ascii(&self, x: u8) -> bool {
        self.table[x as usize]
    }

    #[inline]
    pub(crate) const fn allows_non_ascii(&self) -> bool {
        self.table[INDEX_UCSCHAR] | self.table[INDEX_IPRIVATE]
    }

    pub(crate) const fn allows_code_point(&self, x: u32) -> bool {
        if x < 128 {
            self.table[x as usize]
        } else {
            (self.table[INDEX_UCSCHAR] && is_ucschar(x))
                || (self.table[INDEX_IPRIVATE] && is_iprivate(x))
        }
    }

    /// Checks whether the given unencoded character is allowed by the table.
    #[inline]
    #[must_use]
    pub const fn allows(&self, ch: char) -> bool {
        self.allows_code_point(ch as u32)
    }

    /// Checks whether percent-encoded octets are allowed by the table.
    #[inline]
    #[must_use]
    pub const fn allows_pct_encoded(&self) -> bool {
        self.table[INDEX_PCT_ENCODED]
    }

    /// Validates the given string with the table.
    pub(crate) const fn validate(&self, s: &[u8]) -> bool {
        let mut i = 0;
        let allow_pct_encoded = self.allows_pct_encoded();
        let allow_non_ascii = self.allows_non_ascii();

        while i < s.len() {
            let x = s[i];
            if allow_pct_encoded && x == b'%' {
                if i + 2 >= s.len() {
                    return false;
                }
                let (hi, lo) = (s[i + 1], s[i + 2]);

                if !(HEXDIG.allows_ascii(hi) & HEXDIG.allows_ascii(lo)) {
                    return false;
                }
                i += 3;
            } else if allow_non_ascii {
                let (x, len) = utf8::next_code_point(s, i);
                if !self.allows_code_point(x) {
                    return false;
                }
                i += len;
            } else {
                if !self.allows_ascii(x) {
                    return false;
                }
                i += 1;
            }
        }
        true
    }
}

const fn new(bytes: &[u8]) -> Table {
    Table::new(bytes)
}

// Rules from RFC 3986:

/// `ALPHA = %x41-5A / %x61-7A`
pub const ALPHA: &Table = &new(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");

/// `DIGIT = %x30-39`
pub const DIGIT: &Table = &new(b"0123456789");

/// `HEXDIG = DIGIT / "A" / "B" / "C" / "D" / "E" / "F"`
pub const HEXDIG: &Table = &DIGIT.or(&new(b"ABCDEFabcdef"));

/// `scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )`
pub const SCHEME: &Table = &ALPHA.or(DIGIT).or(&new(b"+-."));

/// `userinfo = *( unreserved / pct-encoded / sub-delims / ":" )`
pub const USERINFO: &Table = &UNRESERVED.or(SUB_DELIMS).or(&new(b":")).or_pct_encoded();

/// `IPvFuture = "v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )`
pub const IPV_FUTURE: &Table = &UNRESERVED.or(SUB_DELIMS).or(&new(b":"));

/// `reg-name = *( unreserved / pct-encoded / sub-delims )`
pub const REG_NAME: &Table = &UNRESERVED.or(SUB_DELIMS).or_pct_encoded();

/// `path = *( pchar / "/" )`
pub const PATH: &Table = &PCHAR.or(&new(b"/"));

/// `segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )`
pub const SEGMENT_NZ_NC: &Table = &UNRESERVED.or(SUB_DELIMS).or(&new(b"@")).or_pct_encoded();

/// `pchar = unreserved / pct-encoded / sub-delims / ":" / "@"`
pub const PCHAR: &Table = &UNRESERVED.or(SUB_DELIMS).or(&new(b":@")).or_pct_encoded();

/// `query = *( pchar / "/" / "?" )`
pub const QUERY: &Table = &PCHAR.or(&new(b"/?"));

/// `fragment = *( pchar / "/" / "?" )`
pub const FRAGMENT: &Table = QUERY;

/// `unreserved = ALPHA / DIGIT / "-" / "." / "_" / "~"`
pub const UNRESERVED: &Table = &ALPHA.or(DIGIT).or(&new(b"-._~"));

/// `reserved = gen-delims / sub-delims`
pub const RESERVED: &Table = &GEN_DELIMS.or(SUB_DELIMS);

/// `gen-delims = ":" / "/" / "?" / "#" / "[" / "]" / "@"`
pub const GEN_DELIMS: &Table = &new(b":/?#[]@");

/// `sub-delims = "!" / "$" / "&" / "'" / "(" / ")"
///             / "*" / "+" / "," / ";" / "="`
pub const SUB_DELIMS: &Table = &new(b"!$&'()*+,;=");

// Rules from RFC 3987:

pub const IUSERINFO: &Table = &USERINFO.or_ucschar();
pub const IREG_NAME: &Table = &REG_NAME.or_ucschar();
pub const IPATH: &Table = &PATH.or_ucschar();
pub const ISEGMENT_NZ_NC: &Table = &SEGMENT_NZ_NC.or_ucschar();
pub const IQUERY: &Table = &QUERY.or_ucschar().or_iprivate();
pub const IFRAGMENT: &Table = &FRAGMENT.or_ucschar();

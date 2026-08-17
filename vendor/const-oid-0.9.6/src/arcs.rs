//! Arcs are integer values which exist within an OID's hierarchy.

use crate::{Error, ObjectIdentifier, Result};
use core::mem;

/// Type alias used to represent an "arc" (i.e. integer identifier value).
///
/// X.660 does not define a maximum size of an arc.
///
/// The current representation is `u32`, which has been selected as being
/// sufficient to cover the current PKCS/PKIX use cases this library has been
/// used in conjunction with.
///
/// Future versions may potentially make it larger if a sufficiently important
/// use case is discovered.
pub type Arc = u32;

/// Maximum value of the first arc in an OID.
pub(crate) const ARC_MAX_FIRST: Arc = 2;

/// Maximum value of the second arc in an OID.
pub(crate) const ARC_MAX_SECOND: Arc = 39;

/// Maximum number of bytes supported in an arc.
const ARC_MAX_BYTES: usize = mem::size_of::<Arc>();

/// Maximum value of the last byte in an arc.
const ARC_MAX_LAST_OCTET: u8 = 0b11110000; // Max bytes of leading 1-bits

/// [`Iterator`] over [`Arc`] values (a.k.a. nodes) in an [`ObjectIdentifier`].
///
/// This iterates over all arcs in an OID, including the root.
pub struct Arcs<'a> {
    /// OID we're iterating over
    oid: &'a ObjectIdentifier,

    /// Current position within the serialized DER bytes of this OID
    cursor: Option<usize>,
}

impl<'a> Arcs<'a> {
    /// Create a new iterator over the arcs of this OID
    pub(crate) fn new(oid: &'a ObjectIdentifier) -> Self {
        Self { oid, cursor: None }
    }

    /// Try to parse the next arc in this OID.
    ///
    /// This method is fallible so it can be used as a first pass to determine
    /// that the arcs in the OID are well-formed.
    pub(crate) fn try_next(&mut self) -> Result<Option<Arc>> {
        match self.cursor {
            // Indicates we're on the root OID
            None => {
                let root = RootArcs::try_from(self.oid.as_bytes()[0])?;
                self.cursor = Some(0);
                Ok(Some(root.first_arc()))
            }
            Some(0) => {
                let root = RootArcs::try_from(self.oid.as_bytes()[0])?;
                self.cursor = Some(1);
                Ok(Some(root.second_arc()))
            }
            Some(offset) => {
                let mut result = 0;
                let mut arc_bytes = 0;

                loop {
                    let len = checked_add!(offset, arc_bytes);

                    match self.oid.as_bytes().get(len).cloned() {
                        // The arithmetic below includes advance checks
                        // against `ARC_MAX_BYTES` and `ARC_MAX_LAST_OCTET`
                        // which ensure the operations will not overflow.
                        #[allow(clippy::integer_arithmetic)]
                        Some(byte) => {
                            arc_bytes = checked_add!(arc_bytes, 1);

                            if (arc_bytes > ARC_MAX_BYTES) && (byte & ARC_MAX_LAST_OCTET != 0) {
                                return Err(Error::ArcTooBig);
                            }

                            result = result << 7 | (byte & 0b1111111) as Arc;

                            if byte & 0b10000000 == 0 {
                                self.cursor = Some(checked_add!(offset, arc_bytes));
                                return Ok(Some(result));
                            }
                        }
                        None => {
                            if arc_bytes == 0 {
                                return Ok(None);
                            } else {
                                return Err(Error::Base128);
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<'a> Iterator for Arcs<'a> {
    type Item = Arc;

    fn next(&mut self) -> Option<Arc> {
        // ObjectIdentifier constructors should ensure the OID is well-formed
        self.try_next().expect("OID malformed")
    }
}

/// Byte containing the first and second arcs of an OID.
///
/// This is represented this way in order to reduce the overall size of the
/// [`ObjectIdentifier`] struct.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct RootArcs(u8);

impl RootArcs {
    /// Create [`RootArcs`] from the first and second arc values represented
    /// as `Arc` integers.
    pub(crate) const fn new(first_arc: Arc, second_arc: Arc) -> Result<Self> {
        if first_arc > ARC_MAX_FIRST {
            return Err(Error::ArcInvalid { arc: first_arc });
        }

        if second_arc > ARC_MAX_SECOND {
            return Err(Error::ArcInvalid { arc: second_arc });
        }

        // The checks above ensure this operation will not overflow
        #[allow(clippy::integer_arithmetic)]
        let byte = (first_arc * (ARC_MAX_SECOND + 1)) as u8 + second_arc as u8;

        Ok(Self(byte))
    }

    /// Get the value of the first arc
    #[allow(clippy::integer_arithmetic)]
    pub(crate) const fn first_arc(self) -> Arc {
        self.0 as Arc / (ARC_MAX_SECOND + 1)
    }

    /// Get the value of the second arc
    #[allow(clippy::integer_arithmetic)]
    pub(crate) const fn second_arc(self) -> Arc {
        self.0 as Arc % (ARC_MAX_SECOND + 1)
    }
}

impl TryFrom<u8> for RootArcs {
    type Error = Error;

    // Ensured not to overflow by constructor invariants
    #[allow(clippy::integer_arithmetic)]
    fn try_from(octet: u8) -> Result<Self> {
        let first = octet as Arc / (ARC_MAX_SECOND + 1);
        let second = octet as Arc % (ARC_MAX_SECOND + 1);
        let result = Self::new(first, second)?;
        debug_assert_eq!(octet, result.0);
        Ok(result)
    }
}

impl From<RootArcs> for u8 {
    fn from(root_arcs: RootArcs) -> u8 {
        root_arcs.0
    }
}

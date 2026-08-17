//! OID Names Database
//!
//! The contents of this database are generated from the official IANA
//! [Object Identifier Descriptors] Registry CSV file and from [RFC 5280].
//! If we are missing values you care about, please contribute a patch to
//! `oiddbgen` (a subcrate in the source code) to generate the values from
//! the relevant standard.
//!
//! [RFC 5280]: https://datatracker.ietf.org/doc/html/rfc5280
//! [Object Identifier Descriptors]: https://www.iana.org/assignments/ldap-parameters/ldap-parameters.xhtml#ldap-parameters-3

#![allow(clippy::integer_arithmetic, missing_docs)]

mod gen;

pub use gen::*;

use crate::{Error, ObjectIdentifier};

/// A const implementation of byte equals.
const fn eq(lhs: &[u8], rhs: &[u8]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }

    let mut i = 0usize;
    while i < lhs.len() {
        if lhs[i] != rhs[i] {
            return false;
        }

        i += 1;
    }

    true
}

/// A const implementation of case-insensitive ASCII equals.
const fn eq_case(lhs: &[u8], rhs: &[u8]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }

    let mut i = 0usize;
    while i < lhs.len() {
        if !lhs[i].eq_ignore_ascii_case(&rhs[i]) {
            return false;
        }

        i += 1;
    }

    true
}

/// A query interface for OIDs/Names.
#[derive(Copy, Clone)]
pub struct Database<'a>(&'a [(&'a ObjectIdentifier, &'a str)]);

impl<'a> Database<'a> {
    /// Looks up a name for an OID.
    ///
    /// Errors if the input is not a valid OID.
    /// Returns the input if no name is found.
    pub fn resolve<'b>(&self, oid: &'b str) -> Result<&'b str, Error>
    where
        'a: 'b,
    {
        Ok(self.by_oid(&oid.parse()?).unwrap_or(oid))
    }

    /// Finds a named oid by its associated OID.
    pub const fn by_oid(&self, oid: &ObjectIdentifier) -> Option<&'a str> {
        let mut i = 0;

        while i < self.0.len() {
            let lhs = self.0[i].0;
            if lhs.length == oid.length && eq(&lhs.bytes, &oid.bytes) {
                return Some(self.0[i].1);
            }

            i += 1;
        }

        None
    }

    /// Finds a named oid by its associated name.
    pub const fn by_name(&self, name: &str) -> Option<&'a ObjectIdentifier> {
        let mut i = 0;

        while i < self.0.len() {
            let lhs = self.0[i].1;
            if eq_case(lhs.as_bytes(), name.as_bytes()) {
                return Some(self.0[i].0);
            }

            i += 1;
        }

        None
    }

    /// Return the list of matched name for the OID.
    pub const fn find_names_for_oid(&self, oid: ObjectIdentifier) -> Names<'a> {
        Names {
            database: *self,
            oid,
            position: 0,
        }
    }
}

/// Iterator returning the multiple names that may be associated with an OID.
pub struct Names<'a> {
    database: Database<'a>,
    oid: ObjectIdentifier,
    position: usize,
}

impl<'a> Iterator for Names<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let mut i = self.position;

        while i < self.database.0.len() {
            let lhs = self.database.0[i].0;

            if lhs.as_bytes().eq(self.oid.as_bytes()) {
                self.position = i + 1;
                return Some(self.database.0[i].1);
            }

            i += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::ObjectIdentifier;

    use super::rfc4519::CN;

    #[test]
    fn by_oid() {
        let cn = super::DB.by_oid(&CN).expect("cn not found");
        assert_eq!("cn", cn);

        let none = ObjectIdentifier::new_unwrap("0.1.2.3.4.5.6.7.8.9");
        assert_eq!(None, super::DB.by_oid(&none));
    }

    #[test]
    fn by_name() {
        let cn = super::DB.by_name("CN").expect("cn not found");
        assert_eq!(&CN, cn);

        assert_eq!(None, super::DB.by_name("purplePeopleEater"));
    }
}

//! Module with all supported comparison operators.
//!
//! This module provides an enum with all comparison operators that can be used with this library.
//! The enum provides various useful helper functions to inverse or flip an operator.
//!
//! Methods like `Cmp::from_sign(">");` can be used to get a comparison operator by it's logical
//! sign from a string.

use std::cmp::Ordering;

/// Comparison operators enum.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Cmp {
    /// Equal (`==`, `=`).
    /// When version `A` is equal to `B`.
    Eq,

    /// Not equal (`!=`, `!`, `<>`).
    /// When version `A` is not equal to `B`.
    Ne,

    /// Less than (`<`).
    /// When version `A` is less than `B` but not equal.
    Lt,

    /// Less or equal (`<=`).
    /// When version `A` is less than or equal to `B`.
    Le,

    /// Greater or equal (`>=`).
    /// When version `A` is greater than or equal to `B`.
    Ge,

    /// Greater than (`>`).
    /// When version `A` is greater than `B` but not equal.
    Gt,
}

impl Cmp {
    /// Get a comparison operator by it's sign.
    /// Whitespaces are stripped from the sign string.
    /// An error is returned if the sign isn't recognized.
    ///
    /// The following signs are supported:
    ///
    /// * `==` _or_ `=` -> `Eq`
    /// * `!=` _or_ `!` _or_ `<>` -> `Ne`
    /// * `< ` -> `Lt`
    /// * `<=` -> `Le`
    /// * `>=` -> `Ge`
    /// * `> ` -> `Gt`
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Cmp;
    ///
    /// assert_eq!(Cmp::from_sign("=="), Ok(Cmp::Eq));
    /// assert_eq!(Cmp::from_sign("<"), Ok(Cmp::Lt));
    /// assert_eq!(Cmp::from_sign("  >=   "), Ok(Cmp::Ge));
    /// assert!(Cmp::from_sign("*").is_err());
    /// ```
    #[allow(clippy::result_unit_err)]
    pub fn from_sign<S: AsRef<str>>(sign: S) -> Result<Cmp, ()> {
        match sign.as_ref().trim() {
            "==" | "=" => Ok(Cmp::Eq),
            "!=" | "!" | "<>" => Ok(Cmp::Ne),
            "<" => Ok(Cmp::Lt),
            "<=" => Ok(Cmp::Le),
            ">=" => Ok(Cmp::Ge),
            ">" => Ok(Cmp::Gt),
            _ => Err(()),
        }
    }

    /// Get a comparison operator by it's name.
    /// Names are case-insensitive, and whitespaces are stripped from the string.
    /// An error is returned if the name isn't recognized.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Cmp;
    ///
    /// assert_eq!(Cmp::from_name("eq"), Ok(Cmp::Eq));
    /// assert_eq!(Cmp::from_name("lt"), Ok(Cmp::Lt));
    /// assert_eq!(Cmp::from_name("  Ge   "), Ok(Cmp::Ge));
    /// assert!(Cmp::from_name("abc").is_err());
    /// ```
    #[allow(clippy::result_unit_err)]
    pub fn from_name<S: AsRef<str>>(sign: S) -> Result<Cmp, ()> {
        match sign.as_ref().trim().to_lowercase().as_str() {
            "eq" => Ok(Cmp::Eq),
            "ne" => Ok(Cmp::Ne),
            "lt" => Ok(Cmp::Lt),
            "le" => Ok(Cmp::Le),
            "ge" => Ok(Cmp::Ge),
            "gt" => Ok(Cmp::Gt),
            _ => Err(()),
        }
    }

    /// Get the comparison operator from Rusts `Ordering` enum.
    ///
    /// The following comparison operators are returned:
    ///
    /// * `Ordering::Less` -> `Lt`
    /// * `Ordering::Equal` -> `Eq`
    /// * `Ordering::Greater` -> `Gt`
    #[deprecated(since = "0.2.0", note = "use Cmp::from(ord) instead")]
    pub fn from_ord(ord: Ordering) -> Cmp {
        Self::from(ord)
    }

    /// Get the name of this comparison operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Cmp;
    ///
    /// assert_eq!(Cmp::Eq.name(), "eq");
    /// assert_eq!(Cmp::Lt.name(), "lt");
    /// assert_eq!(Cmp::Ge.name(), "ge");
    /// ```
    pub fn name<'a>(self) -> &'a str {
        match self {
            Cmp::Eq => "eq",
            Cmp::Ne => "ne",
            Cmp::Lt => "lt",
            Cmp::Le => "le",
            Cmp::Ge => "ge",
            Cmp::Gt => "gt",
        }
    }

    /// Get the inverted comparison operator.
    ///
    /// This uses the following bidirectional rules:
    ///
    /// * `Eq` <-> `Ne`
    /// * `Lt` <-> `Ge`
    /// * `Le` <-> `Gt`
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Cmp;
    ///
    /// assert_eq!(Cmp::Eq.invert(), Cmp::Ne);
    /// assert_eq!(Cmp::Lt.invert(), Cmp::Ge);
    /// assert_eq!(Cmp::Gt.invert(), Cmp::Le);
    /// ```
    #[must_use]
    pub fn invert(self) -> Self {
        match self {
            Cmp::Eq => Cmp::Ne,
            Cmp::Ne => Cmp::Eq,
            Cmp::Lt => Cmp::Ge,
            Cmp::Le => Cmp::Gt,
            Cmp::Ge => Cmp::Lt,
            Cmp::Gt => Cmp::Le,
        }
    }

    /// Get the opposite comparison operator.
    ///
    /// This uses the following bidirectional rules:
    ///
    /// * `Eq` <-> `Ne`
    /// * `Lt` <-> `Gt`
    /// * `Le` <-> `Ge`
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Cmp;
    ///
    /// assert_eq!(Cmp::Eq.opposite(), Cmp::Ne);
    /// assert_eq!(Cmp::Lt.opposite(), Cmp::Gt);
    /// assert_eq!(Cmp::Ge.opposite(), Cmp::Le);
    /// ```
    #[must_use]
    pub fn opposite(self) -> Self {
        match self {
            Cmp::Eq => Cmp::Ne,
            Cmp::Ne => Cmp::Eq,
            Cmp::Lt => Cmp::Gt,
            Cmp::Le => Cmp::Ge,
            Cmp::Ge => Cmp::Le,
            Cmp::Gt => Cmp::Lt,
        }
    }

    /// Get the flipped comparison operator.
    ///
    /// This uses the following bidirectional rules:
    ///
    /// * `Lt` <-> `Gt`
    /// * `Le` <-> `Ge`
    /// * Other operators are returned as is.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Cmp;
    ///
    /// assert_eq!(Cmp::Eq.flip(), Cmp::Eq);
    /// assert_eq!(Cmp::Lt.flip(), Cmp::Gt);
    /// assert_eq!(Cmp::Ge.flip(), Cmp::Le);
    /// ```
    #[must_use]
    pub fn flip(self) -> Self {
        match self {
            Cmp::Lt => Cmp::Gt,
            Cmp::Le => Cmp::Ge,
            Cmp::Ge => Cmp::Le,
            Cmp::Gt => Cmp::Lt,
            _ => self,
        }
    }

    /// Get the sign for this comparison operator.
    ///
    /// The following signs are returned:
    ///
    /// * `Eq` -> `==`
    /// * `Ne` -> `!=`
    /// * `Lt` -> `< `
    /// * `Le` -> `<=`
    /// * `Ge` -> `>=`
    /// * `Gt` -> `> `
    ///
    /// Note: Some comparison operators also support other signs,
    /// such as `=` for `Eq` and `!` for `Ne`,
    /// these are never returned by this method however as the table above is used.
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Cmp;
    ///
    /// assert_eq!(Cmp::Eq.sign(), "==");
    /// assert_eq!(Cmp::Lt.sign(), "<");
    /// assert_eq!(Cmp::Ge.flip().sign(), "<=");
    /// ```
    pub fn sign(self) -> &'static str {
        match self {
            Cmp::Eq => "==",
            Cmp::Ne => "!=",
            Cmp::Lt => "<",
            Cmp::Le => "<=",
            Cmp::Ge => ">=",
            Cmp::Gt => ">",
        }
    }

    /// Get a factor (number) for this comparison operator.
    /// These factors can be useful for quick calculations.
    ///
    /// The following factor numbers are returned:
    ///
    /// * `Eq` _or_ `Ne` -> ` 0`
    /// * `Lt` _or_ `Le` -> `-1`
    /// * `Gt` _or_ `Ge` -> ` 1`
    ///
    /// # Examples
    ///
    /// ```
    /// use version_compare::Version;
    ///
    /// let a = Version::from("1.2.3").unwrap();
    /// let b = Version::from("1.3").unwrap();
    ///
    /// assert_eq!(a.compare(&b).factor(), -1);
    /// assert_eq!(10 * b.compare(a).factor(), 10);
    /// ```
    pub fn factor(self) -> i8 {
        match self {
            Cmp::Eq | Cmp::Ne => 0,
            Cmp::Lt | Cmp::Le => -1,
            Cmp::Gt | Cmp::Ge => 1,
        }
    }

    /// Get Rust's ordering for this comparison operator.
    ///
    /// The following comparison operators are supported:
    ///
    /// * `Eq` -> `Ordering::Equal`
    /// * `Lt` -> `Ordering::Less`
    /// * `Gt` -> `Ordering::Greater`
    ///
    /// For other comparison operators `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use version_compare::Version;
    ///
    /// let a = Version::from("1.2.3").unwrap();
    /// let b = Version::from("1.3").unwrap();
    ///
    /// assert_eq!(a.compare(b).ord().unwrap(), Ordering::Less);
    /// ```
    pub fn ord(self) -> Option<Ordering> {
        match self {
            Cmp::Eq => Some(Ordering::Equal),
            Cmp::Lt => Some(Ordering::Less),
            Cmp::Gt => Some(Ordering::Greater),
            _ => None,
        }
    }
}

impl From<Ordering> for Cmp {
    /// Get the comparison operator from Rusts `Ordering` enum.
    ///
    /// The following comparison operators are returned:
    ///
    /// * `Ordering::Less` -> `Lt`
    /// * `Ordering::Equal` -> `Eq`
    /// * `Ordering::Greater` -> `Gt`
    fn from(ord: Ordering) -> Self {
        match ord {
            Ordering::Less => Cmp::Lt,
            Ordering::Equal => Cmp::Eq,
            Ordering::Greater => Cmp::Gt,
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::Cmp;

    #[test]
    fn from_sign() {
        // Normal signs
        assert_eq!(Cmp::from_sign("==").unwrap(), Cmp::Eq);
        assert_eq!(Cmp::from_sign("=").unwrap(), Cmp::Eq);
        assert_eq!(Cmp::from_sign("!=").unwrap(), Cmp::Ne);
        assert_eq!(Cmp::from_sign("!").unwrap(), Cmp::Ne);
        assert_eq!(Cmp::from_sign("<>").unwrap(), Cmp::Ne);
        assert_eq!(Cmp::from_sign("<").unwrap(), Cmp::Lt);
        assert_eq!(Cmp::from_sign("<=").unwrap(), Cmp::Le);
        assert_eq!(Cmp::from_sign(">=").unwrap(), Cmp::Ge);
        assert_eq!(Cmp::from_sign(">").unwrap(), Cmp::Gt);

        // Exceptional cases
        assert_eq!(Cmp::from_sign("  <=  ").unwrap(), Cmp::Le);
        assert_eq!(Cmp::from_sign("*"), Err(()));
    }

    #[test]
    fn from_name() {
        // Normal names
        assert_eq!(Cmp::from_name("eq").unwrap(), Cmp::Eq);
        assert_eq!(Cmp::from_name("ne").unwrap(), Cmp::Ne);
        assert_eq!(Cmp::from_name("lt").unwrap(), Cmp::Lt);
        assert_eq!(Cmp::from_name("le").unwrap(), Cmp::Le);
        assert_eq!(Cmp::from_name("ge").unwrap(), Cmp::Ge);
        assert_eq!(Cmp::from_name("gt").unwrap(), Cmp::Gt);

        // Exceptional cases
        assert_eq!(Cmp::from_name("  Le  ").unwrap(), Cmp::Le);
        assert_eq!(Cmp::from_name("abc"), Err(()));
    }

    #[test]
    fn from_ord() {
        assert_eq!(Cmp::from(Ordering::Less), Cmp::Lt);
        assert_eq!(Cmp::from(Ordering::Equal), Cmp::Eq);
        assert_eq!(Cmp::from(Ordering::Greater), Cmp::Gt);
    }

    #[test]
    fn name() {
        assert_eq!(Cmp::Eq.name(), "eq");
        assert_eq!(Cmp::Ne.name(), "ne");
        assert_eq!(Cmp::Lt.name(), "lt");
        assert_eq!(Cmp::Le.name(), "le");
        assert_eq!(Cmp::Ge.name(), "ge");
        assert_eq!(Cmp::Gt.name(), "gt");
    }

    #[test]
    fn invert() {
        assert_eq!(Cmp::Ne.invert(), Cmp::Eq);
        assert_eq!(Cmp::Eq.invert(), Cmp::Ne);
        assert_eq!(Cmp::Ge.invert(), Cmp::Lt);
        assert_eq!(Cmp::Gt.invert(), Cmp::Le);
        assert_eq!(Cmp::Lt.invert(), Cmp::Ge);
        assert_eq!(Cmp::Le.invert(), Cmp::Gt);
    }

    #[test]
    fn opposite() {
        assert_eq!(Cmp::Eq.opposite(), Cmp::Ne);
        assert_eq!(Cmp::Ne.opposite(), Cmp::Eq);
        assert_eq!(Cmp::Lt.opposite(), Cmp::Gt);
        assert_eq!(Cmp::Le.opposite(), Cmp::Ge);
        assert_eq!(Cmp::Ge.opposite(), Cmp::Le);
        assert_eq!(Cmp::Gt.opposite(), Cmp::Lt);
    }

    #[test]
    fn flip() {
        assert_eq!(Cmp::Eq.flip(), Cmp::Eq);
        assert_eq!(Cmp::Ne.flip(), Cmp::Ne);
        assert_eq!(Cmp::Lt.flip(), Cmp::Gt);
        assert_eq!(Cmp::Le.flip(), Cmp::Ge);
        assert_eq!(Cmp::Ge.flip(), Cmp::Le);
        assert_eq!(Cmp::Gt.flip(), Cmp::Lt);
    }

    #[test]
    fn sign() {
        assert_eq!(Cmp::Eq.sign(), "==");
        assert_eq!(Cmp::Ne.sign(), "!=");
        assert_eq!(Cmp::Lt.sign(), "<");
        assert_eq!(Cmp::Le.sign(), "<=");
        assert_eq!(Cmp::Ge.sign(), ">=");
        assert_eq!(Cmp::Gt.sign(), ">");
    }

    #[test]
    fn factor() {
        assert_eq!(Cmp::Eq.factor(), 0);
        assert_eq!(Cmp::Ne.factor(), 0);
        assert_eq!(Cmp::Lt.factor(), -1);
        assert_eq!(Cmp::Le.factor(), -1);
        assert_eq!(Cmp::Ge.factor(), 1);
        assert_eq!(Cmp::Gt.factor(), 1);
    }

    #[test]
    fn ord() {
        assert_eq!(Cmp::Eq.ord(), Some(Ordering::Equal));
        assert_eq!(Cmp::Ne.ord(), None);
        assert_eq!(Cmp::Lt.ord(), Some(Ordering::Less));
        assert_eq!(Cmp::Le.ord(), None);
        assert_eq!(Cmp::Ge.ord(), None);
        assert_eq!(Cmp::Gt.ord(), Some(Ordering::Greater));
    }
}

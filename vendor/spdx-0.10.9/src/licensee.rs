use crate::{
    ExceptionId, LicenseItem, LicenseReq,
    error::{ParseError, Reason},
    lexer::{Lexer, Token},
};
use std::fmt;

/// A convenience wrapper for a license and optional exception that can be
/// checked against a license requirement to see if it satisfies the requirement
/// placed by a license holder
///
/// ```
/// let licensee = spdx::Licensee::parse("GPL-2.0").unwrap();
///
/// assert!(licensee.satisfies(&spdx::LicenseReq::from(spdx::license_id("GPL-2.0-only").unwrap())));
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Licensee {
    inner: LicenseReq,
}

impl fmt::Display for Licensee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::str::FromStr for Licensee {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Licensee {
    /// Creates a licensee from its component parts. Note that use of SPDX's
    /// `or_later` is completely ignored for licensees as it only applies
    /// to the license holder(s), not the licensee
    #[must_use]
    pub fn new(license: LicenseItem, exception: Option<ExceptionId>) -> Self {
        if let LicenseItem::Spdx { or_later, .. } = &license {
            debug_assert!(!or_later);
        }

        Self {
            inner: LicenseReq { license, exception },
        }
    }

    /// Parses an simplified version of an SPDX license expression that can
    /// contain at most 1 valid SDPX license with an optional exception joined
    /// by a `WITH`.
    ///
    /// ```
    /// use spdx::Licensee;
    ///
    /// // Normal single license
    /// Licensee::parse("MIT").unwrap();
    ///
    /// // SPDX allows license identifiers outside of the official license list
    /// // via the LicenseRef- prefix
    /// Licensee::parse("LicenseRef-My-Super-Extra-Special-License").unwrap();
    ///
    /// // License and exception
    /// Licensee::parse("Apache-2.0 WITH LLVM-exception").unwrap();
    ///
    /// // `+` is only allowed to be used by license requirements from the license holder
    /// Licensee::parse("Apache-2.0+").unwrap_err();
    ///
    /// Licensee::parse("GPL-2.0").unwrap();
    ///
    /// // GNU suffix license (GPL, AGPL, LGPL, GFDL) must not contain the suffix
    /// Licensee::parse("GPL-3.0-or-later").unwrap_err();
    ///
    /// // GFDL licenses are only allowed to contain the `invariants` suffix
    /// Licensee::parse("GFDL-1.3-invariants").unwrap();
    /// ```
    pub fn parse(original: &str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(original);

        let license = {
            let lt = lexer.next().ok_or_else(|| ParseError {
                original: original.to_owned(),
                span: 0..original.len(),
                reason: Reason::Empty,
            })??;

            match lt.token {
                Token::Spdx(id) => {
                    // If we have one of the GNU licenses which use the `-only`
                    // or `-or-later` suffixes return an error rather than
                    // silently truncating, the `-only` and `-or-later` suffixes
                    // are for the license holder(s) to specify what license(s)
                    // they can be licensed under, not for the licensee,
                    // similarly to the `+`
                    if id.is_gnu() {
                        let is_only = original.ends_with("-only");
                        let or_later = original.ends_with("-or-later");

                        if is_only || or_later {
                            return Err(ParseError {
                                original: original.to_owned(),
                                span: if is_only {
                                    original.len() - 5..original.len()
                                } else {
                                    original.len() - 9..original.len()
                                },
                                reason: Reason::Unexpected(&["<bare-gnu-license>"]),
                            });
                        }

                        // GFDL has `no-invariants` and `invariants` variants, we
                        // treat `no-invariants` as invalid, just the same as
                        // only, it would be the same as a bare GFDL-<version>.
                        // However, the `invariants`...variant we do allow since
                        // it is a modifier on the license...and should therefore
                        // by a WITH exception but GNU licenses are the worst
                        if original.starts_with("GFDL") && original.contains("-no-invariants") {
                            return Err(ParseError {
                                original: original.to_owned(),
                                span: 8..original.len(),
                                reason: Reason::Unexpected(&["<bare-gfdl-license>"]),
                            });
                        }
                    }

                    LicenseItem::Spdx {
                        id,
                        or_later: false,
                    }
                }
                Token::LicenseRef { doc_ref, lic_ref } => LicenseItem::Other {
                    doc_ref: doc_ref.map(String::from),
                    lic_ref: lic_ref.to_owned(),
                },
                _ => {
                    return Err(ParseError {
                        original: original.to_owned(),
                        span: lt.span,
                        reason: Reason::Unexpected(&["<license>"]),
                    });
                }
            }
        };

        let exception = match lexer.next() {
            None => None,
            Some(lt) => {
                let lt = lt?;
                match lt.token {
                    Token::With => {
                        let lt = lexer.next().ok_or(ParseError {
                            original: original.to_owned(),
                            span: lt.span,
                            reason: Reason::Empty,
                        })??;

                        match lt.token {
                            Token::Exception(exc) => Some(exc),
                            _ => {
                                return Err(ParseError {
                                    original: original.to_owned(),
                                    span: lt.span,
                                    reason: Reason::Unexpected(&["<exception>"]),
                                });
                            }
                        }
                    }
                    _ => {
                        return Err(ParseError {
                            original: original.to_owned(),
                            span: lt.span,
                            reason: Reason::Unexpected(&["WITH"]),
                        });
                    }
                }
            }
        };

        Ok(Licensee {
            inner: LicenseReq { license, exception },
        })
    }

    /// Determines whether the specified license requirement is satisfied by
    /// this license (+exception)
    ///
    /// ```
    /// let licensee = spdx::Licensee::parse("Apache-2.0 WITH LLVM-exception").unwrap();
    ///
    /// assert!(licensee.satisfies(&spdx::LicenseReq {
    ///     license: spdx::LicenseItem::Spdx {
    ///         id: spdx::license_id("Apache-2.0").unwrap(),
    ///         // Means the license holder is fine with Apache-2.0 or higher
    ///         or_later: true,
    ///     },
    ///     exception: spdx::exception_id("LLVM-exception"),
    /// }));
    /// ```
    #[must_use]
    pub fn satisfies(&self, req: &LicenseReq) -> bool {
        match (&self.inner.license, &req.license) {
            (LicenseItem::Spdx { id: a, .. }, LicenseItem::Spdx { id: b, or_later }) => {
                if a.index != b.index {
                    if *or_later {
                        let (a_name, a_gfdl_invariants) = if a.name.starts_with("GFDL") {
                            a.name
                                .strip_suffix("-invariants")
                                .map_or((a.name, false), |name| (name, true))
                        } else {
                            (a.name, false)
                        };

                        let (b_name, b_gfdl_invariants) = if b.name.starts_with("GFDL") {
                            b.name
                                .strip_suffix("-invariants")
                                .map_or((b.name, false), |name| (name, true))
                        } else {
                            (b.name, false)
                        };

                        if a_gfdl_invariants != b_gfdl_invariants {
                            return false;
                        }

                        // Many of the SPDX identifiers end with `-<version number>`,
                        // so chop that off and ensure the base strings match, and if so,
                        // just a do a lexical compare, if this "allowed license" is >,
                        // then we satisfed the license requirement
                        let a_test_name = &a_name[..a_name.rfind('-').unwrap_or(a_name.len())];
                        let b_test_name = &b_name[..b_name.rfind('-').unwrap_or(b_name.len())];

                        if a_test_name != b_test_name || a_name < b_name {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
            (
                LicenseItem::Other {
                    doc_ref: doc_a,
                    lic_ref: lic_a,
                },
                LicenseItem::Other {
                    doc_ref: doc_b,
                    lic_ref: lic_b,
                },
            ) => {
                if doc_a != doc_b || lic_a != lic_b {
                    return false;
                }
            }
            _ => return false,
        }

        req.exception == self.inner.exception
    }

    #[must_use]
    pub fn into_req(self) -> LicenseReq {
        self.inner
    }
}

impl PartialOrd<LicenseReq> for Licensee {
    #[inline]
    fn partial_cmp(&self, o: &LicenseReq) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(o)
    }
}

impl PartialEq<LicenseReq> for Licensee {
    #[inline]
    fn eq(&self, o: &LicenseReq) -> bool {
        self.inner.eq(o)
    }
}

impl AsRef<LicenseReq> for Licensee {
    #[inline]
    fn as_ref(&self) -> &LicenseReq {
        &self.inner
    }
}

#[cfg(test)]
mod test {
    use crate::{LicenseItem, LicenseReq, Licensee, exception_id, license_id};

    const LICENSEES: &[&str] = &[
        "LicenseRef-Embark-Proprietary",
        "BSD-2-Clause",
        "Apache-2.0 WITH LLVM-exception",
        "BSD-2-Clause-FreeBSD",
        "BSL-1.0",
        "Zlib",
        "CC0-1.0",
        "FTL",
        "ISC",
        "MIT",
        "MPL-2.0",
        "BSD-3-Clause",
        "Unicode-DFS-2016",
        "Unlicense",
        "Apache-2.0",
    ];

    #[test]
    fn handles_or_later() {
        let mut licensees: Vec<_> = LICENSEES
            .iter()
            .map(|l| Licensee::parse(l).unwrap())
            .collect();
        licensees.sort();

        let mpl_id = license_id("MPL-2.0").unwrap();
        let req = LicenseReq {
            license: LicenseItem::Spdx {
                id: mpl_id,
                or_later: true,
            },
            exception: None,
        };

        // Licensees can't have the `or_later`
        assert!(licensees.binary_search_by(|l| l.inner.cmp(&req)).is_err());

        match &licensees[licensees
            .binary_search_by(|l| l.partial_cmp(&req).unwrap())
            .unwrap()]
        .inner
        .license
        {
            LicenseItem::Spdx { id, .. } => assert_eq!(*id, mpl_id),
            o @ LicenseItem::Other { .. } => panic!("unexpected {o:?}"),
        }
    }

    #[test]
    fn handles_exceptions() {
        let mut licensees: Vec<_> = LICENSEES
            .iter()
            .map(|l| Licensee::parse(l).unwrap())
            .collect();
        licensees.sort();

        let apache_id = license_id("Apache-2.0").unwrap();
        let llvm_exc = exception_id("LLVM-exception").unwrap();
        let req = LicenseReq {
            license: LicenseItem::Spdx {
                id: apache_id,
                or_later: false,
            },
            exception: Some(llvm_exc),
        };

        assert_eq!(
            &req,
            &licensees[licensees
                .binary_search_by(|l| l.partial_cmp(&req).unwrap())
                .unwrap()]
            .inner
        );
    }

    #[test]
    fn handles_license_ref() {
        let mut licensees: Vec<_> = LICENSEES
            .iter()
            .map(|l| Licensee::parse(l).unwrap())
            .collect();
        licensees.sort();

        let req = LicenseReq {
            license: LicenseItem::Other {
                doc_ref: None,
                lic_ref: "Embark-Proprietary".to_owned(),
            },
            exception: None,
        };

        assert_eq!(
            &req,
            &licensees[licensees
                .binary_search_by(|l| l.partial_cmp(&req).unwrap())
                .unwrap()]
            .inner
        );
    }

    #[test]
    fn handles_close() {
        let mut licensees: Vec<_> = LICENSEES
            .iter()
            .map(|l| Licensee::parse(l).unwrap())
            .collect();
        licensees.sort();

        for id in &["BSD-2-Clause", "BSD-2-Clause-FreeBSD"] {
            let lic_id = license_id(id).unwrap();
            let req = LicenseReq {
                license: LicenseItem::Spdx {
                    id: lic_id,
                    or_later: true,
                },
                exception: None,
            };

            // Licensees can't have the `or_later`
            assert!(licensees.binary_search_by(|l| l.inner.cmp(&req)).is_err());

            match &licensees[licensees
                .binary_search_by(|l| l.partial_cmp(&req).unwrap())
                .unwrap()]
            .inner
            .license
            {
                LicenseItem::Spdx { id, .. } => assert_eq!(*id, lic_id),
                o @ LicenseItem::Other { .. } => panic!("unexpected {o:?}"),
            }
        }
    }
}

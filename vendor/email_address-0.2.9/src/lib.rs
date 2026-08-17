/*!
A Rust crate providing an implementation of an RFC-compliant `EmailAddress` newtype.

Primarily for validation, the `EmailAddress` type is constructed with `FromStr::from_str` which will raise any
parsing errors. Prior to constructions the functions `is_valid`, `is_valid_local_part`, and `is_valid_domain` may
also be used to test for validity without constructing an instance. This supports all of the RFC ASCII and UTF-8
character set rules, quoted and unquoted local parts but does not yet support all of the productions required for SMTP
headers; folding whitespace, comments, etc.

```text
"Simon Johnston <johnstonsk@gmail.com>"
                 ^------------------^ email()
                            ^-------^ domain()
                 ^--------^ local_part()
 ^------------^ display_part()
```

# Example

The following shoes the basic `is_valid` and `from_str` functions.

```rust
use email_address::*;
use std::str::FromStr;

assert!(EmailAddress::is_valid("user.name+tag+sorting@example.com"));

assert_eq!(
    EmailAddress::from_str("Abc.example.com"),
    Error::MissingSeparator.into()
);
```

The following shows the three format functions used to output an email address.

```rust
use email_address::*;
use std::str::FromStr;

let email = EmailAddress::from_str("johnstonsk@gmail.com").unwrap();

assert_eq!(
    email.to_string(),
    "johnstonsk@gmail.com".to_string()
);

assert_eq!(
    String::from(email.clone()),
    "johnstonsk@gmail.com".to_string()
);

assert_eq!(
    email.as_ref(),
    "johnstonsk@gmail.com"
);

assert_eq!(
    email.to_uri(),
    "mailto:johnstonsk@gmail.com".to_string()
);

assert_eq!(
    email.to_display("Simon Johnston"),
    "Simon Johnston <johnstonsk@gmail.com>".to_string()
);
```


# Specifications

1. RFC 1123: [_Requirements for Internet Hosts -- Application and Support_](https://tools.ietf.org/html/rfc1123),
   IETF,Oct 1989.
1. RFC 3629: [_UTF-8, a transformation format of ISO 10646_](https://tools.ietf.org/html/rfc3629),
   IETF, Nov 2003.
1. RFC 3696: [_Application Techniques for Checking and Transformation of
   Names_](https://tools.ietf.org/html/rfc3696), IETF, Feb 2004.
1. RFC 4291 [_IP Version 6 Addressing Architecture_](https://tools.ietf.org/html/rfc4291),
   IETF, Feb 2006.
1. RFC 5234: [_Augmented BNF for Syntax Specifications: ABNF_](https://tools.ietf.org/html/rfc5234),
   IETF, Jan 2008.
1. RFC 5321: [_Simple Mail Transfer Protocol_](https://tools.ietf.org/html/rfc5321),
   IETF, Oct 2008.
1. RFC 5322: [_Internet Message Format_](https://tools.ietf.org/html/rfc5322), I
   ETF, Oct 2008.
1. RFC 5890: [_Internationalized Domain Names for Applications (IDNA): Definitions and Document
   Framework_](https://tools.ietf.org/html/rfc5890), IETF, Aug 2010.
1. RFC 6531: [_SMTP Extension for Internationalized Email_](https://tools.ietf.org/html/rfc6531),
   IETF, Feb 2012
1. RFC 6532: [_Internationalized Email Headers_](https://tools.ietf.org/html/rfc6532),
   IETF, Feb 2012.

From RFC 5322: §3.2.1. [Quoted characters](https://tools.ietf.org/html/rfc5322#section-3.2.1):

```ebnf
quoted-pair     =   ("\" (VCHAR / WSP)) / obs-qp
```

From RFC 5322: §3.2.2. [Folding White Space and Comments](https://tools.ietf.org/html/rfc5322#section-3.2.2):

```ebnf
FWS             =   ([*WSP CRLF] 1*WSP) /  obs-FWS
                                       ; Folding white space

ctext           =   %d33-39 /          ; Printable US-ASCII
                    %d42-91 /          ;  characters not including
                    %d93-126 /         ;  "(", ")", or "\"
                    obs-ctext

ccontent        =   ctext / quoted-pair / comment

comment         =   "(" *([FWS] ccontent) [FWS] ")"

CFWS            =   (1*([FWS] comment) [FWS]) / FWS
```

From RFC 5322: §3.2.3. [Atom](https://tools.ietf.org/html/rfc5322#section-3.2.3):

```ebnf
atext           =   ALPHA / DIGIT /    ; Printable US-ASCII
                    "!" / "#" /        ;  characters not including
                    "$" / "%" /        ;  specials.  Used for atoms.
                    "&" / "'" /
                    "*" / "+" /
                    "-" / "/" /
                    "=" / "?" /
                    "^" / "_" /
                    "`" / "{" /
                    "|" / "}" /
                    "~"

atom            =   [CFWS] 1*atext [CFWS]

dot-atom-text   =   1*atext *("." 1*atext)

dot-atom        =   [CFWS] dot-atom-text [CFWS]

specials        =   "(" / ")" /        ; Special characters that do
                    "<" / ">" /        ;  not appear in atext
                    "[" / "]" /
                    ":" / ";" /
                    "@" / "\" /
                    "," / "." /
                    DQUOTE
```

From RFC 5322: §3.2.4. [Quoted Strings](https://tools.ietf.org/html/rfc5322#section-3.2.4):

```ebnf
qtext           =   %d33 /             ; Printable US-ASCII
                    %d35-91 /          ;  characters not including
                    %d93-126 /         ;  "\" or the quote character
                    obs-qtext

qcontent        =   qtext / quoted-pair

quoted-string   =   [CFWS]
                    DQUOTE *([FWS] qcontent) [FWS] DQUOTE
                    [CFWS]
```

From RFC 5322, §3.4.1. [Addr-Spec Specification](https://tools.ietf.org/html/rfc5322#section-3.4.1):

```ebnf
addr-spec       =   local-part "@" domain

local-part      =   dot-atom / quoted-string / obs-local-part

domain          =   dot-atom / domain-literal / obs-domain

domain-literal  =   [CFWS] "[" *([FWS] dtext) [FWS] "]" [CFWS]

dtext           =   %d33-90 /          ; Printable US-ASCII
                    %d94-126 /         ;  characters not including
                    obs-dtext          ;  "[", "]", or "\"
```

RFC 3696, §3. [Restrictions on email addresses](https://tools.ietf.org/html/rfc3696#section-3)
describes in detail the quoting of characters in an address.

## Unicode

RFC 6531, §3.3. [Extended Mailbox Address Syntax](https://tools.ietf.org/html/rfc6531#section-3.3)
extends the rules above for non-ASCII character sets.

```ebnf
sub-domain   =/  U-label
    ; extend the definition of sub-domain in RFC 5321, Section 4.1.2

atext   =/  UTF8-non-ascii
    ; extend the implicit definition of atext in
    ; RFC 5321, Section 4.1.2, which ultimately points to
    ; the actual definition in RFC 5322, Section 3.2.3

qtextSMTP  =/ UTF8-non-ascii
    ; extend the definition of qtextSMTP in RFC 5321, Section 4.1.2

esmtp-value  =/ UTF8-non-ascii
    ; extend the definition of esmtp-value in RFC 5321, Section 4.1.2
```

A "U-label" is an IDNA-valid string of Unicode characters, in Normalization Form C (NFC) and
including at least one non-ASCII character, expressed in a standard Unicode Encoding Form (such as
UTF-8). It is also subject to the constraints about permitted characters that are specified in
Section 4.2 of the Protocol document and the rules in the Sections 2 and 3 of the Tables document,
the Bidi constraints in that document if it contains any character from scripts that are written
right to left, and the symmetry constraint described immediately below. Conversions between U-labels
and A-labels are performed according to the "Punycode" specification RFC3492, adding or removing
the ACE prefix as needed.

RFC 6532: §3.1 [UTF-8 Syntax and Normalization](https://tools.ietf.org/html/rfc6532#section-3.1),
and §3.2 [Syntax Extensions to RFC 5322](https://tools.ietf.org/html/rfc6532#section-3.2) extend
the syntax above with:

```ebnf
UTF8-non-ascii  =   UTF8-2 / UTF8-3 / UTF8-4

...

VCHAR   =/  UTF8-non-ascii

ctext   =/  UTF8-non-ascii

atext   =/  UTF8-non-ascii

qtext   =/  UTF8-non-ascii

text    =/  UTF8-non-ascii
              ; note that this upgrades the body to UTF-8

dtext   =/  UTF8-non-ascii
```

These in turn refer to RFC 6529 §4. [Syntax of UTF-8 Byte Sequences](https://tools.ietf.org/html/rfc3629#section-4):

> A UTF-8 string is a sequence of octets representing a sequence of UCS
> characters.  An octet sequence is valid UTF-8 only if it matches the
> following syntax, which is derived from the rules for encoding UTF-8
> and is expressed in the ABNF of \[RFC2234\].

```ebnf
   UTF8-octets = *( UTF8-char )
   UTF8-char   = UTF8-1 / UTF8-2 / UTF8-3 / UTF8-4
   UTF8-1      = %x00-7F
   UTF8-2      = %xC2-DF UTF8-tail
   UTF8-3      = %xE0 %xA0-BF UTF8-tail / %xE1-EC 2( UTF8-tail ) /
                 %xED %x80-9F UTF8-tail / %xEE-EF 2( UTF8-tail )
   UTF8-4      = %xF0 %x90-BF 2( UTF8-tail ) / %xF1-F3 3( UTF8-tail ) /
                 %xF4 %x80-8F 2( UTF8-tail )
   UTF8-tail   = %x80-BF
```

Comments in addresses are discussed in RFC 5322 Appendix A.5. [White Space, Comments, and Other
Oddities](https://tools.ietf.org/html/rfc5322#appendix-A.5).

An informal description can be found on [Wikipedia](https://en.wikipedia.org/wiki/Email_address).

*/

#![warn(
    unknown_lints,
    // ---------- Stylistic
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    nonstandard_style, /* group */
    noop_method_call,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Future
    future_incompatible, /* group */
    rust_2021_compatibility, /* group */
    // ---------- Public
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    // ---------- Unused
    unused, /* group */
)]
#![deny(
    // ---------- Public
    exported_private_dependencies,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Error type used when parsing an address.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// An invalid character was found in some component of the address.
    InvalidCharacter,
    /// The separator character between `local-part` and `domain` (character: '@') was missing.
    MissingSeparator,
    /// The `local-part` is an empty string.
    LocalPartEmpty,
    /// The `local-part` is is too long.
    LocalPartTooLong,
    /// The `domain` is an empty string.
    DomainEmpty,
    /// The `domain` is is too long.
    DomainTooLong,
    /// The `sub-domain` within the `domain` is empty.
    SubDomainEmpty,
    /// A `sub-domain` within the `domain` is is too long.
    SubDomainTooLong,
    /// Too few `sub-domain`s in `domain`.
    DomainTooFew,
    /// Invalid placement of the domain separator (character: '.').
    DomainInvalidSeparator,
    /// The quotes (character: '"') around `local-part` are unbalanced.
    UnbalancedQuotes,
    /// A Comment within the either the `local-part`, or `domain`, was malformed.
    InvalidComment,
    /// An IP address in a `domain-literal` was malformed.
    InvalidIPAddress,
    /// A `domain-literal` was supplied, but is unsupported by parser configuration.
    UnsupportedDomainLiteral,
    /// Display name was supplied, but is unsupported by parser configuration.
    UnsupportedDisplayName,
    /// Display name was not supplied, but email starts with '<'.
    MissingDisplayName,
    /// An email enclosed within <...> is missing the final '>'.
    MissingEndBracket,
}

///
/// Struct of options that can be configured when parsing with `parse_with_options`.
///
#[derive(Debug, Copy, Clone)]
pub struct Options {
    ///
    /// Sets the minimum number of domain segments that must exist to parse successfully.
    ///
    /// ```rust
    /// use email_address::*;
    ///
    /// assert!(
    ///     EmailAddress::parse_with_options(
    ///         "simon@localhost",
    ///         Options::default().with_no_minimum_sub_domains(),
    ///     ).is_ok()
    /// );
    /// assert_eq!(
    ///     EmailAddress::parse_with_options(
    ///         "simon@localhost",
    ///         Options::default().with_required_tld()
    ///     ),
    ///     Err(Error::DomainTooFew)
    /// );
    /// ```
    ///
    pub minimum_sub_domains: usize,

    ///
    /// Specifies if domain literals are allowed. Defaults to `true`.
    ///
    /// ```rust
    /// use email_address::*;
    ///
    /// assert!(
    ///     EmailAddress::parse_with_options(
    ///         "email@[127.0.0.256]",
    ///         Options::default().with_domain_literal()
    ///     ).is_ok()
    /// );
    ///
    /// assert_eq!(
    ///     EmailAddress::parse_with_options(
    ///         "email@[127.0.0.256]",
    ///         Options::default().without_domain_literal()
    ///     ),
    ///     Err(Error::UnsupportedDomainLiteral),
    /// );
    /// ```
    ///
    pub allow_domain_literal: bool,

    ///
    /// Specified whether display text is allowed. Defaults to `true`. If you want strict
    /// email-only checking setting this to `false` will remove support for the prefix string
    /// and therefore the '<' and '>' brackets around the email part.
    ///
    /// ```rust
    /// use email_address::*;
    ///
    /// assert_eq!(
    ///     EmailAddress::parse_with_options(
    ///         "Simon <simon@example.com>",
    ///         Options::default().without_display_text()
    ///     ),
    ///     Err(Error::UnsupportedDisplayName),
    /// );
    ///
    /// assert_eq!(
    ///     EmailAddress::parse_with_options(
    ///         "<simon@example.com>",
    ///         Options::default().without_display_text()
    ///     ),
    ///     Err(Error::InvalidCharacter),
    /// );
    /// ```
    ///
    pub allow_display_text: bool,
}

///
/// Type representing a single email address. This is basically a wrapper around a String, the
/// email address is parsed for correctness with `FromStr::from_str`, which is the only want to
/// create an instance. The various components of the email _are not_ parsed out to be accessible
/// independently.
///
#[derive(Debug, Clone)]
pub struct EmailAddress(String);

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const LOCAL_PART_MAX_LENGTH: usize = 64;
// see: https://www.rfc-editor.org/errata_search.php?rfc=3696&eid=1690
const DOMAIN_MAX_LENGTH: usize = 254;
const SUB_DOMAIN_MAX_LENGTH: usize = 63;

#[allow(dead_code)]
const CR: char = '\r';
#[allow(dead_code)]
const LF: char = '\n';
const SP: char = ' ';
const HTAB: char = '\t';
const ESC: char = '\\';

const AT: char = '@';
const DOT: char = '.';
const DQUOTE: char = '"';
const LBRACKET: char = '[';
const RBRACKET: char = ']';
#[allow(dead_code)]
const LPAREN: char = '(';
#[allow(dead_code)]
const RPAREN: char = ')';

const DISPLAY_SEP: &str = " <";
const DISPLAY_START: char = '<';
const DISPLAY_END: char = '>';

const MAILTO_URI_PREFIX: &str = "mailto:";

// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidCharacter => write!(f, "Invalid character."),
            Error::LocalPartEmpty => write!(f, "Local part is empty."),
            Error::LocalPartTooLong => write!(
                f,
                "Local part is too long. Length limit: {}",
                LOCAL_PART_MAX_LENGTH
            ),
            Error::DomainEmpty => write!(f, "Domain is empty."),
            Error::DomainTooLong => {
                write!(f, "Domain is too long. Length limit: {}", DOMAIN_MAX_LENGTH)
            }
            Error::SubDomainEmpty => write!(f, "A sub-domain is empty."),
            Error::SubDomainTooLong => write!(
                f,
                "A sub-domain is too long. Length limit: {}",
                SUB_DOMAIN_MAX_LENGTH
            ),
            Error::MissingSeparator => write!(f, "Missing separator character '{}'.", AT),
            Error::DomainTooFew => write!(f, "Too few parts in the domain"),
            Error::DomainInvalidSeparator => {
                write!(f, "Invalid placement of the domain separator '{:?}", DOT)
            }
            Error::InvalidIPAddress => write!(f, "Invalid IP Address specified for domain."),
            Error::UnbalancedQuotes => write!(f, "Quotes around the local-part are unbalanced."),
            Error::InvalidComment => write!(f, "A comment was badly formed."),
            Error::UnsupportedDomainLiteral => write!(f, "Domain literals are not supported."),
            Error::UnsupportedDisplayName => write!(f, "Display names are not supported."),
            Error::MissingDisplayName => write!(
                f,
                "Display name was not supplied, but email starts with '<'."
            ),
            Error::MissingEndBracket => write!(f, "Terminating '>' is missing."),
        }
    }
}

impl std::error::Error for Error {}

impl<T> From<Error> for std::result::Result<T, Error> {
    fn from(err: Error) -> Self {
        Err(err)
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for Options {
    fn default() -> Self {
        Self {
            minimum_sub_domains: Default::default(),
            allow_domain_literal: true,
            allow_display_text: true,
        }
    }
}

impl Options {
    /// Set the value of `minimum_sub_domains`.
    #[inline(always)]
    pub const fn with_minimum_sub_domains(self, min: usize) -> Self {
        Self {
            minimum_sub_domains: min,
            ..self
        }
    }
    #[inline(always)]
    /// Set the value of `minimum_sub_domains` to zero.
    pub const fn with_no_minimum_sub_domains(self) -> Self {
        Self {
            minimum_sub_domains: 0,
            ..self
        }
    }
    #[inline(always)]
    /// Set the value of `minimum_sub_domains` to two, this has the effect of requiring a
    /// domain name with a top-level domain (TLD).
    pub const fn with_required_tld(self) -> Self {
        Self {
            minimum_sub_domains: 2,
            ..self
        }
    }
    /// Set the value of `allow_domain_literal` to `true`.
    #[inline(always)]
    pub const fn with_domain_literal(self) -> Self {
        Self {
            allow_domain_literal: true,
            ..self
        }
    }
    /// Set the value of `allow_domain_literal` to `false`.
    #[inline(always)]
    pub const fn without_domain_literal(self) -> Self {
        Self {
            allow_domain_literal: false,
            ..self
        }
    }
    /// Set the value of `allow_display_text` to `true`.
    #[inline(always)]
    pub const fn with_display_text(self) -> Self {
        Self {
            allow_display_text: true,
            ..self
        }
    }
    /// Set the value of `allow_display_text` to `false`.
    #[inline(always)]
    pub const fn without_display_text(self) -> Self {
        Self {
            allow_display_text: false,
            ..self
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for EmailAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// From RFC 5321, section 2.4:
//
// The local-part of a mailbox MUST BE treated as case sensitive. Therefore,
// SMTP implementations MUST take care to preserve the case of mailbox
// local-parts. In particular, for some hosts, the user "smith" is different
// from the user "Smith". However, exploiting the case sensitivity of mailbox
// local-parts impedes interoperability and is discouraged. Mailbox domains
// follow normal DNS rules and are hence not case sensitive.
//

impl PartialEq for EmailAddress {
    fn eq(&self, other: &Self) -> bool {
        let (left, right) = split_at(&self.0).unwrap();
        let (other_left, other_right) = split_at(&other.0).unwrap();
        left.eq(other_left) && right.eq_ignore_ascii_case(other_right)
    }
}

impl Eq for EmailAddress {}

impl Hash for EmailAddress {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl FromStr for EmailAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_address(s, Default::default())
    }
}

impl From<EmailAddress> for String {
    fn from(email: EmailAddress) -> Self {
        email.0
    }
}

impl AsRef<str> for EmailAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "serde_support")]
impl Serialize for EmailAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde_support")]
impl<'de> Deserialize<'de> for EmailAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, Unexpected, Visitor};

        struct EmailAddressVisitor;

        impl Visitor<'_> for EmailAddressVisitor {
            type Value = EmailAddress;

            fn expecting(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
                fmt.write_str("string containing a valid email address")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                EmailAddress::from_str(s).map_err(|err| {
                    let exp = format!("{}", err);
                    Error::invalid_value(Unexpected::Str(s), &exp.as_ref())
                })
            }
        }

        deserializer.deserialize_str(EmailAddressVisitor)
    }
}

impl EmailAddress {
    ///
    /// Creates an `EmailAddress` without checking if the email is valid. Only
    /// call this method if the address is known to be valid.
    ///
    /// ```
    /// use std::str::FromStr;
    /// use email_address::EmailAddress;
    ///
    /// let unchecked = "john.doe@example.com";
    /// let email = EmailAddress::from_str(unchecked).expect("email is not valid");
    /// let valid_email = String::from(email);
    /// let email = EmailAddress::new_unchecked(valid_email);
    ///
    /// assert_eq!("John Doe <john.doe@example.com>", email.to_display("John Doe"));
    /// ```
    pub fn new_unchecked<S>(address: S) -> Self
    where
        S: Into<String>,
    {
        Self(address.into())
    }

    ///
    /// Parses an [EmailAddress] with custom [Options]. Useful for configuring validations
    /// that aren't mandatory by the specification.
    ///
    /// ```
    /// use email_address::{EmailAddress, Options};
    ///
    /// let options = Options { minimum_sub_domains: 2, ..Options::default() };
    /// let result = EmailAddress::parse_with_options("john.doe@localhost", options).is_ok();
    ///
    /// assert_eq!(result, false);
    /// ```
    pub fn parse_with_options(address: &str, options: Options) -> Result<Self, Error> {
        parse_address(address, options)
    }

    ///
    /// Determine whether the `address` string is a valid email address. Note this is equivalent to
    /// the following:
    ///
    /// ```rust
    /// use email_address::*;
    /// use std::str::FromStr;
    ///
    /// let is_valid = EmailAddress::from_str("johnstonskj@gmail.com").is_ok();
    /// ```
    ///
    pub fn is_valid(address: &str) -> bool {
        Self::from_str(address).is_ok()
    }

    ///
    /// Determine whether the `part` string would be a valid `local-part` if it were in an
    /// email address.
    ///
    pub fn is_valid_local_part(part: &str) -> bool {
        parse_local_part(part, Default::default()).is_ok()
    }

    ///
    /// Determine whether the `part` string would be a valid `domain` if it were in an
    /// email address.
    ///
    pub fn is_valid_domain(part: &str) -> bool {
        parse_domain(part, Default::default()).is_ok()
    }

    ///
    /// Return this email address formatted as a URI. This will also URI-encode the email
    /// address itself. So, `name@example.org` becomes `mailto:name@example.org`.
    ///
    /// ```rust
    /// use email_address::*;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("name@example.org").unwrap().to_uri(),
    ///     String::from("mailto:name@example.org")
    /// );
    /// ```
    ///
    pub fn to_uri(&self) -> String {
        let encoded = encode(&self.0);
        format!("{}{}", MAILTO_URI_PREFIX, encoded)
    }

    ///
    /// Return a string formatted as a display email with the user name. This is commonly used
    /// in email headers and other locations where a display name is associated with the
    /// address.
    ///
    /// ```rust
    /// use email_address::*;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("name@example.org").unwrap().to_display("My Name"),
    ///     String::from("My Name <name@example.org>")
    /// );
    /// ```
    ///
    pub fn to_display(&self, display_name: &str) -> String {
        format!("{} <{}>", display_name, self)
    }

    ///
    /// Returns the local part of the email address. This is borrowed so that no additional
    /// allocation is required.
    ///
    /// ```rust
    /// use email_address::*;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("name@example.org").unwrap().local_part(),
    ///     String::from("name")
    /// );
    /// ```
    ///
    pub fn local_part(&self) -> &str {
        let (local, _, _) = split_parts(&self.0).unwrap();
        local
    }

    ///
    /// Returns the display part of the email address. This is borrowed so that no additional
    /// allocation is required.
    ///
    /// ```rust
    /// use email_address::*;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("Name <name@example.org>").unwrap().display_part(),
    ///     String::from("Name")
    /// );
    /// ```
    ///
    pub fn display_part(&self) -> &str {
        let (_, _, display) = split_parts(&self.0).unwrap();
        display
    }

    ///
    /// Returns the email part of the email address. This is borrowed so that no additional
    /// allocation is required.
    ///
    /// ```rust
    /// use email_address::*;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("Name <name@example.org>").unwrap().email(),
    ///     String::from("name@example.org")
    /// );
    /// ```
    ///
    pub fn email(&self) -> String {
        let (local, domain, _) = split_parts(&self.0).unwrap();
        format!("{}{AT}{}", local, domain)
    }

    ///
    /// Returns the domain of the email address. This is borrowed so that no additional
    /// allocation is required.
    ///
    /// ```rust
    /// use email_address::*;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     EmailAddress::from_str("name@example.org").unwrap().domain(),
    ///     String::from("example.org")
    /// );
    /// ```
    ///
    pub fn domain(&self) -> &str {
        let (_, domain, _) = split_parts(&self.0).unwrap();
        domain
    }

    ///
    /// Returns the entire email address as a string reference.
    ///
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn encode(address: &str) -> String {
    let mut result = String::new();
    for c in address.chars() {
        if is_uri_reserved(c) {
            result.push_str(&format!("%{:02X}", c as u8))
        } else {
            result.push(c);
        }
    }
    result
}

fn is_uri_reserved(c: char) -> bool {
    // No need to encode '@' as this is allowed in the email scheme.
    c == '!'
        || c == '#'
        || c == '$'
        || c == '%'
        || c == '&'
        || c == '\''
        || c == '('
        || c == ')'
        || c == '*'
        || c == '+'
        || c == ','
        || c == '/'
        || c == ':'
        || c == ';'
        || c == '='
        || c == '?'
        || c == '['
        || c == ']'
}

fn parse_address(address: &str, options: Options) -> Result<EmailAddress, Error> {
    //
    // Deals with cases of '@' in `local-part`, if it is quoted they are legal, if
    // not then they'll return an `InvalidCharacter` error later.
    //
    let (local_part, domain, display) = split_parts(address)?;
    match (
        display.is_empty(),
        local_part.starts_with(DISPLAY_START),
        options.allow_display_text,
    ) {
        (false, _, false) => Err(Error::UnsupportedDisplayName),
        (true, true, true) => Err(Error::MissingDisplayName),
        (true, true, false) => Err(Error::InvalidCharacter),
        _ => {
            parse_local_part(local_part, options)?;
            parse_domain(domain, options)?;
            Ok(EmailAddress(address.to_owned()))
        }
    }
}

fn split_parts(address: &str) -> Result<(&str, &str, &str), Error> {
    let (display, email) = split_display_email(address)?;
    let (local_part, domain) = split_at(email)?;
    Ok((local_part, domain, display))
}

fn split_display_email(text: &str) -> Result<(&str, &str), Error> {
    match text.rsplit_once(DISPLAY_SEP) {
        None => Ok(("", text)),
        Some((left, right)) => {
            let right = right.trim();
            if !right.ends_with(DISPLAY_END) {
                Err(Error::MissingEndBracket)
            } else {
                let email = &right[0..right.len() - 1];
                let display_name = left.trim();

                Ok((display_name, email))
            }
        }
    }
}

fn split_at(address: &str) -> Result<(&str, &str), Error> {
    match address.rsplit_once(AT) {
        None => Error::MissingSeparator.into(),
        Some(left_right) => Ok(left_right),
    }
}

fn parse_local_part(part: &str, _: Options) -> Result<(), Error> {
    if part.is_empty() {
        Error::LocalPartEmpty.into()
    } else if part.len() > LOCAL_PART_MAX_LENGTH {
        Error::LocalPartTooLong.into()
    } else if part.starts_with(DQUOTE) && part.ends_with(DQUOTE) {
        // <= to handle `part` = `"` (single quote).
        if part.len() <= 2 {
            Error::LocalPartEmpty.into()
        } else {
            parse_quoted_local_part(&part[1..part.len() - 1])
        }
    } else {
        parse_unquoted_local_part(part)
    }
}

fn parse_quoted_local_part(part: &str) -> Result<(), Error> {
    if is_qcontent(part) {
        Ok(())
    } else {
        Error::InvalidCharacter.into()
    }
}

fn parse_unquoted_local_part(part: &str) -> Result<(), Error> {
    if is_dot_atom_text(part) {
        Ok(())
    } else {
        Error::InvalidCharacter.into()
    }
}

fn parse_domain(part: &str, options: Options) -> Result<(), Error> {
    if part.is_empty() {
        Error::DomainEmpty.into()
    } else if part.len() > DOMAIN_MAX_LENGTH {
        Error::DomainTooLong.into()
    } else if part.starts_with(LBRACKET) && part.ends_with(RBRACKET) {
        if options.allow_domain_literal {
            parse_literal_domain(&part[1..part.len() - 1])
        } else {
            Error::UnsupportedDomainLiteral.into()
        }
    } else {
        parse_text_domain(part, options)
    }
}

fn parse_text_domain(part: &str, options: Options) -> Result<(), Error> {
    let mut sub_domains = 0;

    for sub_part in part.split(DOT) {
        // As per https://www.rfc-editor.org/rfc/rfc1034#section-3.5
        // and https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address,
        // at least one character must exist in a `subdomain`/`label` part of the domain
        if sub_part.is_empty() {
            return Error::SubDomainEmpty.into();
        }

        // As per https://www.rfc-editor.org/rfc/rfc1034#section-3.5,
        // the domain label needs to start with a `letter`;
        // however, https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address
        // specifies a label can start
        // with a `let-dig` (letter or digit), so we allow the wider range

        if !sub_part.starts_with(char::is_alphanumeric) {
            return Error::InvalidCharacter.into();
        }
        // Both specifications mentioned above require the last character to be a
        // `let-dig` (letter or digit)
        if !sub_part.ends_with(char::is_alphanumeric) {
            return Error::InvalidCharacter.into();
        }

        if sub_part.len() > SUB_DOMAIN_MAX_LENGTH {
            return Error::SubDomainTooLong.into();
        }

        if !is_atom(sub_part) {
            return Error::InvalidCharacter.into();
        }

        sub_domains += 1;
    }

    if sub_domains < options.minimum_sub_domains {
        Error::DomainTooFew.into()
    } else {
        Ok(())
    }
}

fn parse_literal_domain(part: &str) -> Result<(), Error> {
    if part.chars().all(is_dtext_char) {
        return Ok(());
    }
    Error::InvalidCharacter.into()
}

// ------------------------------------------------------------------------------------------------

fn is_atext(c: char) -> bool {
    c.is_alphanumeric()
        || c == '!'
        || c == '#'
        || c == '$'
        || c == '%'
        || c == '&'
        || c == '\''
        || c == '*'
        || c == '+'
        || c == '-'
        || c == '/'
        || c == '='
        || c == '?'
        || c == '^'
        || c == '_'
        || c == '`'
        || c == '{'
        || c == '|'
        || c == '}'
        || c == '~'
        || is_utf8_non_ascii(c)
}

//fn is_special(c: char) -> bool {
//    c == '('
//        || c == ')'
//        || c == '<'
//        || c == '>'
//        || c == '['
//        || c == ']'
//        || c == ':'
//        || c == ';'
//        || c == '@'
//        || c == '\\'
//        || c == ','
//        || c == '.'
//        || c == DQUOTE
//}

fn is_utf8_non_ascii(c: char) -> bool {
    let bytes = (c as u32).to_be_bytes();
    // UTF8-non-ascii  =   UTF8-2 / UTF8-3 / UTF8-4
    match (bytes[0], bytes[1], bytes[2], bytes[3]) {
        // UTF8-2      = %xC2-DF UTF8-tail
        (0x00, 0x00, 0xC2..=0xDF, 0x80..=0xBF) => true,
        // UTF8-3      = %xE0 %xA0-BF UTF8-tail /
        //               %xE1-EC 2( UTF8-tail ) /
        //               %xED %x80-9F UTF8-tail /
        //               %xEE-EF 2( UTF8-tail )
        (0x00, 0xE0, 0xA0..=0xBF, 0x80..=0xBF) => true,
        (0x00, 0xE1..=0xEC, 0x80..=0xBF, 0x80..=0xBF) => true,
        (0x00, 0xED, 0x80..=0x9F, 0x80..=0xBF) => true,
        (0x00, 0xEE..=0xEF, 0x80..=0xBF, 0x80..=0xBF) => true,
        // UTF8-4      = %xF0 %x90-BF 2( UTF8-tail ) /
        //               %xF1-F3 3( UTF8-tail ) /
        //               %xF4 %x80-8F 2( UTF8-tail )
        (0xF0, 0x90..=0xBF, 0x80..=0xBF, 0x80..=0xBF) => true,
        (0xF1..=0xF3, 0x80..=0xBF, 0x80..=0xBF, 0x80..=0xBF) => true,
        (0xF4, 0x80..=0x8F, 0x80..=0xBF, 0x80..=0xBF) => true,
        // UTF8-tail   = %x80-BF
        _ => false,
    }
}

fn is_atom(s: &str) -> bool {
    !s.is_empty() && s.chars().all(is_atext)
}

fn is_dot_atom_text(s: &str) -> bool {
    s.split(DOT).all(is_atom)
}

fn is_vchar(c: char) -> bool {
    ('\x21'..='\x7E').contains(&c)
}

fn is_wsp(c: char) -> bool {
    c == SP || c == HTAB
}

fn is_qtext_char(c: char) -> bool {
    c == '\x21'
        || ('\x23'..='\x5B').contains(&c)
        || ('\x5D'..='\x7E').contains(&c)
        || is_utf8_non_ascii(c)
}

fn is_qcontent(s: &str) -> bool {
    let mut char_iter = s.chars();
    while let Some(c) = &char_iter.next() {
        if c == &ESC {
            // quoted-pair
            match char_iter.next() {
                Some(c2) if is_vchar(c2) => (),
                _ => return false,
            }
        } else if !(is_wsp(*c) || is_qtext_char(*c)) {
            // qtext
            return false;
        }
    }
    true
}

fn is_dtext_char(c: char) -> bool {
    ('\x21'..='\x5A').contains(&c) || ('\x5E'..='\x7E').contains(&c) || is_utf8_non_ascii(c)
}

//fn is_ctext_char(c: char) -> bool {
//    (c >= '\x21' && c == '\x27')
//        || ('\x2A'..='\x5B').contains(&c)
//        || ('\x5D'..='\x7E').contains(&c)
//        || is_utf8_non_ascii(c)
//}
//
//fn is_ctext(s: &str) -> bool {
//    s.chars().all(is_ctext_char)
//}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(feature = "serde_support")]
#[cfg(test)]
mod serde_tests {
    use super::*;
    use claims::{assert_err_eq, assert_ok, assert_ok_eq};
    use serde::de::{Error as _, Unexpected};
    use serde_assert::{Deserializer, Serializer, Token};

    #[test]
    fn test_serialize() {
        let email = assert_ok!(EmailAddress::from_str("simple@example.com"));

        let serializer = Serializer::builder().build();

        assert_ok_eq!(
            email.serialize(&serializer),
            [Token::Str("simple@example.com".to_owned())]
        );
    }

    #[test]
    fn test_deserialize() {
        let mut deserializer =
            Deserializer::builder([Token::Str("simple@example.com".to_owned())]).build();

        let email = assert_ok!(EmailAddress::from_str("simple@example.com"));
        assert_ok_eq!(EmailAddress::deserialize(&mut deserializer), email);
    }

    #[test]
    fn test_deserialize_invalid_value() {
        let mut deserializer =
            Deserializer::builder([Token::Str("Abc.example.com".to_owned())]).build();

        assert_err_eq!(
            EmailAddress::deserialize(&mut deserializer),
            serde_assert::de::Error::invalid_value(
                Unexpected::Str("Abc.example.com"),
                &"Missing separator character '@'."
            )
        );
    }

    #[test]
    fn test_deserialize_invalid_type() {
        let mut deserializer = Deserializer::builder([Token::U64(42)]).build();

        assert_err_eq!(
            EmailAddress::deserialize(&mut deserializer),
            serde_assert::de::Error::invalid_type(
                Unexpected::Unsigned(42),
                &"string containing a valid email address"
            )
        );
    }

    // Regression test: GitHub issue #26
    #[test]
    fn test_serde_roundtrip() {
        let email = assert_ok!(EmailAddress::from_str("simple@example.com"));

        let serializer = Serializer::builder().build();
        let mut deserializer =
            Deserializer::builder(assert_ok!(email.serialize(&serializer))).build();

        assert_ok_eq!(EmailAddress::deserialize(&mut deserializer), email);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(address: &str, test_case: Option<&str>) {
        if let Some(test_case) = test_case {
            println!(">> test case: {}", test_case);
            println!("     <{}>", address);
        } else {
            println!(">> <{}>", address);
        }
        assert!(EmailAddress::is_valid(address));
    }

    fn valid_with_options(address: &str, options: Options, test_case: Option<&str>) {
        if let Some(test_case) = test_case {
            println!(">> test case: {}", test_case);
            println!("     <{}>", address);
        } else {
            println!(">> <{}>", address);
        }
        assert!(EmailAddress::parse_with_options(address, options).is_ok());
    }

    #[test]
    fn test_good_examples_from_wikipedia_01() {
        is_valid("simple@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_02() {
        is_valid("very.common@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_03() {
        is_valid("disposable.style.email.with+symbol@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_04() {
        is_valid("other.email-with-hyphen@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_05() {
        is_valid("fully-qualified-domain@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_06() {
        is_valid(
            "user.name+tag+sorting@example.com",
            Some(" may go to user.name@example.com inbox depending on mail server"),
        );
    }

    #[test]
    fn test_good_examples_from_wikipedia_07() {
        is_valid("x@example.com", Some("one-letter local-part"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_08() {
        is_valid("example-indeed@strange-example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_09() {
        is_valid(
            "admin@mailserver1",
            Some("local domain name with no TLD, although ICANN highly discourages dotless email addresses")
        );
    }

    #[test]
    fn test_good_examples_from_wikipedia_10() {
        is_valid(
            "example@s.example",
            Some("see the List of Internet top-level domains"),
        );
    }

    #[test]
    fn test_good_examples_from_wikipedia_11() {
        is_valid("\" \"@example.org", Some("space between the quotes"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_12() {
        is_valid("\"john..doe\"@example.org", Some("quoted double dot"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_13() {
        is_valid(
            "mailhost!username@example.org",
            Some("bangified host route used for uucp mailers"),
        );
    }

    #[test]
    fn test_good_examples_from_wikipedia_14() {
        is_valid(
            "user%example.com@example.org",
            Some("% escaped mail route to user@example.com via example.org"),
        );
    }

    #[test]
    fn test_good_examples_from_wikipedia_15() {
        is_valid("jsmith@[192.168.2.1]", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_16() {
        is_valid("jsmith@[IPv6:2001:db8::1]", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_17() {
        is_valid("user+mailbox/department=shipping@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_18() {
        is_valid("!#$%&'*+-/=?^_`.{|}~@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_19() {
        // '@' is allowed in a quoted local part. Sorry.
        is_valid("\"Abc@def\"@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_20() {
        is_valid("\"Joe.\\\\Blow\"@example.com", None);
    }

    #[test]
    fn test_good_examples_from_wikipedia_21() {
        is_valid("用户@例子.广告", Some("Chinese"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_22() {
        is_valid("अजय@डाटा.भारत", Some("Hindi"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_23() {
        is_valid("квіточка@пошта.укр", Some("Ukranian"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_24() {
        is_valid("θσερ@εχαμπλε.ψομ", Some("Greek"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_25() {
        is_valid("Dörte@Sörensen.example.com", Some("German"));
    }

    #[test]
    fn test_good_examples_from_wikipedia_26() {
        is_valid("коля@пример.рф", Some("Russian"));
    }

    #[test]
    fn test_good_examples_01() {
        valid_with_options(
            "foo@example.com",
            Options {
                minimum_sub_domains: 2,
                ..Default::default()
            },
            Some("minimum sub domains"),
        );
    }

    #[test]
    fn test_good_examples_02() {
        valid_with_options(
            "email@[127.0.0.256]",
            Options {
                allow_domain_literal: true,
                ..Default::default()
            },
            Some("minimum sub domains"),
        );
    }

    #[test]
    fn test_good_examples_03() {
        valid_with_options(
            "email@[2001:db8::12345]",
            Options {
                allow_domain_literal: true,
                ..Default::default()
            },
            Some("minimum sub domains"),
        );
    }

    #[test]
    fn test_good_examples_04() {
        valid_with_options(
            "email@[2001:db8:0:0:0:0:1]",
            Options {
                allow_domain_literal: true,
                ..Default::default()
            },
            Some("minimum sub domains"),
        );
    }

    #[test]
    fn test_good_examples_05() {
        valid_with_options(
            "email@[::ffff:127.0.0.256]",
            Options {
                allow_domain_literal: true,
                ..Default::default()
            },
            Some("minimum sub domains"),
        );
    }

    #[test]
    fn test_good_examples_06() {
        valid_with_options(
            "email@[2001:dg8::1]",
            Options {
                allow_domain_literal: true,
                ..Default::default()
            },
            Some("minimum sub domains"),
        );
    }

    #[test]
    fn test_good_examples_07() {
        valid_with_options(
            "email@[2001:dG8:0:0:0:0:0:1]",
            Options {
                allow_domain_literal: true,
                ..Default::default()
            },
            Some("minimum sub domains"),
        );
    }

    #[test]
    fn test_good_examples_08() {
        valid_with_options(
            "email@[::fTzF:127.0.0.1]",
            Options {
                allow_domain_literal: true,
                ..Default::default()
            },
            Some("minimum sub domains"),
        );
    }

    // ------------------------------------------------------------------------------------------------

    #[test]
    fn test_to_strings() {
        let email = EmailAddress::from_str("коля@пример.рф").unwrap();

        assert_eq!(String::from(email.clone()), String::from("коля@пример.рф"));

        assert_eq!(email.to_string(), String::from("коля@пример.рф"));

        assert_eq!(email.as_ref(), "коля@пример.рф");
    }

    #[test]
    fn test_to_display() {
        let email = EmailAddress::from_str("коля@пример.рф").unwrap();

        assert_eq!(
            email.to_display("коля"),
            String::from("коля <коля@пример.рф>")
        );
    }

    #[test]
    fn test_touri() {
        let email = EmailAddress::from_str("коля@пример.рф").unwrap();

        assert_eq!(email.to_uri(), String::from("mailto:коля@пример.рф"));
    }

    // ------------------------------------------------------------------------------------------------

    fn expect(address: &str, error: Error, test_case: Option<&str>) {
        if let Some(test_case) = test_case {
            println!(">> test case: {}", test_case);
            println!("     <{}>, expecting {:?}", address, error);
        } else {
            println!(">> <{}>, expecting {:?}", address, error);
        }
        assert_eq!(EmailAddress::from_str(address), error.into());
    }

    fn expect_with_options(address: &str, options: Options, error: Error, test_case: Option<&str>) {
        if let Some(test_case) = test_case {
            println!(">> test case: {}", test_case);
            println!("     <{}>, expecting {:?}", address, error);
        } else {
            println!(">> <{}>, expecting {:?}", address, error);
        }
        assert_eq!(
            EmailAddress::parse_with_options(address, options),
            error.into()
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_00() {
        expect(
            "Abc.example.com",
            Error::MissingSeparator,
            Some("no @ character"),
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_01() {
        expect(
            "A@b@c@example.com",
            Error::InvalidCharacter,
            Some("only one @ is allowed outside quotation marks"),
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_02() {
        expect(
            "a\"b(c)d,e:f;g<h>i[j\\k]l@example.com",
            Error::InvalidCharacter,
            Some("none of the special characters in this local-part are allowed outside quotation marks")
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_03() {
        expect(
            "just\"not\"right@example.com",
            Error::InvalidCharacter,
            Some(
                "quoted strings must be dot separated or the only element making up the local-part",
            ),
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_04() {
        expect(
            "this is\"not\\allowed@example.com",
            Error::InvalidCharacter,
            Some("spaces, quotes, and backslashes may only exist when within quoted strings and preceded by a backslash")
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_05() {
        expect(
            "this\\ still\"not\\allowed@example.com",
            Error::InvalidCharacter,
            Some("even if escaped (preceded by a backslash), spaces, quotes, and backslashes must still be contained by quotes")
        );
    }

    #[test]
    fn test_bad_examples_from_wikipedia_06() {
        expect(
            "1234567890123456789012345678901234567890123456789012345678901234+x@example.com",
            Error::LocalPartTooLong,
            Some("local part is longer than 64 characters"),
        );
    }

    #[test]
    fn test_bad_example_01() {
        expect(
            "foo@example.v1234567890123456789012345678901234567890123456789012345678901234v.com",
            Error::SubDomainTooLong,
            Some("domain part is longer than 64 characters"),
        );
    }

    #[test]
    fn test_bad_example_02() {
        expect(
            "@example.com",
            Error::LocalPartEmpty,
            Some("local-part is empty"),
        );
    }

    #[test]
    fn test_bad_example_03() {
        expect(
            "\"\"@example.com",
            Error::LocalPartEmpty,
            Some("local-part is empty"),
        );
        expect(
            "\"@example.com",
            Error::LocalPartEmpty,
            Some("local-part is empty"),
        );
    }

    #[test]
    fn test_bad_example_04() {
        expect("simon@", Error::DomainEmpty, Some("domain is empty"));
    }

    #[test]
    fn test_bad_example_05() {
        expect(
            "example@invalid-.com",
            Error::InvalidCharacter,
            Some("domain label ends with hyphen"),
        );
    }

    #[test]
    fn test_bad_example_06() {
        expect(
            "example@-invalid.com",
            Error::InvalidCharacter,
            Some("domain label starts with hyphen"),
        );
    }

    #[test]
    fn test_bad_example_07() {
        expect(
            "example@invalid.com-",
            Error::InvalidCharacter,
            Some("domain label starts ends hyphen"),
        );
    }

    #[test]
    fn test_bad_example_08() {
        expect(
            "example@inv-.alid-.com",
            Error::InvalidCharacter,
            Some("subdomain label ends hyphen"),
        );
    }

    #[test]
    fn test_bad_example_09() {
        expect(
            "example@-inv.alid-.com",
            Error::InvalidCharacter,
            Some("subdomain label starts hyphen"),
        );
    }

    #[test]
    fn test_bad_example_10() {
        expect(
            "example@-.com",
            Error::InvalidCharacter,
            Some("domain label is hyphen"),
        );
    }

    #[test]
    fn test_bad_example_11() {
        expect(
            "example@-",
            Error::InvalidCharacter,
            Some("domain label is hyphen"),
        );
    }

    #[test]
    fn test_bad_example_12() {
        expect(
            "example@-abc",
            Error::InvalidCharacter,
            Some("domain label starts with hyphen"),
        );
    }

    #[test]
    fn test_bad_example_13() {
        expect(
            "example@abc-",
            Error::InvalidCharacter,
            Some("domain label ends with hyphen"),
        );
    }

    #[test]
    fn test_bad_example_14() {
        expect(
            "example@.com",
            Error::SubDomainEmpty,
            Some("subdomain label is empty"),
        );
    }

    #[test]
    fn test_bad_example_15() {
        expect_with_options(
            "foo@localhost",
            Options::default().with_minimum_sub_domains(2),
            Error::DomainTooFew,
            Some("too few domains"),
        );
    }

    #[test]
    fn test_bad_example_16() {
        expect_with_options(
            "foo@a.b.c.d.e.f.g.h.i",
            Options::default().with_minimum_sub_domains(10),
            Error::DomainTooFew,
            Some("too few domains"),
        );
    }

    #[test]
    fn test_bad_example_17() {
        expect_with_options(
            "email@[127.0.0.256]",
            Options::default().without_domain_literal(),
            Error::UnsupportedDomainLiteral,
            Some("unsupported domain literal (1)"),
        );
    }

    #[test]
    fn test_bad_example_18() {
        expect_with_options(
            "email@[2001:db8::12345]",
            Options::default().without_domain_literal(),
            Error::UnsupportedDomainLiteral,
            Some("unsupported domain literal (2)"),
        );
    }

    #[test]
    fn test_bad_example_19() {
        expect_with_options(
            "email@[2001:db8:0:0:0:0:1]",
            Options::default().without_domain_literal(),
            Error::UnsupportedDomainLiteral,
            Some("unsupported domain literal (3)"),
        );
    }

    #[test]
    fn test_bad_example_20() {
        expect_with_options(
            "email@[::ffff:127.0.0.256]",
            Options::default().without_domain_literal(),
            Error::UnsupportedDomainLiteral,
            Some("unsupported domain literal (4)"),
        );
    }

    // make sure Error impl Send + Sync
    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}

    #[test]
    fn test_error_traits() {
        is_send::<Error>();
        is_sync::<Error>();
    }

    #[test]
    fn test_parse_trimmed() {
        let email = EmailAddress::parse_with_options(
            "  Simons Email    <simon@example.com> ",
            Options::default(),
        )
        .unwrap();

        assert_eq!(email.display_part(), "Simons Email");
        assert_eq!(email.email(), "simon@example.com");
    }

    #[test]
    // Feature test: GitHub PR: #15
    fn test_parse_display_name() {
        let email = EmailAddress::parse_with_options(
            "Simons Email <simon@example.com>",
            Options::default(),
        )
        .unwrap();

        assert_eq!(email.display_part(), "Simons Email");
        assert_eq!(email.email(), "simon@example.com");
        assert_eq!(email.local_part(), "simon");
        assert_eq!(email.domain(), "example.com");
    }

    #[test]
    // Feature test: GitHub PR: #15
    fn test_parse_display_empty_name() {
        expect(
            "<simon@example.com>",
            Error::MissingDisplayName,
            Some("missing display name"),
        );
    }

    #[test]
    // Feature test: GitHub PR: #15
    // Reference: GitHub issue #14
    fn test_parse_display_empty_name_2() {
        expect_with_options(
            "<simon@example.com>",
            Options::default().without_display_text(),
            Error::InvalidCharacter,
            Some("without display text '<' is invalid"),
        );
    }

    #[test]
    // Feature test: GitHub PR: #15
    // Reference: GitHub issue #14
    fn test_parse_display_name_unsupported() {
        expect_with_options(
            "Simons Email <simon@example.com>",
            Options::default().without_display_text(),
            Error::UnsupportedDisplayName,
            Some("unsupported display name (1)"),
        );
    }

    #[test]
    // Regression test: GitHub issue #23
    fn test_missing_tld() {
        EmailAddress::parse_with_options("simon@localhost", Options::default()).unwrap();
        EmailAddress::parse_with_options(
            "simon@localhost",
            Options::default().with_no_minimum_sub_domains(),
        )
        .unwrap();

        expect_with_options(
            "simon@localhost",
            Options::default().with_required_tld(),
            Error::DomainTooFew,
            Some("too few domain segments"),
        );
    }

    #[test]
    // Regression test: GitHub issue #11
    fn test_eq_name_case_sensitive_local() {
        let email = EmailAddress::new_unchecked("simon@example.com");

        assert_eq!(email, EmailAddress::new_unchecked("simon@example.com"));
        assert_ne!(email, EmailAddress::new_unchecked("Simon@example.com"));
        assert_ne!(email, EmailAddress::new_unchecked("simoN@example.com"));
    }

    #[test]
    // Regression test: GitHub issue #11
    fn test_eq_name_case_insensitive_domain() {
        let email = EmailAddress::new_unchecked("simon@example.com");

        assert_eq!(email, EmailAddress::new_unchecked("simon@Example.com"));
        assert_eq!(email, EmailAddress::new_unchecked("simon@example.COM"));
    }

    #[test]
    // Regression test: GitHub issue #21
    fn test_utf8_non_ascii() {
        assert!(!is_utf8_non_ascii('A'));
        assert!(!is_utf8_non_ascii('§'));
        assert!(!is_utf8_non_ascii('�'));
        assert!(!is_utf8_non_ascii('\u{0F40}'));
        assert!(is_utf8_non_ascii('\u{C2B0}'));
    }
}

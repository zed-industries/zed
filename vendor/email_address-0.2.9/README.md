# Crate email_address

A Rust crate providing an implementation of an RFC-compliant `EmailAddress` newtype. 

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![crates.io](https://img.shields.io/crates/v/email_address.svg)](https://crates.io/crates/email_address)
[![docs.rs](https://docs.rs/email_address/badge.svg)](https://docs.rs/email_address)
![Build](https://github.com/johnstonskj/rust-email_address/workflows/Rust/badge.svg)
![Audit](https://github.com/johnstonskj/rust-email_address/workflows/Security%20audit/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-email_address.svg)](https://github.com/johnstonskj/rust-email_address/stargazers)

Primarily for validation, the `EmailAddress` type is constructed with
`FromStr::from_str` which will raise any parsing errors. Prior to constructions
the functions `is_valid`, `is_valid_local_part`, and `is_valid_domain` may also be
used to test for validity without constructing an instance.

## Status

Currently, it supports all the RFC ASCII and UTF-8 character set rules as well
as quoted and unquoted local parts but does not yet support all the productions
required for SMTP headers; folding whitespace, comments, etc.

## Example

```rust
use email_address::*;

assert!(EmailAddress::is_valid("user.name+tag+sorting@example.com"));

assert_eq!(
    EmailAddress::from_str("Abc.example.com"),
    Error::MissingSeparator.into()
);
```

## Specifications

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
1. RFC 5890: [_Internationalized Domain Names for Applications (IDNA): Definitions
   and Document Framework_](https://tools.ietf.org/html/rfc5890), IETF, Aug 2010.
1. RFC 6531: [_SMTP Extension for Internationalized Email_](https://tools.ietf.org/html/rfc6531),
   IETF, Feb 2012
1. RFC 6532: [_Internationalized Email Headers_](https://tools.ietf.org/html/rfc6532),
   IETF, Feb 2012.

## Changes

### Version 0.2.9

* Fixed bug [#21](https://github.com/johnstonskj/rust-email_address/issues/21): Invalid Unicode characters accepted.

### Version 0.2.8

* Fixed bug [#29](https://github.com/johnstonskj/rust-email_address/issues/29): Put back implementation of `Eq`.

### Version 0.2.7

* Feature: added builder functions to the `Option` type.
* Documentation: added examples to the `Option` type documentation.

### Version 0.2.6

* Fix: merge issues.

### Version 0.2.5

* Feature: Pull Request #15 -- Potential enhancement to add any free-text as
  display name.
* Feature: Pull Request #17 -- Check for non-alphanumeric character starting or
  ending domain parts.
* Feature: Pull Request #18 -- Error with `SubDomainEmpty` when empty instead of
  `InvalidCharacter`.
* Feature: Pull Request #19 -- Allow configuring minimum number of subdomains.
* Feature: Pull Request #20 -- Add option to disallow domain literals.
* Feature: Pull Request #22 -- Handle a single qoute in local part of email

Thanks to [ghandic](https://github.com/ghandic), [blaine-arcjet](https://github.com/blaine-arcjet),
[Thomasdezeeuw](https://github.com/Thomasdezeeuw).

### Version 0.2.4

* Fixed bug [#11](https://github.com/johnstonskj/rust-email_address/issues/11):
  1. Add manual implementation of `PartialEq` with case insensitive comparison for
     domain part.
  2. Add manual implementation of `Hash`, because above.
* Change signature for `new_unchecked` to be more flexible.
* Add `as_str` helper method.

### Version 0.2.3

* Added new `EmailAddress::new_unchecked` function ([Sören Meier](https://github.com/soerenmeier)).

### Version 0.2.2

* Removed manual `Send` and `Sync` implementation, and fixed documentation bug
  ([Sören Meier](https://github.com/soerenmeier)).

### Version 0.2.1

* Added `From<EmailAddress>` for `String`.
* Added `AsRef<str` for `EmailAddress`.
* Added `local_part` and `domain` accessors.
* More unit tests, especially for the list above.
* Added more conditions to the warning and deny list.
* Fixed some Clippy warnings.
* Fixed a bug in encoding the mailto URI scheme.

### Version 0.2.0

* Added UTF-8 support.
* Added more test cases, fixing defects in parsing.
* Method `to_uri` now supports URI encoding the address as a part of the URI.
* Added `is_valid_local_part` and `is_valid_domain` methods.

### Version 0.1.0

* Basic type implementation and structure based on RFC 5322.
* See TODO.

## TODO

1. Support comments.
1. Support line-feed and whitespace rules.
1. Does not parse _into_ `domain-literal` values, only does surface syntax check.

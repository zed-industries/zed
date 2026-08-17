# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).
This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html),
with the exception that 0.x versions can break between minor versions.

## [0.10.0] - 2023-06-24
### Added
- New option `url_can_bi_iri` that can be used to disable international
  domain parsing (still enabled by default). This is useful for parsing
  ASCII only URLs out of non-ASCII text even if there's no spaces
  before/after the URL (#49)
### Changed
- Don't require valid hostname for `file://` URLs to allow for relative
  paths used in the wild (#58)
- Support URIs with empty path but with query, e.g. `foo://bar?a=b` (#59)
- Require IPv4 addresses to have 4 numeric parts. This means things
  like `1.0` are no longer recognized as domains (#46)

## [0.9.0] - 2022-07-11
### Changed
- More strict parsing of hostname (authority) part of URLs. Applies to
  emails, plain domains URLs (e.g. `example.com/foo`) and URLs with
  schemes where a host is expected (e.g. `https`).

  This fixes a few problems that have been reported over time, namely:

  - `https://www.example..com` is no longer parsed as an URL (#41)
  - `foo@v1.1.1` is no longer parsed as an email address (#29)
  - `https://*.example.org` is no longer parsed as an URL (#38)

  It's a tricky change and hopefully this solves some problems while
  not introducing too many new ones. If anything unexpectedly changed
  for you, please let us know!

## [0.8.1] - 2022-04-14
### Changed
- Skip parsing very short strings for URLs as a performance optimization

## [0.8.0] - 2021-11-26
### Added
- New option `url_must_have_scheme` on `LinkFinder` that can be set to
  `false` to allow URLs without scheme/protocol such as `example.com`.
  Note that there is no allowlist for top-level domains, if you want
  that you will have to implement it yourself.
### Changed
- Bump MSRV (minimum supported Rust version) to 1.46

## [0.7.0] - 2021-05-18
### Changed
- URLs that have a quote character like `'` or `"` before them will stop
  when that quote character is encountered, e.g. in
  `"https://example.org/","`, the URL will not include any quotes. Before,
  it would run until the end because the quotes after the slash were an
  even number. (#20)
- Bump MSRV (minimum supported Rust version) to 1.41

## [0.6.0] - 2021-04-09
### Changed
- Stop URLs when encountering `|`. Consistent with RFC and will
  hopefully not cause problems with real URLs.

## [0.5.0] - 2021-02-13
### Changed
- Treat `*` as a delimiter like `.` or `,`, which means they can be part
  of an URL but not at the end.

## [0.4.0] - 2019-08-05
### Changed
- Stop URLs when encountering ". This is consistent with RFC 3986, and
  it seems unlikely that a user would have an unescaped " in an URL
  anyway, as browsers escape it when copying an URL with it.
- Stop URLs at \` characters too, same as < and >
- Bump MSRV (minimum supported Rust version) to 1.31 (2018 edition)

## [0.3.1] - 2018-02-04
### Changed
- Bump memchr dependency to 2 (for wasm support)

## [0.3.0] - 2018-01-25
### Added
- New API to iterate over both plain text and link spans using the
  `spans` method. This is useful for iterating over all parts of the
  input, not just the detected links. Thanks @srijs!

## [0.2.0] - 2017-09-18
### Changed
- Don't autolink if authority is only "end" characters, e.g. like
  `http://.` or `http://"`

## [0.1.2] - 2017-06-09
### Fixed
- Fix `html_root_url` attribute

## [0.1.1] - 2017-06-08
### Added
- More docs
- Add `Debug` impls for `Links` and `LinkFinder`

## [0.1.0] - 2017-05-13
### Added
Initial release of linkify, a Rust library to find links such as URLs and email
addresses in plain text, handling surrounding punctuation correctly.


[0.10.0]: https://github.com/robinst/linkify/compare/0.9.0...0.10.0
[0.9.0]: https://github.com/robinst/linkify/compare/0.8.1...0.9.0
[0.8.1]: https://github.com/robinst/linkify/compare/0.8.0...0.8.1
[0.8.0]: https://github.com/robinst/linkify/compare/0.7.0...0.8.0
[0.7.0]: https://github.com/robinst/linkify/compare/0.6.0...0.7.0
[0.6.0]: https://github.com/robinst/linkify/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/robinst/linkify/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/robinst/linkify/compare/0.3.1...0.4.0
[0.3.1]: https://github.com/robinst/linkify/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/robinst/linkify/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/robinst/linkify/compare/0.1.2...0.2.0
[0.1.2]: https://github.com/robinst/linkify/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/robinst/linkify/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/robinst/linkify/commits/0.1.0

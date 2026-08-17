# Unreleased

# 4.1.2

* fix: unexpected namespace switches after cleanup can cause mXSS

# 4.1.1

* chore: upgrade to [html5ever 0.35][]

[html5ever 0.35]: https://github.com/servo/html5ever/releases/tag/html5ever-v0.35.0

# 4.1.0

* chore: bump MSRV to 1.80.0
* chore: switch to `std`'s `LazyLock` instead of `once_cell`'s `Lazy`
* feature: improve panic message when `clean_content_tags` conflicts with other options
* feature: add support for sanitizing the `style` attribute

# 4.0.0

* Update html5ever to 0.27
* feature: change lifetimes on UrlRelative (breaking change)
* chore: use the `Display` trait (and `ToString`) instead of an inherent `to_string` method (breaking change)
* Security fixes are not backported to the 2.0 branch any more.
  Only the v3 and v4 branches are supported.

# 3.3.0

* chore: bump MSRV to 1.60.0
* docs: fix incorrect XSS example
* feature: add `UrlRelative::RewriteWithRoot`

# 3.2.1

* chore: bump MSRV to 1.51.0 <https://github.com/serde-rs/serde/issues/2255>
* chore: fix broken links in documentation

# 3.2.0

* chore: bump MSRV to 1.49.0
* chore: update to html5ever 0.26
* chore: switch from lazy_static to once_cell
* feature: add `Builder::empty()` constructor

# 3.1.4

* chore: use `#[non_exhaustive]` instead of hidden variant for `UrlRelative` policy
* chore: remove unnecessary dependency on third-party `matches!` macro

# 3.1.3

* fix: incorrect FF/CR handling in `clean_text`
* fix: split class name attribute by all ASCII whitespace, not just SP 0x20
* docs: fix incorrect English in `is_html` documentation

# 3.1.2

* fix: unexpected namespace switches can allow XSS via svg/mathml parsing

# 3.1.1

* fix: Crash on invalid URLs in some configurations ([issue #136](https://github.com/rust-ammonia/ammonia/issues/136))

# 3.1.0

* feature: Whitelist generic attribute prefix
* chore: Update html5ever to 0.25
* feature: support adding attribute values to elements

# 3.0.0

* Add [`clean_text`] function.
* Updated to [rust-url 2.0].
* Updated to the 2018 edition.

[`clean_text`]: https://docs.rs/ammonia/3.0.0/ammonia/fn.clean_text.html
[rust-url 2.0]: https://docs.rs/url/2.0.0/url/

# 2.1.4

* fix: split class name attribute by all ASCII whitespace, not just SP 0x20 (backported from 3.1.3)

# 2.1.3

* fix: unexpected namespace switches can allow XSS via svg/mathml parsing (backported from 3.1.2)

# 2.1.2

* Fix a memory leak caused by certain node types.

# 2.1.1

* Update dependencies

# 2.1.0

* Bump minimum supported Rust version to 1.30.
* Fix a potential DoS attack from pathologically nested input.

# 2.0.0

* Add "script" and "style" to the default set of [clean content tags]
* Make all iterator-accepting APIs use `IntoIterator` and `Borrow`, so that you can pass slices directly to them.

[clean content tags]: https://docs.rs/ammonia/2.0.0/ammonia/struct.Builder.html#method.clean_content_tags

# 1.2.0

* Recognize `action`, `formaction` and `ping` as [URL attributes] for scheme and origin filtering
* Add [`Builder::url_filter_map`] which allows URLs, both relative and absolute, to be pre-filtered

[URL attributes]: https://html.spec.whatwg.org/multipage/indices.html#attributes-3
[`Builder::url_filter_map`]: https://docs.rs/ammonia/1.2/ammonia/struct.Builder.html#method.url_filter_map

# 1.1.0

* Add [`Builder::clean_content_tags`] which allows elements to be removed entirely instead of just having the tags removed

[`Builder::clean_content_tags`]: https://docs.rs/ammonia/1.1/ammonia/struct.Builder.html#method.clean_content_tags

# 1.0.1

* Update dependencies

# 1.0.0

* Breaking change: The `Ammonia` struct is now called `Builder` and uses that pattern for better forward compatibility
* Breaking change: The `Builder::clean()` method now returns a `Document` struct instead of a `String`. You can use the `Document::to_string` method to obtain a `String`.
* Breaking change: `keep_cleaned_elements` has changed from being an off-by-default option to the only supported behavior
* Breaking change: Using a tag with `allowed_classes` means that the class attribute is banned from `tag_attributes` (it used to be required)
* Breaking change: The default set of allowed elements and attributes was expanded
* Added support for reading the input from a stream
* Added `UrlRelative::Custom`, allowing you to write your own relative URL resolver
* Changed `UrlRelative::RewriteWithBase` take a custom URL. This made the `url` crate a public dependency.
* Added `id_prefix`, which can be used to avoid element `id` collisions with the rest of the page
* Added property getters to `Builder`, to see what everything is currently set to
* Added property modifiers, to change the existing whitelist (instead of completely replacing it)

# 0.7.0

* Add `allowed_classes`, allowing the user to set only specific items that can go in the class attribute

# 0.6.1

* Fix a bug in the traversal code

# 0.6.0

* Resolve relative URLs with a given base (off by default, you need to specify that base URL)
* Add `rel="noreferrer noopener"` to links, as a security measure
* Avoid closing void tags, such as turning `<br>` into `<br></br>`
* Bump the html5ever version
* Switch to using docs.rs to host docs

# 0.5.0

* Bump html5ever to 0.18 (this updates serde from 0.9 to 1.0)

# 0.4.0

* Upgrade to html5ever 0.17

# 0.3.0

* Add an option to keep elements that had attributes removed

# 0.2.0

* Removed the strip option. Not a security problem, but it was wrong and looked stupid. I'm not going to reintroduce this until html5ever allows me to preserve the original text enough to have non-stripped tags come out exactly like they go in.
* Treat the data attribute of object as a URL. In non-default configurations, this could have been a leak.
* Update to the newest html5ever.

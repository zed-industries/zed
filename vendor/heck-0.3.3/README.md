# **heck** is a case conversion library

!["I specifically requested the opposite of this."](./no_step_on_snek.png)

This library exists to provide case conversion between common cases like
CamelCase and snake_case. It is intended to be unicode aware, internally
consistent, and reasonably well performing.

## Definition of a word boundary

Word boundaries are defined as the "unicode words" defined in the
`unicode_segmentation` library, as well as within those words in this manner:

1. All underscore characters are considered word boundaries.
2. If an uppercase character is followed by lowercase letters, a word boundary
is considered to be just prior to that uppercase character.
3. If multiple uppercase characters are consecutive, they are considered to be
within a single word, except that the last will be part of the next word if it
is followed by lowercase characters (see rule 2).

That is, "HelloWorld" is segmented `Hello|World` whereas "XMLHttpRequest" is
segmented `XML|Http|Request`.

Characters not within words (such as spaces, punctuations, and underscores)
are not included in the output string except as they are a part of the case
being converted to. Multiple adjacent word boundaries (such as a series of
underscores) are folded into one. ("hello__world" in snake case is therefore
"hello_world", not the exact same string). Leading or trailing word boundary
indicators are dropped, except insofar as CamelCase capitalizes the first word.

## Cases contained in this library:

1. CamelCase
2. snake_case
3. kebab-case
4. SHOUTY_SNAKE_CASE
5. mixedCase
6. Title Case
7. SHOUTY-KEBAB-CASE

## Contributing

PRs of additional well-established cases welcome.

This library is a little bit opinionated (dropping punctuation, for example).
If that doesn't fit your use case, I hope there is another crate that does. I
would prefer **not** to receive PRs to make this behavior more configurable.

Bug reports & fixes always welcome. :-)

## MSRV

The minimum supported Rust version for this crate is 1.32.0. This may change in
minor or patch releases, but we probably won't ever require a very recent
version. If you would like to have a stronger guarantee than that, please open
an issue.

## License

heck is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.

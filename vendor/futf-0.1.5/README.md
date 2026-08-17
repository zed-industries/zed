# futf

[![Build Status](https://travis-ci.org/servo/futf.svg?branch=master)](https://travis-ci.org/kmcallister/futf)

futf is a library for *flexible* UTF-8, or UTF-8 *fragments*. I don't know.
Check out the [API documentation](http://doc.servo.org/futf/index.html).

Anyway, it takes an index into a byte buffer and tells you things about the
UTF-8 codepoint containing that byte. It can deal with incomplete codepoint
prefixes / suffixes at the ends of a buffer, which is useful for incremental
I/O. It can also handle UTF-16 surrogate code units encoded in the manner of
[CESU-8][] or [WTF-8][].

This is a low-level helper for [tendril][] that might be useful more generally.

[CESU-8]: http://www.unicode.org/reports/tr26/
[WTF-8]: http://simonsapin.github.io/wtf-8/
[tendril]: https://github.com/kmcallister/tendril

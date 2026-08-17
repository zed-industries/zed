# chardetng

[![crates.io](https://meritbadge.herokuapp.com/chardetng)](https://crates.io/crates/chardetng)
[![docs.rs](https://docs.rs/chardetng/badge.svg)](https://docs.rs/chardetng/)
[![Apache 2 / MIT dual-licensed](https://img.shields.io/badge/license-Apache%202%20%2F%20MIT-blue.svg)](https://github.com/hsivonen/chardetng/blob/master/COPYRIGHT)

A character encoding detector for legacy Web content.

## Licensing

Please see the file named
[COPYRIGHT](https://github.com/hsivonen/chardetng/blob/master/COPYRIGHT).

## Documentation

Generated [API documentation](https://docs.rs/chardetng/) is available
online.

There is a [long-form write-up](https://hsivonen.fi/chardetng/) about the
design and motivation of the crate.

## Purpose

The purpose of this detector is user retention for Firefox by ensuring that the long tail of the legacy Web is not more convenient to use in Chrome than in Firefox. (Chrome deployed [ced](https://github.com/google/compact_enc_det/), which left Firefox less convenient to use until the deployment of this detector.)

## About the Name

`chardet` was the name of Mozilla's old encoding detector. I named this one `chardetng`, because this the next generation of encoding detector in Firefox. There is no code reuse from the old `chardet`.

## Optimization Goals

This crate aims to be more accurate than ICU, more complete than `chardet`, more explainable and modifiable than `compact_enc_det` (aka. ced), and, in an application that already depends on `encoding_rs` for other reasons, smaller in added binary footprint than `compact_enc_det`.

## Rayon support

Enabling the optional feature `multithreading` makes `chardetng` run the detectors for individual encodings in parallel. Unfortunately, the performance doesn't scale linearly with CPU cores, but it's still better than single-threaded performance in terms of wall-clock time if a single instance of `chardetng` is running. In terms of combined CPU core usage, the `multithreading` mode is quite a bit worse than the single-threaded more, so if you can find a parallelization point at some higher-level task such that you could have multiple instances of `chardetng` running in paraller each on a single thread, you'll get better results doing that.

## `no_std` support

`chardetng` works in a `no_std` environment that does not have an allocator.

## Principle of Operation

In general `chardetng` prefers to do negative matching (rule out possibilities from the set of plausible encodings) than to do positive matching. Since negative matching is insufficient, there is positive matching, too.

* Except for ISO-2022-JP, pairs of ASCII bytes never contribute to the detection, which has the effect of ignoring HTML syntax without an HTML-aware state machine.
* A single encoding error disqualifies an encoding from the set of possible outcomes. Notably, as the length of the input increases, it becomes increasingly improbable for the input to be valid according to a legacy CJK encoding without being intended as such. Also, there are single-byte encodings that have unmapped bytes in areas that are in active use by other encodings, so such bytes narrow the set of possibilities very effectively.
* A single occurrence of a C1 control character disqualifies an encoding from possible outcomes.
* The first non-ASCII character being a half-width katakana character disqualifies an encoding. (This is _very_ effective for deciding between Shift_JIS and EUC-JP.)
* For single-byte encodings, character pairs are given scores according to their relative frequencies in the applicable Wikipedias.
* There's a variety of smaller penalty rules, such as:
   - For encodings for bicameral scripts, having an upper-case letter follow a lower-case letter is penalized.
   - For Latin encodings, having three non-ASCII letters in a row is penalized a little and having four or more is penalized a lot.
   - For non-Latin encodings, having a non-Latin letter right next to a Latin letter is penalized.
   - For single-byte encodings, having a character pair (excluding pairs where both characters are ASCII) that never occurs in the Wikipedias for the applicable languages is heavily penalized.
   - Turkish I paired with a space-like character does not get a score to avoid detecting English as Turkish.
   - There's a dedicated state machine for giving score to windows-1252 ordinal indicators, which would otherwise be hard to give score to without breaking windows-1250 Romanian detection.
   - 0xA0, which is no-break space in most single-byte encodings, is special-cased in IBM866 and CJK encodings to avoid misdetection.

## Notes About Encodings

<dl>
<dt>UTF-8</dt>
<dd>Detected only if explicitly permitted by the argument to the `guess` method. It's harmful for Web browsers to detect UTF-8 without requiring user action, such as choosing a menu item, because Web developers would start relying on the detection.</dd>
<dt>UTF-16[BE|LE]</dt>
<dd>Not detected: Detecting these belongs on the BOM layer.</dd>
<dt>x-user-defined</dt>
<dd>Not detected: This encoding is for XHR. <code>&lt;meta charset=x-user-defined></code> in HTML is not unlabeled and means windows-1252.</dd>
<dt>Replacement</dt>
<dd>Not detected.</dd>
<dt>GB18030</dt>
<dd>Detected as GBK.</dd>
<dt>GBK</dt>
<dt>Big5</dt>
<dt>EUC-KR</dt>
<dt>Shift_JIS</dt>
<dt>windows-1250</dt>
<dt>windows-1251</dt>
<dt>windows-1252</dt>
<dt>windows-1253</dt>
<dt>windows-1254</dt>
<dt>windows-1255</dt>
<dt>windows-1256</dt>
<dt>windows-1257</dt>
<dt>windows-1258</dt>
<dt>windows-874</dt>
<dt>ISO-8859-2</dt>
<dt>ISO-8859-7</dt>
<dd>Detected: Historical locale-specific fallbacks.</dd>
<dt>EUC-JP</dt>
<dt>ISO-2022-JP</dt>
<dt>KOI8-U</dt>
<dt>ISO-8859-5</dt>
<dt>IBM866</dt>
<dd>Detected: Detected by multiple browsers past and present.</dd>
<dt>KOI8-R</dt>
<dd>Detected as KOI8-U. (Always guessing the U variant is less likely to corrupt non-box drawing characters.)</dd>
<dt>ISO-8859-8-I</dt>
<dd>Detected as windows-1255.</dd>
<dt>ISO-8859-4</dt>
<dd>Detected: Detected by IE and Chrome; in menu in IE and Firefox.</dd>
<dt>ISO-8859-6</dt>
<dd>Detected: Detected by IE and Chrome.</dd>
<dt>ISO-8859-8</dt>
<dd>Detected: Available in menu in IE and Firefox.</dd>
<dt>ISO-8859-13</dt>
<dd>Detected: Detected by Chrome. This encoding is so similar to windows-1257 that menu items for windows-1257 can be considered to accommodate this one in IE and Firefox. Due to the mechanics of this detector, if this wasn't included as a separate item, the windows-1257 detection wouldn't catch the cases that use curly quotes and are invalid as windows-1257.</dd>
<dt>x-mac-cyrillic</dt>
<dd>Not detected: Not detected by IE and Chrome. (Was previously detected by Firefox.)</dd>
<dt>ISO-8859-3</dt>
<dt>ISO-8859-10</dt>
<dt>ISO-8859-14</dt>
<dt>ISO-8859-15</dt>
<dt>ISO-8859-16</dt>
<dt>macintosh</dt>
<dd>Not detected: These encodings have never been a locale-specific fallback in a major browser or a menu item in IE.</dd>
</dl>

## Known Problems

* GBK detection is less accurate than in ced for short titles consisting of fewer than six hanzi. This is mostly due to the design that prioritizes optimizing binary size over accuracy on very short inputs.
* Thai detection is inaccurate for short inputs.
* windows-1257 detection is very inaccurate. (This detector currently doesn't use trigrams. ced uses 8 KB of trigram data to solve this.)
* On non-generic domains, some encodings that are confusable with the legacy encodings native to the TLD are excluded from guesses outright unless the input is invalid according to all the TLD-native encodings.

## Associated tools

* [traindet](https://github.com/hsivonen/traindet) tool for computing the statistics for the generated code
* [detector_char_classes](https://github.com/hsivonen/detector_char_classes/) classification of characters in the single-byte encodings
* [charcounts](https://github.com/hsivonen/charcounts) intermediate files for traindet that make it possible to rerun the code generation without rerunning the statistic gathering
* [testdet](https://github.com/hsivonen/testdet) testing tool

## Roadmap

No planned improvements.

- [x] Investigate parallelizing the `feed` method using Rayon.
- [x] Improve windows-874 detection for short inputs.
- [ ] ~Improve GBK detection for short inputs.~
- [ ] ~Reorganize the frequency data for telling short GBK, EUC-JP, and EUC-KR inputs apart.~
- [ ] ~Make Lithuanian and Latvian detection on generic domains a lot more accurate (likely requires looking at trigrams).~
- [x] Tune Central European detection.
- [ ] ~Tune the penalties applied to confusable encodings on non-generic TLDs to make detection of confusable encodings possible on non-generic TLDs.~
- [x] Reduce the binary size by not storing the scoring for implausible-next-to-alphabetic character classes.
- [ ] ~Reduce the binary size by classifying ASCII algorithmically.~
- [ ] ~Reduce the binary size by not storing the scores for C1 controls.~

## Release Notes

### 0.1.17

* Handle non-space space-like bytes following a windows-1252 copyright sign.

### 0.1.16

* Detect windows-1252 copyright sign surrounded by spaces as windows-1252.

### 0.1.15

* Make the crate work in `no_std` without an allocator.

### 0.1.14

* Add `guess_assess` that provides more information about the guess.
* Upgrade the `cfg-if` dependency to 1.0.

### 0.1.13

* Undo the limit on CJK extra scoring (from version 0.1.10). This change never made it to Gecko and, therefore, wasn't validated with in-practice telemetry.
* Detect inputs that have a lot of half-width katakana.

### 0.1.12

* Fix edge case oversights in code added in the previous release.

### 0.1.11

* Tolerate Windows and Classic Mac OS extensions to legacy CJK encodings and tolerate JIS X 0213 extensions to Shift_JIS and EUC-JP (not ISO-2022-JP).

### 0.1.10

* Stop computing extra scores for common CJK characters after enough extra-score-eligible characters have been seen. (Improves performance with long CJK inputs.)
* Add Rayon support.
* Avoid detecting windows-1252 euro sign as GBK.

### 0.1.9

* Fix a bug in ASCII prefix skipping. (Was introduced in 0.1.7.)

### 0.1.8

* Avoid detecting English with no-break spaces as GBK or EUC-KR.

### 0.1.7

* Avoid misdetecting windows-1252 English as windows-1254.
* Avoid misdetecting windows-1252 English as IBM866.
* Improve Chinese and Japanese detection by not giving single-byte encodings score for letter next to digit.
* Improve Italian, Portuguese, Castilian, Catalan, and Galician detection by taking into account ordinal indicator use.
* Reduce lookup table size.

### 0.1.6

* Tune Central European detection.

### 0.1.5

* Improve Thai accuracy a lot.
* Improve accuracy of some languages a bit.
* Remove unused Hebrew ASCII table.

### 0.1.4

* Properly take into account non-ASCII bytes at word boundaries for windows-1252. (Especially relevant for Italian and Catalan.)
* Move Estonian from the Baltic model to the Western model. This improves overall Estonian detection but causes š and ž encoded as windows-1257, ISO-8859-13, or ISO-8859-4 to get misdecoded. (It would be possible to add a post-processing step to adjust for š and ž, but this would cause reloads given the way chardetng is integrated with Firefox.)
* Properly classify letters that ISO-8859-4 has but windows-1257 doesn't have in order to avoid misdetecting non-ISO-8859-4 input as ISO-8859-4.
* Improve character classification of windows-1254.
* Avoid classifying byte 0xA1 or above as space-like.
* Reduce binary size by collapsing similar character classes.

### 0.1.3

* Return TLD-affiliated encoding if UTF-8 is valid but prohibited.

### 0.1.2

* Return UTF-8 if valid and allowed even if all-ASCII.
* Return windows-1252 if UTF-8 valid and prohibited, because various test cases require this.

### 0.1.1

* Detect Visual Hebrew more often.

### 0.1.0

* Initial release.

# roxmltree
![Build Status](https://github.com/RazrFalcon/roxmltree/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/roxmltree.svg)](https://crates.io/crates/roxmltree)
[![Documentation](https://docs.rs/roxmltree/badge.svg)](https://docs.rs/roxmltree)
[![Rust 1.60+](https://img.shields.io/badge/rust-1.60+-orange.svg)](https://www.rust-lang.org)

Represents an [XML](https://www.w3.org/TR/xml/) document as a read-only tree.

```rust
// Find element by id.
let doc = roxmltree::Document::parse("<rect id='rect1'/>")?;
let elem = doc.descendants().find(|n| n.attribute("id") == Some("rect1"))?;
assert!(elem.has_tag_name("rect"));
```

## Why read-only?

Because in some cases all you need is to retrieve some data from an XML document.
And for such cases, we can make a lot of optimizations.

## Parsing behavior

Sadly, XML can be parsed in many different ways. *roxmltree* tries to mimic the
behavior of Python's [lxml](https://lxml.de/).
For more details see [docs/parsing.md](https://github.com/RazrFalcon/roxmltree/blob/master/docs/parsing.md).

## Alternatives

| Feature/Crate                   | roxmltree        | [libxml2]           | [xmltree]        | [sxd-document]   |
| ------------------------------- | :--------------: | :-----------------: | :--------------: | :--------------: |
| Element namespace resolving     | ✓                | ✓                   | ✓                | ~<sup>1</sup>    |
| Attribute namespace resolving   | ✓                | ✓                   |                  | ✓                |
| [Entity references]             | ✓                | ✓                   | ×                | ×                |
| [Character references]          | ✓                | ✓                   | ✓                | ✓                |
| [Attribute-Value normalization] | ✓                | ✓                   |                  |                  |
| Comments                        | ✓                | ✓                   |                  | ✓                |
| Processing instructions         | ✓                | ✓                   | ✓                | ✓                |
| UTF-8 BOM                       | ✓                | ✓                   | ×                | ×                |
| Non UTF-8 input                 |                  | ✓                   |                  |                  |
| Complete DTD support            |                  | ✓                   |                  |                  |
| Position preserving<sup>2</sup> | ✓                | ✓                   |                  |                  |
| HTML support                    |                  | ✓                   |                  |                  |
| Tree modification               |                  | ✓                   | ✓                | ✓                |
| Writing                         |                  | ✓                   | ✓                | ✓                |
| No **unsafe**                   | ✓                |                     | ✓                |                  |
| Language                        | Rust             | C                   | Rust             | Rust             |
| Dependencies                    | **0**            | -                   | 2                | 2                |
| Tested version                  | 0.20.0           | Apple-provided      | 0.10.3           | 0.3.2            |
| License                         | MIT / Apache-2.0 | MIT                 | MIT              | MIT              |

Legend:

- ✓ - supported
- × - parsing error
- ~ - partial
- *nothing* - not supported

Notes:

1. No default namespace propagation.
2. *roxmltree* keeps all node and attribute positions in the original document,
   so you can easily retrieve it if you need it.
   See [examples/print_pos.rs](examples/print_pos.rs) for details.

There is also `elementtree` and `treexml` crates, but they are abandoned for a long time.

[Entity references]: https://www.w3.org/TR/REC-xml/#dt-entref
[Character references]: https://www.w3.org/TR/REC-xml/#NT-CharRef
[Attribute-Value Normalization]: https://www.w3.org/TR/REC-xml/#AVNormalize

[libxml2]: http://xmlsoft.org/
[xmltree]: https://crates.io/crates/xmltree
[sxd-document]: https://crates.io/crates/sxd-document

## Performance

Here are some benchmarks comparing `roxmltree` to other XML tree libraries.

```text
test huge_roxmltree      ... bench:   2,997,887 ns/iter (+/- 48,976)
test huge_libxml2        ... bench:   6,850,666 ns/iter (+/- 306,180)
test huge_sdx_document   ... bench:   9,440,412 ns/iter (+/- 117,106)
test huge_xmltree        ... bench:  41,662,316 ns/iter (+/- 850,360)

test large_roxmltree     ... bench:   1,494,886 ns/iter (+/- 30,384)
test large_libxml2       ... bench:   3,250,606 ns/iter (+/- 140,201)
test large_sdx_document  ... bench:   4,242,162 ns/iter (+/- 99,740)
test large_xmltree       ... bench:  13,980,228 ns/iter (+/- 229,363)

test medium_roxmltree    ... bench:     421,137 ns/iter (+/- 13,855)
test medium_libxml2      ... bench:     950,984 ns/iter (+/- 34,099)
test medium_sdx_document ... bench:   1,618,270 ns/iter (+/- 23,466)
test medium_xmltree      ... bench:   4,315,974 ns/iter (+/- 31,849)

test tiny_roxmltree      ... bench:       2,522 ns/iter (+/- 31)
test tiny_libxml2        ... bench:       8,931 ns/iter (+/- 235)
test tiny_sdx_document   ... bench:      11,658 ns/iter (+/- 82)
test tiny_xmltree        ... bench:      20,215 ns/iter (+/- 303)
```

When comparing to streaming XML parsers `roxmltree` is slightly slower than `quick-xml`,
but still way faster than `xmlrs`.
Note that streaming parsers usually do not provide a proper string unescaping,
DTD resolving and namespaces support.

```text
test huge_quick_xml      ... bench:   2,997,887 ns/iter (+/- 48,976)
test huge_roxmltree      ... bench:   3,147,424 ns/iter (+/- 49,153)
test huge_xmlrs          ... bench:  36,258,312 ns/iter (+/- 180,438)

test large_quick_xml     ... bench:   1,250,053 ns/iter (+/- 21,943)
test large_roxmltree     ... bench:   1,494,886 ns/iter (+/- 30,384)
test large_xmlrs         ... bench:  11,239,516 ns/iter (+/- 76,937)

test medium_quick_xml    ... bench:     206,232 ns/iter (+/- 2,157)
test medium_roxmltree    ... bench:     421,137 ns/iter (+/- 13,855)
test medium_xmlrs        ... bench:   3,975,916 ns/iter (+/- 44,967)

test tiny_quick_xml      ... bench:       2,233 ns/iter (+/- 70)
test tiny_roxmltree      ... bench:       2,522 ns/iter (+/- 31)
test tiny_xmlrs          ... bench:      17,155 ns/iter (+/- 429)
```

### Notes

The benchmarks were taken on a Apple M1 Pro.
You can try running the benchmarks yourself by running `cargo bench` in the `benches` dir.

- Since all libraries have a different XML support, benchmarking is a bit pointless.
- We bench *libxml2* using the *[rust-libxml]* wrapper crate

[xml-rs]: https://crates.io/crates/xml-rs
[quick-xml]: https://crates.io/crates/quick-xml
[rust-libxml]: https://github.com/KWARC/rust-libxml

## Memory overhead

`roxmltree` tries to use as little memory as possible to allow parsing
very large (multi-GB) XML files.

The peak memory usage doesn't directly correlate with the file size
but rather with the amount of nodes and attributes a file has.
How many attributes had to be normalized (i.e. allocated).
And how many text nodes had to be preprocessed (i.e. allocated).

`roxmltree` never allocates element and attribute names, processing instructions
and comments.

By disabling the `positions` feature, you can shave 8 bytes from each node and attribute.

On average, the overhead is around 6-8x the file size.
For example, our 1.1GB sample XML will peak at 7.6GB RAM with default features enabled
and at 6.8GB RAM when `positions` is disabled.

## Safety

- This library must not panic. Any panic should be considered a critical bug and reported.
- This library forbids `unsafe` code.

## API

This library uses Rust's idiomatic API based on iterators.
In case you are more familiar with browser/JS DOM APIs - you can check out
[tests/dom-api.rs](tests/dom-api.rs) to see how it can be mapped onto the Rust one.

## License

Licensed under either of

- [Apache License v2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

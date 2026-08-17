# elasticlunr-rs 

![Build Status](https://github.com/mattico/elasticlunr-rs/workflows/CI/badge.svg)
[![Documentation](https://docs.rs/elasticlunr-rs/badge.svg)](https://docs.rs/elasticlunr-rs)
[![Crates.io](https://img.shields.io/crates/v/elasticlunr-rs.svg)](https://crates.io/crates/elasticlunr-rs)
![Maintenance](https://img.shields.io/badge/Maintenance-Passive-yellow)
![MSRV](https://img.shields.io/badge/MSRV-1.57.0-orange)

A partial port of [elasticlunr.js][eljs] to Rust. Intended to be used for generating compatible search indices.

This library is passively maintained to support existing users. New users are encouraged to use a different library such as [stork](https://github.com/jameslittle230/stork).

## Example

```Rust
use std::fs::File;
use std::io::Write;
use elasticlunr::Index;

let mut index = Index::new(&["title", "body"]);
index.add_doc("1", &["This is a title", "This is body text!"]);
// Add more documents...
let mut file = File::create("out.json").unwrap();
file.write_all(index.to_json_pretty().as_bytes());
```

## Minimum Supported Rust Version

1.60.0

Changing the minimum supported Rust version is not considered a breaking change for semver purposes.

The supported version is constrained by the version supported by our transitive dependencies. Earlier rustc versions may
work if you have older versions of these in your `Cargo.lock`, but this is not tested.

## Languages

This library includes optional support for non-English languages, see the features in `Cargo.toml`. Like in the JavaScript
version, the language support is designed to be compatible with the [lunr-languages plugins][lunr-languages]. Some
languages use a modified version, which is included in the `js` directory of the repository.

## License

This repository is offered under the terms of the

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted 
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Includes code ported from [elasticlunr.js][eljs] Copyright (C) 2017 by Wei Song, 
used under license. See LICENSE-JS for details.

Includes stop word lists ported from [stopwords-filter][swft] Copyright (C) 2012 
David J. Brenes, used under license. See LICENSE-WORDS for details.

Bundled javascript code in the repository (not included in the cargo package) may have other licenses.

[lunr-languages]: https://github.com/MihaiValentin/lunr-languages
[eljs]: https://github.com/weixsong/elasticlunr.js
[swft]: https://github.com/brenes/stopwords-filter
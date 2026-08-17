# svg_path_ops

[![Crates.io](https://img.shields.io/crates/v/svg_path_ops.svg)](https://crates.io/crates/svg_path_ops)
[![Documentation](https://docs.rs/svg_path_ops/badge.svg)](https://docs.rs/svg_path_ops)
[![License](https://img.shields.io/github/license/orhanbalci/rough-rs.svg)](https://github.com/orhanbalci/rough-rs/blob/main/svg_path_ops/LICENSE)

<!-- cargo-sync-readme start -->


This crate includes utility functions to work with svg paths. Works on types from [svgtypes](https://github.com/RazrFalcon/svgtypes)
crate.

This package exposes functions to manipulate svg paths with simplification purposes. Also a path transformer fully compatible with
[svgpath](https://github.com/fontello/svgpath) is provided.


## 📦 Cargo.toml

```toml
[dependencies]
svg_path_ops = "0.6"
```

## 🔧 Example

### Translate

``` rust,ignore
let translated_path = PathTransformer::new(cat_svg_path)
    .translate(230.0, 0.0)
    .to_string();
```

[full example](https://github.com/orhanbalci/rough-rs/blob/main/rough_piet/examples/translate.rs)

### 🖨️ Output Translate
![translate](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/svg_path_ops/assets/translated_cat.png)

### Rotate

``` rust,ignore
let translated_path = PathTransformer::new(cat_svg_path)
    .rotate(90.0, 126.0, 140.0)
    .translate(220.0, 0.0)
    .to_string();
```

[full example](https://github.com/orhanbalci/rough-rs/blob/main/rough_piet/examples/rotate.rs)

### 🖨️ Output Rotate
![translate](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/svg_path_ops/assets/rotated_cat.png)

### Skew
``` rust,ignore
let translated_path = PathTransformer::new(cat_svg_path)
    .skew_x(20.0)
    .translate(180.0, 0.0)
    .to_string();
```

[full example](https://github.com/orhanbalci/rough-rs/blob/main/rough_piet/examples/skew.rs)

### 🖨️ Output Skew
![translate](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/svg_path_ops/assets/skewed_cat.png)

### Scale
``` rust,ignore
let translated_path = PathTransformer::new(cat_svg_path)
    .scale(0.5, 0.5)
    .translate(220.0, 60.0)
    .to_string();
```

[full example](https://github.com/orhanbalci/rough-rs/blob/main/rough_piet/examples/scale.rs)

### 🖨️ Output Scale
![translate](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/svg_path_ops/assets/scaled_cat.png)

<!-- cargo-sync-readme end -->

## 📝 License

Licensed under MIT License ([LICENSE](LICENSE)).

### 🚧 Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.

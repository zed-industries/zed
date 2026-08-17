# points_on_curve 〰️ 📌

[![Crates.io](https://img.shields.io/crates/v/points_on_curve.svg)](https://crates.io/crates/points_on_curve)
[![Documentation](https://docs.rs/points_on_curve/badge.svg)](https://docs.rs/points_on_curve)
[![License](https://img.shields.io/github/license/orhanbalci/rough-rs.svg)](https://github.com/orhanbalci/rough-rs/blob/main/points_on_curve/LICENSE)

<!-- cargo-sync-readme start -->


This crate is a rustlang port of [points-on-curve](https://github.com/pshihn/bezier-points) npm package written by
[@pshihn](https://github.com/pshihn).

This package exposes functions to sample points on a bezier curve with certain tolerance.
There is also a utility funtion to simplify the shape to use fewer points.
This can really be useful when estimating lines/polygons for curves in WebGL or for Hit/Collision detections.
Reverse of this operation is also supported meaning given some points generate bezier curve points passing through this points


## 📦 Cargo.toml

```toml
[dependencies]
points_on_curve = "0.1"
```

## 🔧 Example

```rust
use euclid::{default, point2};
use points_on_curve::points_on_bezier_curves;

let input = vec![
        point2(70.0, 240.0),
        point2(145.0, 60.0),
        point2(275.0, 90.0),
        point2(300.0, 230.0),
    ];
let result_015 = points_on_bezier_curves(&input, 0.2, Some(0.15));

```


## 🖨️ Output
This picture shows computed points with 4 different distance values 0.15, 0.75, 1.5 and 3.0 with tolerance 2.0.

![tolerance](https://raw.githubusercontent.com/orhanbalci/rough-rs/main/points_on_curve/assets/tolerance.png)


## 🔭 Examples

For more examples have a look at the
[examples](https://github.com/orhanbalci/rough-rs/blob/main/points_on_curve/examples) folder.




<!-- cargo-sync-readme end -->

## 📝 License

Licensed under MIT License ([LICENSE](LICENSE)).

### 🚧 Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.

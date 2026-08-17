## 1.1.0

- Unify with `image::math::nq` as per https://github.com/image-rs/image/issues/1338 (https://github.com/image-rs/color_quant/pull/10)
  - A new method `lookup` from `image::math::nq` is added
  - More references in docs
  - Some style improvements and better names for functions borrowed from  `image::math::nq`
- Replace the internal `clamp!` macro with the `clamp` function (https://github.com/image-rs/color_quant/pull/8)

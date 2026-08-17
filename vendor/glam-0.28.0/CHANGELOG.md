# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [0.28.0] - 2024-06-10

### Breaking changes

* Removed derives from `glam::deref` types used by `Deref` on SIMD vector
  types.  These unintentionally added support for traits like `PartialOrd` to
  SIMD vector types. This may break existing code that was depending on this.
  Please use `cmple().all()` etc. instead of `PartialOrd` methods.

* Removed `impl From<Vec4> for Vec3A` as this violated the `From` trait
  contract that conversions should be lossless. Please use the explicit
  `Vec3A::from_vec4` method instead.

* Renamed 2D vector `angle_between` to `angle_to` to differentiate from the 3D
  `angle_between` which has different semantics to the 2D version.

### Added

* Added aarch64 neon support.

* Added `rotate_towards` methods for 2D vectors and quaternions.

* Added `Vec3A::from_vec4` method which can perform a no-op conversion when
  SIMD is used. This replaces the `impl From<Vec4> for Vec3A` implementation.

## [0.27.0] - 2024-03-23

### Breaking changes

* Changed implementation of vector `fract` method to match the Rust
  implementation instead of the GLSL implementation, that is `self -
  self.trunc()` instead of `self - self.floor()`.

### Added

* Added vector `fract_gl` which uses the GLSL specification of fract,
 `self - self.floor()`.

## [0.26.0] - 2024-03-18

### Breaking changes

* Minimum Supported Rust Version bumped to 1.68.2 for
 `impl From<bool> for {f32,f64}` support.

### Fixed

* Respect precision format specifier in Display implementations. Previously it
  was ignored.

* Corrected precision documentation for vector `is_normalized` methods and
  changed the internal check to use `2e-4` to better match the documented
  precision value of `1e-4`.

### Added

 * Added `with_x`, `with_y`, etc. to vector types which returns a copy of
   the vector with the new component value.

 * Added `midpoint` method to vector types that returns the point between two
   points.

 * Added `move_towards` for float vector types.

 * Added saturating add and sub methods for signed and unsigned integer vector
   types.

 * Added element wise sum and product methods for vector types.

 * Added element wise absolute values method for matrix types.

 * Added `from_array` method for boolean vector types.

 * Added `normalize_or` method to vector types that returns the specified value
   if normalization failed.

 * Added `From<BVecN>` support for all vector types.

 * Added `Div` and `DivAssign` by scalar implementations to matrix types.

## [0.25.0] - 2023-12-19

### Breaking changes

* Changed `Vec4` to always used `BVec4A` as a mask type, regardless if the
  target architecture has SIMD support in glam. Previously this was inconsistent
  on different hardware like ARM. This will have a slight performance cost when
  SIMD is not available. `Vec4` will continue to use `BVec4` as a mask type when
  the `scalar-math` feature is used.

### Fixed

* Made `Affine2` implement the `bytemuck::AnyBitPattern` trait instead of
  `bytemuck::Pod` as it contains internal padding due to `Mat2` being 16 byte
  aligned.

* Updated the `core-simd` implementation to build on latest nightly.

### Added

* Added `to_angle` method to 2D vectors.

* Added `FloatExt` trait which adds `lerp`, `inverse_lerp` and `remap` methods
  to `f32` and `f64` types.

* Added `i16` and `u16` vector types, `I16Vec2`, `I16Vec3`, `I16Vec4`,
  `U16Vec2`, `U16Vec3` and `U16Vec4`.

### Changed

* Renamed `Quat::as_f64()` to `Quat::as_dquat()` and `DQuat::as_f32()` to
  `DQuat::as_quat()` to be consistent with other types. The old methods have
  been deprecated.

* Added the `#[must_use]` attribute to all pure functions following the
  guidelines for the Rust standard library.

## [0.24.2] - 2023-09-23

### Fixed

* Fixed singularities in `Quat::to_euler`.

### Added

* Added `div_euclid` and `rem_euclid` to integer vector types.

* Added wrapping and saturating arithmetic operations to integer vector types.

* Added `to_scale_angle_translation` to 2D affine types.

* Added `mul_assign` ops to affine types.

### Changed

* Disable default features on optional `rkyv` dependency.

## [0.24.1] - 2023-06-24

### Added

* Implemented missing `bytemuck`, `mint`, `rand`, `rkyv` and `serde` traits for
  `i64` and `u64` types.

* Added missing safe `From` conversions from `f32` vectors to `f64` vectors.

* Added `TryFrom` implementations between different vector types.

* Added `test` and `set` methods to `bool` vector types for testing and setting
  individual mask elements.

* Added `MIN`, `MAX`, `INFINITY` and `NEG_INFINITY` vector constants.

## [0.24.0] - 2023-04-24

### Breaking changes

* Enabling `libm` in a `std` build now overrides the `std` math functions. This
  is unlikely to break anything but it is a change in behavior.

### Added

* Added `i64` and `u64` vector types; `I64Vec2`, `I64Vec3`, `I64Vec4`,
  `U64Vec2`, `U64Vec3` and `U64Vec4`.

* Added `length_squared` method on signed and unsigned integer vector types.

* Added `distance_squared` method on signed integer vector types.

* Implemented the `bytemuck` `AnyBitPattern` trait on `Vec3A`, `Mat3A` and
  `Affine3A`.

### Changed

* Changed quaternion `to_axis_angle` for improved numerical stability.

### Removed

* Removed dependency on `serde_derive` for improved compile times when using
  `serde`.

## [0.23.0] - 2023-02-22

### Breaking changes

* When the `scalar-math` feature is enabled the vector mask type for `Vec3A` was
  changed from `BVec3` to `BVec3A`.

### Added

* Added `copysign` method to signed vector types.

## [0.22.0] - 2022-10-24

### Breaking changes

* Added `u32` implementation of `BVec3A` and `BVec4` when SIMD is not available.
  These are used instead of aliasing to the `bool` implementations.

* Removed `Add`, `Sub`, and scalar `Mul` implementations from affine types as
  they didn't make sense on these types.

* Removed deprecated `const_*` macros. These have been replaced by `const fn`
  methods.

### Fixed

* Fixed `neg` and `signum` to consistently handle negative zero across multiple
  platforms.

* Removed `register_attr` feature usage for SPIR-V targets.

### Added

* Added missing `Serialize`, `Deserialize` and `PartialEq` implementations.

* Added `Sum<Self>` and `Product<Self>` implementations for all vector, matrix
  and quaternion types.

* Added 4x4 matrix methods `look_to_lh` and `look_to_rh`. These were previously
  private.

* Added `dot_into_vec` methods to vector which returns the result of the dot
  product splatted to all vector lanes.

* Added `is_negative_bitmask` to vector types which returns a `u32` of bits for
  each negative vector lane.

* Added `splat` method and `TRUE` and `FALSE` constants to all `BVec` types.

* Added `from_mat3a` methods to `Affine2`, `Mat2`, `Mat4` and `Quat` types.

### Changed

* Disable `serde` default features.

* Made `to_cols_array`, `to_cols_array_2d`, and `from_diagonal` methods
 `const fn`.

## [0.21.3] - 2022-08-02

### Fixed

* Fixed `glam_assert` being too restrictive in matrix transform point and
  transform vector methods.

### Added

* Added experimental `core-simd` feature which enables SIMD support via the
  unstable `core::simd` module.

### Changed

* Derive from `PartialEq` and `Eq` instead of providing a trait implementation
  for all non SIMD types.

## [0.21.2] - 2022-06-25

### Fixed

* Restore missing `$crate::` prefix in deprecated `const_*` macros.

* Fixed some performance regressions in affine and matrix determinant and
  inverses due to lack of inlining.

* Fixed some performance regressions in the SSE2 `Vec3A` to `Vec3` from
  conversion.

### Added

* Implemented `BitXor` and `BitXorAssign` traits for `bool` vectors.

## [0.21.1] - 2022-06-22

### Fixed

* Fix compilation when FMA support is enabled.

## [0.21.0] - 2022-06-22

### Breaking changes

* Minimum Supported Version of Rust bumped to 1.58.1 to allow `const` pointer
  dereferences in constant evaluation.

* The `abs_diff_eq` method on `Mat2` and `DMat2` now takes `other` by value
  instead of reference. This is consistent with the other matrix types.

* The `AsMut` and `Deref` trait implementations on `Quat` and `DQuat` was
  removed. Quaternion fields are now public.

* The `AsRef` trait implementations were removed from `BVec2`, `BVec3`,
  `BVec3A`, `BVec4` and `BVec4A`.

### Added

* `NEG_ONE` constant was added to all signed vector types.

* `NEG_X`, `NEG_Y`, `NEG_Z` and `NEG_W` negative axis vectors were added to
  signed vector types.

* The `rotate` and `from_angle` methods were added to `Vec2` and `DVec2`.
  `from_angle` returns a 2D vector containing `[angle.cos(), angle.sin()]` that
  can be used to `rotate` another 2D vector.

* The `from_array` `const` function was added to all vector types.

### Changed

* Source code is now largely generated. This removes most usage of macros
  internally to improve readability. There should be no change in API or
  behavior other than what is documented here.

* Many methods have been made `const fn`:
  * `new`, `splat`, `from_slice`, `to_array` and `extend` on vector types
  * `from_cols`, `from_cols_array`, `from_cols_array_2d`, `from_cols_slice` on
    matrix types
  * `from_xyzw` and `from_array` on quaternion types
  * `from_cols` on affine types

* The `const` new macros where deprecated.

### Removed

* Deleted deprecated `TransformRT` and `TransformSRT` types.

## [0.20.5] - 2022-04-12

### Fixed

* Fixed a bug in the scalar implementation of 4D vector `max_element` method
  where the `w` element check was incorrect.

## [0.20.4] - 2022-04-11

### Fixed

* Fixed a bug with quaternion `slerp` with a rotation of tau.

## [0.20.3] - 2022-03-28

### Added

* Added `to_array()` to `Quat` and `DQuat`.
* Added `mul_add` method to all vector types - note that this will be slower
  without hardware support enabled.
* Added the `fast-math` flag which will sacrifice some float determinism for
  speed.

### Fixed

* Fixed a bug in the `sse2` and `wasm32` implementations of
  `Mat4::determinant()`.

## [0.20.2] - 2021-12-20

### Fixed

* Fixed SPIR-V build which was broken due to a typo.

## [0.20.1] - 2021-11-23

### Added

* Added the `from_rotation_arc_2d()` method to `Quat` and `DQuat` which will
  return a rotation between two 2D vectors around the z axis.
* Added impl of `Neg` operator for matrix types.
* Added `cuda` feature which forces `glam` types to match cuda's alignment
  requirements.

### Changed

* The `Quat` and `DQuat` methods `from_rotation_arc()` and
  `from_rotation_arc_colinear()` are now available in `no_std`.
* The `Vec3` and `DVec3` methods `any_orthogonal_vector()`,
  `any_orthonormal_vector()` and `any_orthonormal_pair()` are now available in
  `no_std`.
* Added `repr(C)` attribute to affine types.

### Removed

* Removed deprecated `as_f32()`, `as_f64()`, `as_i32()` and `as_u32()` methods.

## [0.20.0] - 2021-11-01

### Breaking changes

* Minimum Supported Version of Rust bumped to 1.52.1 for an update to the `mint`
  crate.

### Added

* Added implementations for new `IntoMint` trait from the `mint` crate.
* Added `mint` conversions for `Mat3A`.
* Added `as_vec3a` cast methods to vector types.

## [0.19.0] - 2021-10-05

### Breaking changes

* Removed truncating vector `From` implementations. Use `.truncate()` or swizzle
  methods instead.

### Added

* Added `Not`, `Shl`, `Shr`, `BitAnd`, `BitOr` and `BitXor` implementations for
  all `IVec` and `UVec` vector types.
* Added `NAN` constant for all types.
* Documented `glam`'s [architecture](ARCHITECTURE.md).

### Changed

* `Sum` and `Product` traits are now implemented in `no_std` builds.

## [0.18.0] - 2021-08-26

### Breaking changes

* Minimum Supported Version of Rust bumped to 1.51.0 for `wasm-bindgen-test`
  and `rustdoc` `alias` support.

### Added

* Added `wasm32` SIMD intrinsics support.
* Added optional support for the `rkyv` serialization crate.
* Added `Rem` and `RemAssign` implementations for all vector types.
* Added quaternion `xyz()` method for returning the vector part of the
  quaternion.
* Added `From((Scalar, Vector3))` for 4D vector types.

### Changed

* Deprecated `as_f32()`, `as_f64()`, `as_i32()` and `as_u32()` methods in favor
  of more specific methods such as `as_vec2()`, `as_dvec2()`, `as_ivec2()` and
  `as_uvec2()` and so on.

## [0.17.3] - 2021-07-18

### Fixed

* Fix alignment unit tests on non x86 platforms.

## [0.17.2] - 2021-07-15

### Fixed

* Fix alignment unit tests on i686 and S390x.

## [0.17.1] - 2021-06-29

### Added

* Added `serde` support for `Affine2`, `DAffine2`, `Affine3A` and `DAffine3`.

## [0.17.0] - 2021-06-26

### Breaking changes

* The addition of `Add` and `Sub` implementations of scalar values for vector
  types may create ambiguities with existing calls to `add` and `sub`.
* Removed `From<Mat3>` implementation for `Mat2` and `From<DMat3>` for `DMat2`.
  These have been replaced by `Mat2::from_mat3()` and `DMat2::from_mat3()`.
* Removed `From<Mat4>` implementation for `Mat3` and `From<DMat4>` for `DMat3`.
  These have been replaced by `Mat3::from_mat4()` and `DMat3::from_mat4()`.
* Removed deprecated `from_slice_unaligned()`, `write_to_slice_unaligned()`,
  `from_rotation_mat4` and `from_rotation_ypr()` methods.

### Added

* Added `col_mut()` method which returns a mutable reference to a matrix column
  to all matrix types.
* Added `AddAssign`, `MulAssign` and `SubAssign` implementations for all matrix
  types.
* Added `Add` and `Sub` implementations of scalar values for vector types.
* Added more `glam_assert!` checks and documented methods where they are used.
* Added vector projection and rejection methods `project_onto()`,
  `project_onto_normalized()`, `reject_from()` and `reject_from_normalized()`.
* Added `Mat2::from_mat3()`, `DMat2::from_mat3()`, `Mat3::from_mat4()`,
  `DMat3::from_mat4()` which create a smaller matrix from a larger one,
  discarding a final row and column of the input matrix.
* Added `Mat3::from_mat2()`, `DMat3::from_mat2()`, `Mat4::from_mat3()` and
  `DMat4::from_mat3()` which create an affine transform from a smaller linear
  transform matrix.

### Changed

* Don't support `AsRef` and `AsMut` on SPIR-V targets. Also removed SPIR-V
  support for some methods that used `as_ref()`, including `hash()`. Not a
  breaking change as these methods would not have worked anyway.

### Fixed

* Fixed compile time alignment checks failing on i686 targets.

## [0.16.0] - 2021-06-06

### Breaking changes

* `sprirv-std` dependency was removed, rust-gpu depends on glam internally
  again for now.
* Added `must_use` attribute to all `inverse()`, `normalize()`,
  `try_normalize()`, `transpose()` and `conjugate()` methods.

### Added

* Added `fract()` method to float vector types which return a vector containing
  `self - self.floor()`.
* Added optional support for the `approx` crate. Note that all glam types
  implement their own `abs_diff_eq()` method without requiring the `approx`
  dependency.

## [0.15.2] - 2021-05-20

### Added

* Added `from_cols()` methods to affine types.
* Added methods for reading and writing affine types from and to arrays and
  slices, including `from_cols_array()`, `to_cols_array()`,
  `from_cols_array_2d()`, `to_cols_array_2d()`, `from_cols_slice()` and
  `write_cols_to_slice()`.
* Added `core::fmt::Display` trait implementations for affine types.
* Added `core::ops::Add`, `core::ops::Mul` scalar and `core::ops::Sub` trait
  implementations for affine types.
* Added `from_array()` methods to quaternion types.

### Changed

* Renamed vector and quaternion `from_slice_unaligned()` and
  `write_to_slice_unaligned()` methods to `from_slice()` and
  `write_to_slice()`.
* Removed usage of `_mm_rcp_ps` from SSE2 implementation of `Quat::slerp` as
  this instruction is not deterministic between Intel and AMD chips.

## [0.15.1] - 2021-05-14

### Changed

* Disable `const_assert_eq!` size and alignment checks for SPIR-V targets.

## [0.15.0] - 2021-05-14

### Breaking changes

* Removed `PartialOrd` and `Ord` trait implementations for all `glam` types.
* Removed deprecated `zero()`, `one()`, `unit_x()`, `unit_y()`, `unit_z()`,
  `unit_w()`, `identity()` and `Mat2::scale()` methods.
* Remove problematic `Quat` `From` trait conversions which would allow creating
  a non-uniform quaternion without necessarily realizing, including from
  `Vec4`, `(x, y, z, w)` and `[f32; 4]`.

### Added

* Added `EulerRot` enum for specifying Euler rotation order and
  `Quat::from_euler()`, `Mat3::from_euler()` and `Mat4::from_euler()` which
  support specifying a rotation order and angles of rotation.
* Added `Quat::to_euler()` method for extracting Euler angles.
* Added `Quat::from_vec4()` which is an explicit method for creating a
  quaternion from a 4D vector. The method does not normalize the resulting
  quaternion.
* Added `Mat3A` type which uses `Vec3A` columns. It is 16 byte aligned and
  contains internal padding but it generally faster than `Mat3` for most
  operations if SIMD is available.
* Added 3D affine transform types `Affine3A` and `DAffine3`. These are more
  efficient than using `Mat4` and `DMat4` respectively when working with 3D
  affine transforms.
* Added 2D affine transform types `Affine2` and `DAffine2`. These are more
  efficient than using `Mat3` and `DMat3` respectively when working with 2D
  affine transforms.
* Added `Quat::from_affine3()` to create a quaternion from an affine transform
  rotation.
* Added explicit `to_array()` method to vector types to better match the matrix
  methods.

### Changed

* Deprecated `Quat::from_rotation_ypr()`, `Mat3::from_rotation_ypr()` and
  `Mat4::from_rotation_ypr()` in favor of new `from_euler()` methods.
* Deprecated `Quat::from_rotation_mat3()` and `Quat::from_rotation_mat4()` in
  favor of new `from_mat3` and `from_mat4` methods.
* Deprecated `TransformSRT` and `TransformRT` which are under the
  `transform-types` feature. These will be moved to a separate experimental
  crate.
* Updated `spirv-std` dependency version to `0.4.0-alpha7`.

## [0.14.0] - 2021-04-09

### Breaking changes

* Minimum Supported Version of Rust bumped to 1.45.0 for the `spirv-std`
  dependency.

### Added

* Added `AXES[]` constants to all vector types. These are arrays containing the
  unit vector for each axis.
* Added quaternion `from_scaled_axis` and `to_scaled_axis` methods.

### Changed

* Updated dependency versions of `bytemuck` to `1.5`, `rand` to `0.8`,
  `rand_xoshiro` to `0.6` and `spirv-std` to `0.4.0-alpha4`.

## [0.13.1] - 2021-03-24

### Added

* Added vector `clamp()` functions.
* Added matrix column and row accessor methods, `col()` and `row()`.
* Added SPIR-V module and dependency on `spirv-std` for the SPIR-V target.
* Added matrix truncation from 4x4 to 3x3 and 3x3 to 2x2 via `From` impls.

### Changed

* Documentation corrections and improvements.

## [0.13.0] - 2021-03-04

### Breaking Changes

* The behavior of the 4x4 matrix method `transform_point3()` was changed to not
  perform the perspective divide. This is an optimization for use with affine
  transforms where perspective correction is not required. The
  `project_point3()` method was added for transforming points by perspective
  projections.
* The 3x3 matrix `from_scale()` method was changed to
  create a affine transform containing a 2-dimensional non-uniform scale to be
  consistent with the 4x4 matrix version. The
  `from_diagonal()` method can be used to create a 3x3 scale matrix.
* The 3x3 matrix methods `transform_point2_as_vec3a`,
  `transform_vector2_as_vec3a` and `mul_vec3_as_vec3a` were unintentionally
  `pub` and are no longer publicly accessible.

### Added

* Added `Vec2::X`, `Vec4::W` etc constants as a shorter versions of `unit_x()`
  and friends.
* Added `ONE` constants for vectors.
* Added `IDENTITY` constants for `Mat2`, `Mat3`, `Mat4` and `Quat`.
* Added `ZERO` constant for vectors and matrices.
* Added `clamp_length()`, `clamp_length_max()`, and `clamp_length_min` methods
  for `f32` and `f64` vector types.
* Added `try_normalize()` and `normalize_or_zero()` for all real vector types.
* Added `from_diagonal()` methods to all matrix types for creating diagonal
  matrices from a vector.
* Added `angle_between()`, `from_rotation_arc()` and
  `from_rotation_arc_colinear()` to quaternion types.
* Added quaternion `inverse()` which assumes the quaternion is already
  normalized and returns the conjugate.
* Added `from_translation()` and `from_angle()` methods to 3x3 matrix types.
* Added `project_point3()` method to 4x4 matrix types. This method is for
  transforming 3D vectors by perspective projection transforms.
* Added `Eq` and `Hash` impls for integer vector types.

### Changed

* Deprecated `::unit_x/y/z()`, `::zero()`, `::one()`, `::identity()` functions
  in favor of constants.

## [0.12.0] - 2021-01-15

### Breaking Changes

* `Vec2Mask`, `Vec3Mask` and `Vec4Mask` have been replaced by `BVec2`, `BVec3`,
  `BVec3A`, `BVec4` and `BVec4A`. These types are used by some vector methods
  and are not typically referenced directly.

### Added

* Added `f64` primitive type support
  * vectors: `DVec2`, `DVec3` and `DVec4`
  * square matrices: `DMat2`, `DMat3` and `DMat4`
  * a quaternion type: `DQuat`
* Added `i32` primitive type support
  * vectors: `IVec2`, `IVec3` and `IVec4`
* Added `u32` primitive type support
  * vectors: `UVec2`, `UVec3` and `UVec4`
* Added `bool` primitive type support
  * vectors: `BVec2`, `BVec3` and `BVec4`

### Removed

* `build.rs` has been removed.

## [0.11.3] - 2020-12-29

### Changed

* Made `Vec3` `repr(simd)` for `spirv` targets.

### Added

* Added `From<(Vec2, f32)>` for `Vec3` and `From<(Vec3, f32)` for `Vec4`.

## [0.11.2] - 2020-12-04

### Changed

* Compilation fixes for Rust 1.36.0.

## [0.11.1] - 2020-12-03

### Added

* Added support for the [Rust GPU](https://github.com/EmbarkStudios/rust-gpu)
  SPIR-V target architecture.

## [0.11.0] - 2020-11-26

### Added

* Added `is_finite` method to all types which returns `true` if, and only if,
  all contained elements are finite.
* Added `exp` and `powf` methods for all vector types.

### Changed

* The `is_nan` method now returns a `bool` to match the new `is_finite` method
  and to be consistent with the same methods on the `f32` and `f64` primitive
  types.
* Renamed `is_nan` which returns a vector mask to `is_nan_mask`.
* Don't use the `cfg` definitions added by `build.rs` for defining structs as
  `rust-analyzer` is not aware of them.

### Removed

* Removed deprecated accessor methods.

## [0.10.2] - 2020-11-17

### Changed

* Deprecated element accessor members `.x()`, `.x_mut()`, `.set_x()`, etc. on
  vector and quaternion types.
* Deprecated column accessor members `.x_axis()`, `.x_axis_mut()`,
  `.set_x_axis()`, etc. on matrix types.

## [0.10.1] - 2020-11-15

### Added

* Added the `Vec2::perp` method which returns a `Vec2` perpendicular to `self`.

### Changed

* `Vec2` and `Vec3` types were changed to use public named fields for `.x`,
  `.y`, and `.z` instead of accessors.
* `Quat`, `Vec3A` and `Vec4` implement `Deref` and `DerefMut` for the new `XYZ`
  and `XYZW` structs to emulate public named field access.
* `Mat3` and `Mat4` had their axis members made public instead of needing
  accessors.
* `Mat2` implements `Deref` and `DerefMut` for the new `XYAxes` struct to
  emulate public named field access.

### Removed

* Removed deprecated `length_reciprocal` and `sign` methods.

### Fixed

* Adding `glam` as a `no_std` dependency should now work as expected.

## [0.10.0] - 2020-10-31

### Breaking Changes

* Changed the return type of `Vec4::truncate` from `Vec3A` to `Vec3`.

### Added

* Added `From` implementations to truncate to narrower vector types, e.g.
  `Vec4` to `Vec3A`, `Vec3` and `Vec2` and from `Vec3A` and `Vec3` to `Vec2`.
* Added swizzles for `Vec4`, `Vec3A`, `Vec3` and `Vec2`. These can be used to
  reorder elements in the same type and also to create larger or smaller
  vectors from the given vectors elements.
* Added `Quat` operators `Add<Quat>`, `Sub<Quat>`, `Mul<f32>` and `Div<f32`.
  These are used by other crates for interpolation quaternions along splines.
  Note that these operations will not return unit length quaternions, thus the
  results must be normalized before performing other `Quat` operations.
* Added `Mat4::transform_point3a` and `Mat4::transform_vector3a`.
* Added `AsRef<[f32; 9]>` and `AsMut<[f32; 9]>` trait implementations to `Mat3`.
* Added optional `bytemuck` support primarily for casting types to `&[u8]`.
* Added support for compiling with `no_std` by disabling the default `std`
  feature and adding the `libm` feature.
* Added `distance` and `distance_squared` methods to `Vec2`, `Vec3`, `Vec3A`
  and `Vec4`.

## [0.9.5] - 2020-10-10

### Added

* `glam` uses SSE2 for some types which prevents constructor functions can not
  be made `const fn`. To work around this limitation the following macro
  functions have been added to support creating `const` values of `glam` types:
  `const_mat2`, `const_mat3`, `const_mat4`, `const_quat`, `const_vec2`,
  `const_vec3`, `const_vec3a` and `const_vec4`.
* Added `is_nan` methods to `Vec2`, `Vec3`, `Vec3A` and `Vec4` which return a
  mask.

## Changed

* Renamed the vector `reciprocal` and `length_reciprocal` methods to `recip`
  and `length_recip` to match the Rust standard library naming. The old methods
  have been deprecated.
* Renamed the vector `sign` methods to `signum` match the Rust standard library
  naming. The new methods now check for `NAN`. The old methods have been
  deprecated.
* Added SSE2 optimized implementations of `Mat4::determinant` and
  `Mat4::inverse`.

### Removed

* Removed deprecated function `Mat4::perspective_glu_rh`.

## [0.9.4] - 2020-08-31

### Fixed

* Fixed `Mat4::transform_point3` to account for homogeneous w coordinate.
  Previously this would have been incorrect when the resulting homogeneous
  coordinate was not 1.0, e.g. when transforming by a perspective projection.
* Fixed `Mat3::transform_point2` to account for homogeneous z coordinate.

## [0.9.3] - 2020-08-11

### Added

* Added `Mat4::perspective_rh`.

## [0.9.2] - 2020-07-09

### Added

* Added `Mat3::mul_vec3a` and `Quat::mul_vec3a`.

### Changed

* Changed `Quat::mul_vec3` to accept and return `Vec3` instead of `Vec3A`.

## [0.9.1] - 2020-07-01

### Added

* Added `Mat3 * Vec3A` implementation.
* Added `Vec3A` benches.

### Changed

* Some documentation improvements around the new `Vec3A` type.

## [0.9.0] - 2020-06-28

### Added

* `Vec3` has been split into scalar `Vec3` and 16 byte aligned `Vec3A` types.
  Only the `Vec3A` type currently uses SIMD optimizations.
* `Vec3Mask` has been split into scalar `Vec3Mask` and 16 byte aligned
  `Vec3AMask` types.
* Added `mut` column accessors to all matrix types, e.g. `Mat2::x_axis_mut()`.
* Added `From` trait implementations for `Vec3AMask` and `Vec4Mask` to `__m128`.

### Changed

* The `Mat3` type is using the scalar `Vec3` type for storage.
* Simplified `Debug` trait output for `Quat`, `Vec4` and `Vec3A`.

## Removed

* Removed the `packed-vec3` feature flag as it is now redundant.

## [0.8.7] - 2020-04-28

### Added

* Added `Quat::slerp` - note that this uses a `sin` approximation.
* Added `angle_between` method for `Vec2` and `Vec3`.
* Implemented `Debug`, `Display`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`,
  `Hash`, and `AsRef` traits for `Vec2Mask`, `Vec3Mask` and `Vec4Mask`.
* Added conversion functions from `Vec2Mask`, `Vec3Mask` and `Vec4Mask` to an
  array of `[u32]`.
* Added `build.rs` to simplify conditional feature compilation.

### Changed

* Increased test coverage.

### Removed

* Removed `cfg-if` dependency.

## [0.8.6] - 2020-02-18

### Added

* Added the `packed-vec3` feature flag to disable using SIMD types for `Vec3`
  and `Mat3` types. This avoids wasting some space due to 16 byte alignment at
  the cost of some performance.
* Added `x_mut`, `y_mut`, `z_mut`, `w_mut` where appropriate to `Vec2`, `Vec3`
  and `Vec4`.
* Added implementation of `core::ops::Index` and `core::ops::IndexMut` for
  `Vec2`, `Vec3` and `Vec4`.

### Changed

* Merged SSE2 and scalar `Vec3` and `Vec4` implementations into single files
  using the `cfg-if` crate.

## [0.8.5] - 2020-01-02

### Added

* Added projection functions `Mat4::perspective_lh`,
  `Mat4::perspective_infinite_lh`, `Mat4::perspective_infinite_reverse_lh`,
  `Mat4::orthgraphic_lh` and `Mat4::orthographic_rh`.
* Added `round`, `ceil` and `floor` methods to `Vec2`, `Vec3` and `Vec4`.

## [0.8.4] - 2019-12-17

### Added

* Added `Mat4::to_scale_rotation_translation` for extracting scale, rotation and
  translation from a 4x4 homogeneous transformation matrix.
* Added `cargo-deny` GitHub Action.

### Changed

* Renamed `Quat::new` to `Quat::from_xyzw`.

## [0.8.3] - 2019-11-27

### Added

* Added `Mat4::orthographic_rh_gl`.

### Changed

* Renamed `Mat4::perspective_glu_rh` to `Mat4::perspective_rh_gl`.
* SSE2 optimizations for `Mat2::determinant`, `Mat2::inverse`,
  `Mat2::transpose`, `Mat3::transpose`, `Quat::conjugate`, `Quat::lerp`,
  `Quat::mul_vec3`, `Quat::mul_quat` and `Quat::from_rotation_ypr`.
* Disabled optimizations to `Mat4::transform_point3` and
  `Mat4::transform_vector3` as they are probably incorrect and need
  investigating.
* Added missing `#[repr(C)]` to `Mat2`, `Mat3` and `Mat4`.
* Benchmarks now store output of functions to better estimate the cost of a
  function call.

### Removed

* Removed deprecated functions `Mat2::new`, `Mat3::new` and `Mat4::new`.

## [0.8.2] - 2019-11-06

### Changed

* `glam_assert!` is no longer enabled by default in debug builds, it can be
  enabled in any configuration using the `glam-assert` feature or in debug
  builds only using the `debug-glam-assert` feature.

### Removed

* `glam_assert!`'s checking `lerp` is bounded between 0.0 and 1.0 and that
  matrix scales are non-zero have been removed.

## [0.8.1] - 2019-11-03

### Added

* Added `Display` trait implementations for `Mat2`, `Mat3` and `Mat4`.

### Changed

* Disabled `glam`'s SSE2 `sin_cos` implementation - it became less precise for
  large angle values.
* Reduced the default epsilon used by the `is_normalized!` macro from
  `std::f32::EPSILON` to `1e-6`.

## [0.8.0] - 2019-10-14

### Removed

* Removed the `approx` crate dependency. Each `glam` type has an `abs_diff_eq`
  method added which is used by unit tests for approximate floating point
  comparisons.
* Removed the `Angle` type. All angles are now `f32` and are expected to
  be in radians.
* Removed the deprecated `Vec2b`, `Vec3b` and `Vec4b` types and the `mask`
  methods on `Vec2Mask`, `Vec3Mask` and `Vec4Mask`.

### Changed

* The `rand` crate dependency has been removed from default features. This was
  required for benchmarking but a simple random number generator has been added
  to the benches `support` module instead.
* The `From` trait implementation converting between 1D and 2D `f32` arrays and
  matrix types have been removed. It was ambiguous how array data would map to
  matrix columns so these have been replaced with explicit methods
  `from_cols_array` and `from_cols_array_2d`.
* Matrix `new` methods have been renamed to `from_cols` to be consistent with
  the other methods that create matrices from data.
* Renamed `Mat4::perspective_glu` to `Mat4::perspective_glu_rh`.

## [0.7.2] - 2019-09-22

### Fixed

* Fixed incorrect projection matrix methods `Mat4::look_at_lh`
  and `Mat4::look_at_rh`.

### Added

* Added support for building infinite projection matrices, including both
  standard and reverse depth `Mat4::perspective_infinite_rh` and
  `Mat4::perspective_infinite_rh`.
* Added `Vec2Mask::new`, `Vec3Mask::new` and `Vec4Mask::new` methods.
* Implemented `std::ops` `BitAnd`, `BitAndAssign`, `BitOr`, `BitOrAssign`
  and `Not` traits for `Vec2Mask`, `Vec3Mask` and `Vec4Mask`.
* Added method documentation for `Vec4` and `Vec4Mask` types.
* Added missing `serde` implementations for `Mat2`, `Mat3` and `Mat4`.
* Updated `rand` and `criterion` versions.

## [0.7.1] - 2019-07-08

### Fixed

* The SSE2 implementation of `Vec4` `dot` was missing a shuffle, meaning the
  `dot`, `length`, `length_squared`, `length_reciprocal` and `normalize`
  methods were sometimes incorrect.

### Added

* Added the `glam_assert` macro which behaves like Rust's `debug_assert` but
  can be enabled separately to `debug_assert`. This is used to perform
  asserts on correctness.
* Added `is_normalized` method to `Vec2`, `Vec3` and `Vec4`.

### Changed

* Replaced usage of `std::mem::uninitialized` with `std::mem::MaybeUninit`. This
  change requires stable Rust 1.36.
* Renamed `Vec2b` to `Vec2Mask`, `Vec3b` to `Vec3Mask` and `Vec4b` to
  `Vec4Mask`. Old names are aliased to the new name and deprecated.
* Deprecate `VecNMask` `mask` method, use new `bitmask` method instead
* Made fallback version of `VecNMask` types the same size and alignment as the
  SIMD versions.
* Added `Default` support to `VecNMask` types, will add more common traits in
  the future.
* Added `#[inline]` to `mat2`, `mat3` and `mat4` functions.

## [0.7.0] - 2019-06-28

### Added

* Added `Mat2` into `[f32; 4]`, `Mat3` into `[f32; 9]` and `Mat4` into
  `[f32; 16]`.

### Removed

* Removed `impl Mul<&Vec2> for Mat2` and `impl Mul<&Vec3> for Vec3` as these
  don't exist for any other types.

## [0.6.1] - 2019-06-22

### Changed

* `Mat2` now uses a `Vec4` internally which gives it some performance
   improvements when SSE2 is available.

## 0.6.0 - 2019-06-13

### Changed

* Switched from row vectors to column vectors
* Vectors are now on the right of multiplications with matrices and quaternions.

[Keep a Changelog]: https://keepachangelog.com/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[Unreleased]: https://github.com/bitshifter/glam-rs/compare/0.28.0...HEAD
[0.28.0]: https://github.com/bitshifter/glam-rs/compare/0.27.0...0.28.0
[0.27.0]: https://github.com/bitshifter/glam-rs/compare/0.26.0...0.27.0
[0.26.0]: https://github.com/bitshifter/glam-rs/compare/0.25.0...0.26.0
[0.25.0]: https://github.com/bitshifter/glam-rs/compare/0.24.2...0.25.0
[0.24.2]: https://github.com/bitshifter/glam-rs/compare/0.24.1...0.24.2
[0.24.1]: https://github.com/bitshifter/glam-rs/compare/0.24.0...0.24.1
[0.24.0]: https://github.com/bitshifter/glam-rs/compare/0.23.0...0.24.0
[0.23.0]: https://github.com/bitshifter/glam-rs/compare/0.22.0...0.23.0
[0.22.0]: https://github.com/bitshifter/glam-rs/compare/0.21.3...0.22.0
[0.21.3]: https://github.com/bitshifter/glam-rs/compare/0.21.2...0.21.3
[0.21.2]: https://github.com/bitshifter/glam-rs/compare/0.21.1...0.21.2
[0.21.1]: https://github.com/bitshifter/glam-rs/compare/0.21.0...0.21.1
[0.21.0]: https://github.com/bitshifter/glam-rs/compare/0.20.5...0.21.0
[0.20.5]: https://github.com/bitshifter/glam-rs/compare/0.20.4...0.20.5
[0.20.4]: https://github.com/bitshifter/glam-rs/compare/0.20.3...0.20.4
[0.20.3]: https://github.com/bitshifter/glam-rs/compare/0.20.2...0.20.3
[0.20.2]: https://github.com/bitshifter/glam-rs/compare/0.20.1...0.20.2
[0.20.1]: https://github.com/bitshifter/glam-rs/compare/0.20.0...0.20.1
[0.20.0]: https://github.com/bitshifter/glam-rs/compare/0.19.0...0.20.0
[0.19.0]: https://github.com/bitshifter/glam-rs/compare/0.18.0...0.19.0
[0.18.0]: https://github.com/bitshifter/glam-rs/compare/0.17.3...0.18.0
[0.17.3]: https://github.com/bitshifter/glam-rs/compare/0.17.2...0.17.3
[0.17.2]: https://github.com/bitshifter/glam-rs/compare/0.17.1...0.17.2
[0.17.1]: https://github.com/bitshifter/glam-rs/compare/0.17.0...0.17.1
[0.17.0]: https://github.com/bitshifter/glam-rs/compare/0.16.0...0.17.0
[0.16.0]: https://github.com/bitshifter/glam-rs/compare/0.15.2...0.16.0
[0.15.2]: https://github.com/bitshifter/glam-rs/compare/0.15.1...0.15.2
[0.15.1]: https://github.com/bitshifter/glam-rs/compare/0.15.0...0.15.1
[0.15.0]: https://github.com/bitshifter/glam-rs/compare/0.14.0...0.15.0
[0.14.0]: https://github.com/bitshifter/glam-rs/compare/0.13.1...0.14.0
[0.13.1]: https://github.com/bitshifter/glam-rs/compare/0.13.0...0.13.1
[0.13.0]: https://github.com/bitshifter/glam-rs/compare/0.12.0...0.13.0
[0.12.0]: https://github.com/bitshifter/glam-rs/compare/0.11.3...0.12.0
[0.11.3]: https://github.com/bitshifter/glam-rs/compare/0.11.2...0.11.3
[0.11.2]: https://github.com/bitshifter/glam-rs/compare/0.11.1...0.11.2
[0.11.1]: https://github.com/bitshifter/glam-rs/compare/0.11.0...0.11.1
[0.11.0]: https://github.com/bitshifter/glam-rs/compare/0.10.2...0.11.0
[0.10.2]: https://github.com/bitshifter/glam-rs/compare/0.10.1...0.10.2
[0.10.1]: https://github.com/bitshifter/glam-rs/compare/0.10.0...0.10.1
[0.10.0]: https://github.com/bitshifter/glam-rs/compare/0.9.5...0.10.0
[0.9.5]: https://github.com/bitshifter/glam-rs/compare/0.9.4...0.9.5
[0.9.4]: https://github.com/bitshifter/glam-rs/compare/0.9.3...0.9.4
[0.9.3]: https://github.com/bitshifter/glam-rs/compare/0.9.2...0.9.3
[0.9.2]: https://github.com/bitshifter/glam-rs/compare/0.9.1...0.9.2
[0.9.1]: https://github.com/bitshifter/glam-rs/compare/0.9.0...0.9.1
[0.9.0]: https://github.com/bitshifter/glam-rs/compare/0.8.7...0.9.0
[0.8.7]: https://github.com/bitshifter/glam-rs/compare/0.8.6...0.8.7
[0.8.6]: https://github.com/bitshifter/glam-rs/compare/0.8.5...0.8.6
[0.8.5]: https://github.com/bitshifter/glam-rs/compare/0.8.4...0.8.5
[0.8.4]: https://github.com/bitshifter/glam-rs/compare/0.8.3...0.8.4
[0.8.3]: https://github.com/bitshifter/glam-rs/compare/0.8.2...0.8.3
[0.8.2]: https://github.com/bitshifter/glam-rs/compare/0.8.1...0.8.2
[0.8.1]: https://github.com/bitshifter/glam-rs/compare/0.8.0...0.8.1
[0.8.0]: https://github.com/bitshifter/glam-rs/compare/0.7.2...0.8.0
[0.7.2]: https://github.com/bitshifter/glam-rs/compare/0.7.1...0.7.2
[0.7.1]: https://github.com/bitshifter/glam-rs/compare/0.7.0...0.7.1
[0.7.0]: https://github.com/bitshifter/glam-rs/compare/0.6.1...0.7.0
[0.6.1]: https://github.com/bitshifter/glam-rs/compare/0.6.0...0.6.1

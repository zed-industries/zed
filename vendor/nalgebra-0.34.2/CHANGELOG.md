# Changelog

All notable changes to `nalgebra`, starting with the version 0.6.0 will be
documented here.

This project adheres to [Semantic Versioning](https://semver.org/).

**nalgebra-lapack change log** is found [here](https://github.com/dimforge/nalgebra/blob/main/nalgebra-lapack/CHANGELOG.md)
starting with `nalgebra-lapack` version `0.27.0`.

## [0.34.2] (28 March 2026)

### Added

- Add `convert-glam031` and `convert-glam032` features for conversion from/to `glam` v0.31 and v0.32 [#1597](https://github.com/dimforge/nalgebra/pull/1597).

### Fixed

- Fix `SymmetricEigen` routine [#1210](https://github.com/dimforge/nalgebra/pull/1210).
- Fix matrix parsing grammar to accept numbers without leading zeros, e.g. ".45" [#1578](https://github.com/dimforge/nalgebra/pull/1578).
- Fix build with `glam` in `no_std` environments [#1555](https://github.com/dimforge/nalgebra/pull/1555).
- Fix rustdoc warnings [#1511](https://github.com/dimforge/nalgebra/pull/1511).
- Implement absolute threshold for early deflation in Schur algorithm [#1565](https://github.com/dimforge/nalgebra/pull/1565).
- Assert matrix shapes for `Matrix::abs_diff_eq` [#1568](https://github.com/dimforge/nalgebra/pull/1568).

## [0.34.1] (20 Sept. 2025)

### Added

* Added `encase` feature, providing `encase` trait implementations for `nalgebra` types.

## [0.34.0] (31 July 2025)

### Added

- Add the `convert-glam030` feature to enable conversion from/to types from `glam` v0.30.
- Add the `defmt` cargo feature that enables derives of `defmt::Format` for all no-std types.

### Changed

- Bumped MSRV to 1.87.0.
- Updated rand dependency to 0.9.0.
- Renamed associated const `DimName::USIZE` to `DimName::DIM`.
- Moved to Rust 2024 edition.
- Several methods are now `const` whenever possible. See details in [#1522](https://github.com/dimforge/nalgebra/pull/1522).
- Features for conversion from/to types from `glam` (such as `convert-glam029`) no longer enable default features for
  `glam`, allowing use in `no_std` environments.

### Fixed

- Fix infinite loop when attempting to take the Schur decomposition of a 0 matrix.

## [0.33.2] (29 October 2024)

### Added

- Add the `convert-glam029` feature to enable conversion from/to types from `glam` v0.29.

## [0.33.1] (16 October 2024)

### Added

- Add implementations of `bytemuck` traits for isometries and similarities.
- Implement `AsRef<[T]>` for matrices with contiguous storage.
- Enable the `num-complex/bytemuck` feature when the `convert-bytemuck` feature is enabled.

## [0.33.0] (23 June 2024)

### Fixed

- Fix a memory leak in `Matrix::generic_resize`.
- Fix `glm::is_null` to check the vector magnitude instead of individual components.
- Ensure that inverting a 4x4 matrix leaves it unchanged if the inversion fails.

### Added

- Add the `glam-0.28` feature to enable conversion from/to types from `glam` v0.28.
- Add a `stack!` macro for concatenating matrices. See [#1375](https://github.com/dimforge/nalgebra/pull/1375).

### Modified

- The `cuda` feature has been removed, as the toolchain it depends on is long abandoned.
- Update to `simba` 0.9. See the [changelog](https://github.com/dimforge/simba/blob/master/CHANGELOG) of `simba` for
  details.
- Update the `nalgebra-macros` crate to `syn` 2.0.
- Remove the scalar type `T` from the `Allocator` trait parameters. Instead of `Allocator<T, R, C>`, use the simpler
  `Allocator<R, C>`.

## [0.32.6] (12 June 2024)

### Added

- Add the `glam-0.27` feature to enable conversion from/to types from `glam` v0.27.

## [0.32.5] (28 March 2024)

## Fixed

- Fix numerical issue on SVD with near-identity matrix.

## [0.32.4] (19 Feb 2023)

### Added

- Add the `glam-0.25` feature to enable conversion from/to types from `glam` v0.25.

## [0.32.3] (09 July 2023)

### Modified

- Statically sized matrices are now serialized as tuples to match how serde
  serialized plain arrays.
- Don’t require `Scalar` for matrix `PartialEq` and `Eq`.

### Added

- Allow trailing punctuation in macros `vector!`, `matrix!`, `point!`, etc.
- Add the methods `Matrix1::as_scalar`, `::as_scalar_mut`, `::to_scalar`, `::into_scalar`.
- Add `Rotation3::euler_angles_ordered`, a generalized euler angles calculation.
- Add the `glam-0.24` feature to enable conversion from/to types from `glam` v0.24.
- Add the `glam-0.25` feature to enable conversion from/to types from `glam` v0.25.
- Add the `lerp` method to points.
- Implement `Clone` for `MatrixIter`.

### Fixed

- Fixed severe catastrophic cancellation issue in variance calculation.

## [0.32.2] (07 March 2023)

### Added

- Add the `glam-0.23` to enable conversion from/to type from `glam` v0.23.

## [0.32.1] (14 Jan. 2023)

### Modified

- Updated `nalgebra-macros` to use the new `Dyn`, avoiding macro-generated deprecation warnings.

## [0.32.0] (14 Jan. 2023)

### Modified

- Renamed all `MatrixSlice` types to `MatrixView`. In general all occurrences of the world `Slice` or `slice` have been
  replaced by `View` or `view`.
- Deprecated all the types involving `Slice` in its name, in favor of the word `View`.
- Make it so that most `nalgebra` objects archive as themselves (when using `rkyv` for serialization).
- Renamed `Dynamic` to `Dyn` and make `Dyn` a tuple struct.

### Added

- Add `Cholesky::ln_determinant` to compute the natural logarithm of the determinant of a matrix decomposed
  with Cholesky. This can be more numerically stable than computing the determinant itself when very small and/or
  large values are involved.
- Added new methods `Matrix::as_view` and `Matrix::as_view_mut`, which are very useful when working with view-based
  APIs.
- Added parallel column iterator using `rayon`: `Matrix::par_column_iter` and `Matrix::par_column_iter_mut`. The `rayon`
  feature must be enabled to access these methods.
- Implement `ReshapableStorage` for matrix slices (only for unit strides at the moment).
- Add `U0, U1, …` constants alongside the `U0, U1, …` types. This lets us write `U4` instead of `U4::name()` or
  `Const::<4>` when we need const dimensions.

### Fixed

- Fixed the implementation of `Rotation3::euler_angles` to return the angles in the documented order (roll, pitch, yaw).

## [0.31.4] (13 Nov. 2022)

### Added

- Add a `convert-glam022` feature to enable conversion between `nalgebra` and `glam v0.22`.

## [0.31.3] (30 Oct. 2022)

### Added

- Add `Matrix::try_cast` to attempt casting the inner scalar types when that cast may fail.

### Fixed

- Fixed the usage of `CheckBytes` with `rkyv`.

## [0.31.2] (09 Oct. 2022)

### Modified

- Use `#[inline]` on the `Dim` implementation for `Const` to improve opt-level 1 performance.
- Make the `Point::new` constructions const-fn.

### Added

- Add `UnitVector::cast` to change the underlying scalar type.

## [0.31.1] (31 July 2022)

### Modified

- Improve performances of multiplication of two sparse matrices.

### Added

- Add `Matrix::from_row_iterator` to build a matrix from an iterator yielding components in row-major order.
- Add support for conversion from/to types of `glam` 0.21.
- `nalgebra-sparse`: add support for the matrix-market export of sparse matrices.
- `nalgebra-lapack`: add a `GE` for solving the generalized eigenvalues problem.

### Fixed

- Fix `Rotation3::from_matrix` and `UnitQuaternion::from_matrix` when the input matrix is already a valid
  rotation matrix.

## [0.31.0] (30 Apr. 2022)

### Breaking changes

- Switch to `cust` 0.3 (for CUDA support).
- Switch to `rkyv` 0.7
- Remove support for serialization based on `abomonation`.
- Remove support for conversions between `nalgebra` types and `glam` 0.13.

### Modified

- The aliases for `Const` types have been simplified to help `rust-analyzer`.

### Added

- Add `TryFrom` conversion between `UnitVector2/3/4` and `glam`’s `Vec2/3/4`.
- `nalgebra-sparse`: added support for serialization of sparse matrices with `serde`.
- `nalgebra-sparse`: add a CSC matrix constructor from unsorted (but valid) data.
- `nalgebra-lapack`: add generalized eigenvalues/eigenvectors calculation + QZ decomposition.

### Fixed

- Improve stability of SVD.
- Fix slerp for `UnitComplex`.

## [0.30.1] (09 Jan. 2022)

### Added

- Add conversion from/to types of `glam` 0.19 and 0.20.

## [0.30.0] (02 Jan. 2022)

### Breaking changes

- The `Dim` trait is now marked as unsafe.
- The `Matrix::pow` and `Matrix::pow_mut` methods only allow positive integer exponents now. To compute negative
  exponents, the user is free to invert the matrix before calling `pow` with the exponent’s absolute value.
- Remove the `Bounded` requirement from `RealField`. Replace it by methods returning `Option<Self>` so that they can
  still be implemented by unbounded types (by returning `None`).
- The `ComplexField` trait derives from `FromPrimitive` again. We can actually keep this because all its methods
  return `Option<Self>`, meaning that it could be implemented by any type.

### Modified

- Use more concise debug impls for matrices and geometric transformation types.
- The singular values computed by the SVD are now sorted in increasing order by default. Use `SVD::new_unordered`
  instead to reproduce the older behavior without the sorting overhead.
- The `UnitDualQuaternion::sclerp` method will no longer panic when given two equal rotations.
- The `Matrix::select_rows` and `Matrix::select_columns` methods no longer require the matrix components to implement
  the trait `Zero`.
- The `Matrix::pow` and `Matrix::pow_mut` methods will now also work with integer matrices.

### Added

- Added the conversion trait `From<Vec<T>>` and method `from_vec_storage` for `RowDVector`.
- Added implementation of `From` and `Into` for converting between `nalgebra` types and types from
  `glam 0.18`. These can be enabled by enabling the `convert-glam018` cargo features.
- Added the methods `Matrix::product`, `::row_product`, `::row_product_tr`, and `::column_product` to compute the
  product of the components, rows, or columns, of a single matrix or vector.
- The `Default` trait is now implemented for most geometric types: `Point`, `Isometry`, `Rotation`, `Similarity`,
  `Transform`, `UnitComplex`, and `UnitQuaternion`.
- Added the `Scale` geometric type for representing non-uniform scaling.
- Added `Cholesky::new_with_substitute` that will replace diagonal elements by a given constant whenever `Cholesky`
  meets a non-definite-positiveness.
- Re-added the conversion from a vector/matrix slice to a static array.
- Added the `cuda` feature that enables the support of [rust-cuda](https://github.com/Rust-GPU/Rust-CUDA) for using
  `nalgebra` features with CUDA kernels written in Rust.
- Added special-cases implementations for the 2x2 and 3x3 SVDs for better accuracy and performances.
- Added the methods `Matrix::polar`, `Matrix::try_polar`, and `SVD::to_polar` to compute the polar decomposition of
  a matrix, based on its SVD.
- `nalgebra-sparse`: provide constructors for unsorted but otherwise valid data using the CSR format.
- `nalgebra-sparse`: added reading MatrixMarked data files to a sparse `CooMatrix`.

### Fixed

- Fixed a potential unsoundness with `matrix.get(i)` and `matrix.get_mut(i)` where `i`  is an `usize`, and `matrix`
  is a matrix slice with non-default strides.
- Fixed potential unsoundness with `vector.perp` where `vector` isn’t actually a 2D vector as expected.
- Fixed linkage issue with `nalgebra-lapack`: the user of `nalgebra-lapack` no longer have to add
  `extern crate lapack-src` to their `main.rs`.
- Fixed the `no-std` build of `nalgebra-glm`.
- Fix the `pow` and `pow_mut` functions (the result was incorrect for some exponent values).

## [0.29.0]

### Breaking changes

- We updated to the version 0.6 of `simba`. This means that the trait bounds `T: na::RealField`, `na::ComplexField`,
  `na::SimdRealField`, `na:SimdComplexField` no imply that `T: Copy` (they only imply that `T: Clone`). This may affect
  generic code.
- The closure given to `apply`, `zip_apply`, `zip_zip_apply` must now modify the
  first argument inplace, instead of returning a new value. This makes these
  methods more versatile, and avoid useless clones when using non-Copy scalar
  types.
- The `Allocator` trait signature has been significantly modified in order to handle uninitialized matrices in a sound
  way.

### Modified

- `Orthographic3::from_matrix_unchecked` is now `const fn`.
- `Perspective3::from_matrix_unchecked` is now `const fn`.
- `Rotation::from_matrix_unchecked` is now `const fn`.
- The `Scalar` is now automatically implemented for most `'static + Clone` types. Type that implement `Clone` but not
  `Copy` are now much safer to work with thanks to the refactoring of the `Allocator` system.

### Added

- The conversion traits form the `bytemuck` crates are now implemented for the geometric types too.
- Added operator overloading for `Transform * UnitComplex`, `UnitComplex * Transform`, `Transform ×= UnitComplex`,
  `Transform ÷= UnitComplex`.
- Added `Reflection::bias()` to retrieve the bias of the reflection.
- Added `Reflection1..Reflection6` aliases for 1D to 6D reflections.
- Added implementation of `From` and `Into` for converting between `nalgebra` types and types from
  `glam 0.16` and `glam 0.17`. These can be enabled by enabling the `convert-glam016`, and/or `convert-glam017`
  cargo features.

## [0.28.0]

### Added

- Implement `Hash` for `Transform`.
- Implement `Borrow` and `BorrowMut` for contiguous slices.

### Modified

- The `OPoint<T, D>` type has been added. It takes the dimension number as a type-level integer (e.g. `Const<3>`)
  instead
  of a const-generic. The type `Point<T, const D: usize>` is now an alias for `OPoint`. This changes doesn't affect any
  of the existing code using `Point`. However, it will allow the use `OPoint` in a generic context where the dimension
  cannot be easily expressed as a const-generic (because of the current limitation of const-generics in Rust).
- Several clippy warnings were fixed. This results in some method signature changes (e.g. taking `self` instead
  of `&self`)
  but this should not have any practical infulances on existing codebase.
- The `Point::new` constructors are no longer const-fn. This is due to some limitations in const-fn
  not allowing custom trait-bounds. Use the `point!` macro instead to build points in const environments.
- `Dynamic::new` and `Unit::new_unchecked` are now const-fn.
- Methods returning `Result<(), ()>` now return `bool` instead.

### Fixed

- Fixed a potential unsoundess issue when converting a mutable slice to a `&mut[T]`.

## [0.27.1]

### Fixed

- Fixed a bug in the conversion from `glam::Vec2` or `glam::DVec2` to `Isometry2`.

## [0.27.0]

This removes the `convert-glam` and `convert-glam-unchecked` optional features.
Instead, this adds the `convert-glam013`, `convert-glam014`, and `convert-glam015` optional features for
conversions targeting the versions 0.13, 0.14, and 0.15 of `glam`.

### Added

- Add macros `matrix!`, `dmatrix!`, `vector!`, `dvector!`, `point!` for constructing matrices/vectors/points in a
  more convenient way. See [#886](https://github.com/dimforge/nalgebra/pull/886)
  and [#899](https://github.com/dimforge/nalgebra/pull/899).
- Add `CooMatrix::reserve` to `nalgebra-sparse`.
- Add basic support for serialization using `rkyv`. Can be enabled with the features `rkyv-serialize` or
  `rkyv-serialize-no-std`.

### Fixed

- Fixed a potential unsoundness issue after deserializing an invalid `DVector` using `serde`.

## [0.26.2]

### Added

- Conversion from an array `[T; D]` to an isometry `Isometry<T, _, D>` (as a translation).
- Conversion from a static vector `SVector<T; D>` to an isometry `Isometry<T, _, D>` (as a translation).
- Conversion from a point `Point<T; D>` to an isometry `Isometry<T, _, D>` (as a translation).
- Conversion of an array `[T; D]` from/to a translation `Translation<T, D>`.
- Conversion of a point `Point<T, D>` to a translation `Translation<T, D>`.
- Conversion of the tuple of glam types `(Vec3, Quat)` from/to an `Isometry2` or `Isometry3`.
- Conversion of a glam type `Vec2/3/4` from/to a `Translation2/3/4`.

## [0.26.1]

Fix a regression introduced in 0.26.0 preventing `DVector` from being serialized with `serde`.

## [0.26.0]

This release integrates `min-const-generics` to nalgebra. See
[our blog post](https://www.dimforge.com/blog/2021/04/12/integrating-const-generics-to-nalgebra)
for details about this release.

### Added

- Add type aliases for unit vector, e.g., `UnitVector3`.
- Add a `pow` and `pow_mut` function to square matrices.
- Add `Cholesky::determinant` to compute the determinant of a matrix decomposed
  with Cholesky.
- Add the `serde-serialize-no-std` feature to enable serialization of static matrices/vectors
  with serde, but without requiring `std`.

### Modified

- The `serde` crate isn't enabled by default now. Enable the `serde-serialize` or the
  `serde-serialize-no-std` features instead.
- The `Const<const D: usize>` type has been introduced to represent dimensions known
  at compile-time. This replaces the type-level integers from `typenum` as well as
  the `U1, U2, ..., U127` types from `nalgebra`. These `U1, U2, ..., U127` are now
  just aliases for `Const<D>`, e.g., `type U2 = Const<2>`.
- The `ArrayStorage` now uses a standard array `[[T; R]; C]` instead of a `GenericArray`.
- Many trait bounds were changed to accommodate const-generics. Most of these changes
  should be transparent wrt. non-generic code.
- The `MatrixMN` alias has been deprecated. Use `OMatrix` or `SMatrix` instead.
- The `MatrixN<T, D>` alias has been deprecated. Use `OMatrix<T, D, D>` or `SMatrix` instead.
- The `VectorN<T, D>` alias has been deprecated. Use `OVector` or `SVector` instead.
- The `Point`, `Translation`, `Isometry`, `Similarity`, and `Transformation` types now take an
  integer for their dimension (instead of a type-level integer).
- The type parameter order of `Isometry`, `Similarity`, `Transformation` changed to put
  the integer dimensions in the last position (this is required by the compiler).
- The `::new` constructors of translations, points, matrices, and vectors of dimensions `<= 6`
  are now `const fn`, making them usable to define constant globals. The `Quaternion::new`
  constructor is also a `const fn` now.

## [0.25.4]

### Fixed

- Fix a compilation error when only the `serde-serialize` feature is enabled.

## [0.25.3]

### Added

- The `Vector::simd_cap_magnitude` method to cap the magnitude of the vector with
  SIMD components.

## [0.25.2]

### Added

- A `convert-glam` cargo feature to enable implementations of `From` traits to convert
  between `glam` types and `nalgebra` types.
- A `convert-glam-unchecked` cargo feature to enable some extra `glam`/`nalgebra` conversions that may
  lead to unexpected results if used improperly. For example, this enables the conversion from a
  `glam::Mat4` to a `na::Isometry3`. This conversion will be cheap (without any check) but willlead to
  unexpected results if the glam matrix contains non-isometric components (like scaling for example).
- A `cast` method has been added to most types. This can be used to change the
  type of the components of a given entity. Example: `vector.cast::<f32>()`.

## [0.25.1]

This release replaces the version 0.25.0 which has been yanked. The 0.25.0 version
added significant complication to build `nalgebra` targeting a `#[no-std]` platform
not supported by `rand`.

The `rand` dependency is now optional (and disabled by default). You may enable it with:

- The `rand-no-std` cargo feature when targeting a `#[no-std]` environment.
- The `rand` cargo feature when targeting a `std` environment.

## [0.25.0] - Yanked

This updates all the dependencies of nalgebra to their latest version, including:

- rand 0.8
- proptest 1.0
- simba 0.4

### New crate: nalgebra-sparse

Alongside this release of `nalgebra`, we are releasing `nalgebra-sparse`: a crate dedicated to sparse matrix
computation with `nalgebra`. The `sparse` module of `nalgebra`itself still exists for backward compatibility,
but it will be deprecated soon in favor of the `nalgebra-sparse` crate.

### Added

- Add `UnitDualQuaternion`, a dual-quaternion with unit magnitude which can be used as an isometry transformation.
- Add `UDU::new()` and `matrix.udu()` to compute the UDU factorization of a matrix.
- Add `ColPivQR::new()` and `matrix.col_piv_qr()` to compute the QR decomposition with column pivoting of a matrix.
- Add `from_basis_unchecked` to all the rotation types. This builds a rotation from a set of basis vectors (representing
  the columns of the corresponding rotation matrix).
- Add `Matrix::cap_magnitude` to cap the magnitude of a vector.
- Add `UnitQuaternion::append_axisangle_linearized` to approximately append a rotation represented as an axis-angle to a
  rotation represented as an unit quaternion.
- Mark the iterators on matrix components as `DoubleEndedIter`.
- Re-export `simba::simd::SimdValue` at the root of the `nalgebra` crate.

## [0.24.0]

### Added

- The `DualQuaternion` type. It is still work-in-progress, but the basics are here:
  creation from its real and dual part, multiplication of two dual quaternions,
  and normalization.

### Removed

- There is no blanket `impl<T> PartialEq for Unit<T>` anymore. Instead, it is
  implemented specifically for `UnitComplex`, `UnitQuaternion` and `Unit<Vector>`.

## [0.23.2]

In this release, we improved the documentation of some of the geometric types
by applying changes similar to what we did in the version 0.23.1 for matrices.

### Added

- The `Isometry::inv_mul` method which is a more efficient way of doing
  `isometry1.inverse() * isometry2`.

## [0.23.1]

In this release we improved the documentation of the matrix and vector types by:

- Grouping `impl` bocks logically, adding a title comment to these impl blocks.
- Reference these impl blocks docs at the top of the documentation page for `Matrix`.
- Reduce the depth of type aliasing. Now all vector and matrix types are aliases of `Matrix`
  directly (instead of being aliases for other aliases).

## [0.23.0]

### Added

- The `.inverse_transform_unit_vector(v)` was added to `Rotation2/3`, `Isometry2/3`, `UnitQuaternion`,
  and `UnitComplex`.
  It applies the corresponding rotation to a unit vector `Unit<Vector2/3>`.
- The `Point.map(f)` and `Point.apply(f)` to apply a function to each component of the point, similarly
  to `Vector.map(f)`
  and `Vector.apply(f)`.
- The `Quaternion::from([N; 4])` conversion to build a quaternion from an array of four elements.
- The `Isometry::from(Translation)` conversion to build an isometry from a translation.
- The `Vector::ith_axis(i)` which build a unit vector, e.g., `Unit<Vector3<f32>>` with its i-th component set to 1.0,
  and the
  others set to zero.
- The `Isometry.lerp_slerp` and `Isometry.try_lerp_slerp` methods to interpolate between two isometries using linear
  interpolation for the translational part, and spherical interpolation for the rotational part.
- The `Rotation2.slerp`, `Rotation3.slerp`, and `UnitQuaternion.slerp` method for
  spherical interpolation.

## [0.22.0]

In this release, we are using the new version 0.2 of simba. One major change of that version is that the
use of `libm` is now opt-in when building targeting `no-std` environment. If you are using floating-point
operations with nalgebra in a `no-std` environment, you will need to enable the new `libm` feature
of nalgebra for your code to compile again.

### Added

- The `libm` feature that enables `libm` when building for `no-std` environment.
- The `libm-force` feature that enables `libm` even when building for a not `no-std` environment.
- `Cholesky::new_unchecked` which build a Cholesky decomposition without checking that its input is
  positive-definite. It can be used with SIMD types.
- The `Default` trait is now implemented for matrices, and quaternions. They are all filled with zeros,
  except for `UnitQuaternion` which is initialized with the identity.
- Matrix exponential `matrix.exp()`.
- The `Vector::ith(i, x)` that builds a vector filled with zeros except for the `i`-th component set to `x`.

## [0.21.0]

In this release, we are no longer relying on traits from the __alga__ crate for our generic code.
Instead, we use traits from the new [simba](https://crates.io/crates/simba) crate which are both
simpler, and allow for significant optimizations like AoSoA SIMD.

Refer to the [monthly dimforge blogpost](https://www.dimforge.org/blog/2020/04/01/this-month-in-dimforge/)
for details about this switch and its benefits.

### Added

- It is now possible to use SIMD types like `simba::f32x4` as scalar types for nalgebra's matrices and
  geometric types.

### Modified

- Use of traits like `alga::general::{RealField, ComplexField}` have now been replaced by
  `simba::scalar::{RealField, ComplexField}`.
- The implementation of traits from the __alga__ crate (and well as the dependency to _alga__) are now
  omitted unless the `alga` cargo feature is activated.

### Removed

- The `Neg` unary operator is no longer implemented for `UnitComplex` and `UnitQuaternion`. This caused
  hard-to-track errors when we mistakenly write, e.g., `-q * v` instead of `-(q * v)`.
- The `na::convert_unchecked` is no longer marked as unsafe.

## [0.20.0]

### Added

- `cholesky.rank_one_update(...)` which performs a rank-one update on the cholesky decomposition of a matrix.
- `From<&Matrix>` is now implemented for matrix slices.
- `.try_set_magnitude(...)` which sets the magnitude of a vector, while keeping its direction.
- Implementations of `From` and `Into` for the conversion between matrix slices and standard (`&[N]` `&mut [N]`) slices.

### Modified

- We started some major changes in order to allow non-Copy types to be used as scalar types inside of matrices/vectors.

## [0.19.0]

### Added

- `.remove_rows_at` and `remove_columns_at` which removes a set of rows or columns (specified by indices) from a matrix.
- Several formatting traits have been implemented for all matrices/vectors: `LowerExp`, `UpperExp`, `Octal`, `LowerHex`,
  `UpperHex`, `Binary`, `Pointer`.
- `UnitQuaternion::quaternions_mean(...)` which computes the mean rotation of a set of unit quaternions. This implements
  the algorithm from _Oshman, Yaakov, and Avishy Carmi, "Attitude estimation from vector observations using a
  genetic-algorithm-embedded quaternion particle filter."

### Modified

- It is now possible to get the `min/max` element of unsigned integer matrices.

### Added to nalgebra-glm

- Some infinite and reversed perspectives: `::infinite_perspective_rh_no`, `::infinite_perspective_rh_zo`,
  `::reversed_perspective_rh_zo`, and `::reversed_infinite_perspective_rh_zo`.

## [0.18.0]

This release adds full complex number support to nalgebra. This includes all common vector/matrix operations as well
as matrix decomposition. This excludes geometric type (like `Isometry`, `Rotation`, `Translation`, etc.) from the
`geometry` module.

### Added

#### Quaternion and geometric operations

- Add trigonometric functions for
  quaternions: `.cos, .sin, .tan, .acos, .asin, .atan, .cosh, .sinh, .tanh, .acosh, .asinh, .atanh`.
- Add geometric algebra operations for quaternions: `.inner, .outer, .project, .rejection`
- Add `.left_div, .right_div` for quaternions.
- Add `.renormalize` to `Unit<...>` and `Rotation3` to correct potential drift due to repeated operations.
  Those drifts could cause them not to be pure rotations anymore.

#### Convolution

- `.convolve_full(kernel)` returns the convolution of `self` by `kernel`.
- `.convolve_valid(kernel)` returns the convolution of `self` by `kernel` after removal of all the elements relying on
  zero-padding.
- `.convolve_same(kernel)` returns the convolution of `self` by `kernel` with a result of the same size as `self`.

#### Complex number support

- Add the `::from_matrix` constructor too all rotation types to extract a rotation from a raw matrix.
- Add the `::from_matrix_eps` constructor too all rotation types to extract a rotation from a raw matrix. This takes
  more argument than `::from_matrix` to control the convergence of the underlying optimization algorithm.
- Add `.camax()` which returns the matrix component with the greatest L1-norm.
- Add `.camin()` which returns the matrix component with the smallest L1-norm.
- Add `.ad_mul(b)` for matrix-multiplication of `self.adjoint() * b`.
- Add `.ad_mul_to(b)` which is the same as `.ad_mul` but with a provided matrix to be filled with the result of the
  multiplication.
- Add BLAS operations involving complex conjugation (following similar names as the original BLAS spec):
  - `.dotc(rhs)` equal to  `self.adjoint() * rhs`.
  - `.gerc(alpha, x, y, beta)` equivalent to `self = alpha * x * y.adjoint() + beta * self`
  - `.hegerc` which is like `gerc` but for Hermitian matrices.
  - `.syger` which is the new name of `.ger_symm` which is equivalent
      to `self = alpha * x * y.transpose() + beta * self`.
  - `.sygemv` which is the new name of `.gemv_symm` which is equivalent to `self = alpha * a * x + beta * self`
      with `a` symmetric.
  - `.hegemv(alpha, a, x, beta)` which is like `.sygemv` but with `a` Hermitian.
  - `.gemv_ad(alpha, a, x, beta)` which is equivalent to `self = alpha * a.adjoint() * x + beta * self`.
  - `.gemm_ad(alpha, a, b, beta)` which is equivalent to `self = alpha * a.adjoint() * b + beta * self`.
  - `.icamax()` which returns the index of the complex vector component with the greatest L1-norm.

Note that all the other BLAS operation will continue to work for all fields, including floats and complex numbers.

### Renamed

- `RealSchur` has been renamed `Schur` because it can now work with complex matrices.

## [0.17.0]

### Added

- Add swizzling up to dimension 3 for vectors. For example, you can do `v.zxy()` as an equivalent
  to `Vector3::new(v.z, v.x, v.y)`.
- Add swizzling up to dimension 3 for points. For example, you can do `p.zxy()` as an equivalent
  to `Point3::new(p.z, p.x, p.y)`.
- Add `.copy_from_slice` to copy matrix components from a slice in column-major order.
- Add `.dot` to quaternions.
- Add `.zip_zip_map` for iterating on three matrices simultaneously, and applying a closure to them.
- Add `.slerp` and `.try_slerp` to unit vectors.
- Add `.lerp` to vectors.
- Add `.to_projective` and `.as_projective` to `Perspective3` and `Orthographic3` in order to
  use them as `Projective3` structures.
- Add `From/Into` impls to allow the conversion of any transformation type to a matrix.
- Add `Into` impls to convert a matrix slice into an owned matrix.
- Add `Point*::from_slice` to create a point from a slice.
- Add `.map_with_location` to matrices to apply a map which passes the component indices to the user-defined closure
  alongside
  the component itself.
- Add impl `From<Vector>` for `Point`.
- Add impl `From<Vector4>` for `Quaternion`.
- Add impl `From<Vector>` for `Translation`.
- Add the `::from_vec` constructor to construct a matrix from a `Vec` (a `DMatrix` will reuse the original `Vec`
  as-is for its storage).
- Add `.to_homogeneous` to square matrices (and with dimensions higher than 1x1). This will increase their number of row
  and columns by 1. The new column and row are filled with 0, except for the diagonal element which is set to 1.
- Implement `Extend<Vec>` for matrices with a dynamic storage. The provided `Vec` is assumed to represent a column-major
  matrix with the same number of rows as the one being extended. This will effectively append new columns on the right
  of
  the matrix being extended.
- Implement `Extend<Vec>` for vectors with a dynamic storage. This will concatenate the vector with the given `Vec`.
- Implement `Extend<Matrix<...>>` for matrices with dynamic storage. This will concatenate the columns of both matrices.
- Implement `Into<Vec>` for the `MatrixVec` storage.
- Implement `Hash` for all matrices.
- Add a `.len()` method to retrieve the size of a `MatrixVec`.

### Modified

- The orthographic projection no longer require that `bottom < top`, that `left < right`, and that `znear < zfar`. The
  only restriction now ith that they must not be equal (in which case the projection would be singular).
- The `Point::from_coordinates` methods is deprecated. Use `Point::from` instead.
- The `.transform_point` and `.transform_vector` methods are now inherent methods for matrices so that the user does not
  have to
  explicitly import the `Transform` trait from the alga crate.
- Renamed the matrix storage types: `MatrixArray` -> `ArrayStorage` and `MatrixVec` -> `VecStorage`.
- Renamed `.unwrap()` to `.into_inner()` for geometric types that wrap another type.
  This is for the case of `Unit`, `Transform`, `Orthographic3`, `Perspective3`, `Rotation`.
- Deprecate several functions at the root of the crate (replaced by methods).

### Removed

    * Remove the `Deref` impl for `MatrixVec` as it could cause hard-to-understand compilation errors.

### nalgebra-glm

- Add several alternative projection computations, e.g., `ortho_lh`, `ortho_lh_no`, `perspective_lh`, etc.
- Add features matching those of nalgebra, in particular:`serde-serialize`, `abmonation-serialize`, std` (enabled by
  default).

## [0.16.0]

All dependencies have been updated to their latest versions.

## Modified

- Adjust `UnitQuaternion`s, `Rotation3`s, and `Rotation2`s generated from the `Standard` distribution to be uniformly
  distributed.

### Added

- Add a feature `stdweb` to activate the dependency feature `rand/stdweb`.
- Add blas-like methods `.imin()` and `.imax()` that return the index of the minimum and maximum entry of a vector.
- Add construction of a `Point` from an array by implementing the `From` trait.
- Add support for generating uniformly distributed random unit column vectors using the `Standard` distribution.

## [0.15.0]

The most notable change of this release is the support for using part of the library without the rust standard
library (i.e. it supports `#![no_std]`). See the
corresponding [documentation](https://nalgebra.rs/docs/user_guide/wasm_and_embedded_targets/).

### Modified

- Rename the `core` module to `base` to avoid conflicts with the `core` crate implicitly imported when
  `#![no_std]` is enabled.
- Constructors of the `MatrixSlice*` types have been renamed from `new_*` to `from_slice_*`. This was
  necessary to avoid the `incoherent_fundamental_impls` lint that is going to become a hard error.

### Added

- Add `UnitQuaternion` constructor `::new_eps(...)` and `::from_scaled_axis_eps(...)` that return the
  identity if the magnitude of the input axisangle is smaller than the epsilon provided.
- Add methods `.rotation_between_axis(...)` and `.scaled_rotation_between_axis(...)` to `UnitComplex`
  to compute the rotation matrix between two 2D __unit__ vectors.
- Add methods `.axis_angle()` to `UnitComplex` and `UnitQuaternion` in order to retrieve both the
  unit rotation axis, and the rotation angle simultaneously.
- Add functions to construct a random matrix with a user-defined distribution: `::from_distribution(...)`.

## [0.14.0]

### Modified

- Allow the `Isometry * Unit<Vector>` multiplication.

### Added

- Add blas-like operations: `.quadform(...)` and `.quadform_tr(...)` to compute respectively
  the quadratic forms `self = alpha * A.transpose() * B * A + beta * self` and
  `alpha * A * B * A.transpose() + beta * self`. Here, `A, B` are matrices with
  `B` square, and `alpha, beta` are reals.
- Add blas-like operations: `.gemv_tr(...)` that behaves like `.gemv` except that the
  provided matrix is assumed to be transposed.
- Add blas-like operations: `cmpy, cdpy` for component-wise multiplications and
  division with scalar factors:
  - `self <- alpha * self + beta * a * b`
  - `self <- alpha * self + beta * a / b`
- `.cross_matrix()` returns the cross-product matrix of a given 3D vector, i.e.,
  the matrix `M` such that for all vector `v` we have
  `M * v == self.cross(&v)`.
- `.iamin()` that returns the index of the vector entry with
  the smallest absolute value.
- The `mint` feature that can be enabled in order to allow conversions from
  and to types of the [mint](https://crates.io/crates/mint) crate.
- Aliases for matrix and vector slices. Their are named by adding `Slice`
  before the dimension numbers, i.e., a 3x5 matrix slice with dimensions known
  at compile-time is called `MatrixSlice3x5`. A vector slice with dimensions
  unknown at compile-time is called `DVectorSlice`.
- Add functions for constructing matrix slices from a slice `&[N]`, e.g.,
  `MatrixSlice2::new(...)` and `MatrixSlice2::new_with_strides(...)`.
- The `::repeat(...)` constructor that is an alternative name to
  `::from_element(...)`.
- `UnitQuaternion::scaled_rotation_between_axis(...)` and
  `UnitQuaternion::rotation_between_axis(...)` that take Unit vectors instead of
  Vector as arguments.

## [0.13.0]

The __nalgebra-lapack__ crate has been updated. This now includes a broad range
matrix decompositions using LAPACK bindings.

This adds support for serialization using the
[abomonation](https://crates.io/crates/abomonation) crate.

### Breaking semantic change

- The implementation of slicing with steps now matches the documentation.
  Before, step identified the number to add to pass from one column/row index
  to the next one. This made 0 step invalid. Now (and on the documentation so
  far), the step is the number of ignored row/columns between each
  row/column. Thus, a step of 0 means that no row/column is ignored. For
  example, a step of, say, 3 on previous versions should now bet set to 2.

### Modified

- The trait `Axpy` has been replaced by a method `.axpy`.
- The alias `MatrixNM` is now deprecated. Use `MatrixMN` instead (we
  reordered M and N to be in alphabetical order).
- In-place componentwise multiplication and division
  `.component_mul_mut(...)` and `.component_div_mut(...)` have been deprecated
  for a future renaming. Use `.component_mul_assign(...)` and
  `.component_div_assign(...)` instead.

### Added

- `alga::general::Real` is now re-exported by nalgebra.
  elements.)
- `::zeros(...)` that creates a matrix filled with zeroes.
- `::from_partial_diagonal(...)` that creates a matrix from diagonal elements.
  The matrix can be rectangular. If not enough elements are provided, the rest
  of the diagonal is set to 0.
- `.conjugate_transpose()` computes the transposed conjugate of a
  complex matrix.
- `.conjugate_transpose_to(...)` computes the transposed conjugate of a
  complex matrix. The result written into a user-provided matrix.
- `.transpose_to(...)` is the same as `.transpose()` but stores the result in
  the provided matrix.
- `.conjugate_transpose_to(...)` is the same as `.conjugate_transpose()` but
  stores the result in the provided matrix.
- Implements `IntoIterator` for `&Matrix`, `&mut Matrix` and `Matrix`.
- `.mul_to(...)` multiplies two matrices and stores the result to the given buffer.
- `.tr_mul_to(...)` left-multiplies `self.transpose()` to another matrix and stores the result to the given buffer.
- `.add_scalar(...)` that adds a scalar to each component of a matrix.
- `.add_scalar_mut(...)` that adds in-place a scalar to each component of a matrix.
- `.kronecker(a, b)` computes the kronecker product (i.e. matrix tensor
  product) of two matrices.
- `.apply(f)` replaces each component of a matrix with the results of the
  closure `f` called on each of them.

Pure Rust implementation of some Blas operations:

- `.iamax()` returns the index of the maximum value of a vector.
- `.axpy(...)` computes `self = a * x + b * self`.
- `.gemv(...)` computes `self = alpha * a * x + beta * self` with a matrix and vector `a` and `x`.
- `.ger(...)` computes `self = alpha * x^t * y + beta * self` where `x` and `y` are vectors.
- `.gemm(...)` computes `self = alpha * a * b + beta * self` where `a` and `b` are matrices.
- `.gemv_symm(...)` is the same as `.gemv` except that `self` is assumed symmetric.
- `.ger_symm(...)` is the same as `.ger` except that `self` is assumed symmetric.

New slicing methods:

- `.rows_range(...)` that retrieves a reference to a range of rows.
- `.rows_range_mut(...)` that retrieves a mutable reference to a range of rows.
- `.columns_range(...)` that retrieves a reference to a range of columns.
- `.columns_range_mut(...)` that retrieves a mutable reference to a range of columns.

Matrix decompositions implemented in pure Rust:

- Cholesky, SVD, LU, QR, Hessenberg, Schur, Symmetric eigendecompositions,
  Bidiagonal, Symmetric tridiagonal
- Computation of householder reflectors and givens rotations.

Matrix edition:

- `.upper_triangle()` extracts the upper triangle of a matrix, including the diagonal.
- `.lower_triangle()` extracts the lower triangle of a matrix, including the diagonal.
- `.fill(...)` fills the matrix with a single value.
- `.fill_with_identity(...)` fills the matrix with the identity.
- `.fill_diagonal(...)` fills the matrix diagonal with a single value.
- `.fill_row(...)` fills a selected matrix row with a single value.
- `.fill_column(...)` fills a selected matrix column with a single value.
- `.set_diagonal(...)` sets the matrix diagonal.
- `.set_row(...)` sets a selected row.
- `.set_column(...)` sets a selected column.
- `.fill_lower_triangle(...)` fills some sub-diagonals below the main diagonal with a value.
- `.fill_upper_triangle(...)` fills some sub-diagonals above the main diagonal with a value.
- `.swap_rows(...)` swaps two rows.
- `.swap_columns(...)` swaps two columns.

Column removal:

- `.remove_column(...)` removes one column.
- `.remove_fixed_columns<D>(...)` removes `D` columns.
- `.remove_columns(...)` removes a number of columns known at run-time.

Row removal:

- `.remove_row(...)` removes one row.
- `.remove_fixed_rows<D>(...)` removes `D` rows.
- `.remove_rows(...)` removes a number of rows known at run-time.

Column insertion:

- `.insert_column(...)` adds one column at the given position.
- `.insert_fixed_columns<D>(...)` adds `D` columns at the given position.
- `.insert_columns(...)` adds at the given position a number of columns known at run-time.

Row insertion:

- `.insert_row(...)` adds one row at the given position.
- `.insert_fixed_rows<D>(...)` adds `D` rows at the given position.
- `.insert_rows(...)` adds at the given position a number of rows known at run-time.

## [0.12.0]

The main change of this release is the update of the dependency serde to 1.0.

### Added

- `.trace()` that computes the trace of a matrix (the sum of its diagonal
  elements.)

## [0.11.0]

The [website](https://nalgebra.rs) has been fully rewritten and gives a good
overview of all the added/modified features.

This version is a major rewrite of the library. Major changes are:

- Algebraic traits are now defined by the [alga](https://crates.io/crates/alga) crate.
  All other mathematical traits, except `Axpy` have been removed from
  __nalgebra__.
- Methods are now preferred to free functions because they do not require any
  trait to be used anymore.
- Most algebraic entities can be parametrized by type-level integers
  to specify their dimensions. Using `Dynamic` instead of a type-level
  integer indicates that the dimension known at run-time only.
- Statically-sized __rectangular__ matrices.
- More transformation types have been added: unit-sized complex numbers (for
  2D rotations), affine/projective/general transformations with `Affine2/3`,
  `Projective2/3`, and `Transform2/3`.
- Serde serialization is now supported instead of `rustc_serialize`. Enable
  it with the `serde-serialize` feature.
- Matrix __slices__ are now implemented.

### Added

Lots of features including rectangular matrices, slices, and Serde
serialization. Refer to the brand new [website](https://nalgebra.rs) for more
details. The following free-functions have been added as well:

* `::id()` that returns the universal [identity element](https://nalgebra.rs/performance_tricks/#the-id-type)
  of type `Id`.
- `::inf_sup()` that returns both the infimum and supremum of a value at the
  same time.
- `::partial_sort2()` that attempts to sort two values in increasing order.
- `::wrap()` that moves a value to the given interval by adding or removing
  the interval width to it.

### Modified

- `::cast`            -> `::convert`
- `point.as_vector()` -> `point.coords`
- `na::origin`        -> `P::origin()`
- `na::is_zero`       -> `.is_zero()` (from num::Zero)
- `.transform`        -> `.transform_point`/`.transform_vector`
- `.translate`        -> `.translate_point`
- `::dimension::<P>`  -> `::dimension::<P::Vector>`
- `::angle_between`   -> `::angle`

Componentwise multiplication and division has been replaced by methods:

- multiplication -> `.componentwise_mul`, `.componentwise_mul_mut`.
- division -> `.componentwise_div`, `.componentwise_div_mut`.

The following free-functions are now replaced by methods (with the same names)
only:
`::cross`, `::cholesky`, `::determinant`, `::diagonal`, `::eigen_qr` (becomes
`.eig`), `::hessenberg`, `::qr`, `::to_homogeneous`, `::to_rotation_matrix`,
`::transpose`, `::shape`.

The following free-functions are now replaced by static methods only:

- `::householder_matrix` under the name `::new_householder_generic`
- `::identity`
- `::new_identity` under the name `::identity`
- `::from_homogeneous`
- `::repeat` under the name `::from_element`

The following free-function are now replaced methods accessible through traits
only:

- `::transform` -> methods `.transform_point` and `.transform_vector` of the `alga::linear::Transformation` trait.
- `::inverse_transform` -> methods `.inverse_transform_point` and
  `.inverse_transform_vector` of the `alga::linear::ProjectiveTransformation`
  trait.
- `::translate`, `::inverse_translate`, `::rotate`, `::inverse_rotate` ->
  methods from the `alga::linear::Similarity` trait instead. Those have the
  same names but end with `_point` or `_vector`, e.g., `.translate_point` and
  `.translate_vector`.
- `::orthonormal_subspace_basis` -> method with the same name from
  `alga::linear::FiniteDimInnerSpace`.
- `::canonical_basis_element` and `::canonical_basis` -> methods with the
  same names from `alga::linear::FiniteDimVectorSpace`.
- `::rotation_between` -> method with the same name from the
  `alga::linear::Rotation` trait.
- `::is_zero` -> method with the same name from `num::Zero`.

### Removed

- The free functions `::prepend_rotation`, `::append_rotation`,
  `::append_rotation_wrt_center`, `::append_rotation_wrt_point`,
  `::append_transformation`, and `::append_translation` have been removed.
  Instead, create the rotation or translation object explicitly and use
  multiplication to compose it with anything else.

- The free function `::outer` has been removed. Use column-vector ×
  row-vector multiplication instead.

- `::approx_eq`, `::approx_eq_eps` have been removed. Use the `relative_eq!`
  macro from the [approx](https://crates.io/crates/approx) crate instead.

- `::covariance` has been removed. There is no replacement for now.
- `::mean` has been removed. There is no replacement for now.
- `::sample_sphere` has been removed. There is no replacement for now.
- `::cross_matrix` has been removed. There is no replacement for now.
- `::absolute_rotate` has been removed. There is no replacement for now.
- `::rotation`, `::transformation`, `::translation`, `::inverse_rotation`,
  `::inverse_transformation`, `::inverse_translation` have been removed. Use
  the appropriate methods/field of each transformation type, e.g.,
  `rotation.angle()` and `rotation.axis()`.

## [0.10.0]

### Added

Binary operations are now allowed between references as well. For example
`Vector3<f32> + &Vector3<f32>` is now possible.

### Modified

Removed unused parameters to methods from the `ApproxEq` trait. Those were
required before rust 1.0 to help type inference. They are not needed any more
since it now allowed to write for a type `T` that implements `ApproxEq`:
`<T as ApproxEq>::approx_epsilon()`. This replaces the old form:
`ApproxEq::approx_epsilon(None::<T>)`.

## [0.9.0]

### Modified

- Renamed:
  - `::from_col_vector` -> `::from_column_vector`
  - `::from_col_iter` -> `::from_column_iter`
  - `.col_slice` -> `.column_slice`
  - `.set_col` -> `.set_column`
  - `::canonical_basis_with_dim` -> `::canonical_basis_with_dimension`
  - `::from_elem` -> `::from_element`
  - `DiagMut` -> `DiagonalMut`
  - `UnitQuaternion::new` becomes `UnitQuaternion::from_scaled_axis` or
      `UnitQuaternion::from_axisangle`. The new `::new` method now requires a
      not-normalized quaternion.

Method names starting with `new_with_` now start with `from_`. This is more
idiomatic in Rust.

The `Norm` trait now uses an associated type instead of a type parameter.
Other similar trait changes are to be expected in the future, e.g., for the
`Diagonal` trait.

Methods marked `unsafe` for reasons unrelated to memory safety are no
longer unsafe. Instead, their name end with `_unchecked`. In particular:

- `Rotation3::new_with_matrix` -> `Rotation3::from_matrix_unchecked`
- `PerspectiveMatrix3::new_with_matrix` -> `PerspectiveMatrix3::from_matrix_unchecked`
- `OrthographicMatrix3::new_with_matrix` -> `OrthographicMatrix3::from_matrix_unchecked`

### Added

- A `Unit<T>` type that wraps normalized values. In particular,
  `UnitQuaternion<N>` is now an alias for `Unit<Quaternion<N>>`.
- `.ln()`, `.exp()` and `.powf(..)` for quaternions and unit quaternions.
- `::from_parts(...)` to build a quaternion from its scalar and vector
  parts.
- The `Norm` trait now has a `try_normalize()` that returns `None` if the
  norm is too small.
- The `BaseFloat` and `FloatVector` traits now inherit from `ApproxEq` as
  well. It is clear that performing computations with floats requires
  approximate equality.

Still WIP: add implementations of abstract algebra traits from the `algebra`
crate for vectors, rotations and points. To enable them, activate the
`abstract_algebra` feature.

## [0.8.0]

### Modified

- Almost everything (types, methods, and traits) now use fulls names instead
  of abbreviations (e.g. `Vec3` becomes `Vector3`). Most changes are obvious.
  Note however that:
  - `::sqnorm` becomes `::norm_squared`.
  - `::sqdist` becomes `::distance_squared`.
  - `::abs`, `::min`, etc. did not change as this is a common name for
      absolute values on, e.g., the libc.
  - Dynamically sized structures keep the `D` prefix, e.g., `DMat` becomes
      `DMatrix`.
- All files with abbreviated names have been renamed to their full version,
  e.g., `vec.rs` becomes `vector.rs`.

## [0.7.0]

### Added

- Added implementation of assignment operators (+=, -=, etc.) for
  everything.

### Modified

- Points and vectors are now linked to each other with associated types
  (on the PointAsVector trait).

## [0.6.0]

__Announcement:__ a users forum has been created for `nalgebra`, `ncollide`, and `nphysics`. See
you [there](https://users.nphysics.org)!

### Added

- Added a dependency to [generic-array](https://crates.io/crates/generic-array). Feature-gated:
  requires `features="generic_sizes"`.
- Added statically sized vectors with user-defined sizes: `VectorN`. Feature-gated: requires
  `features="generic_sizes"`.
- Added similarity transformations (an uniform scale followed by a rotation followed by a
  translation): `Similarity2`, `Similarity3`.

### Removed

- Removed zero-sized elements `Vector0`, `Point0`.
- Removed 4-dimensional transformations `Rotation4` and `Isometry4` (which had an implementation too incomplete to be
  useful).

### Modified

- Vectors are now multipliable with isometries. This will result into a pure rotation (this is how
  vectors differ from point semantically: they design directions, so they are not translatable).
- `{Isometry3, Rotation3}::look_at` reimplemented and renamed to `::look_at_rh` and `::look_at_lh` to agree
  with the computer graphics community (in particular, the GLM library). Use the `::look_at_rh`
  variant to build a view matrix that
  may be successfully used with `Persp` and `Ortho`.
- The old `{Isometry3, Rotation3}::look_at` implementations are now called `::new_observer_frame`.
- Rename every `fov` on `Persp` to `fovy`.
- Fixed the perspective and orthographic projection matrices.

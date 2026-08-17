## 0.4.1 (2025-05-20)

- Added `from_f32_slice` function to `HalfVector`

## 0.4.0 (2024-07-28)

- Added support for SQLx 0.8
- Dropped support for SQLx < 0.8

## 0.3.4 (2024-07-17)

- Added `Eq` trait to `Bit`

## 0.3.3 (2024-06-25)

- Added support for `halfvec`, `bit`, and `sparsevec` types to Rust-Postgres
- Added support for `halfvec`, `bit`, and `sparsevec` type to SQLx
- Added support for `halfvec`, `bit`, and `sparsevec` type to Diesel
- Added `l1_distance`, `hamming_distance`, and `jaccard_distance` functions for Diesel

## 0.3.2 (2023-10-30)

- Fixed error with Diesel without `with-deprecated` feature

## 0.3.1 (2023-10-19)

- Added `as_slice` method

## 0.3.0 (2023-10-17)

- Added `serde` feature
- Removed `postgres` from default features
- Reduced dependencies
- Updated Rust edition to 2021

## 0.2.2 (2023-06-02)

- Added `Clone` trait to `Vector`
- Fixed deprecation warning with Diesel 2.1

## 0.2.1 (2023-05-23)

- Added support for `vector[]` type with SQLx

## 0.2.0 (2022-09-05)

- Added support for Diesel 2
- Dropped support for Diesel 1

## 0.1.4 (2022-01-12)

- Added `into`

## 0.1.3 (2021-06-22)

- Fixed SQL type of Diesel operators

## 0.1.2 (2021-06-18)

- Added support for SQLx
- Added operators for Diesel

## 0.1.1 (2021-06-17)

- Added support for Diesel

## 0.1.0 (2021-06-09)

- First release

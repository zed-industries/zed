## Version 0.3.8

- Eliminate unsound code
- Remove several unnecessary dependencies
- Internal refactoring

## Version 0.3.7

- Add documentation for `Frame` struct
- Replace `hawktracer` with `tracing` crate

## Version 0.3.6

- Revert changes in downsampling in 0.3.4 which changed its behavior

## Version 0.3.5

- Bump num-derive to 0.4

## Version 0.3.4

- Fix cases of unsoundness (#14)
- Slight optimizations for downsampling

## Version 0.3.3

- Add `row_cropped` and `row_slice_cropped` methods to get rows without padding
- Make `RowsIter` and `RowsIterMut` return rows without right-side padding for greater consistency/predictability
- Fix clippy lints

## Version 0.3.1

- Add `rows_iter_mut` method to `Plane`

## Version 0.2.6

- Split into separate repository
- Remove unused rayon dependency
- Fix some clippy lints

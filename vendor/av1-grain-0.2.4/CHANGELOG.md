<!-- ## Upcoming (WIP)

- [Feature] Add a new function, `generate_film_grain_params`. This works like `generate_photon_noise_params` but generates more coarse, film-like grain.
- [Feature] Add the `estimate` module which contains the `estimate_plane_noise` function. This takes in a series of frames and estimates the amount of noise for each of them. This feature is enabled by default. -->

## Version 0.2.4

- Fix a compilation issue with `--no-default-features`

## Version 0.2.3

- Many speed optimizations to diff

## Version 0.2.2

- Fix issue where `NoiseModel` may fail in certain circumstances.
- Considerably speed up `NoiseModel` calculations.

## Version 0.2.1

- Bump `v_frame` to 0.3
- Fix a clippy warning

## Version 0.2.0

- [Breaking] Change the name of `generate_grain_params` to `generate_photon_noise_params`. This was done to support the future `generate_film_grain_params` feature.
- [Feature] Add the `diff` module which contains the `DiffGenerator` struct. This takes in a series of source frames and denoised frames and generates a grain table based on the difference. This feature is enabled by default.

## Version 0.1.4

- Fix a bug that prevented `generate_luma_noise_points` from generating any luma noise points.
  - ALL previous versions have been yanked because of the severity of this bug. Please update to this one.

## Version 0.1.3

- Be more consistent in using `anyhow::Result`

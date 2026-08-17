## 0.4.0
* fix `from` and `merge_fn` when using `cfg` attrs on the original struct fields
* upgrade dependencies, bumping minimum supported rustc version to 1.61 (required by latest `syn`)

## 0.3.0
* update to syn v2.0, bumping minimum rustc version to 1.56.0
* add feature list and attribute order note to documentation

## 0.2.0
* add ability to generate `impl From<Original> for Opt` thanks to [Han Yang](https://github.com/billythedummy)

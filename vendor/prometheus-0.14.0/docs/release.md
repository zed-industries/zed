# Release process

## Crate prometheus

1. Create pull request with bumped `version` in `Cargo.toml` and updated `CHANGELOG.md`.

2. Once merged clean your local environment.

  ```bash
  cargo clean
  git clean -fd
  ```
  
3. Tag the release.

  ```bash
  tag="v$(sed -En 's/^version = \"(.*)\"$/\1/p' Cargo.toml)"
  git tag -s "${tag}" -m "${tag}"
  ```
  
4. Publish the release.

  ```bash
  cargo publish
  ```
  
5. Push the tag.

  ```bash
  git push origin $tag
  ```

## Crate prometheus-static-metric

1. Create pull request with bumped `version` in `static-metric/Cargo.toml` and updated `static-metric/CHANGELOG.md`.

2. Once merged clean your local environment.

  ```bash
  cd static-metric
  cargo clean
  git clean -fd
  ```
  
3. Tag the release.

  ```bash
  tag="$(sed -En 's/^name = \"(.*)\"$/\1/p' Cargo.toml | head -n 1)-v$(sed -En 's/^version = \"(.*)\"$/\1/p' Cargo.toml)"
  git tag -s "${tag}" -m "${tag}"
  ```
  
4. Publish the release.

  ```bash
  cargo publish
  ```
  
5. Push the tag.

  ```bash
  git push origin $tag
  ```


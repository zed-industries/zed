How to make a release of ammonia
================================

* Make a pull request with all these changes, wait until it's approved:

  * Bump the version in Cargo.toml

  * Check if all the dependencies are up-to-date

  * Put all the Unreleased stuff in CHANGELOG.md under the new version

* Check out and pull down `master`

* Copy the CHANGELOG into a GitHub release

* Make sure that README.md requests the same major.minor version, and that SECURITY.md describes the backport plan

* Run `cargo publish`

# Changelog - imara-diff

All notable changes to imara-diff will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## 0.1.8 - 2025-2-1

### Changed

* update MSRV to 1.71
* drop ahash dependency in favour of hashbrowns default hasher (foldhash)

### Fixed

* incomplete documentation for sink and interner

## 0.1.7 - 2024-26-7

### Fixed

* pointer alias violations in myers diff algorithm implementation

## 0.1.5 - 2022-11-4

### Fixed

* `inter::Interner::erase_tokens_after` not removing tokens from the LUT in some cases.

## 0.1.4 - 2022-11-4

### Fixed

* `inter::Interner::erase_tokens_after` only removed tokens from the LUT of the interner but did not actually remove the value from the list of tokens. This cause iteration to still access these values.

## 0.1.3 - 2022-10-26

### Fixed

* Dependency on multiple `ahash` versions

## 0.1.2 - 2022-10-26

# Documentation

* Add multiple usage examples to the crate documentation

## 0.1.1 - 2022-10-25

### Added

* `Interner::erase_tokens_after` - allows reusing the interner without leaking memory.

## 0.1.0 - 2022-10-24

Initial Release

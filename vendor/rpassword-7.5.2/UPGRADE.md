# Upgrade Policy for `rpassword`

## Overview
This policy outlines the versioning and upgrade path for `rpassword`.

Please report any upgrading issues or feedback to the project's issue tracker.

## Versioning Rules

### Patch Versions (`x.y.Z`)
- **Definition**: Bug fixes, performance improvements, and minor documentation updates.
- **Compatibility**: Fully backward-compatible. No breaking changes.
- **User Action**: Safe to upgrade without code modifications.

### Minor Versions (`x.Y.z`)
- **Definition**: New features, enhancements, and deprecations.
- **Compatibility**: Backward-compatible. Deprecated APIs remain functional but emit warnings.
- **User Action**:
    1. Upgrade to the latest minor version.
    2. Address deprecation warnings to prepare for the next major version.

### Major Versions (`X.y.z`)
- **Definition**: Breaking changes, API redesigns, or significant architectural shifts.
- **Compatibility**: Not backward-compatible. Deprecated APIs may be removed.
- **User Action**:
    1. Upgrade to the latest minor version of the current major release.
    2. Fix all deprecation warnings.
    3. Upgrade to the next major version.
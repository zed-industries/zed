# Changelog

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.8.0 to 0.8.1
    * accesskit_consumer bumped from 0.9.1 to 0.10.0

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.11.0 to 0.12.0

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.12.0 to 0.12.1

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.10.1 to 0.11.0
    * accesskit_consumer bumped from 0.14.1 to 0.14.2

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.11.0 to 0.11.1
    * accesskit_consumer bumped from 0.15.0 to 0.15.1

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.11.1 to 0.11.2
    * accesskit_consumer bumped from 0.15.1 to 0.15.2

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.12.2 to 0.12.3
    * accesskit_consumer bumped from 0.17.0 to 0.17.1

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.2 to 0.16.3
    * accesskit_consumer bumped from 0.24.2 to 0.24.3

## [0.33.1](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.33.0...accesskit_windows-v0.33.1) (2026-06-12)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.24.0 to 0.24.1
    * accesskit_consumer bumped from 0.36.0 to 0.37.0

## [0.33.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.32.1...accesskit_windows-v0.33.0) (2026-05-11)


### ⚠ BREAKING CHANGES

* Prepare consumer crate and Windows adapter for 2024 edition

### Features

* Support tree views on Windows ([#698](https://github.com/AccessKit/accesskit/issues/698)) ([1d3b189](https://github.com/AccessKit/accesskit/commit/1d3b18912ea5f4b0abd37ddb0a771768155fbd7b))


### Code Refactoring

* Prepare consumer crate and Windows adapter for 2024 edition ([258b2e6](https://github.com/AccessKit/accesskit/commit/258b2e66b741b142c84563ef00f70bf6bc8da9d8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.35.0 to 0.36.0

## [0.32.1](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.32.0...accesskit_windows-v0.32.1) (2026-03-04)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.34.0 to 0.35.0

## [0.32.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.31.1...accesskit_windows-v0.32.0) (2026-02-01)


### ⚠ BREAKING CHANGES

* Update minimum supported Rust version to 1.85 ([#683](https://github.com/AccessKit/accesskit/issues/683))

### Bug Fixes

* Update minimum supported Rust version to 1.85 ([#683](https://github.com/AccessKit/accesskit/issues/683)) ([f8c22cb](https://github.com/AccessKit/accesskit/commit/f8c22cbb53ce6b1859adeff3c0b7154ad9e1f25e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.23.0 to 0.24.0
    * accesskit_consumer bumped from 0.33.1 to 0.34.0

## [0.31.1](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.31.0...accesskit_windows-v0.31.1) (2026-01-18)


### Bug Fixes

* Emit initial focus event for active descendant node ([#681](https://github.com/AccessKit/accesskit/issues/681)) ([13a37d1](https://github.com/AccessKit/accesskit/commit/13a37d144e2fafceaf26e7645f6acc679ccb2f34))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.33.0 to 0.33.1

## [0.31.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.30.0...accesskit_windows-v0.31.0) (2026-01-15)


### ⚠ BREAKING CHANGES

* Multiple tree support ([#655](https://github.com/AccessKit/accesskit/issues/655))

### Features

* Implement the `url` property ([#669](https://github.com/AccessKit/accesskit/issues/669)) ([6202e63](https://github.com/AccessKit/accesskit/commit/6202e63c667e73216a306b3dd45ff25c364a4f45))
* More text attributes on Windows ([#676](https://github.com/AccessKit/accesskit/issues/676)) ([259e832](https://github.com/AccessKit/accesskit/commit/259e83271371007d7564588939c6aafd0b7b3ac0))
* Multiple tree support ([#655](https://github.com/AccessKit/accesskit/issues/655)) ([c16eb80](https://github.com/AccessKit/accesskit/commit/c16eb80c49d14f39e8a7defb1694bf22b8ceed6f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.22.0 to 0.23.0
    * accesskit_consumer bumped from 0.32.0 to 0.33.0

## [0.30.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.29.2...accesskit_windows-v0.30.0) (2025-12-23)


### ⚠ BREAKING CHANGES

* Drop deprecated roles ([#642](https://github.com/AccessKit/accesskit/issues/642))
* Infrastructure for supporting text formatting changes ([#626](https://github.com/AccessKit/accesskit/issues/626))

### Features

* Add GridCell role ([#643](https://github.com/AccessKit/accesskit/issues/643)) ([1e5abca](https://github.com/AccessKit/accesskit/commit/1e5abca737d1ee942c0804fec2c06d3cb08faa94))
* Add simple text attributes on Windows ([#659](https://github.com/AccessKit/accesskit/issues/659)) ([f2bfa63](https://github.com/AccessKit/accesskit/commit/f2bfa6322c737be695438d728fdd2d9451b3b7e9))
* Implement BrailleLabel and BrailleRoleDescription roles ([#638](https://github.com/AccessKit/accesskit/issues/638)) ([0fdcebb](https://github.com/AccessKit/accesskit/commit/0fdcebb55e308e039ec99fbc31e94e8087a69f2d))
* Implement the UIA `Culture` property ([#631](https://github.com/AccessKit/accesskit/issues/631)) ([06fc934](https://github.com/AccessKit/accesskit/commit/06fc934d96e4ca0f0610edcbe9add3abec3486cd))
* Rename HeaderAsNonLandmark to SectionHeader, FooterAsNonLandmark to SectionFooter ([#635](https://github.com/AccessKit/accesskit/issues/635)) ([dd69526](https://github.com/AccessKit/accesskit/commit/dd695260877cb862ece4c2b7d8607308f5fed608))


### Bug Fixes

* Cast font size to `f64` on Windows ([#664](https://github.com/AccessKit/accesskit/issues/664)) ([a45124d](https://github.com/AccessKit/accesskit/commit/a45124d6824b31fe720cc711ca2646017f63f7f1))
* Expose dialogs in the platform adapters ([1576a94](https://github.com/AccessKit/accesskit/commit/1576a940ec3e9066fc5e181419dae6899bf2db46))
* expose more node data to UIA in aria_properties ([fa15c84](https://github.com/AccessKit/accesskit/commit/fa15c84fecbe97da889ec75d26fae98fce93b500))
* Extend AriaRoleProperty in Windows adapter ([#648](https://github.com/AccessKit/accesskit/issues/648)) ([4b8e01a](https://github.com/AccessKit/accesskit/commit/4b8e01a4c5c4373d8a4ce9721418c719cfe5a7a0))
* Upgrade hashbrown, indexmap ([#653](https://github.com/AccessKit/accesskit/issues/653)) ([fd73a85](https://github.com/AccessKit/accesskit/commit/fd73a85e57a6591a449b0a1880025fb940873c66))


### Code Refactoring

* Drop deprecated roles ([#642](https://github.com/AccessKit/accesskit/issues/642)) ([4d46c27](https://github.com/AccessKit/accesskit/commit/4d46c2740631c5fe4f057707b949d12b26931d0b))
* Infrastructure for supporting text formatting changes ([#626](https://github.com/AccessKit/accesskit/issues/626)) ([ea23ec4](https://github.com/AccessKit/accesskit/commit/ea23ec424c7dbb8841e03d71b6a15b74264850a9))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.21.1 to 0.22.0
    * accesskit_consumer bumped from 0.31.0 to 0.32.0

## [0.29.2](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.29.1...accesskit_windows-v0.29.2) (2025-10-20)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.30.1 to 0.31.0

## [0.29.1](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.29.0...accesskit_windows-v0.29.1) (2025-10-02)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.21.0 to 0.21.1
    * accesskit_consumer bumped from 0.30.0 to 0.30.1

## [0.29.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.28.0...accesskit_windows-v0.29.0) (2025-07-16)


### ⚠ BREAKING CHANGES

* Implement refactored `ScrollIntoView` action across desktop platforms ([#594](https://github.com/AccessKit/accesskit/issues/594))

### Features

* Implement refactored `ScrollIntoView` action across desktop platforms ([#594](https://github.com/AccessKit/accesskit/issues/594)) ([1d9b74c](https://github.com/AccessKit/accesskit/commit/1d9b74c057051509b3ffbf63afeacfb16f544ff8))
* Let parents declare actions supported on their children ([#593](https://github.com/AccessKit/accesskit/issues/593)) ([70b534b](https://github.com/AccessKit/accesskit/commit/70b534bed168a84b84cc35199588aa8ab784fb43))


### Bug Fixes

* Panic if the subclassing adapter is double-instantiated ([#596](https://github.com/AccessKit/accesskit/issues/596)) ([0598573](https://github.com/AccessKit/accesskit/commit/05985732f0611a9ac96eb458e586c6dd3c1a131d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.20.0 to 0.21.0
    * accesskit_consumer bumped from 0.29.0 to 0.30.0

## [0.28.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.27.0...accesskit_windows-v0.28.0) (2025-06-26)


### ⚠ BREAKING CHANGES

* Force a semver-breaking release ([#589](https://github.com/AccessKit/accesskit/issues/589))

### Bug Fixes

* Force a semver-breaking release ([#589](https://github.com/AccessKit/accesskit/issues/589)) ([2887cdd](https://github.com/AccessKit/accesskit/commit/2887cddde817ba3851688068d8d10de5cef7c624))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.19.0 to 0.20.0
    * accesskit_consumer bumped from 0.28.0 to 0.29.0

## [0.27.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.26.0...accesskit_windows-v0.27.0) (2025-05-06)


### ⚠ BREAKING CHANGES

* Drop redundant `HasPopup::True` ([#550](https://github.com/AccessKit/accesskit/issues/550))

### Bug Fixes

* Expose tabs in the platform adapters ([341a11b](https://github.com/AccessKit/accesskit/commit/341a11bca2c8a29682c11ddcfe91fa58776ea11d))
* Update windows-rs to 0.61 ([#541](https://github.com/AccessKit/accesskit/issues/541)) ([2f86c45](https://github.com/AccessKit/accesskit/commit/2f86c453a776956ca36c06c9689be22323646421))


### Code Refactoring

* Drop redundant `HasPopup::True` ([#550](https://github.com/AccessKit/accesskit/issues/550)) ([56abf17](https://github.com/AccessKit/accesskit/commit/56abf17356e4c7f13f64aaeaca6a63c8f7ede553))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.18.0 to 0.19.0
    * accesskit_consumer bumped from 0.27.0 to 0.28.0

## [0.26.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.25.0...accesskit_windows-v0.26.0) (2025-03-17)


### ⚠ BREAKING CHANGES

* Panic if the window is visible when the adapter is created, for adapters where this is a problem ([#529](https://github.com/AccessKit/accesskit/issues/529))

### Bug Fixes

* Eliminate the dependency on `paste` ([#528](https://github.com/AccessKit/accesskit/issues/528)) ([4aef05d](https://github.com/AccessKit/accesskit/commit/4aef05d0b34b434c0f0ce2e7583adef3e73bda4d))
* Panic if the window is visible when the adapter is created, for adapters where this is a problem ([#529](https://github.com/AccessKit/accesskit/issues/529)) ([c43c37b](https://github.com/AccessKit/accesskit/commit/c43c37ba2502656fcae4fd726b9b7db0bb520f31))

## [0.25.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.24.1...accesskit_windows-v0.25.0) (2025-03-06)


### ⚠ BREAKING CHANGES

* Optimize simple string getters ([#493](https://github.com/AccessKit/accesskit/issues/493))
* Drop `Tree::app_name` ([#492](https://github.com/AccessKit/accesskit/issues/492))

### Features

* Expose the `is_required` property ([#497](https://github.com/AccessKit/accesskit/issues/497)) ([46ed99b](https://github.com/AccessKit/accesskit/commit/46ed99bb958ddb32cbf1bee2fcfb7b328bcbe0ab))


### Bug Fixes

* Add list box support to the platform adapters ([6c622cf](https://github.com/AccessKit/accesskit/commit/6c622cff4c0c989d9a5f16b775adff9cdacf534c))
* Derive `Debug` for adapters ([#513](https://github.com/AccessKit/accesskit/issues/513)) ([753d904](https://github.com/AccessKit/accesskit/commit/753d90473cf57682568c7a17c82474c8e5d00b25))
* Expose password input fields on Windows ([#516](https://github.com/AccessKit/accesskit/issues/516)) ([19514da](https://github.com/AccessKit/accesskit/commit/19514dabc40bcfc01bee1b1efa77355ec5b0822b))
* Fix some clippy warnings ([#509](https://github.com/AccessKit/accesskit/issues/509)) ([579b9c1](https://github.com/AccessKit/accesskit/commit/579b9c12dd8abc44ecab41fa3c326a1d8999871d))
* Optimize dynamic string building ([#491](https://github.com/AccessKit/accesskit/issues/491)) ([a86901d](https://github.com/AccessKit/accesskit/commit/a86901ddea5d5ba72ab237e98b53d6adcc6087bb))
* Remove unnecessary explicit lifetimes ([#488](https://github.com/AccessKit/accesskit/issues/488)) ([d2bcd6d](https://github.com/AccessKit/accesskit/commit/d2bcd6d3048d23df4e132bee6171eb247b2dc2c8))


### Code Refactoring

* Drop `Tree::app_name` ([#492](https://github.com/AccessKit/accesskit/issues/492)) ([089794c](https://github.com/AccessKit/accesskit/commit/089794c8f74957e91a19ae3df508e2a892f39ebc))
* Optimize simple string getters ([#493](https://github.com/AccessKit/accesskit/issues/493)) ([484fd7c](https://github.com/AccessKit/accesskit/commit/484fd7cbfb778222369d3f57d31dd998f6fa80d8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.17.1 to 0.18.0
    * accesskit_consumer bumped from 0.26.0 to 0.27.0

## [0.24.1](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.24.0...accesskit_windows-v0.24.1) (2024-11-23)


### Bug Fixes

* Optimize use of hash tables in platform adapters ([#485](https://github.com/AccessKit/accesskit/issues/485)) ([f4f0bfb](https://github.com/AccessKit/accesskit/commit/f4f0bfbf21b8e22e80ab07deb432f9e7e16469ab))
* Tell docs.rs to only build accesskit_windows on Windows ([#483](https://github.com/AccessKit/accesskit/issues/483)) ([1fead29](https://github.com/AccessKit/accesskit/commit/1fead296ed012f83afb482021051ccc1d4946167))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.17.0 to 0.17.1
    * accesskit_consumer bumped from 0.25.0 to 0.26.0

## [0.24.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.23.2...accesskit_windows-v0.24.0) (2024-10-31)


### ⚠ BREAKING CHANGES

* Rename `name` to `label` and use `value` for label content ([#475](https://github.com/AccessKit/accesskit/issues/475))
* Rename `NodeBuilder` to `Node` and the old `Node` to `FrozenNode` ([#476](https://github.com/AccessKit/accesskit/issues/476))
* Rename `Role::InlineTextBox` to `TextRun` ([#473](https://github.com/AccessKit/accesskit/issues/473))
* Drop `DefaultActionVerb` ([#472](https://github.com/AccessKit/accesskit/issues/472))
* Make the core crate no-std ([#468](https://github.com/AccessKit/accesskit/issues/468))

### Features

* Make the core crate no-std ([#468](https://github.com/AccessKit/accesskit/issues/468)) ([2fa0d3f](https://github.com/AccessKit/accesskit/commit/2fa0d3f5b2b7ac11ef1751c133706f29e548bd6d))


### Code Refactoring

* Drop `DefaultActionVerb` ([#472](https://github.com/AccessKit/accesskit/issues/472)) ([ef3b003](https://github.com/AccessKit/accesskit/commit/ef3b0038224459094f650368412650bc3b69526b))
* Rename `name` to `label` and use `value` for label content ([#475](https://github.com/AccessKit/accesskit/issues/475)) ([e0053a5](https://github.com/AccessKit/accesskit/commit/e0053a5399929e8e0d4f07aa18de604ed8766ace))
* Rename `NodeBuilder` to `Node` and the old `Node` to `FrozenNode` ([#476](https://github.com/AccessKit/accesskit/issues/476)) ([7d8910e](https://github.com/AccessKit/accesskit/commit/7d8910e35f7bc0543724cc124941a3bd0304bcc0))
* Rename `Role::InlineTextBox` to `TextRun` ([#473](https://github.com/AccessKit/accesskit/issues/473)) ([29fa341](https://github.com/AccessKit/accesskit/commit/29fa34125a811bd3a0f9da579a9f35c9da90bf29))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.3 to 0.17.0
    * accesskit_consumer bumped from 0.24.3 to 0.25.0

## [0.23.1](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.23.0...accesskit_windows-v0.23.1) (2024-10-07)


### Bug Fixes

* Update minimum supported Rust version to 1.75 ([#457](https://github.com/AccessKit/accesskit/issues/457)) ([fc622fe](https://github.com/AccessKit/accesskit/commit/fc622fe7657c80a4eedad6f6cded11d2538b54d5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.1 to 0.16.2
    * accesskit_consumer bumped from 0.24.1 to 0.24.2

## [0.23.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.22.0...accesskit_windows-v0.23.0) (2024-09-24)


### ⚠ BREAKING CHANGES

* Update windows to 0.58 on accesskit_windows ([#453](https://github.com/AccessKit/accesskit/issues/453))

### Bug Fixes

* Update windows to 0.58 on accesskit_windows ([#453](https://github.com/AccessKit/accesskit/issues/453)) ([cda35e7](https://github.com/AccessKit/accesskit/commit/cda35e77a78f72386c2bfd88e9fd29000106f7e6))
* Use the new HWND type on accesskit_winit ([#453](https://github.com/AccessKit/accesskit/issues/453)) ([68a2462](https://github.com/AccessKit/accesskit/commit/68a24629381f0b18f6ed1ee008fe72ce9330092e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.0 to 0.16.1
    * accesskit_consumer bumped from 0.24.0 to 0.24.1

## [0.22.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.21.0...accesskit_windows-v0.22.0) (2024-06-29)


### ⚠ BREAKING CHANGES

* Rename the `StaticText` role to `Label` ([#434](https://github.com/AccessKit/accesskit/issues/434))

### Code Refactoring

* Rename the `StaticText` role to `Label` ([#434](https://github.com/AccessKit/accesskit/issues/434)) ([7086bc0](https://github.com/AccessKit/accesskit/commit/7086bc0fad446d3ed4a0fd5eff641a1e75f6c599))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.15.0 to 0.16.0
    * accesskit_consumer bumped from 0.23.0 to 0.24.0

## [0.21.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.20.0...accesskit_windows-v0.21.0) (2024-06-09)


### Features

* Add `author_id` property ([#424](https://github.com/AccessKit/accesskit/issues/424)) ([0d1c56f](https://github.com/AccessKit/accesskit/commit/0d1c56f0bdde58715e1c69f6015df600cb7cb8c1))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.14.0 to 0.15.0
    * accesskit_consumer bumped from 0.22.0 to 0.23.0

## [0.20.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.19.0...accesskit_windows-v0.20.0) (2024-05-27)


### Features

* Expose the `orientation` property ([#421](https://github.com/AccessKit/accesskit/issues/421)) ([590aada](https://github.com/AccessKit/accesskit/commit/590aada070dc812f9b8f171fb9e43ac984fad2a1))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.21.0 to 0.22.0

## [0.19.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.18.2...accesskit_windows-v0.19.0) (2024-05-26)


### Features

* Expose the `placeholder` property ([#417](https://github.com/AccessKit/accesskit/issues/417)) ([8f4a0a1](https://github.com/AccessKit/accesskit/commit/8f4a0a1c10f83fcc8580a37d8013fec2d110865b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.20.0 to 0.21.0

## [0.18.2](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.18.1...accesskit_windows-v0.18.2) (2024-05-13)


### Bug Fixes

* Fix platform adapters to support copy-on-write tree snapshots again ([#411](https://github.com/AccessKit/accesskit/issues/411)) ([d3a130a](https://github.com/AccessKit/accesskit/commit/d3a130a5ec8ae1d9edf0bf85a44f35f0e365242c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.19.1 to 0.20.0

## [0.18.1](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.18.0...accesskit_windows-v0.18.1) (2024-05-11)


### Bug Fixes

* Make the transition from placeholder to real tree more robust ([#405](https://github.com/AccessKit/accesskit/issues/405)) ([928e11d](https://github.com/AccessKit/accesskit/commit/928e11d00e7c60b3cafcc0ac623f6377b36f7ea7))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.19.0 to 0.19.1

## [0.18.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.17.0...accesskit_windows-v0.18.0) (2024-04-30)


### ⚠ BREAKING CHANGES

* Update winit to 0.30 ([#397](https://github.com/AccessKit/accesskit/issues/397))
* Clean up table roles and properties ([#393](https://github.com/AccessKit/accesskit/issues/393))
* Drop `NodeClassSet` ([#389](https://github.com/AccessKit/accesskit/issues/389))
* Rename `Checked` to `Toggled`; drop `ToggleButton` role ([#388](https://github.com/AccessKit/accesskit/issues/388))

### Features

* Expose the class name property ([#385](https://github.com/AccessKit/accesskit/issues/385)) ([53dcf2a](https://github.com/AccessKit/accesskit/commit/53dcf2ae47546273590c46a9b31b708aa1409837))
* Implement the `description` property ([#382](https://github.com/AccessKit/accesskit/issues/382)) ([d49f406](https://github.com/AccessKit/accesskit/commit/d49f40660b5dc23ed074cd72a91e511b130756ae))


### Bug Fixes

* Increase minimum supported Rust version to `1.70` ([#396](https://github.com/AccessKit/accesskit/issues/396)) ([a8398b8](https://github.com/AccessKit/accesskit/commit/a8398b847aa003de91042ac45e33126fc2cae053))
* Update winit to 0.30 ([#397](https://github.com/AccessKit/accesskit/issues/397)) ([de93be3](https://github.com/AccessKit/accesskit/commit/de93be387c03a438fbf598670207e578686e6bcf))


### Code Refactoring

* Clean up table roles and properties ([#393](https://github.com/AccessKit/accesskit/issues/393)) ([e34dad9](https://github.com/AccessKit/accesskit/commit/e34dad94448a5321ece9def3f2e054aa5f62dd79))
* Drop `NodeClassSet` ([#389](https://github.com/AccessKit/accesskit/issues/389)) ([1b153ed](https://github.com/AccessKit/accesskit/commit/1b153ed51f8421cdba2dc98beca2e8f5f8c781bc))
* Rename `Checked` to `Toggled`; drop `ToggleButton` role ([#388](https://github.com/AccessKit/accesskit/issues/388)) ([6bc040b](https://github.com/AccessKit/accesskit/commit/6bc040b7cf75cdbd6a019cc380d8dbce804b3c81))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.13.0 to 0.14.0
    * accesskit_consumer bumped from 0.18.0 to 0.19.0

## [0.17.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.16.4...accesskit_windows-v0.17.0) (2024-04-14)


### ⚠ BREAKING CHANGES

* New approach to lazy initialization ([#375](https://github.com/AccessKit/accesskit/issues/375))

### Code Refactoring

* New approach to lazy initialization ([#375](https://github.com/AccessKit/accesskit/issues/375)) ([9baebdc](https://github.com/AccessKit/accesskit/commit/9baebdceed7300389b6768815d7ae48f1ce401e4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.12.3 to 0.13.0
    * accesskit_consumer bumped from 0.17.1 to 0.18.0

## [0.16.4](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.16.3...accesskit_windows-v0.16.4) (2024-03-18)


### Bug Fixes

* Update `windows` to `0.54` ([#373](https://github.com/AccessKit/accesskit/issues/373)) ([50f112f](https://github.com/AccessKit/accesskit/commit/50f112f0085a03f0180f86915d2ac4e5845f6b63))

## [0.16.2](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.16.1...accesskit_windows-v0.16.2) (2024-02-25)


### Bug Fixes

* Bump windows-rs dependency to 0.52 ([#359](https://github.com/AccessKit/accesskit/issues/359)) ([69d74f6](https://github.com/AccessKit/accesskit/commit/69d74f6947922b76d4aee683eb53d8fbdd2259f2))

## [0.16.1](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.16.0...accesskit_windows-v0.16.1) (2024-02-24)


### Bug Fixes

* Avoid `unnecessary_literal_unwrap` clippy lint ([#346](https://github.com/AccessKit/accesskit/issues/346)) ([59a6eae](https://github.com/AccessKit/accesskit/commit/59a6eae40ba35069b90d4cc0b765d838f8477c66))

## [0.16.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.15.1...accesskit_windows-v0.16.0) (2024-01-03)


### Features

* Support custom role descriptions ([#316](https://github.com/AccessKit/accesskit/issues/316)) ([c8d1a56](https://github.com/AccessKit/accesskit/commit/c8d1a5638fa6c33adfa059815c04f7e043c56026))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.12.1 to 0.12.2
    * accesskit_consumer bumped from 0.16.1 to 0.17.0

## [0.15.1](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.15.0...accesskit_windows-v0.15.1) (2023-11-04)


### Bug Fixes

* Make the test focus event handler thread-safe again ([#306](https://github.com/AccessKit/accesskit/issues/306)) ([bb479c6](https://github.com/AccessKit/accesskit/commit/bb479c69bddff77a1102549a7ff6ea5c7155d90d))
* Make the Windows subclassing test more robust ([#308](https://github.com/AccessKit/accesskit/issues/308)) ([0078f79](https://github.com/AccessKit/accesskit/commit/0078f7997f11a3bb1f3c33dc7a91f427b1a60db8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.12.0 to 0.12.1
    * accesskit_consumer bumped from 0.16.0 to 0.16.1

## [0.15.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.14.3...accesskit_windows-v0.15.0) (2023-09-27)


### ⚠ BREAKING CHANGES

* Allow providing app_name, toolkit_name and toolkit_version in Tree, remove parameters from unix adapter constructor ([#291](https://github.com/AccessKit/accesskit/issues/291))
* Make `ActionHandler::do_action` take `&mut self` ([#296](https://github.com/AccessKit/accesskit/issues/296))
* Clean up roles and properties ([#289](https://github.com/AccessKit/accesskit/issues/289))
* Decouple in-tree focus from host window/view focus ([#278](https://github.com/AccessKit/accesskit/issues/278))
* Switch to simple unsigned 64-bit integer for node IDs ([#276](https://github.com/AccessKit/accesskit/issues/276))

### Features

* Add role for terminals ([#282](https://github.com/AccessKit/accesskit/issues/282)) ([ddbef37](https://github.com/AccessKit/accesskit/commit/ddbef37158b57f56217317b480e40d58f83a9c24))
* Allow providing app_name, toolkit_name and toolkit_version in Tree, remove parameters from unix adapter constructor ([#291](https://github.com/AccessKit/accesskit/issues/291)) ([5313860](https://github.com/AccessKit/accesskit/commit/531386023257150f49b5e4be942f359855fb7cb6))


### Bug Fixes

* Support text fields without a value property ([#274](https://github.com/AccessKit/accesskit/issues/274)) ([5ae557b](https://github.com/AccessKit/accesskit/commit/5ae557b40d395b4a9966a90a2d80e7d97ad50bf9))
* Use common filters across platform adapters ([#287](https://github.com/AccessKit/accesskit/issues/287)) ([09c1204](https://github.com/AccessKit/accesskit/commit/09c12045ff4ccdb22f0cf643077a27465013572d))


### Code Refactoring

* Clean up roles and properties ([#289](https://github.com/AccessKit/accesskit/issues/289)) ([4fc9c55](https://github.com/AccessKit/accesskit/commit/4fc9c55c91812472593923d93ff89d75ff305ee4))
* Decouple in-tree focus from host window/view focus ([#278](https://github.com/AccessKit/accesskit/issues/278)) ([d360d20](https://github.com/AccessKit/accesskit/commit/d360d20cf951e7643b81a5303006c9f7daa5bd56))
* Make `ActionHandler::do_action` take `&mut self` ([#296](https://github.com/AccessKit/accesskit/issues/296)) ([4fc7846](https://github.com/AccessKit/accesskit/commit/4fc7846d732d61fb45c023060ebab96801a0053e))
* Switch to simple unsigned 64-bit integer for node IDs ([#276](https://github.com/AccessKit/accesskit/issues/276)) ([3eadd48](https://github.com/AccessKit/accesskit/commit/3eadd48ec47854faa94a94ebf910ec08f514642f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.11.2 to 0.12.0
    * accesskit_consumer bumped from 0.15.2 to 0.16.0

## [0.14.3](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.14.2...accesskit_windows-v0.14.3) (2023-08-08)


### Bug Fixes

* Update windows crate to v0.48 ([#257](https://github.com/AccessKit/accesskit/issues/257)) ([cc703ed](https://github.com/AccessKit/accesskit/commit/cc703ed33d535aa1803e423a53beff9354b5b0df))

## [0.14.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.13.3...accesskit_windows-v0.14.0) (2023-03-30)


### ⚠ BREAKING CHANGES

* Force a semver-breaking version bump in downstream crates ([#234](https://github.com/AccessKit/accesskit/issues/234))

### Bug Fixes

* Force a semver-breaking version bump in downstream crates ([#234](https://github.com/AccessKit/accesskit/issues/234)) ([773389b](https://github.com/AccessKit/accesskit/commit/773389bff857fa18edf15de426e029251fc34591))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.14.2 to 0.15.0

## [0.13.2](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.13.1...accesskit_windows-v0.13.2) (2023-02-23)


### Bug Fixes

* Fix Windows 32-bit build errors ([#223](https://github.com/AccessKit/accesskit/issues/223)) ([41f28b6](https://github.com/AccessKit/accesskit/commit/41f28b670ac457b2d067bbc4ba40aa0fc8842e4d))

## [0.13.1](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.13.0...accesskit_windows-v0.13.1) (2023-02-20)


### Bug Fixes

* Update windows-rs to 0.44 ([#220](https://github.com/AccessKit/accesskit/issues/220)) ([a6b0a12](https://github.com/AccessKit/accesskit/commit/a6b0a124e7511e37760d769b517fd5fc9050160b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.10.0 to 0.10.1
    * accesskit_consumer bumped from 0.14.0 to 0.14.1

## [0.13.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.12.0...accesskit_windows-v0.13.0) (2023-02-12)


### ⚠ BREAKING CHANGES

* Move thread synchronization into platform adapters; drop parking_lot ([#212](https://github.com/AccessKit/accesskit/issues/212))

### Code Refactoring

* Move thread synchronization into platform adapters; drop parking_lot ([#212](https://github.com/AccessKit/accesskit/issues/212)) ([5df52e5](https://github.com/AccessKit/accesskit/commit/5df52e5545faddf6a51905409013c2f5be23981e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.9.0 to 0.10.0
    * accesskit_consumer bumped from 0.13.0 to 0.14.0

## [0.12.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.11.0...accesskit_windows-v0.12.0) (2023-02-05)


### ⚠ BREAKING CHANGES

* Make `Node` opaque and optimize it for size ([#205](https://github.com/AccessKit/accesskit/issues/205))

### Code Refactoring

* Make `Node` opaque and optimize it for size ([#205](https://github.com/AccessKit/accesskit/issues/205)) ([4811152](https://github.com/AccessKit/accesskit/commit/48111521439b76c1a8687418a4b20f9b705eac6d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.8.1 to 0.9.0
    * accesskit_consumer bumped from 0.12.1 to 0.13.0

## [0.11.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.10.4...accesskit_windows-v0.11.0) (2023-02-02)


### ⚠ BREAKING CHANGES

* Update winit to 0.28 ([#207](https://github.com/AccessKit/accesskit/issues/207))

### Miscellaneous Chores

* Update winit to 0.28 ([#207](https://github.com/AccessKit/accesskit/issues/207)) ([3ff0cf5](https://github.com/AccessKit/accesskit/commit/3ff0cf59f982af504499142a3804f7aeeb4defe0))

## [0.10.2](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.10.1...accesskit_windows-v0.10.2) (2022-12-17)


### Bug Fixes

* Correct broken UIA method implementation that was incompatible with Windows 11 ATs ([#193](https://github.com/AccessKit/accesskit/issues/193)) ([3c527c7](https://github.com/AccessKit/accesskit/commit/3c527c76cb4139402d2b5550d2eb1ad12e07ebe5))
* More reliable handling of the edge case for wrapped lines ([#192](https://github.com/AccessKit/accesskit/issues/192)) ([c626d2c](https://github.com/AccessKit/accesskit/commit/c626d2c3028085b076ada7dd31242cf3ca3c0f08))
* Provide fallback property implementations for the window root ([#194](https://github.com/AccessKit/accesskit/issues/194)) ([f3d30b9](https://github.com/AccessKit/accesskit/commit/f3d30b9ba2f66e08fb7f78c304ab8a9e83e1aeca))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.10.0 to 0.11.0

## [0.10.0](https://github.com/AccessKit/accesskit/compare/accesskit_windows-v0.9.3...accesskit_windows-v0.10.0) (2022-11-29)


### ⚠ BREAKING CHANGES

* Move lazy initialization from the core platform adapter to the caller ([#179](https://github.com/AccessKit/accesskit/issues/179))

### Code Refactoring

* Move lazy initialization from the core platform adapter to the caller ([#179](https://github.com/AccessKit/accesskit/issues/179)) ([f35c941](https://github.com/AccessKit/accesskit/commit/f35c941f395f3162db376a69cfaaaf770d376267))

### [0.9.3](https://www.github.com/AccessKit/accesskit/compare/accesskit_windows-v0.9.2...accesskit_windows-v0.9.3) (2022-11-25)


### Bug Fixes

* Gracefully handle nodes that only support text ranges some of the time ([#169](https://www.github.com/AccessKit/accesskit/issues/169)) ([1f50df6](https://www.github.com/AccessKit/accesskit/commit/1f50df6820b9d23fe2e579f043f4981acf285de2))
* **platforms/windows:** Change default minimum and maximum for range value pattern ([#166](https://www.github.com/AccessKit/accesskit/issues/166)) ([176ec58](https://www.github.com/AccessKit/accesskit/commit/176ec5853ca127b1e12f9a992b75478177a4acce))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.9.0 to 0.9.1

### [0.9.2](https://www.github.com/AccessKit/accesskit/compare/accesskit_windows-v0.9.1...accesskit_windows-v0.9.2) (2022-11-24)


### Bug Fixes

* **platforms/windows:** Re-export more types from windows-rs ([#162](https://www.github.com/AccessKit/accesskit/issues/162)) ([eed692b](https://www.github.com/AccessKit/accesskit/commit/eed692b27407e1ddd5f200464f0e2d52a272b958))

### [0.9.1](https://www.github.com/AccessKit/accesskit/compare/accesskit_windows-v0.9.0...accesskit_windows-v0.9.1) (2022-11-23)


### Bug Fixes

* **platforms/windows:** Re-export the windows-rs HWND type ([#159](https://www.github.com/AccessKit/accesskit/issues/159)) ([389187a](https://www.github.com/AccessKit/accesskit/commit/389187ac5e96895ed1763d14d315d2f8f4256460))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.8.0 to 0.9.0

## [0.9.0](https://www.github.com/AccessKit/accesskit/compare/accesskit_windows-v0.8.0...accesskit_windows-v0.9.0) (2022-11-17)


### ⚠ BREAKING CHANGES

* **consumer:** Eliminate the dependency on `im` due to licensing (#153)

### Code Refactoring

* **consumer:** Eliminate the dependency on `im` due to licensing ([#153](https://www.github.com/AccessKit/accesskit/issues/153)) ([b4c4cb5](https://www.github.com/AccessKit/accesskit/commit/b4c4cb5713d4833d8ee7979e4f4e39c7e96a3ed4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.7.0 to 0.8.0
    * accesskit_consumer bumped from 0.7.1 to 0.8.0

## [0.8.0](https://www.github.com/AccessKit/accesskit/compare/accesskit_windows-v0.7.0...accesskit_windows-v0.8.0) (2022-11-12)


### ⚠ BREAKING CHANGES

* **platforms/windows:** Update to windows-rs 0.42.0 (#148)

### Bug Fixes

* **consumer, platforms/windows, platforms/winit:** Update to parking_lot 0.12.1 ([#146](https://www.github.com/AccessKit/accesskit/issues/146)) ([6772855](https://www.github.com/AccessKit/accesskit/commit/6772855a7b540fd728faad15d8d208b05c1bbd8a))
* **platforms/windows:** Update to windows-rs 0.42.0 ([#148](https://www.github.com/AccessKit/accesskit/issues/148)) ([70d1a89](https://www.github.com/AccessKit/accesskit/commit/70d1a89f51fd6c3a32b7192d9d7f3937db09d196))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit_consumer bumped from 0.7.0 to 0.7.1

## [0.7.0](https://www.github.com/AccessKit/accesskit/compare/accesskit_windows-v0.6.1...accesskit_windows-v0.7.0) (2022-11-11)


### ⚠ BREAKING CHANGES

* Text range support (#145)
* Drop the `ignored` field and implement generic filtered tree traversal (#143)

### Features

* Text range support ([#145](https://www.github.com/AccessKit/accesskit/issues/145)) ([455e6f7](https://www.github.com/AccessKit/accesskit/commit/455e6f73bc058644d299c06eeeda9cc4cbe8844f))


### Code Refactoring

* Drop the `ignored` field and implement generic filtered tree traversal ([#143](https://www.github.com/AccessKit/accesskit/issues/143)) ([a4befe6](https://www.github.com/AccessKit/accesskit/commit/a4befe6e8a5afbe4a52dfd09eb87fdf2078d6c1d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.6.1 to 0.7.0
    * accesskit_consumer bumped from 0.6.1 to 0.7.0

### [0.6.1](https://www.github.com/AccessKit/accesskit/compare/accesskit_windows-v0.6.0...accesskit_windows-v0.6.1) (2022-10-10)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.6.0 to 0.6.1
    * accesskit_consumer bumped from 0.6.0 to 0.6.1

## [0.6.0](https://www.github.com/AccessKit/accesskit/compare/accesskit_windows-v0.5.1...accesskit_windows-v0.6.0) (2022-10-09)


### ⚠ BREAKING CHANGES

* **consumer:** Optimize tree access and change handling (#134)
* Wrap `TreeUpdate` nodes in `Arc` (#135)
* **consumer:** Make `Node::data` private to the crate (#137)
* Store node ID in `TreeUpdate`, not `accesskit::Node` (#132)

### Code Refactoring

* **consumer:** Make `Node::data` private to the crate ([#137](https://www.github.com/AccessKit/accesskit/issues/137)) ([adb372d](https://www.github.com/AccessKit/accesskit/commit/adb372dda78d183c7189966e3bbc2d3780070513))
* **consumer:** Optimize tree access and change handling ([#134](https://www.github.com/AccessKit/accesskit/issues/134)) ([765ab74](https://www.github.com/AccessKit/accesskit/commit/765ab74efcf10a3b3871dc901d28f3cf1ff6020c))
* Store node ID in `TreeUpdate`, not `accesskit::Node` ([#132](https://www.github.com/AccessKit/accesskit/issues/132)) ([0bb86dd](https://www.github.com/AccessKit/accesskit/commit/0bb86ddb298cb5a253a91f07be0bad8b84b2fda3))
* Wrap `TreeUpdate` nodes in `Arc` ([#135](https://www.github.com/AccessKit/accesskit/issues/135)) ([907bc18](https://www.github.com/AccessKit/accesskit/commit/907bc1820b80d95833b6c5c3acaa2a8a4e93a6c2))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.5.1 to 0.6.0
    * accesskit_consumer bumped from 0.5.1 to 0.6.0

### [0.5.1](https://www.github.com/AccessKit/accesskit/compare/accesskit_windows-v0.5.0...accesskit_windows-v0.5.1) (2022-10-03)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.5.0 to 0.5.1
    * accesskit_consumer bumped from 0.5.0 to 0.5.1

## [0.5.0](https://www.github.com/AccessKit/accesskit/compare/accesskit_windows-v0.4.0...accesskit_windows-v0.5.0) (2022-09-23)


### ⚠ BREAKING CHANGES

* Basic live regions (#128)
* **platforms/windows:** Bump windows-rs dependency (#126)

### Features

* Basic live regions ([#128](https://www.github.com/AccessKit/accesskit/issues/128)) ([03d745b](https://www.github.com/AccessKit/accesskit/commit/03d745b891147175bde2693cc10b96a2f6e31f39))


### Miscellaneous Chores

* **platforms/windows:** Bump windows-rs dependency ([#126](https://www.github.com/AccessKit/accesskit/issues/126)) ([472a75e](https://www.github.com/AccessKit/accesskit/commit/472a75e4214b90396f3282f247df08100ed8362d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.4.0 to 0.5.0
    * accesskit_consumer bumped from 0.4.0 to 0.5.0

## [0.4.0](https://www.github.com/AccessKit/accesskit/compare/accesskit_windows-v0.3.0...accesskit_windows-v0.4.0) (2022-07-22)


### ⚠ BREAKING CHANGES

* String indices are always in UTF-8 code units (#114)
* **platforms/windows:** Refactor window subclassing to avoid lifetime issue (#120)
* **platforms/windows:** Simplify the adapter API by always boxing the tree source (#119)
* Drop unused tree IDs (#113)
* **platforms/windows:** Migrate to windows-rs 0.37 (#109)
* Switch to NonZeroU128 for NodeIDs (#99)

### Features

* **platforms/windows:** Win32 subclassing support ([#118](https://www.github.com/AccessKit/accesskit/issues/118)) ([60c69b7](https://www.github.com/AccessKit/accesskit/commit/60c69b7b8a18ca8db62a84495b9e71a6e8140204))
* **platforms/winit:** New winit adapter ([#121](https://www.github.com/AccessKit/accesskit/issues/121)) ([fdc274e](https://www.github.com/AccessKit/accesskit/commit/fdc274e7d3a901873d2ad0c7a4824a19111787ef))


### Bug Fixes

* **consumer, platforms/windows:** Resolve new clippy warning ([#100](https://www.github.com/AccessKit/accesskit/issues/100)) ([e8cd95c](https://www.github.com/AccessKit/accesskit/commit/e8cd95c3741b39b77e4ddc8ce82efdc20f93f096))
* Migrate to 2021 edition ([#115](https://www.github.com/AccessKit/accesskit/issues/115)) ([f2333c8](https://www.github.com/AccessKit/accesskit/commit/f2333c8ce17d46aab6fc190338ab4cfcf8569f9e))
* **platforms/windows:** Print usage text to the terminal from the Windows example ([#103](https://www.github.com/AccessKit/accesskit/issues/103)) ([7fba3ce](https://www.github.com/AccessKit/accesskit/commit/7fba3ce55345d7787f08d2ae60d841dd13b27693))
* **platforms/windows:** Restore the optimization of the FragmentRoot method ([#116](https://www.github.com/AccessKit/accesskit/issues/116)) ([d48c31b](https://www.github.com/AccessKit/accesskit/commit/d48c31b41f35baebe59bb654b38dd48265062b14))
* Switch to NonZeroU128 for NodeIDs ([#99](https://www.github.com/AccessKit/accesskit/issues/99)) ([25a1a52](https://www.github.com/AccessKit/accesskit/commit/25a1a52c4562b163bfcc8c625a233c00a41aacf2))


### Miscellaneous Chores

* **platforms/windows:** Migrate to windows-rs 0.37 ([#109](https://www.github.com/AccessKit/accesskit/issues/109)) ([1065e11](https://www.github.com/AccessKit/accesskit/commit/1065e11421176a8abc37ef579cb47d973c968462))


### Code Refactoring

* Drop unused tree IDs ([#113](https://www.github.com/AccessKit/accesskit/issues/113)) ([ca60770](https://www.github.com/AccessKit/accesskit/commit/ca607702cee13c93fe538d2faec88e474261f7ab))
* **platforms/windows:** Refactor window subclassing to avoid lifetime issue ([#120](https://www.github.com/AccessKit/accesskit/issues/120)) ([37579aa](https://www.github.com/AccessKit/accesskit/commit/37579aa8dd0c019ffaf4eac1b0bf1f7a8c719323))
* **platforms/windows:** Simplify the adapter API by always boxing the tree source ([#119](https://www.github.com/AccessKit/accesskit/issues/119)) ([27d5c78](https://www.github.com/AccessKit/accesskit/commit/27d5c78afa0f8d1ae3b626265da8bccd3e5b09d1))
* String indices are always in UTF-8 code units ([#114](https://www.github.com/AccessKit/accesskit/issues/114)) ([386ca0a](https://www.github.com/AccessKit/accesskit/commit/386ca0a89c42fd201843f617b2fd6b6d1de77f59))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.3.0 to 0.4.0
    * accesskit_consumer bumped from 0.3.0 to 0.4.0

## [0.3.0](https://www.github.com/AccessKit/accesskit/compare/accesskit_windows-v0.2.0...accesskit_windows-v0.3.0) (2021-12-29)


### ⚠ BREAKING CHANGES

* Drop `TreeUpdate::clear` (#96)

### Code Refactoring

* Drop `TreeUpdate::clear` ([#96](https://www.github.com/AccessKit/accesskit/issues/96)) ([38f520b](https://www.github.com/AccessKit/accesskit/commit/38f520b960c6db7b3927b369aee206ee6bc5e8aa))



### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.2.0 to 0.3.0
    * accesskit_consumer bumped from 0.2.0 to 0.3.0

# Changelog

## [0.24.1](https://github.com/AccessKit/accesskit/compare/accesskit-v0.24.0...accesskit-v0.24.1) (2026-06-12)


### Bug Fixes

* Update pyo3 dependency to 0.29 ([#724](https://github.com/AccessKit/accesskit/issues/724)) ([c9725e2](https://github.com/AccessKit/accesskit/commit/c9725e2bc2094c68ba152c929fb31d4a09e44af5))

## [0.24.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.23.0...accesskit-v0.24.0) (2026-02-01)


### ⚠ BREAKING CHANGES

* Update minimum supported Rust version to 1.85 ([#683](https://github.com/AccessKit/accesskit/issues/683))

### Bug Fixes

* Update minimum supported Rust version to 1.85 ([#683](https://github.com/AccessKit/accesskit/issues/683)) ([f8c22cb](https://github.com/AccessKit/accesskit/commit/f8c22cbb53ce6b1859adeff3c0b7154ad9e1f25e))

## [0.23.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.22.0...accesskit-v0.23.0) (2026-01-15)


### ⚠ BREAKING CHANGES

* Store text decoration style and color in a struct ([#674](https://github.com/AccessKit/accesskit/issues/674))
* Multiple tree support ([#655](https://github.com/AccessKit/accesskit/issues/655))

### Features

* Multiple tree support ([#655](https://github.com/AccessKit/accesskit/issues/655)) ([c16eb80](https://github.com/AccessKit/accesskit/commit/c16eb80c49d14f39e8a7defb1694bf22b8ceed6f))


### Bug Fixes

* Implement the `active_descendant` property ([#675](https://github.com/AccessKit/accesskit/issues/675)) ([863755d](https://github.com/AccessKit/accesskit/commit/863755d6898c7006a5f49c3661a26cb62d837d1b))


### Code Refactoring

* Store text decoration style and color in a struct ([#674](https://github.com/AccessKit/accesskit/issues/674)) ([c2e8487](https://github.com/AccessKit/accesskit/commit/c2e8487d0a243da0d1c996887fb4c9a7e3beacc7))

## [0.22.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.21.1...accesskit-v0.22.0) (2025-12-23)


### ⚠ BREAKING CHANGES

* Drop the redundant "bold" flag ([#660](https://github.com/AccessKit/accesskit/issues/660))
* Use a proper `Color` struct for the color properties ([#663](https://github.com/AccessKit/accesskit/issues/663))
* Change font size and weight to `f32` ([#662](https://github.com/AccessKit/accesskit/issues/662))
* Drop deprecated roles ([#642](https://github.com/AccessKit/accesskit/issues/642))
* Infrastructure for supporting text formatting changes ([#626](https://github.com/AccessKit/accesskit/issues/626))

### Features

* Add GridCell role ([#643](https://github.com/AccessKit/accesskit/issues/643)) ([1e5abca](https://github.com/AccessKit/accesskit/commit/1e5abca737d1ee942c0804fec2c06d3cb08faa94))
* Implement BrailleLabel and BrailleRoleDescription roles ([#638](https://github.com/AccessKit/accesskit/issues/638)) ([0fdcebb](https://github.com/AccessKit/accesskit/commit/0fdcebb55e308e039ec99fbc31e94e8087a69f2d))
* Rename HeaderAsNonLandmark to SectionHeader, FooterAsNonLandmark to SectionFooter ([#635](https://github.com/AccessKit/accesskit/issues/635)) ([dd69526](https://github.com/AccessKit/accesskit/commit/dd695260877cb862ece4c2b7d8607308f5fed608))


### Bug Fixes

* Actually update schemars to version 1 ([#650](https://github.com/AccessKit/accesskit/issues/650)) ([329cadb](https://github.com/AccessKit/accesskit/commit/329cadba6647d9b8d694033affb19b8bd616aa8b))
* Update schemars dependency to version 1 ([#647](https://github.com/AccessKit/accesskit/issues/647)) ([0efca17](https://github.com/AccessKit/accesskit/commit/0efca176ef892b04edbdf19a12594f9cbcbe9ab3))


### Code Refactoring

* Change font size and weight to `f32` ([#662](https://github.com/AccessKit/accesskit/issues/662)) ([ae0ead8](https://github.com/AccessKit/accesskit/commit/ae0ead825298539f08140b3775f0fdbfc17acd13))
* Drop deprecated roles ([#642](https://github.com/AccessKit/accesskit/issues/642)) ([4d46c27](https://github.com/AccessKit/accesskit/commit/4d46c2740631c5fe4f057707b949d12b26931d0b))
* Drop the redundant "bold" flag ([#660](https://github.com/AccessKit/accesskit/issues/660)) ([5958b2a](https://github.com/AccessKit/accesskit/commit/5958b2a9a91ac0b5e6831380d07f107783201a8b))
* Infrastructure for supporting text formatting changes ([#626](https://github.com/AccessKit/accesskit/issues/626)) ([ea23ec4](https://github.com/AccessKit/accesskit/commit/ea23ec424c7dbb8841e03d71b6a15b74264850a9))
* Use a proper `Color` struct for the color properties ([#663](https://github.com/AccessKit/accesskit/issues/663)) ([922e882](https://github.com/AccessKit/accesskit/commit/922e882a8cd9aa372038fb9c66ff57a557e7888d))

## [0.21.1](https://github.com/AccessKit/accesskit/compare/accesskit-v0.21.0...accesskit-v0.21.1) (2025-10-02)


### Bug Fixes

* Fix clippy warnings introduced in 1.89 ([#606](https://github.com/AccessKit/accesskit/issues/606)) ([b2c07d6](https://github.com/AccessKit/accesskit/commit/b2c07d654a8ce6f01e61a79c91f2f9d5a96afdc9))
* Fix link in `ActionData::ScrollHint` documentation ([#598](https://github.com/AccessKit/accesskit/issues/598)) ([28b82e9](https://github.com/AccessKit/accesskit/commit/28b82e9941b10ac0643201b9b3989f0184346216))
* Update pyo3 to 0.26 ([#620](https://github.com/AccessKit/accesskit/issues/620)) ([61e9dcc](https://github.com/AccessKit/accesskit/commit/61e9dccb2c2e035463593aacbac7ae25573a9006))

## [0.21.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.20.0...accesskit-v0.21.0) (2025-07-16)


### ⚠ BREAKING CHANGES

* Implement refactored `ScrollIntoView` action across desktop platforms ([#594](https://github.com/AccessKit/accesskit/issues/594))

### Features

* Implement refactored `ScrollIntoView` action across desktop platforms ([#594](https://github.com/AccessKit/accesskit/issues/594)) ([1d9b74c](https://github.com/AccessKit/accesskit/commit/1d9b74c057051509b3ffbf63afeacfb16f544ff8))
* Let parents declare actions supported on their children ([#593](https://github.com/AccessKit/accesskit/issues/593)) ([70b534b](https://github.com/AccessKit/accesskit/commit/70b534bed168a84b84cc35199588aa8ab784fb43))

## [0.20.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.19.0...accesskit-v0.20.0) (2025-06-26)


### ⚠ BREAKING CHANGES

* Refactor actions for scrolling by discrete units ([#573](https://github.com/AccessKit/accesskit/issues/573))

### Bug Fixes

* impl `From<bool>` for `Toggled` ([#585](https://github.com/AccessKit/accesskit/issues/585)) ([d38776a](https://github.com/AccessKit/accesskit/commit/d38776a014744db849edbfc9e0a7d0378709ed4b))
* Resolve new clippy warning about using variables directly in format strings ([#590](https://github.com/AccessKit/accesskit/issues/590)) ([ccc62b7](https://github.com/AccessKit/accesskit/commit/ccc62b7f1dd32f0c372ba127a1e65c377048f670))


### Code Refactoring

* Refactor actions for scrolling by discrete units ([#573](https://github.com/AccessKit/accesskit/issues/573)) ([fad11a1](https://github.com/AccessKit/accesskit/commit/fad11a1b66340e7be6b2eb00dfd07004451a17eb))

## [0.19.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.18.0...accesskit-v0.19.0) (2025-05-06)


### ⚠ BREAKING CHANGES

* Drop redundant `HasPopup::True` ([#550](https://github.com/AccessKit/accesskit/issues/550))
* Drop unused `Node::is_linked` ([#545](https://github.com/AccessKit/accesskit/issues/545))
* Drop `FrozenNode` ([#496](https://github.com/AccessKit/accesskit/issues/496))

### Bug Fixes

* Improve `NodeId`'s debug representation ([#547](https://github.com/AccessKit/accesskit/issues/547)) ([a47bca1](https://github.com/AccessKit/accesskit/commit/a47bca1e376de7b0a22a7dfe6c23dedad315c449))
* Update pyo3 to 0.24 ([#544](https://github.com/AccessKit/accesskit/issues/544)) ([6338e45](https://github.com/AccessKit/accesskit/commit/6338e45097662bf39994e19a09054c20cb2ee782))


### Code Refactoring

* Drop `FrozenNode` ([#496](https://github.com/AccessKit/accesskit/issues/496)) ([f8c0d0a](https://github.com/AccessKit/accesskit/commit/f8c0d0a6fc9613cf1a2a6d8cfba11ebc892dfeb8))
* Drop redundant `HasPopup::True` ([#550](https://github.com/AccessKit/accesskit/issues/550)) ([56abf17](https://github.com/AccessKit/accesskit/commit/56abf17356e4c7f13f64aaeaca6a63c8f7ede553))
* Drop unused `Node::is_linked` ([#545](https://github.com/AccessKit/accesskit/issues/545)) ([3aab4ac](https://github.com/AccessKit/accesskit/commit/3aab4ac6f0193b8a06d7962f933582a4dbdf0c98))

## [0.18.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.17.1...accesskit-v0.18.0) (2025-03-06)


### ⚠ BREAKING CHANGES

* Drop `Tree::app_name` ([#492](https://github.com/AccessKit/accesskit/issues/492))

### Features

* Android adapter ([#500](https://github.com/AccessKit/accesskit/issues/500)) ([7e65ac7](https://github.com/AccessKit/accesskit/commit/7e65ac77d7e108ac5b9f3722f488a2fdf2e3b3e0))


### Bug Fixes

* Update pyo3 to 0.23 ([#512](https://github.com/AccessKit/accesskit/issues/512)) ([93d3a27](https://github.com/AccessKit/accesskit/commit/93d3a27ac4af60eef4a1faf26392a6f7ff69cf81))


### Code Refactoring

* Drop `Tree::app_name` ([#492](https://github.com/AccessKit/accesskit/issues/492)) ([089794c](https://github.com/AccessKit/accesskit/commit/089794c8f74957e91a19ae3df508e2a892f39ebc))

## [0.17.1](https://github.com/AccessKit/accesskit/compare/accesskit-v0.17.0...accesskit-v0.17.1) (2024-11-23)


### Bug Fixes

* Fix some broken links in the documentation ([#484](https://github.com/AccessKit/accesskit/issues/484)) ([0a51225](https://github.com/AccessKit/accesskit/commit/0a5122561c6f6aca5cf802464220056d763040f8))

## [0.17.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.16.3...accesskit-v0.17.0) (2024-10-31)


### ⚠ BREAKING CHANGES

* Drop the `is_hovered` property ([#479](https://github.com/AccessKit/accesskit/issues/479))
* Rename `name` to `label` and use `value` for label content ([#475](https://github.com/AccessKit/accesskit/issues/475))
* Rename `NodeBuilder` to `Node` and the old `Node` to `FrozenNode` ([#476](https://github.com/AccessKit/accesskit/issues/476))
* Rename `Role::InlineTextBox` to `TextRun` ([#473](https://github.com/AccessKit/accesskit/issues/473))
* Drop `DefaultActionVerb` ([#472](https://github.com/AccessKit/accesskit/issues/472))
* Make the core crate no-std ([#468](https://github.com/AccessKit/accesskit/issues/468))

### Features

* Make the core crate no-std ([#468](https://github.com/AccessKit/accesskit/issues/468)) ([2fa0d3f](https://github.com/AccessKit/accesskit/commit/2fa0d3f5b2b7ac11ef1751c133706f29e548bd6d))


### Bug Fixes

* Drop the `is_hovered` property ([#479](https://github.com/AccessKit/accesskit/issues/479)) ([95dfdb6](https://github.com/AccessKit/accesskit/commit/95dfdb6c88f7d705f6a7283cb8524168a9f542b2))


### Code Refactoring

* Drop `DefaultActionVerb` ([#472](https://github.com/AccessKit/accesskit/issues/472)) ([ef3b003](https://github.com/AccessKit/accesskit/commit/ef3b0038224459094f650368412650bc3b69526b))
* Rename `name` to `label` and use `value` for label content ([#475](https://github.com/AccessKit/accesskit/issues/475)) ([e0053a5](https://github.com/AccessKit/accesskit/commit/e0053a5399929e8e0d4f07aa18de604ed8766ace))
* Rename `NodeBuilder` to `Node` and the old `Node` to `FrozenNode` ([#476](https://github.com/AccessKit/accesskit/issues/476)) ([7d8910e](https://github.com/AccessKit/accesskit/commit/7d8910e35f7bc0543724cc124941a3bd0304bcc0))
* Rename `Role::InlineTextBox` to `TextRun` ([#473](https://github.com/AccessKit/accesskit/issues/473)) ([29fa341](https://github.com/AccessKit/accesskit/commit/29fa34125a811bd3a0f9da579a9f35c9da90bf29))

## [0.16.3](https://github.com/AccessKit/accesskit/compare/accesskit-v0.16.2...accesskit-v0.16.3) (2024-10-08)


### Bug Fixes

* Eliminate duplicate definitions ([#461](https://github.com/AccessKit/accesskit/issues/461)) ([59826d4](https://github.com/AccessKit/accesskit/commit/59826d4500ddfe880181f7087f9fe83ff2209fc4))

## [0.16.2](https://github.com/AccessKit/accesskit/compare/accesskit-v0.16.1...accesskit-v0.16.2) (2024-10-07)


### Bug Fixes

* Don't use a macro to generate Action debug helper function ([#459](https://github.com/AccessKit/accesskit/issues/459)) ([ed1fb73](https://github.com/AccessKit/accesskit/commit/ed1fb7370780c9dd15028cdfd13e2065642bf490))
* Update minimum supported Rust version to 1.75 ([#457](https://github.com/AccessKit/accesskit/issues/457)) ([fc622fe](https://github.com/AccessKit/accesskit/commit/fc622fe7657c80a4eedad6f6cded11d2538b54d5))

## [0.16.1](https://github.com/AccessKit/accesskit/compare/accesskit-v0.16.0...accesskit-v0.16.1) (2024-09-24)


### Bug Fixes

* Improve debug representation of `Node` and `NodeBuilder` ([#452](https://github.com/AccessKit/accesskit/issues/452)) ([119aa1d](https://github.com/AccessKit/accesskit/commit/119aa1dca8fe734112ecbd59568c876b336ccb6c))

## [0.16.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.15.0...accesskit-v0.16.0) (2024-06-29)


### ⚠ BREAKING CHANGES

* Optimize serialization and make it compatible with more data formats ([#437](https://github.com/AccessKit/accesskit/issues/437))
* Rename the `StaticText` role to `Label` ([#434](https://github.com/AccessKit/accesskit/issues/434))

### Code Refactoring

* Optimize serialization and make it compatible with more data formats ([#437](https://github.com/AccessKit/accesskit/issues/437)) ([5a80d3a](https://github.com/AccessKit/accesskit/commit/5a80d3ae46cfe85780d4900f4fa9f4feaba52053))
* Rename the `StaticText` role to `Label` ([#434](https://github.com/AccessKit/accesskit/issues/434)) ([7086bc0](https://github.com/AccessKit/accesskit/commit/7086bc0fad446d3ed4a0fd5eff641a1e75f6c599))

## [0.15.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.14.0...accesskit-v0.15.0) (2024-06-09)


### Features

* Add `author_id` property ([#424](https://github.com/AccessKit/accesskit/issues/424)) ([0d1c56f](https://github.com/AccessKit/accesskit/commit/0d1c56f0bdde58715e1c69f6015df600cb7cb8c1))


### Bug Fixes

* Add explicit cargo features for `enumn` and `pyo3` ([#425](https://github.com/AccessKit/accesskit/issues/425)) ([71ad45b](https://github.com/AccessKit/accesskit/commit/71ad45be1651409ee6918cf835b656e6b5e0fe2d))

## [0.14.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.13.0...accesskit-v0.14.0) (2024-04-30)


### ⚠ BREAKING CHANGES

* Clean up table roles and properties ([#393](https://github.com/AccessKit/accesskit/issues/393))
* Drop `SortDirection::Unsorted` ([#391](https://github.com/AccessKit/accesskit/issues/391))
* Rename `hierarchical_level` to `level` ([#390](https://github.com/AccessKit/accesskit/issues/390))
* Drop `NodeClassSet` ([#389](https://github.com/AccessKit/accesskit/issues/389))
* Rename `Checked` to `Toggled`; drop `ToggleButton` role ([#388](https://github.com/AccessKit/accesskit/issues/388))

### Features

* Add the `owns` relation ([#392](https://github.com/AccessKit/accesskit/issues/392)) ([fd668dd](https://github.com/AccessKit/accesskit/commit/fd668ddc4b64cb05ab3600972b3d3823a037f2d5))


### Bug Fixes

* Increase minimum supported Rust version to `1.70` ([#396](https://github.com/AccessKit/accesskit/issues/396)) ([a8398b8](https://github.com/AccessKit/accesskit/commit/a8398b847aa003de91042ac45e33126fc2cae053))


### Code Refactoring

* Clean up table roles and properties ([#393](https://github.com/AccessKit/accesskit/issues/393)) ([e34dad9](https://github.com/AccessKit/accesskit/commit/e34dad94448a5321ece9def3f2e054aa5f62dd79))
* Drop `NodeClassSet` ([#389](https://github.com/AccessKit/accesskit/issues/389)) ([1b153ed](https://github.com/AccessKit/accesskit/commit/1b153ed51f8421cdba2dc98beca2e8f5f8c781bc))
* Drop `SortDirection::Unsorted` ([#391](https://github.com/AccessKit/accesskit/issues/391)) ([b86f484](https://github.com/AccessKit/accesskit/commit/b86f484b7e6645e63362896b744a71ec758f810d))
* Rename `Checked` to `Toggled`; drop `ToggleButton` role ([#388](https://github.com/AccessKit/accesskit/issues/388)) ([6bc040b](https://github.com/AccessKit/accesskit/commit/6bc040b7cf75cdbd6a019cc380d8dbce804b3c81))
* Rename `hierarchical_level` to `level` ([#390](https://github.com/AccessKit/accesskit/issues/390)) ([2d61e01](https://github.com/AccessKit/accesskit/commit/2d61e01fffff1265b348c141715f6f9b6fe4081b))

## [0.13.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.12.3...accesskit-v0.13.0) (2024-04-14)


### ⚠ BREAKING CHANGES

* New approach to lazy initialization ([#375](https://github.com/AccessKit/accesskit/issues/375))

### Code Refactoring

* New approach to lazy initialization ([#375](https://github.com/AccessKit/accesskit/issues/375)) ([9baebdc](https://github.com/AccessKit/accesskit/commit/9baebdceed7300389b6768815d7ae48f1ce401e4))

## [0.12.3](https://github.com/AccessKit/accesskit/compare/accesskit-v0.12.2...accesskit-v0.12.3) (2024-03-07)


### Bug Fixes

* Derive `PartialOrd` and `Ord` on `NodeId` ([#363](https://github.com/AccessKit/accesskit/issues/363)) ([ce3bba1](https://github.com/AccessKit/accesskit/commit/ce3bba1e043d650c406d8814b4f33e9104199c8b))
* Make `NodeClassSet::new` const ([#368](https://github.com/AccessKit/accesskit/issues/368)) ([11d2968](https://github.com/AccessKit/accesskit/commit/11d2968464d50c3e3f55e9a872d0d454c19e7e51))

## [0.12.2](https://github.com/AccessKit/accesskit/compare/accesskit-v0.12.1...accesskit-v0.12.2) (2024-01-03)


### Bug Fixes

* Bump pyo3; add `rename_all` attribute to enums ([#330](https://github.com/AccessKit/accesskit/issues/330)) ([5a4c6f3](https://github.com/AccessKit/accesskit/commit/5a4c6f399837d67b066451a8fb4d43d03c8acb8b))
* Document the `role_description` node property ([#331](https://github.com/AccessKit/accesskit/issues/331)) ([936fa2c](https://github.com/AccessKit/accesskit/commit/936fa2c23190c5d7cd4eb880612295785a009721))

## [0.12.1](https://github.com/AccessKit/accesskit/compare/accesskit-v0.12.0...accesskit-v0.12.1) (2023-11-04)


### Bug Fixes

* Add missing semicolons when not returning anything ([#303](https://github.com/AccessKit/accesskit/issues/303)) ([38d4de1](https://github.com/AccessKit/accesskit/commit/38d4de1442247e701047d75122a9638a2ed99b1f))

## [0.12.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.11.2...accesskit-v0.12.0) (2023-09-27)


### ⚠ BREAKING CHANGES

* Allow providing app_name, toolkit_name and toolkit_version in Tree, remove parameters from unix adapter constructor ([#291](https://github.com/AccessKit/accesskit/issues/291))
* Make `ActionHandler::do_action` take `&mut self` ([#296](https://github.com/AccessKit/accesskit/issues/296))
* Clean up roles and properties ([#289](https://github.com/AccessKit/accesskit/issues/289))
* Drop next/previous focus properties ([#288](https://github.com/AccessKit/accesskit/issues/288))
* Drop `Tree::root_scroller` ([#279](https://github.com/AccessKit/accesskit/issues/279))
* Decouple in-tree focus from host window/view focus ([#278](https://github.com/AccessKit/accesskit/issues/278))
* Switch to simple unsigned 64-bit integer for node IDs ([#276](https://github.com/AccessKit/accesskit/issues/276))

### Features

* Add role for terminals ([#282](https://github.com/AccessKit/accesskit/issues/282)) ([ddbef37](https://github.com/AccessKit/accesskit/commit/ddbef37158b57f56217317b480e40d58f83a9c24))
* Allow providing app_name, toolkit_name and toolkit_version in Tree, remove parameters from unix adapter constructor ([#291](https://github.com/AccessKit/accesskit/issues/291)) ([5313860](https://github.com/AccessKit/accesskit/commit/531386023257150f49b5e4be942f359855fb7cb6))


### Bug Fixes

* Drop `Tree::root_scroller` ([#279](https://github.com/AccessKit/accesskit/issues/279)) ([fc6c4e0](https://github.com/AccessKit/accesskit/commit/fc6c4e0091d5b257a3869a468fca144a1453cebc))
* Drop next/previous focus properties ([#288](https://github.com/AccessKit/accesskit/issues/288)) ([d35c7c1](https://github.com/AccessKit/accesskit/commit/d35c7c149a650dfedf1b031c0668adad585659fa))
* Support the pyo3 crate in all public enums ([#270](https://github.com/AccessKit/accesskit/issues/270)) ([9b12d0c](https://github.com/AccessKit/accesskit/commit/9b12d0c3d828d4c847510b611d891872c4666984))


### Code Refactoring

* Clean up roles and properties ([#289](https://github.com/AccessKit/accesskit/issues/289)) ([4fc9c55](https://github.com/AccessKit/accesskit/commit/4fc9c55c91812472593923d93ff89d75ff305ee4))
* Decouple in-tree focus from host window/view focus ([#278](https://github.com/AccessKit/accesskit/issues/278)) ([d360d20](https://github.com/AccessKit/accesskit/commit/d360d20cf951e7643b81a5303006c9f7daa5bd56))
* Make `ActionHandler::do_action` take `&mut self` ([#296](https://github.com/AccessKit/accesskit/issues/296)) ([4fc7846](https://github.com/AccessKit/accesskit/commit/4fc7846d732d61fb45c023060ebab96801a0053e))
* Switch to simple unsigned 64-bit integer for node IDs ([#276](https://github.com/AccessKit/accesskit/issues/276)) ([3eadd48](https://github.com/AccessKit/accesskit/commit/3eadd48ec47854faa94a94ebf910ec08f514642f))

## [0.11.2](https://github.com/AccessKit/accesskit/compare/accesskit-v0.11.1...accesskit-v0.11.2) (2023-08-08)


### Bug Fixes

* Support the enumn crate in all public enums ([#264](https://github.com/AccessKit/accesskit/issues/264)) ([b9b3cd1](https://github.com/AccessKit/accesskit/commit/b9b3cd18fccdd6526fb4f58c13eb91599452a3d6))

## [0.11.1](https://github.com/AccessKit/accesskit/compare/accesskit-v0.11.0...accesskit-v0.11.1) (2023-07-30)


### Bug Fixes

* Fix broken intra-doc-link. ([#262](https://github.com/AccessKit/accesskit/issues/262)) ([63c1715](https://github.com/AccessKit/accesskit/commit/63c17152d1eb8ae6ff19c2bc4a6756372bc490c2))

## [0.11.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.10.1...accesskit-v0.11.0) (2023-03-29)


### Features

* Add C bindings ([#230](https://github.com/AccessKit/accesskit/issues/230)) ([7f7f4c7](https://github.com/AccessKit/accesskit/commit/7f7f4c755890ab8210a5a8bf8e237ba6a51dd205))

## [0.10.1](https://github.com/AccessKit/accesskit/compare/accesskit-v0.10.0...accesskit-v0.10.1) (2023-02-20)


### Bug Fixes

* Set appropriate representations on all public types that will be exposed via FFI ([54e82f6](https://github.com/AccessKit/accesskit/commit/54e82f673f5c7b46d9077fe5f946305800862bf0))

## [0.10.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.9.0...accesskit-v0.10.0) (2023-02-12)


### ⚠ BREAKING CHANGES

* Move thread synchronization into platform adapters; drop parking_lot ([#212](https://github.com/AccessKit/accesskit/issues/212))

### Code Refactoring

* Move thread synchronization into platform adapters; drop parking_lot ([#212](https://github.com/AccessKit/accesskit/issues/212)) ([5df52e5](https://github.com/AccessKit/accesskit/commit/5df52e5545faddf6a51905409013c2f5be23981e))

## [0.9.0](https://github.com/AccessKit/accesskit/compare/accesskit-v0.8.1...accesskit-v0.9.0) (2023-02-05)


### ⚠ BREAKING CHANGES

* Make `Node` opaque and optimize it for size ([#205](https://github.com/AccessKit/accesskit/issues/205))

### Code Refactoring

* Make `Node` opaque and optimize it for size ([#205](https://github.com/AccessKit/accesskit/issues/205)) ([4811152](https://github.com/AccessKit/accesskit/commit/48111521439b76c1a8687418a4b20f9b705eac6d))

## [0.8.1](https://github.com/AccessKit/accesskit/compare/accesskit-v0.8.0...accesskit-v0.8.1) (2022-12-04)


### Documentation

* Fix outdated documentation for `TreeUpdate` ([#182](https://github.com/AccessKit/accesskit/issues/182)) ([dd658c7](https://github.com/AccessKit/accesskit/commit/dd658c70df55b2234a0346220362b0b9a40bb41d))

## [0.8.0](https://www.github.com/AccessKit/accesskit/compare/accesskit-v0.7.0...accesskit-v0.8.0) (2022-11-17)


### ⚠ BREAKING CHANGES

* Be opinionated about coordinates after all (#151)

### Code Refactoring

* Be opinionated about coordinates after all ([#151](https://www.github.com/AccessKit/accesskit/issues/151)) ([91a29a1](https://www.github.com/AccessKit/accesskit/commit/91a29a1bf99bca39e9a00a744025533924e45190))

## [0.7.0](https://www.github.com/AccessKit/accesskit/compare/accesskit-v0.6.1...accesskit-v0.7.0) (2022-11-11)


### ⚠ BREAKING CHANGES

* Text range support (#145)
* Drop the `ignored` field and implement generic filtered tree traversal (#143)

### Features

* Text range support ([#145](https://www.github.com/AccessKit/accesskit/issues/145)) ([455e6f7](https://www.github.com/AccessKit/accesskit/commit/455e6f73bc058644d299c06eeeda9cc4cbe8844f))


### Code Refactoring

* Drop the `ignored` field and implement generic filtered tree traversal ([#143](https://www.github.com/AccessKit/accesskit/issues/143)) ([a4befe6](https://www.github.com/AccessKit/accesskit/commit/a4befe6e8a5afbe4a52dfd09eb87fdf2078d6c1d))

### [0.6.1](https://www.github.com/AccessKit/accesskit/compare/accesskit-v0.6.0...accesskit-v0.6.1) (2022-10-10)


### Bug Fixes

* **common:** Restore compatibility with Rust 1.61 ([#139](https://www.github.com/AccessKit/accesskit/issues/139)) ([d8c6b16](https://www.github.com/AccessKit/accesskit/commit/d8c6b166c83796bfd6d748df60136029a9ec81d2))

## [0.6.0](https://www.github.com/AccessKit/accesskit/compare/accesskit-v0.5.1...accesskit-v0.6.0) (2022-10-09)


### ⚠ BREAKING CHANGES

* Wrap `TreeUpdate` nodes in `Arc` (#135)
* Store node ID in `TreeUpdate`, not `accesskit::Node` (#132)

### Bug Fixes

* Don't try to optimize tree updates with unchanged nodes ([#138](https://www.github.com/AccessKit/accesskit/issues/138)) ([7721719](https://www.github.com/AccessKit/accesskit/commit/7721719fb0ab90bf41cc30dd0469c7de90228fe9))


### Code Refactoring

* Store node ID in `TreeUpdate`, not `accesskit::Node` ([#132](https://www.github.com/AccessKit/accesskit/issues/132)) ([0bb86dd](https://www.github.com/AccessKit/accesskit/commit/0bb86ddb298cb5a253a91f07be0bad8b84b2fda3))
* Wrap `TreeUpdate` nodes in `Arc` ([#135](https://www.github.com/AccessKit/accesskit/issues/135)) ([907bc18](https://www.github.com/AccessKit/accesskit/commit/907bc1820b80d95833b6c5c3acaa2a8a4e93a6c2))

### [0.5.1](https://www.github.com/AccessKit/accesskit/compare/accesskit-v0.5.0...accesskit-v0.5.1) (2022-10-03)


### Bug Fixes

* **common:** Write a README specifically for the accesskit crate ([#130](https://www.github.com/AccessKit/accesskit/issues/130)) ([0c2f5cf](https://www.github.com/AccessKit/accesskit/commit/0c2f5cf71bdacf3142bff77defea36eeb2b4e1e9)), closes [#129](https://www.github.com/AccessKit/accesskit/issues/129)

## [0.5.0](https://www.github.com/AccessKit/accesskit/compare/accesskit-v0.4.0...accesskit-v0.5.0) (2022-09-23)


### ⚠ BREAKING CHANGES

* Basic live regions (#128)

### Features

* Basic live regions ([#128](https://www.github.com/AccessKit/accesskit/issues/128)) ([03d745b](https://www.github.com/AccessKit/accesskit/commit/03d745b891147175bde2693cc10b96a2f6e31f39))


### Bug Fixes

* **common:** Enable the serde feature when the schemars feature is turned on ([#122](https://www.github.com/AccessKit/accesskit/issues/122)) ([126b6e1](https://www.github.com/AccessKit/accesskit/commit/126b6e13294bee2b4c905a78147b49d763a61d05))
* **common:** Skip `ActionRequest::data` if it is `None` during serialization ([#123](https://www.github.com/AccessKit/accesskit/issues/123)) ([2d88ea8](https://www.github.com/AccessKit/accesskit/commit/2d88ea8518c99692beacfb955ef0bd4f388a4908))

## [0.4.0](https://www.github.com/AccessKit/accesskit/compare/accesskit-v0.3.0...accesskit-v0.4.0) (2022-07-22)


### ⚠ BREAKING CHANGES

* String indices are always in UTF-8 code units (#114)
* Drop unused tree IDs (#113)
* Switch to NonZeroU128 for NodeIDs (#99)

### Features

* **common:** Conversion from `NonZeroU64` to `NodeId` ([#112](https://www.github.com/AccessKit/accesskit/issues/112)) ([b7adfb9](https://www.github.com/AccessKit/accesskit/commit/b7adfb906cb09107be71a148b5199ba87df2a6b3))


### Bug Fixes

* **common:** Various documentation fixes and improvements ([#111](https://www.github.com/AccessKit/accesskit/issues/111)) ([4d27234](https://www.github.com/AccessKit/accesskit/commit/4d27234195e96de65bf55869877405cb5e45f6fc))
* Migrate to 2021 edition ([#115](https://www.github.com/AccessKit/accesskit/issues/115)) ([f2333c8](https://www.github.com/AccessKit/accesskit/commit/f2333c8ce17d46aab6fc190338ab4cfcf8569f9e))
* Switch to NonZeroU128 for NodeIDs ([#99](https://www.github.com/AccessKit/accesskit/issues/99)) ([25a1a52](https://www.github.com/AccessKit/accesskit/commit/25a1a52c4562b163bfcc8c625a233c00a41aacf2))


### Code Refactoring

* Drop unused tree IDs ([#113](https://www.github.com/AccessKit/accesskit/issues/113)) ([ca60770](https://www.github.com/AccessKit/accesskit/commit/ca607702cee13c93fe538d2faec88e474261f7ab))
* String indices are always in UTF-8 code units ([#114](https://www.github.com/AccessKit/accesskit/issues/114)) ([386ca0a](https://www.github.com/AccessKit/accesskit/commit/386ca0a89c42fd201843f617b2fd6b6d1de77f59))

## [0.3.0](https://www.github.com/AccessKit/accesskit/compare/accesskit-v0.2.0...accesskit-v0.3.0) (2021-12-29)


### ⚠ BREAKING CHANGES

* Drop `TreeUpdate::clear` (#96)

### Code Refactoring

* Drop `TreeUpdate::clear` ([#96](https://www.github.com/AccessKit/accesskit/issues/96)) ([38f520b](https://www.github.com/AccessKit/accesskit/commit/38f520b960c6db7b3927b369aee206ee6bc5e8aa))

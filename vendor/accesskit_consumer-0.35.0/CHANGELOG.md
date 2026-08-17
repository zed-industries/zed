# Changelog

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.10.0 to 0.10.1

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.10.1 to 0.11.0

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.11.0 to 0.11.1

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.11.1 to 0.11.2

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.12.2 to 0.12.3

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.2 to 0.16.3

## [0.35.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.34.0...accesskit_consumer-v0.35.0) (2026-03-04)


### ⚠ BREAKING CHANGES

* Allow locating nodes without Tree access

### Features

* Expose text attributes on macOS ([#691](https://github.com/AccessKit/accesskit/issues/691)) ([641b05b](https://github.com/AccessKit/accesskit/commit/641b05b10d3d55631a73d10a1f753111dde372e8))


### Code Refactoring

* Allow locating nodes without Tree access ([941a96d](https://github.com/AccessKit/accesskit/commit/941a96d161414ea6db448ecafdd11fa6e3f1680d))

## [0.34.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.33.1...accesskit_consumer-v0.34.0) (2026-02-01)


### ⚠ BREAKING CHANGES

* Update minimum supported Rust version to 1.85 ([#683](https://github.com/AccessKit/accesskit/issues/683))

### Bug Fixes

* Update minimum supported Rust version to 1.85 ([#683](https://github.com/AccessKit/accesskit/issues/683)) ([f8c22cb](https://github.com/AccessKit/accesskit/commit/f8c22cbb53ce6b1859adeff3c0b7154ad9e1f25e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.23.0 to 0.24.0

## [0.33.1](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.33.0...accesskit_consumer-v0.33.1) (2026-01-18)


### Bug Fixes

* Fix next word and line at the end of text runs ([#678](https://github.com/AccessKit/accesskit/issues/678)) ([828979e](https://github.com/AccessKit/accesskit/commit/828979e5f668bc8c4ceba018d417956a5c69b2bd))

## [0.33.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.32.0...accesskit_consumer-v0.33.0) (2026-01-15)


### ⚠ BREAKING CHANGES

* Store text decoration style and color in a struct ([#674](https://github.com/AccessKit/accesskit/issues/674))
* Multiple tree support ([#655](https://github.com/AccessKit/accesskit/issues/655))

### Features

* Implement the `url` property ([#669](https://github.com/AccessKit/accesskit/issues/669)) ([6202e63](https://github.com/AccessKit/accesskit/commit/6202e63c667e73216a306b3dd45ff25c364a4f45))
* Multiple tree support ([#655](https://github.com/AccessKit/accesskit/issues/655)) ([c16eb80](https://github.com/AccessKit/accesskit/commit/c16eb80c49d14f39e8a7defb1694bf22b8ceed6f))


### Bug Fixes

* Implement the `active_descendant` property ([#675](https://github.com/AccessKit/accesskit/issues/675)) ([863755d](https://github.com/AccessKit/accesskit/commit/863755d6898c7006a5f49c3661a26cb62d837d1b))


### Code Refactoring

* Store text decoration style and color in a struct ([#674](https://github.com/AccessKit/accesskit/issues/674)) ([c2e8487](https://github.com/AccessKit/accesskit/commit/c2e8487d0a243da0d1c996887fb4c9a7e3beacc7))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.22.0 to 0.23.0

## [0.32.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.31.0...accesskit_consumer-v0.32.0) (2025-12-23)


### ⚠ BREAKING CHANGES

* Use a proper `Color` struct for the color properties ([#663](https://github.com/AccessKit/accesskit/issues/663))
* Change font size and weight to `f32` ([#662](https://github.com/AccessKit/accesskit/issues/662))
* Drop deprecated roles ([#642](https://github.com/AccessKit/accesskit/issues/642))
* Infrastructure for supporting text formatting changes ([#626](https://github.com/AccessKit/accesskit/issues/626))
* Support properties on text ranges ([#627](https://github.com/AccessKit/accesskit/issues/627))

### Features

* Add `map` function to `TextRangePropertyValue<Option<T>>` ([#632](https://github.com/AccessKit/accesskit/issues/632)) ([555617a](https://github.com/AccessKit/accesskit/commit/555617a004c1d57c7ed114a4fd38c0c48ac2d149))
* expose more data on Node ([3d0d873](https://github.com/AccessKit/accesskit/commit/3d0d8739ef53214ea6da2fcfbc7fc29d01f53a40))
* Implement BrailleLabel and BrailleRoleDescription roles ([#638](https://github.com/AccessKit/accesskit/issues/638)) ([0fdcebb](https://github.com/AccessKit/accesskit/commit/0fdcebb55e308e039ec99fbc31e94e8087a69f2d))
* Support dialogs in the consumer crate ([028f611](https://github.com/AccessKit/accesskit/commit/028f611b7324bf4bcc121e1e047c426075c34982))
* Support inherited flags ([#661](https://github.com/AccessKit/accesskit/issues/661)) ([176c90c](https://github.com/AccessKit/accesskit/commit/176c90cac8f93447a7ac354efca853a394a25e33))
* Support properties on text ranges ([#627](https://github.com/AccessKit/accesskit/issues/627)) ([b854ad8](https://github.com/AccessKit/accesskit/commit/b854ad8dbd54eeb38105c9b39c9e76310a753570))


### Bug Fixes

* Upgrade hashbrown, indexmap ([#653](https://github.com/AccessKit/accesskit/issues/653)) ([fd73a85](https://github.com/AccessKit/accesskit/commit/fd73a85e57a6591a449b0a1880025fb940873c66))


### Code Refactoring

* Change font size and weight to `f32` ([#662](https://github.com/AccessKit/accesskit/issues/662)) ([ae0ead8](https://github.com/AccessKit/accesskit/commit/ae0ead825298539f08140b3775f0fdbfc17acd13))
* Drop deprecated roles ([#642](https://github.com/AccessKit/accesskit/issues/642)) ([4d46c27](https://github.com/AccessKit/accesskit/commit/4d46c2740631c5fe4f057707b949d12b26931d0b))
* Infrastructure for supporting text formatting changes ([#626](https://github.com/AccessKit/accesskit/issues/626)) ([ea23ec4](https://github.com/AccessKit/accesskit/commit/ea23ec424c7dbb8841e03d71b6a15b74264850a9))
* Use a proper `Color` struct for the color properties ([#663](https://github.com/AccessKit/accesskit/issues/663)) ([922e882](https://github.com/AccessKit/accesskit/commit/922e882a8cd9aa372038fb9c66ff57a557e7888d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.21.1 to 0.22.0

## [0.31.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.30.1...accesskit_consumer-v0.31.0) (2025-10-20)


### Features

* Expose text properties on consumer `Node` with inheritance ([#617](https://github.com/AccessKit/accesskit/issues/617)) ([0f132ea](https://github.com/AccessKit/accesskit/commit/0f132ea7b94c33d0ee47dfe6b0549a68d331f493))

## [0.30.1](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.30.0...accesskit_consumer-v0.30.1) (2025-10-02)


### Bug Fixes

* Fix clippy warnings introduced in 1.89 ([#606](https://github.com/AccessKit/accesskit/issues/606)) ([b2c07d6](https://github.com/AccessKit/accesskit/commit/b2c07d654a8ce6f01e61a79c91f2f9d5a96afdc9))
* Only expose the `placeholder` property on empty text inputs ([#607](https://github.com/AccessKit/accesskit/issues/607)) ([1764cef](https://github.com/AccessKit/accesskit/commit/1764cef1892e3bf05182fb9c4c65d5ba4f157f50))
* Prevent filtered node iterators from panicking when exhausted ([#621](https://github.com/AccessKit/accesskit/issues/621)) ([1c8071f](https://github.com/AccessKit/accesskit/commit/1c8071f62dd91e398f0df1618b9a2858c8793d98))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.21.0 to 0.21.1

## [0.30.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.29.0...accesskit_consumer-v0.30.0) (2025-07-16)


### Features

* Let parents declare actions supported on their children ([#593](https://github.com/AccessKit/accesskit/issues/593)) ([70b534b](https://github.com/AccessKit/accesskit/commit/70b534bed168a84b84cc35199588aa8ab784fb43))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.20.0 to 0.21.0

## [0.29.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.28.0...accesskit_consumer-v0.29.0) (2025-06-26)


### Features

* Consumer support for scrolling and clipping children ([#574](https://github.com/AccessKit/accesskit/issues/574)) ([4094dec](https://github.com/AccessKit/accesskit/commit/4094dec2ad512570c7837d057f1d5893e89ff9b4))


### Bug Fixes

* Eliminate incorrect removal of reparented nodes ([#576](https://github.com/AccessKit/accesskit/issues/576)) ([db7d4d0](https://github.com/AccessKit/accesskit/commit/db7d4d050d89a4aafa6b5ad2097d0bd8a7997940))
* Resolve new clippy warning about using variables directly in format strings ([#590](https://github.com/AccessKit/accesskit/issues/590)) ([ccc62b7](https://github.com/AccessKit/accesskit/commit/ccc62b7f1dd32f0c372ba127a1e65c377048f670))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.19.0 to 0.20.0

## [0.28.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.27.0...accesskit_consumer-v0.28.0) (2025-05-06)


### ⚠ BREAKING CHANGES

* Drop unused `Node::is_linked` ([#545](https://github.com/AccessKit/accesskit/issues/545))
* Drop `FrozenNode` ([#496](https://github.com/AccessKit/accesskit/issues/496))
* Replace `immutable-chunkmap` with dual tree states ([#495](https://github.com/AccessKit/accesskit/issues/495))

### Features

* Expose tabs in consumer and atspi-common ([b1fb5b3](https://github.com/AccessKit/accesskit/commit/b1fb5b3de12c001e34021263038b66a6e3a7dd1e))


### Bug Fixes

* Improve `NodeId`'s debug representation ([#547](https://github.com/AccessKit/accesskit/issues/547)) ([a47bca1](https://github.com/AccessKit/accesskit/commit/a47bca1e376de7b0a22a7dfe6c23dedad315c449))


### Code Refactoring

* Drop `FrozenNode` ([#496](https://github.com/AccessKit/accesskit/issues/496)) ([f8c0d0a](https://github.com/AccessKit/accesskit/commit/f8c0d0a6fc9613cf1a2a6d8cfba11ebc892dfeb8))
* Drop unused `Node::is_linked` ([#545](https://github.com/AccessKit/accesskit/issues/545)) ([3aab4ac](https://github.com/AccessKit/accesskit/commit/3aab4ac6f0193b8a06d7962f933582a4dbdf0c98))
* Replace `immutable-chunkmap` with dual tree states ([#495](https://github.com/AccessKit/accesskit/issues/495)) ([a74dbfc](https://github.com/AccessKit/accesskit/commit/a74dbfcd2d30f9fbec781db811243ec070cbf8c5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.18.0 to 0.19.0

## [0.27.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.26.0...accesskit_consumer-v0.27.0) (2025-03-06)


### ⚠ BREAKING CHANGES

* Optimize simple string getters ([#493](https://github.com/AccessKit/accesskit/issues/493))
* Drop `Tree::app_name` ([#492](https://github.com/AccessKit/accesskit/issues/492))

### Features

* Add list box support to the `consumer` and `atspi-common` crates ([d6dca15](https://github.com/AccessKit/accesskit/commit/d6dca15d5c298c797ab7a702f0186043eac33c5c))
* Android adapter ([#500](https://github.com/AccessKit/accesskit/issues/500)) ([7e65ac7](https://github.com/AccessKit/accesskit/commit/7e65ac77d7e108ac5b9f3722f488a2fdf2e3b3e0))
* Expose the `is_required` property ([#497](https://github.com/AccessKit/accesskit/issues/497)) ([46ed99b](https://github.com/AccessKit/accesskit/commit/46ed99bb958ddb32cbf1bee2fcfb7b328bcbe0ab))


### Bug Fixes

* Derive `Debug` for adapters ([#513](https://github.com/AccessKit/accesskit/issues/513)) ([753d904](https://github.com/AccessKit/accesskit/commit/753d90473cf57682568c7a17c82474c8e5d00b25))
* Optimize dynamic string building ([#491](https://github.com/AccessKit/accesskit/issues/491)) ([a86901d](https://github.com/AccessKit/accesskit/commit/a86901ddea5d5ba72ab237e98b53d6adcc6087bb))
* Optimize removal of unreachable nodes ([#486](https://github.com/AccessKit/accesskit/issues/486)) ([93d0a72](https://github.com/AccessKit/accesskit/commit/93d0a72880901479fe44ed92ef24fa71b7bb4803))
* Optimize the "short node list" helper used in panic messages ([#490](https://github.com/AccessKit/accesskit/issues/490)) ([b4a89a3](https://github.com/AccessKit/accesskit/commit/b4a89a386474b9a71f22aa36d09c2d07bca084cd))
* Remove unnecessary explicit lifetimes ([#488](https://github.com/AccessKit/accesskit/issues/488)) ([d2bcd6d](https://github.com/AccessKit/accesskit/commit/d2bcd6d3048d23df4e132bee6171eb247b2dc2c8))


### Code Refactoring

* Drop `Tree::app_name` ([#492](https://github.com/AccessKit/accesskit/issues/492)) ([089794c](https://github.com/AccessKit/accesskit/commit/089794c8f74957e91a19ae3df508e2a892f39ebc))
* Optimize simple string getters ([#493](https://github.com/AccessKit/accesskit/issues/493)) ([484fd7c](https://github.com/AccessKit/accesskit/commit/484fd7cbfb778222369d3f57d31dd998f6fa80d8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.17.1 to 0.18.0

## [0.26.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.25.0...accesskit_consumer-v0.26.0) (2024-11-23)


### Features

* Make the consumer crate no-std ([#471](https://github.com/AccessKit/accesskit/issues/471)) ([f25d03a](https://github.com/AccessKit/accesskit/commit/f25d03ad81736017a29ce0f5ed1b387047534d2d))


### Bug Fixes

* Avoid reallocations when processing tree updates ([#482](https://github.com/AccessKit/accesskit/issues/482)) ([dcb17bc](https://github.com/AccessKit/accesskit/commit/dcb17bc1e69eccc2fea6af6a6b61f71c9e73a0b9))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.17.0 to 0.17.1

## [0.25.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.24.3...accesskit_consumer-v0.25.0) (2024-10-31)


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

## [0.24.2](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.24.1...accesskit_consumer-v0.24.2) (2024-10-07)


### Bug Fixes

* Update minimum supported Rust version to 1.75 ([#457](https://github.com/AccessKit/accesskit/issues/457)) ([fc622fe](https://github.com/AccessKit/accesskit/commit/fc622fe7657c80a4eedad6f6cded11d2538b54d5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.1 to 0.16.2

## [0.24.1](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.24.0...accesskit_consumer-v0.24.1) (2024-09-24)


### Bug Fixes

* `Node::is_focusable` always returns true if the node is focused ([#451](https://github.com/AccessKit/accesskit/issues/451)) ([d286883](https://github.com/AccessKit/accesskit/commit/d286883d88b5c1e51f6e8bbfbc2e0e5b1986d9b5))
* Extend the implicit labelled-by relation to more parent roles ([#448](https://github.com/AccessKit/accesskit/issues/448)) ([df518c7](https://github.com/AccessKit/accesskit/commit/df518c71934cb4e0071764643968e67f9908a8dd))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.16.0 to 0.16.1

## [0.24.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.23.0...accesskit_consumer-v0.24.0) (2024-06-29)


### ⚠ BREAKING CHANGES

* Rename the `StaticText` role to `Label` ([#434](https://github.com/AccessKit/accesskit/issues/434))

### Bug Fixes

* Correctly handle recursive filtering ([#438](https://github.com/AccessKit/accesskit/issues/438)) ([72f9b42](https://github.com/AccessKit/accesskit/commit/72f9b424a5c6e7914df8bf31eeb2fc61be35f47b))


### Code Refactoring

* Rename the `StaticText` role to `Label` ([#434](https://github.com/AccessKit/accesskit/issues/434)) ([7086bc0](https://github.com/AccessKit/accesskit/commit/7086bc0fad446d3ed4a0fd5eff641a1e75f6c599))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.15.0 to 0.16.0

## [0.23.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.22.0...accesskit_consumer-v0.23.0) (2024-06-09)


### Features

* Add `author_id` property ([#424](https://github.com/AccessKit/accesskit/issues/424)) ([0d1c56f](https://github.com/AccessKit/accesskit/commit/0d1c56f0bdde58715e1c69f6015df600cb7cb8c1))


### Bug Fixes

* Clamp character index when getting focus from a text selection ([#428](https://github.com/AccessKit/accesskit/issues/428)) ([38e649d](https://github.com/AccessKit/accesskit/commit/38e649de6b72c99d1e438b26b3fc1f647ac39e6c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.14.0 to 0.15.0

## [0.22.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.21.0...accesskit_consumer-v0.22.0) (2024-05-27)


### Features

* Expose the `orientation` property ([#421](https://github.com/AccessKit/accesskit/issues/421)) ([590aada](https://github.com/AccessKit/accesskit/commit/590aada070dc812f9b8f171fb9e43ac984fad2a1))

## [0.21.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.20.0...accesskit_consumer-v0.21.0) (2024-05-26)


### Features

* Add basic text support on Unix ([#362](https://github.com/AccessKit/accesskit/issues/362)) ([52540f8](https://github.com/AccessKit/accesskit/commit/52540f82cf9fc148358351ed486bab3e7e91f1d6))
* Expose the `placeholder` property ([#417](https://github.com/AccessKit/accesskit/issues/417)) ([8f4a0a1](https://github.com/AccessKit/accesskit/commit/8f4a0a1c10f83fcc8580a37d8013fec2d110865b))


### Bug Fixes

* Clamp character indices when converting a text selection to a range ([#416](https://github.com/AccessKit/accesskit/issues/416)) ([5c550af](https://github.com/AccessKit/accesskit/commit/5c550af7afc81b3a32c30d31327ff95b93718545))
* Fix a logic error that sometimes caused filtered traversal to stop prematurely ([#412](https://github.com/AccessKit/accesskit/issues/412)) ([9946d38](https://github.com/AccessKit/accesskit/commit/9946d38b9d13489517713f43284cf6b96d88cb8c))
* Go back to detecting unchanged nodes when processing tree updates ([#415](https://github.com/AccessKit/accesskit/issues/415)) ([489302d](https://github.com/AccessKit/accesskit/commit/489302db7143a016605145682b989ab18583d59c))
* Update minimum version of immutable-chunkmap ([#419](https://github.com/AccessKit/accesskit/issues/419)) ([893f688](https://github.com/AccessKit/accesskit/commit/893f68845dd322da5f3ae4d39fc2b1cc01f88888))

## [0.20.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.19.1...accesskit_consumer-v0.20.0) (2024-05-13)


### ⚠ BREAKING CHANGES

* Restore full copy-on-write tree snapshots, now using `immutable-chunkmap` ([#365](https://github.com/AccessKit/accesskit/issues/365))

### Bug Fixes

* Fix the filtered sibling iterators to use the filtered parent to find the back node ([#408](https://github.com/AccessKit/accesskit/issues/408)) ([2f8155c](https://github.com/AccessKit/accesskit/commit/2f8155ca260d7e50de5de502744b420769875e83))


### Code Refactoring

* Restore full copy-on-write tree snapshots, now using `immutable-chunkmap` ([#365](https://github.com/AccessKit/accesskit/issues/365)) ([441bf5f](https://github.com/AccessKit/accesskit/commit/441bf5ff77d1785dfea228de9109aceff4773da1))

## [0.19.1](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.19.0...accesskit_consumer-v0.19.1) (2024-05-11)


### Bug Fixes

* Improve panic messages ([#401](https://github.com/AccessKit/accesskit/issues/401)) ([e6ce021](https://github.com/AccessKit/accesskit/commit/e6ce021b3b172f5ea7ee31496c9afaf66b1871f2))

## [0.19.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.18.0...accesskit_consumer-v0.19.0) (2024-04-30)


### ⚠ BREAKING CHANGES

* Drop `NodeClassSet` ([#389](https://github.com/AccessKit/accesskit/issues/389))
* Rename `Checked` to `Toggled`; drop `ToggleButton` role ([#388](https://github.com/AccessKit/accesskit/issues/388))

### Features

* Expose the class name property ([#385](https://github.com/AccessKit/accesskit/issues/385)) ([53dcf2a](https://github.com/AccessKit/accesskit/commit/53dcf2ae47546273590c46a9b31b708aa1409837))
* Implement the `description` property ([#382](https://github.com/AccessKit/accesskit/issues/382)) ([d49f406](https://github.com/AccessKit/accesskit/commit/d49f40660b5dc23ed074cd72a91e511b130756ae))


### Bug Fixes

* Increase minimum supported Rust version to `1.70` ([#396](https://github.com/AccessKit/accesskit/issues/396)) ([a8398b8](https://github.com/AccessKit/accesskit/commit/a8398b847aa003de91042ac45e33126fc2cae053))


### Code Refactoring

* Drop `NodeClassSet` ([#389](https://github.com/AccessKit/accesskit/issues/389)) ([1b153ed](https://github.com/AccessKit/accesskit/commit/1b153ed51f8421cdba2dc98beca2e8f5f8c781bc))
* Rename `Checked` to `Toggled`; drop `ToggleButton` role ([#388](https://github.com/AccessKit/accesskit/issues/388)) ([6bc040b](https://github.com/AccessKit/accesskit/commit/6bc040b7cf75cdbd6a019cc380d8dbce804b3c81))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.13.0 to 0.14.0

## [0.18.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.17.1...accesskit_consumer-v0.18.0) (2024-04-14)


### ⚠ BREAKING CHANGES

* New approach to lazy initialization ([#375](https://github.com/AccessKit/accesskit/issues/375))

### Code Refactoring

* New approach to lazy initialization ([#375](https://github.com/AccessKit/accesskit/issues/375)) ([9baebdc](https://github.com/AccessKit/accesskit/commit/9baebdceed7300389b6768815d7ae48f1ce401e4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.12.3 to 0.13.0

## [0.17.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.16.1...accesskit_consumer-v0.17.0) (2024-01-03)


### Features

* Support custom role descriptions ([#316](https://github.com/AccessKit/accesskit/issues/316)) ([c8d1a56](https://github.com/AccessKit/accesskit/commit/c8d1a5638fa6c33adfa059815c04f7e043c56026))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.12.1 to 0.12.2

## [0.16.1](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.16.0...accesskit_consumer-v0.16.1) (2023-11-04)


### Bug Fixes

* Add missing semicolons when not returning anything ([#303](https://github.com/AccessKit/accesskit/issues/303)) ([38d4de1](https://github.com/AccessKit/accesskit/commit/38d4de1442247e701047d75122a9638a2ed99b1f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.12.0 to 0.12.1

## [0.16.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.15.2...accesskit_consumer-v0.16.0) (2023-09-27)


### ⚠ BREAKING CHANGES

* Allow providing app_name, toolkit_name and toolkit_version in Tree, remove parameters from unix adapter constructor ([#291](https://github.com/AccessKit/accesskit/issues/291))
* Clean up roles and properties ([#289](https://github.com/AccessKit/accesskit/issues/289))
* Drop `Tree::root_scroller` ([#279](https://github.com/AccessKit/accesskit/issues/279))
* Decouple in-tree focus from host window/view focus ([#278](https://github.com/AccessKit/accesskit/issues/278))
* Switch to simple unsigned 64-bit integer for node IDs ([#276](https://github.com/AccessKit/accesskit/issues/276))

### Features

* Add role for terminals ([#282](https://github.com/AccessKit/accesskit/issues/282)) ([ddbef37](https://github.com/AccessKit/accesskit/commit/ddbef37158b57f56217317b480e40d58f83a9c24))
* Allow providing app_name, toolkit_name and toolkit_version in Tree, remove parameters from unix adapter constructor ([#291](https://github.com/AccessKit/accesskit/issues/291)) ([5313860](https://github.com/AccessKit/accesskit/commit/531386023257150f49b5e4be942f359855fb7cb6))


### Bug Fixes

* Drop `Tree::root_scroller` ([#279](https://github.com/AccessKit/accesskit/issues/279)) ([fc6c4e0](https://github.com/AccessKit/accesskit/commit/fc6c4e0091d5b257a3869a468fca144a1453cebc))
* Support text fields without a value property ([#274](https://github.com/AccessKit/accesskit/issues/274)) ([5ae557b](https://github.com/AccessKit/accesskit/commit/5ae557b40d395b4a9966a90a2d80e7d97ad50bf9))
* Use common filters across platform adapters ([#287](https://github.com/AccessKit/accesskit/issues/287)) ([09c1204](https://github.com/AccessKit/accesskit/commit/09c12045ff4ccdb22f0cf643077a27465013572d))


### Code Refactoring

* Clean up roles and properties ([#289](https://github.com/AccessKit/accesskit/issues/289)) ([4fc9c55](https://github.com/AccessKit/accesskit/commit/4fc9c55c91812472593923d93ff89d75ff305ee4))
* Decouple in-tree focus from host window/view focus ([#278](https://github.com/AccessKit/accesskit/issues/278)) ([d360d20](https://github.com/AccessKit/accesskit/commit/d360d20cf951e7643b81a5303006c9f7daa5bd56))
* Switch to simple unsigned 64-bit integer for node IDs ([#276](https://github.com/AccessKit/accesskit/issues/276)) ([3eadd48](https://github.com/AccessKit/accesskit/commit/3eadd48ec47854faa94a94ebf910ec08f514642f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.11.2 to 0.12.0

## [0.15.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.14.2...accesskit_consumer-v0.15.0) (2023-03-30)


### ⚠ BREAKING CHANGES

* Force a semver-breaking version bump in downstream crates ([#234](https://github.com/AccessKit/accesskit/issues/234))

### Bug Fixes

* Force a semver-breaking version bump in downstream crates ([#234](https://github.com/AccessKit/accesskit/issues/234)) ([773389b](https://github.com/AccessKit/accesskit/commit/773389bff857fa18edf15de426e029251fc34591))

## [0.14.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.13.0...accesskit_consumer-v0.14.0) (2023-02-12)


### ⚠ BREAKING CHANGES

* Move thread synchronization into platform adapters; drop parking_lot ([#212](https://github.com/AccessKit/accesskit/issues/212))

### Code Refactoring

* Move thread synchronization into platform adapters; drop parking_lot ([#212](https://github.com/AccessKit/accesskit/issues/212)) ([5df52e5](https://github.com/AccessKit/accesskit/commit/5df52e5545faddf6a51905409013c2f5be23981e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.9.0 to 0.10.0

## [0.13.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.12.1...accesskit_consumer-v0.13.0) (2023-02-05)


### ⚠ BREAKING CHANGES

* Make `Node` opaque and optimize it for size ([#205](https://github.com/AccessKit/accesskit/issues/205))

### Code Refactoring

* Make `Node` opaque and optimize it for size ([#205](https://github.com/AccessKit/accesskit/issues/205)) ([4811152](https://github.com/AccessKit/accesskit/commit/48111521439b76c1a8687418a4b20f9b705eac6d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.8.1 to 0.9.0

## [0.12.1](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.12.0...accesskit_consumer-v0.12.1) (2023-01-06)


### Bug Fixes

* Make `Node::filtered_parent` recursive as it was meant to be ([#203](https://github.com/AccessKit/accesskit/issues/203)) ([d2faef5](https://github.com/AccessKit/accesskit/commit/d2faef5a2ad61b9e4d3f3d5c89570cdeec6fe6e6))

## [0.12.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.11.0...accesskit_consumer-v0.12.0) (2023-01-05)


### Features

* Basic Unix platform adapter ([#198](https://github.com/AccessKit/accesskit/issues/198)) ([1cea32e](https://github.com/AccessKit/accesskit/commit/1cea32e44ee743b778ac941ceff9087ae745cb37))

## [0.11.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.10.0...accesskit_consumer-v0.11.0) (2022-12-17)


### Features

* Text support on macOS ([#191](https://github.com/AccessKit/accesskit/issues/191)) ([3a35dbe](https://github.com/AccessKit/accesskit/commit/3a35dbe02122c789fe682995c5b7e022aef5cc36))


### Bug Fixes

* More reliable handling of the edge case for wrapped lines ([#192](https://github.com/AccessKit/accesskit/issues/192)) ([c626d2c](https://github.com/AccessKit/accesskit/commit/c626d2c3028085b076ada7dd31242cf3ca3c0f08))

## [0.10.0](https://github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.9.1...accesskit_consumer-v0.10.0) (2022-12-04)


### Features

* Automatically get button and link labels from descendants ([#184](https://github.com/AccessKit/accesskit/issues/184)) ([ec5c38e](https://github.com/AccessKit/accesskit/commit/ec5c38ef3001a10b7a135df1438901246463f3e1))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.8.0 to 0.8.1

### [0.9.1](https://www.github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.9.0...accesskit_consumer-v0.9.1) (2022-11-25)


### Bug Fixes

* **consumer:** Allow editable spin buttons ([#167](https://www.github.com/AccessKit/accesskit/issues/167)) ([65a7aa0](https://www.github.com/AccessKit/accesskit/commit/65a7aa0114bfc6e17189e834578e256945b84a98))
* Gracefully handle nodes that only support text ranges some of the time ([#169](https://www.github.com/AccessKit/accesskit/issues/169)) ([1f50df6](https://www.github.com/AccessKit/accesskit/commit/1f50df6820b9d23fe2e579f043f4981acf285de2))

## [0.9.0](https://www.github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.8.0...accesskit_consumer-v0.9.0) (2022-11-23)


### Features

* **platforms/macos:** Basic macOS platform adapter ([#158](https://www.github.com/AccessKit/accesskit/issues/158)) ([a06725e](https://www.github.com/AccessKit/accesskit/commit/a06725e952e6041dbd366944fa793b746c9f195e))

## [0.8.0](https://www.github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.7.1...accesskit_consumer-v0.8.0) (2022-11-17)


### ⚠ BREAKING CHANGES

* **consumer:** Eliminate the dependency on `im` due to licensing (#153)

### Code Refactoring

* **consumer:** Eliminate the dependency on `im` due to licensing ([#153](https://www.github.com/AccessKit/accesskit/issues/153)) ([b4c4cb5](https://www.github.com/AccessKit/accesskit/commit/b4c4cb5713d4833d8ee7979e4f4e39c7e96a3ed4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.7.0 to 0.8.0

### [0.7.1](https://www.github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.7.0...accesskit_consumer-v0.7.1) (2022-11-12)


### Bug Fixes

* **consumer, platforms/windows, platforms/winit:** Update to parking_lot 0.12.1 ([#146](https://www.github.com/AccessKit/accesskit/issues/146)) ([6772855](https://www.github.com/AccessKit/accesskit/commit/6772855a7b540fd728faad15d8d208b05c1bbd8a))

## [0.7.0](https://www.github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.6.1...accesskit_consumer-v0.7.0) (2022-11-11)


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

### [0.6.1](https://www.github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.6.0...accesskit_consumer-v0.6.1) (2022-10-10)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.6.0 to 0.6.1

## [0.6.0](https://www.github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.5.1...accesskit_consumer-v0.6.0) (2022-10-09)


### ⚠ BREAKING CHANGES

* **consumer:** Optimize tree access and change handling (#134)
* Wrap `TreeUpdate` nodes in `Arc` (#135)
* **consumer:** Make `Node::data` private to the crate (#137)
* Store node ID in `TreeUpdate`, not `accesskit::Node` (#132)

### Bug Fixes

* **consumer:** Drop printing of detached nodes before panic ([#136](https://www.github.com/AccessKit/accesskit/issues/136)) ([2f20477](https://www.github.com/AccessKit/accesskit/commit/2f204772a97d4e21205609f31f3e84bc878554cd))
* Don't try to optimize tree updates with unchanged nodes ([#138](https://www.github.com/AccessKit/accesskit/issues/138)) ([7721719](https://www.github.com/AccessKit/accesskit/commit/7721719fb0ab90bf41cc30dd0469c7de90228fe9))


### Code Refactoring

* **consumer:** Make `Node::data` private to the crate ([#137](https://www.github.com/AccessKit/accesskit/issues/137)) ([adb372d](https://www.github.com/AccessKit/accesskit/commit/adb372dda78d183c7189966e3bbc2d3780070513))
* **consumer:** Optimize tree access and change handling ([#134](https://www.github.com/AccessKit/accesskit/issues/134)) ([765ab74](https://www.github.com/AccessKit/accesskit/commit/765ab74efcf10a3b3871dc901d28f3cf1ff6020c))
* Store node ID in `TreeUpdate`, not `accesskit::Node` ([#132](https://www.github.com/AccessKit/accesskit/issues/132)) ([0bb86dd](https://www.github.com/AccessKit/accesskit/commit/0bb86ddb298cb5a253a91f07be0bad8b84b2fda3))
* Wrap `TreeUpdate` nodes in `Arc` ([#135](https://www.github.com/AccessKit/accesskit/issues/135)) ([907bc18](https://www.github.com/AccessKit/accesskit/commit/907bc1820b80d95833b6c5c3acaa2a8a4e93a6c2))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.5.1 to 0.6.0

### [0.5.1](https://www.github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.5.0...accesskit_consumer-v0.5.1) (2022-10-03)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.5.0 to 0.5.1

## [0.5.0](https://www.github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.4.0...accesskit_consumer-v0.5.0) (2022-09-23)


### ⚠ BREAKING CHANGES

* Basic live regions (#128)

### Features

* Basic live regions ([#128](https://www.github.com/AccessKit/accesskit/issues/128)) ([03d745b](https://www.github.com/AccessKit/accesskit/commit/03d745b891147175bde2693cc10b96a2f6e31f39))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.4.0 to 0.5.0

## [0.4.0](https://www.github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.3.0...accesskit_consumer-v0.4.0) (2022-07-22)


### ⚠ BREAKING CHANGES

* String indices are always in UTF-8 code units (#114)
* Drop unused tree IDs (#113)
* Switch to NonZeroU128 for NodeIDs (#99)

### Bug Fixes

* **consumer, platforms/windows:** Resolve new clippy warning ([#100](https://www.github.com/AccessKit/accesskit/issues/100)) ([e8cd95c](https://www.github.com/AccessKit/accesskit/commit/e8cd95c3741b39b77e4ddc8ce82efdc20f93f096))
* Migrate to 2021 edition ([#115](https://www.github.com/AccessKit/accesskit/issues/115)) ([f2333c8](https://www.github.com/AccessKit/accesskit/commit/f2333c8ce17d46aab6fc190338ab4cfcf8569f9e))
* Switch to NonZeroU128 for NodeIDs ([#99](https://www.github.com/AccessKit/accesskit/issues/99)) ([25a1a52](https://www.github.com/AccessKit/accesskit/commit/25a1a52c4562b163bfcc8c625a233c00a41aacf2))


### Code Refactoring

* Drop unused tree IDs ([#113](https://www.github.com/AccessKit/accesskit/issues/113)) ([ca60770](https://www.github.com/AccessKit/accesskit/commit/ca607702cee13c93fe538d2faec88e474261f7ab))
* String indices are always in UTF-8 code units ([#114](https://www.github.com/AccessKit/accesskit/issues/114)) ([386ca0a](https://www.github.com/AccessKit/accesskit/commit/386ca0a89c42fd201843f617b2fd6b6d1de77f59))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.3.0 to 0.4.0

## [0.3.0](https://www.github.com/AccessKit/accesskit/compare/accesskit_consumer-v0.2.0...accesskit_consumer-v0.3.0) (2021-12-29)


### ⚠ BREAKING CHANGES

* Drop `TreeUpdate::clear` (#96)

### Code Refactoring

* Drop `TreeUpdate::clear` ([#96](https://www.github.com/AccessKit/accesskit/issues/96)) ([38f520b](https://www.github.com/AccessKit/accesskit/commit/38f520b960c6db7b3927b369aee206ee6bc5e8aa))



### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * accesskit bumped from 0.2.0 to 0.3.0

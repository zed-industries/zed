# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 5.2.5 (2025-10-25)

### Bug Fixes

 - <csr-id-23ca2a2ea182fe551ac0d4630ebbb98c3db0abad/> set the `objc2-foundation` to the one that's actually required
   Otherwise, downstream with Cargo.lock may see build failures.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#142](https://github.com/Byron/trash-rs/issues/142)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#142](https://github.com/Byron/trash-rs/issues/142)**
    - Set the `objc2-foundation` to the one that's actually required ([`23ca2a2`](https://github.com/Byron/trash-rs/commit/23ca2a2ea182fe551ac0d4630ebbb98c3db0abad))
</details>

## 5.2.4 (2025-10-24)

### Bug Fixes

 - <csr-id-c3d2d1d094ea4bde31398855b78af20b19abc6f8/> handle cross-device link errors by falling back to copy+delete on Linux

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 69 calendar days.
 - 69 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v5.2.4 ([`e3092ad`](https://github.com/Byron/trash-rs/commit/e3092add67470b904e327d1183c7692c55b65bf1))
    - Merge pull request #141 from muni-corn/cross-device-trash ([`d0784e4`](https://github.com/Byron/trash-rs/commit/d0784e42f99ef572050ffc8bc9abd4002cc9dd78))
    - Refactor and fixes ([`1894bfe`](https://github.com/Byron/trash-rs/commit/1894bfe4ab6677ca833cc60bbd6e568480abbf77))
    - Handle cross-device link errors by falling back to copy+delete on Linux ([`c3d2d1d`](https://github.com/Byron/trash-rs/commit/c3d2d1d094ea4bde31398855b78af20b19abc6f8))
    - Merge branch 'feature-upgrade-objc2' ([`cd22e14`](https://github.com/Byron/trash-rs/commit/cd22e1456bffc6027622cd55f131d43dc5372015))
</details>

## 5.2.3 (2025-08-15)

Updated the `obj2` crate when building or MacOS.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 114 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v5.2.3 ([`a628ccc`](https://github.com/Byron/trash-rs/commit/a628cccc688c6172bd793ef5dd5791fc987402d7))
    - Prepare patch release notes. ([`f9be785`](https://github.com/Byron/trash-rs/commit/f9be7851acc660023ee0f3a6d1e58d04a4731c9f))
    - Upgrade objc2 ([`134c357`](https://github.com/Byron/trash-rs/commit/134c3577e4892878e65fa7af8d24d1910df0fe6e))
    - Thanks clippy ([`b80f7ed`](https://github.com/Byron/trash-rs/commit/b80f7edb1e3db64ae029b02a26d77c11986d9f11))
</details>

## 5.2.2 (2025-02-22)

<csr-id-083743e848ff1b2a61af47bb3afdd8aa04e3eace/>

### Chore

 - <csr-id-083743e848ff1b2a61af47bb3afdd8aa04e3eace/> prepare for objc2 frameworks v0.3
   These will have a bunch of default features enabled, so let's
   pre-emptively disable them.

### Bug Fixes

 - <csr-id-dffb80d0950c2edd52a7883162fa8923393ea5c8/> Use octal for S_ISVTX sticky bit check

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 76 calendar days.
 - 77 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v5.2.2 ([`2ac29d2`](https://github.com/Byron/trash-rs/commit/2ac29d20a74c88d5933c1545dd4dd285910d2478))
    - Merge pull request #136 from nebel/master ([`1704f72`](https://github.com/Byron/trash-rs/commit/1704f72605fd1daf1a61d3caaeb6312260df8367))
    - Use octal for S_ISVTX sticky bit check ([`dffb80d`](https://github.com/Byron/trash-rs/commit/dffb80d0950c2edd52a7883162fa8923393ea5c8))
    - Merge pull request #135 from madsmtm/objc2-disable-default-features ([`74894a8`](https://github.com/Byron/trash-rs/commit/74894a8a7a32e66d2ef02ae70285437184a07bd3))
    - Prepare for objc2 frameworks v0.3 ([`083743e`](https://github.com/Byron/trash-rs/commit/083743e848ff1b2a61af47bb3afdd8aa04e3eace))
    - Merge pull request #132 from eugenesvk/fr-mac-test-out ([`357c3b8`](https://github.com/Byron/trash-rs/commit/357c3b81111c82184ac83b94b74627266314aa82))
    - Move MacOS specific tests to their own directory ([`ee7f256`](https://github.com/Byron/trash-rs/commit/ee7f2562ee5fbc13702b5bfbd55fc0214a9ea6e8))
</details>

## 5.2.1 (2024-12-07)

<csr-id-415c87d81ff859ae40ba5d2e31ffcc44a1ebfffa/>
<csr-id-6fbad98299ffde1acf2a63552d39e4085664d6f1/>
<csr-id-3978204c7b5d7ca1038717da3238c82f7bb6a6c6/>
<csr-id-e58e92baee1f3121114befe73e2a7a1d1dba363e/>
<csr-id-9ed83e724f944f4eacf2e4cafdf8025548f7a17b/>
<csr-id-175d6f5de323b2fed7c8049eaf6bb91266171b30/>
<csr-id-bfbc394a1aba8cb3f348c77f3dffc18a59dde28f/>
<csr-id-dc7dca02ba13b34d57f63244522044a17e88cecc/>
<csr-id-9c213c91817d718b1785b9cd8a52d6c87beef936/>

### Bug Fixes

 - <csr-id-e1bb697a510ec49008d0b4f9a58b38dc061d7901/> Escape quoted paths when deleting with AppleScript
 - <csr-id-6f0b737668c0f9c19e09657e8cbc98caf90e30a9/> Support for non-UTF8 paths on HFS+ on MacOS
   Now illegal UTF8 is percent-encoded. Previously this code would have panicked.

### Other

 - <csr-id-415c87d81ff859ae40ba5d2e31ffcc44a1ebfffa/> add an overview table to DeleteMethod on Mac
 - <csr-id-6fbad98299ffde1acf2a63552d39e4085664d6f1/> move macos deps behind macos cfg target
 - <csr-id-3978204c7b5d7ca1038717da3238c82f7bb6a6c6/> add simdutf8 for fast utf8 validation
 - <csr-id-e58e92baee1f3121114befe73e2a7a1d1dba363e/> add percent encoding support
 - <csr-id-9ed83e724f944f4eacf2e4cafdf8025548f7a17b/> replace create with create_new to avoid potentially nulling existing files

### Test

 - <csr-id-175d6f5de323b2fed7c8049eaf6bb91266171b30/> new delete illegal bytes via Finder
   Disabled since only works on older FS, but tested manually to work on a USB HFS drive
 - <csr-id-bfbc394a1aba8cb3f348c77f3dffc18a59dde28f/> new delete illegal bytes
   Disabled since only works on older FS, but tested manually to work on a USB HFS drive
 - <csr-id-dc7dca02ba13b34d57f63244522044a17e88cecc/> add for from_utf8_lossy_pc
 - <csr-id-9c213c91817d718b1785b9cd8a52d6c87beef936/> replace create with create_new to avoid potentially nulling existing files

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 26 commits contributed to the release.
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v5.2.1 ([`59b0a8a`](https://github.com/Byron/trash-rs/commit/59b0a8a1a1a4625e281b417f7a6e4d4c2a077aea))
    - Escape quoted paths when deleting with AppleScript ([`e1bb697`](https://github.com/Byron/trash-rs/commit/e1bb697a510ec49008d0b4f9a58b38dc061d7901))
    - Support for non-UTF8 paths on HFS+ on MacOS ([`6f0b737`](https://github.com/Byron/trash-rs/commit/6f0b737668c0f9c19e09657e8cbc98caf90e30a9))
    - Various refactors ([`d23a591`](https://github.com/Byron/trash-rs/commit/d23a59166d52c90d7ee02ca2fb356cad0b330eca))
    - Merge pull request #125 from eugenesvk/fr-doc-deletemethod ([`47ed29d`](https://github.com/Byron/trash-rs/commit/47ed29da5773e3f94285ee6f77fe4b9ba484ea3b))
    - Add an overview table to DeleteMethod on Mac ([`415c87d`](https://github.com/Byron/trash-rs/commit/415c87d81ff859ae40ba5d2e31ffcc44a1ebfffa))
    - Clippy ([`b147384`](https://github.com/Byron/trash-rs/commit/b147384fb820a0577d8228b5872d6dac338d4cf5))
    - Cargo fmt ([`e499a0e`](https://github.com/Byron/trash-rs/commit/e499a0e1a49dbf3f2e672d1d42cc7b54944b6d9d))
    - New delete illegal bytes via Finder ([`175d6f5`](https://github.com/Byron/trash-rs/commit/175d6f5de323b2fed7c8049eaf6bb91266171b30))
    - Fix Finder path generation for AS ([`0359a4d`](https://github.com/Byron/trash-rs/commit/0359a4d13975e2fc5717052b2aa23e0394c7b619))
    - Cargo fmt ([`5d17879`](https://github.com/Byron/trash-rs/commit/5d17879c0c823209f05d4cbc21629079c67e51ba))
    - Fix finder extra escaping ([`d7295e8`](https://github.com/Byron/trash-rs/commit/d7295e8edd0934d3d1b3e7e0f9cc745b5d103ce2))
    - Cargo fmt ([`ab5c49b`](https://github.com/Byron/trash-rs/commit/ab5c49ba45d94fdfa1ed6919daf1d61627de8ab3))
    - New delete illegal bytes ([`bfbc394`](https://github.com/Byron/trash-rs/commit/bfbc394a1aba8cb3f348c77f3dffc18a59dde28f))
    - Move macos deps behind macos cfg target ([`6fbad98`](https://github.com/Byron/trash-rs/commit/6fbad98299ffde1acf2a63552d39e4085664d6f1))
    - Convert delete_using_finder to use binary Paths ([`c013b9a`](https://github.com/Byron/trash-rs/commit/c013b9a95bfde455a85dcf5b382ee7a6aaf145a2))
    - Convert delete_using_file_mgr to use binary Paths ([`1d18e7a`](https://github.com/Byron/trash-rs/commit/1d18e7a9ae7a92a909c4dc34b441d799f0466b0b))
    - Remove automatic panicky conversion of potentially binary paths into non-binary strings ([`d7d2187`](https://github.com/Byron/trash-rs/commit/d7d218743dc1e4793a49b0aa95e752e740c2a9c3))
    - Add simdutf8 for fast utf8 validation ([`3978204`](https://github.com/Byron/trash-rs/commit/3978204c7b5d7ca1038717da3238c82f7bb6a6c6))
    - Add for from_utf8_lossy_pc ([`dc7dca0`](https://github.com/Byron/trash-rs/commit/dc7dca02ba13b34d57f63244522044a17e88cecc))
    - Add from_utf8_lossy_pc ([`8481d3c`](https://github.com/Byron/trash-rs/commit/8481d3cccb561e67a98f79ae1bfa8f7ade87d6c4))
    - Add percent encoding support ([`e58e92b`](https://github.com/Byron/trash-rs/commit/e58e92baee1f3121114befe73e2a7a1d1dba363e))
    - Merge pull request #126 from eugenesvk/fr-test-file-new ([`823f6fb`](https://github.com/Byron/trash-rs/commit/823f6fb3856f67a99e40ec943488615eb12d3edc))
    - Try de-clippy ([`243b00d`](https://github.com/Byron/trash-rs/commit/243b00d75f165472f55127cd40cbcbf08f715400))
    - Replace create with create_new to avoid potentially nulling existing files ([`9c213c9`](https://github.com/Byron/trash-rs/commit/9c213c91817d718b1785b9cd8a52d6c87beef936))
    - Replace create with create_new to avoid potentially nulling existing files ([`9ed83e7`](https://github.com/Byron/trash-rs/commit/9ed83e724f944f4eacf2e4cafdf8025548f7a17b))
</details>

## 5.2.0 (2024-10-26)

### New Features

 - <csr-id-6d59fa939429d2eede8b7cf22b2e084bc3c546f4/> Short circuiting check for empty trash
   `is_empty()` is a short circuiting function that checks if the trash is
   empty on Freedesktop compatible systems and Windows.
   
   The main purpose of `is_empty()` is to avoid evaluating the entire trash
   context when the caller is only interested in whether the trash is empty
   or not. This is especially useful for full trashes with many items.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 56 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v5.2.0 ([`1a0fc59`](https://github.com/Byron/trash-rs/commit/1a0fc5908a29c7e648a76ca706e2aa2a40eedda6))
    - Merge pull request #120 from joshuamegnauth54/feat-short-circuiting-is-empty ([`0120bbe`](https://github.com/Byron/trash-rs/commit/0120bbe66889e3659b9f09598f3567dd6c00d4b6))
    - Short circuiting check for empty trash ([`6d59fa9`](https://github.com/Byron/trash-rs/commit/6d59fa939429d2eede8b7cf22b2e084bc3c546f4))
</details>

## 5.1.1 (2024-08-31)

This release accelerates removing large folders by removing an unnecessary recursive check.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 22 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#114](https://github.com/Byron/trash-rs/issues/114)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#114](https://github.com/Byron/trash-rs/issues/114)**
    - Merge pull request #114 from sungsphinx/fix-fedora-atomic ([`3d95173`](https://github.com/Byron/trash-rs/commit/3d95173d19bedf18d8b5b687567707bd99871e19))
 * **Uncategorized**
    - Release trash v5.1.1 ([`a2920fa`](https://github.com/Byron/trash-rs/commit/a2920fa50ad6dec4fc430c48b9837df2f17cd2f4))
    - Adjust changelog prior to release ([`bc3e9c1`](https://github.com/Byron/trash-rs/commit/bc3e9c11426df512e3b056111863f8b410eaf043))
    - Merge pull request #115 from NeumoNeumo/NeumoNeumo-patch-1 ([`df6f3b9`](https://github.com/Byron/trash-rs/commit/df6f3b99728a469f06027b2df486adc631ebc4ba))
    - Accelerate by removing recursive renaming ([`8f8f5c0`](https://github.com/Byron/trash-rs/commit/8f8f5c06b2ce43d30c373311c643f184b7176d9f))
    - Fix trashing files on Fedora Atomic variants ([`4d22ee4`](https://github.com/Byron/trash-rs/commit/4d22ee4852ba9b300489d332c210b920d01db8d9))
</details>

## 5.1.0 (2024-08-09)

### New Features

 - <csr-id-791917843a988396935ceff1eb5c982da6655d80/> check for operation abort

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 52 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v5.1.0 ([`26e55ae`](https://github.com/Byron/trash-rs/commit/26e55aebf89ca3211787c787e8e2b22a412c4203))
    - Merge pull request #113 from anatawa12/master ([`ca6d598`](https://github.com/Byron/trash-rs/commit/ca6d5980216eb9f2e4d709a08e0454502655e454))
    - Check for operation abort ([`7919178`](https://github.com/Byron/trash-rs/commit/791917843a988396935ceff1eb5c982da6655d80))
</details>

## 5.0.0 (2024-06-18)

<csr-id-58b99ef34a0dc6cce11fdc46c9fa18ffb013e33e/>

To support non-UTF8 encoding in paths, the `name` field changed from `String` 
to `OsString` in the `TrashItem` struct. As it's a return value, one won't see
code break unless `name` is actually used.

### Bug Fixes

 - <csr-id-15a15f8ad10791318c6d9de95d4fbaefa345fb56/> Support non-Unicode paths
   There are several spots where paths are assumed to be Unicode. However,
   some (all?) operating systems support non-Unicode paths which causes
   `trash-rs` to panic if encountered. I switched some of those code to use
   `OsString`s instead of `String`s. Unfortunately, I had to add a new
   dependency, `urlencoding`, in order to properly handle decoding non-UTF8
   byte slices.
   
   As of this commit, the test suite passes and code should be ready, but I
   will try to remove the `url` crate and use `urlencoding` in its place
   in the next commit.

### Other

 - <csr-id-58b99ef34a0dc6cce11fdc46c9fa18ffb013e33e/> Use objc2-foundation

### Bug Fixes (BREAKING)

 - <csr-id-0971b8f7f0f1e20ee4356a40ae6b2ba41900c4b3/> Support non-UTF8 paths.
   Note that this changes the type of returned paths to `OsString` from String,
   hence the breaking change.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release over the course of 34 calendar days.
 - 47 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v5.0.0 ([`a754f4a`](https://github.com/Byron/trash-rs/commit/a754f4a8c62737085c31982c1025544f3f36f5e8))
    - Prepare changelog prior to release ([`02d1a8d`](https://github.com/Byron/trash-rs/commit/02d1a8d2494f26b79b907d516c0444df16d3e55a))
    - Support non-UTF8 paths. ([`0971b8f`](https://github.com/Byron/trash-rs/commit/0971b8f7f0f1e20ee4356a40ae6b2ba41900c4b3))
    - Update Windows code to account for API change ([`e4b7119`](https://github.com/Byron/trash-rs/commit/e4b7119fcc369c5594e9e2b5dad8f1a6616593f7))
    - Simplify Linux/BSD only tests for non-UTF8 paths ([`559b57b`](https://github.com/Byron/trash-rs/commit/559b57bc1497d2a49ca4f463cc27f6c94697939c))
    - Impl test for listing invalid UTF8 trash items ([`209db9d`](https://github.com/Byron/trash-rs/commit/209db9d76de1f233b05b10f5f3f008b5968b0232))
    - Cleanup non-Unicode support for readability ([`2f31116`](https://github.com/Byron/trash-rs/commit/2f311164ff44077dc5450ebc0f14c29f70fe57d7))
    - Remove `url` and replace with `urlencoding` ([`67fb256`](https://github.com/Byron/trash-rs/commit/67fb2568384b7ebd96acba54e40236d9f3e9eb07))
    - Support non-Unicode paths ([`15a15f8`](https://github.com/Byron/trash-rs/commit/15a15f8ad10791318c6d9de95d4fbaefa345fb56))
    - Merge pull request #107 from madsmtm/objc2 ([`46585ce`](https://github.com/Byron/trash-rs/commit/46585ceacc3799f74ce9793e6d0669eb4e48b3f8))
    - Use objc2-foundation ([`58b99ef`](https://github.com/Byron/trash-rs/commit/58b99ef34a0dc6cce11fdc46c9fa18ffb013e33e))
</details>

## 4.1.1 (2024-05-01)

This release updates the `windows` dependency (on Windows) to v0.56.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v4.1.1 ([`c14d904`](https://github.com/Byron/trash-rs/commit/c14d904864b4c030cc8ce9d8d394c719f40c5a5b))
    - Update changelog prior to release. ([`47baa0e`](https://github.com/Byron/trash-rs/commit/47baa0ed3aaefa4167b1a972db54223ac710cb8d))
    - Merge pull request #106 from YizhePKU/bump-windows ([`02f1e6c`](https://github.com/Byron/trash-rs/commit/02f1e6c5620f9a0bbeae68246cb2b180f946a1be))
    - Bump windows crate to 0.56.0 ([`c0e0f7a`](https://github.com/Byron/trash-rs/commit/c0e0f7a6397bdb65de013ca8e2b58c6ea7ab73af))
</details>

## 4.1.0 (2024-03-19)

### New Features

 - <csr-id-eb659cb0ef5401c7ac1fb514c5d639c10464b730/> add `os_limited::trash_folders()` for use on many unixes.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 7 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v4.1.0 ([`35b549f`](https://github.com/Byron/trash-rs/commit/35b549f158428f0016a26c23249c76c6795ba9c3))
    - Add `os_limited::trash_folders()` for use on many unixes. ([`eb659cb`](https://github.com/Byron/trash-rs/commit/eb659cb0ef5401c7ac1fb514c5d639c10464b730))
    - Fix lint on Windows ([`daad5b7`](https://github.com/Byron/trash-rs/commit/daad5b7ad04192c7fac48b697e90ca40fb0cb94c))
    - `trash_folders()` is invalid on Windows ([`32719fb`](https://github.com/Byron/trash-rs/commit/32719fbc82b572b77cd80d4f3645bb44ebca4640))
    - List valid trash bin paths ([`3eba5c3`](https://github.com/Byron/trash-rs/commit/3eba5c36354eb246e74e31cab655d361313ff3e5))
</details>

## 4.0.0 (2024-03-12)

### Bug Fixes (BREAKING)

 - <csr-id-146ea03fe1c1c168b8a6fd135d9dc5c5c93f35d5/> Assure directory deletions on Windows don't put the entire contents into the trash.
   Instead, like on other platforms, on Windows it will now put the folder into the trash instead.
   
   Please note that this is not a breaking change in terms of API, but a *potentially* breaking change with older Windows versions. It's unknown if there are side-effects, as it's unknown why Windows had special behaviour previously.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 28 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v4.0.0 ([`b6acac9`](https://github.com/Byron/trash-rs/commit/b6acac9ef574649e381d10e3930348b4385ff551))
    - Assure directory deletions on Windows don't put the entire contents into the trash. ([`146ea03`](https://github.com/Byron/trash-rs/commit/146ea03fe1c1c168b8a6fd135d9dc5c5c93f35d5))
    - Fix lint by removing unused code ([`d03934f`](https://github.com/Byron/trash-rs/commit/d03934f668d1d405c2030685c318ba19b825c74e))
    - Attempt to fix second argument issue ([`1435c3d`](https://github.com/Byron/trash-rs/commit/1435c3d566970f5366e990cdea512e1d0a6c738d))
    - Make delete_all not recursive ([`03ae59d`](https://github.com/Byron/trash-rs/commit/03ae59d7473746e2d704aabab0d1065d3b9a6f58))
</details>

## 3.3.1 (2024-02-12)

### Bug Fixes

 - <csr-id-98049f1316e3902f2c9d5cd51f8de14b86ec5828/> Use `AtomicI32` instead of I64 for compatibility with `armel`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#99](https://github.com/Byron/trash-rs/issues/99)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#99](https://github.com/Byron/trash-rs/issues/99)**
    - Use `AtomicI32` instead of I64 for compatibility with `armel` ([`98049f1`](https://github.com/Byron/trash-rs/commit/98049f1316e3902f2c9d5cd51f8de14b86ec5828))
 * **Uncategorized**
    - Release trash v3.3.1 ([`b6e2d6c`](https://github.com/Byron/trash-rs/commit/b6e2d6c57f499a1851e8b2e4a724b1e0ef5ae54d))
</details>

## 3.3.0 (2024-02-10)

### New Features

 - <csr-id-452be8303c797f44409b487c0cf1e6ffb2899110/> improved error granularity
   Inform about operating-system specific errors more clearly, thus avoid degenerating error information.

### Bug Fixes

 - <csr-id-920ff0c69f6d0309e73f86aaa437aec9508cc873/> Use `AtomicI32` in tests for compatibility with `armel` platform

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 5 calendar days.
 - 25 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#99](https://github.com/Byron/trash-rs/issues/99)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#99](https://github.com/Byron/trash-rs/issues/99)**
    - Use `AtomicI32` in tests for compatibility with `armel` platform ([`920ff0c`](https://github.com/Byron/trash-rs/commit/920ff0c69f6d0309e73f86aaa437aec9508cc873))
 * **Uncategorized**
    - Release trash v3.3.0 ([`d0d8f26`](https://github.com/Byron/trash-rs/commit/d0d8f26030e0936aa57aa1d0d4e1a34f6a91f5b9))
    - Improved error granularity ([`452be83`](https://github.com/Byron/trash-rs/commit/452be8303c797f44409b487c0cf1e6ffb2899110))
    - Removed tracing. ([`2b1c9fa`](https://github.com/Byron/trash-rs/commit/2b1c9fa2a9743c1d5477bf5512ba0f260cfdacb5))
    - Bug fix for macOS. ([`b238938`](https://github.com/Byron/trash-rs/commit/b238938d7d6387d7340f9c6a30025c9255973180))
    - Enhanced error reporting. ([`671cef9`](https://github.com/Byron/trash-rs/commit/671cef91f4e3c216f84683e07c82c5849d641b3b))
</details>

## 3.2.1 (2024-01-15)

### Bug Fixes

 - <csr-id-bb868d6812988b56082c2faea083617402e1a259/> find best-possible trash dir, e.g. use `/run/foo/.trash` instead of`/run/.trash` when deleting `/run/foo/bar`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 5 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v3.2.1 ([`d7abb5b`](https://github.com/Byron/trash-rs/commit/d7abb5bb735827b88479fc4879dcfcdcae6e08df))
    - Find best-possible trash dir, e.g. use `/run/foo/.trash` instead of`/run/.trash` when deleting `/run/foo/bar`. ([`bb868d6`](https://github.com/Byron/trash-rs/commit/bb868d6812988b56082c2faea083617402e1a259))
    - Refactor ([`8cb3f75`](https://github.com/Byron/trash-rs/commit/8cb3f7519b1294fe8b2e03c0f51fd129bb9f4cf4))
    - Cargo fmt ([`0b42fc0`](https://github.com/Byron/trash-rs/commit/0b42fc06b44e076aa7aebaee6f8730bc762ee5ed))
    - Use unstable sort ([`18dadef`](https://github.com/Byron/trash-rs/commit/18dadef0dd39bf3e57450fbf4a7098688fb81df0))
    - Fixing method os ([`8ba855e`](https://github.com/Byron/trash-rs/commit/8ba855e4bf9982e8b4be993d8df59739b88d72c6))
    - Sort mount points first ([`b2e4cf2`](https://github.com/Byron/trash-rs/commit/b2e4cf202e108bb419d7a7e5959b45408dac836c))
    - Refactor ([`da8ce63`](https://github.com/Byron/trash-rs/commit/da8ce63afd331b4e41455be0587a2736c42815bd))
    - Fix clippy error ([`8f74b17`](https://github.com/Byron/trash-rs/commit/8f74b1789a2257ba5a7acda560f1811df8f5f1ea))
    - Fixing sometimes choosing incorrect mount point if substring of each other ([`1e9df03`](https://github.com/Byron/trash-rs/commit/1e9df0347cd1298844222a43a6424400e7dc787b))
</details>

## 3.2.0 (2024-01-10)

<csr-id-be43b098c6c4db66f19c90471cd6ff0c066832ef/>

### New Features

 - <csr-id-aa8e5043e285d31644e697aa264f8a11e5dfa2e8/> provide `os_limited::metadata()`.
   Metadata is currently limited to the amount of things, like bytes or entries,
   in the metadata item, but there is potential for adding more later.

### Other

 - <csr-id-be43b098c6c4db66f19c90471cd6ff0c066832ef/> update ci job to use cargo-cross

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v3.2.0 ([`03aa7ac`](https://github.com/Byron/trash-rs/commit/03aa7ac1fc279d1cb598c451d8ef342d13232489))
    - Provide `os_limited::metadata()`. ([`aa8e504`](https://github.com/Byron/trash-rs/commit/aa8e5043e285d31644e697aa264f8a11e5dfa2e8))
    - Refactor ([`8dad3df`](https://github.com/Byron/trash-rs/commit/8dad3dfc45657962a57a932c40bc37ea1ebe0d7f))
    - Address review comments ([`63639c3`](https://github.com/Byron/trash-rs/commit/63639c3337cc282a1aaa69ef5afd00f8516e3dcd))
    - Stub for get_mount_points on unsupported targets ([`fd89ea5`](https://github.com/Byron/trash-rs/commit/fd89ea5d780fa111d12fbe6644dc4153a78565c5))
    - Windows implementation ([`1a1f75e`](https://github.com/Byron/trash-rs/commit/1a1f75e59b4c18abdf6bc8790a4e54b53dff50df))
    - Add metadata function, implement for freedesktop ([`3bea3e2`](https://github.com/Byron/trash-rs/commit/3bea3e2f11d5def136455e7bc2377cb05b80147e))
    - Merge pull request #92 from TD-Sky/unknown-to-fs-error ([`916d769`](https://github.com/Byron/trash-rs/commit/916d7698ebceb0529fa3c43f6baddbd4c39d55f2))
    - Accepting generic type instead of `&Path` ([`17411be`](https://github.com/Byron/trash-rs/commit/17411be41b96f4a81df8a9cc6fa558d0d250c749))
    - Be consistent with the style of the project ([`7ee2617`](https://github.com/Byron/trash-rs/commit/7ee26179e59c4920b83fffed20a049e9171e4878))
    - Keep error converter function and rename it `fs_error` ([`a08118c`](https://github.com/Byron/trash-rs/commit/a08118cf2a924a3224b05d76dd5b012036ef5e05))
    - More precise file system error ([`c51aa78`](https://github.com/Byron/trash-rs/commit/c51aa7820c70e6d5fc4d408f5c01cd4c8701c59d))
    - Merge pull request #90 from fujiapple852/build-add-cargo-cross-ci ([`695af32`](https://github.com/Byron/trash-rs/commit/695af324e6ddaee00ea0ee5e44c7d815fd1158ec))
    - Update ci job to use cargo-cross ([`be43b09`](https://github.com/Byron/trash-rs/commit/be43b098c6c4db66f19c90471cd6ff0c066832ef))
</details>

## 3.1.2 (2023-10-18)

This release fixes compile errors on DragonFly, a fork of FreeBSD.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v3.1.2 ([`609f6b3`](https://github.com/Byron/trash-rs/commit/609f6b39f6a743e1cbd9226c873b0463730e10ed))
    - Prepare changelog ([`c81d4dd`](https://github.com/Byron/trash-rs/commit/c81d4ddccc9bd30b5baecb9c69f01437b467a703))
    - Merge pull request #89 from jbeich/dragonfly ([`ad26100`](https://github.com/Byron/trash-rs/commit/ad261004b4fe350bf7963cc4354e4b5808c61156))
    - Add DragonFly support via FreeBSD codepath ([`ed1984b`](https://github.com/Byron/trash-rs/commit/ed1984b923a7cdd7dbf03484d02b5da07e27779c))
</details>

## 3.1.1 (2023-10-18)

### Bug Fixes

 - <csr-id-1a5bc2de178ca76fe06631a09305e4f014764084/> compilation on FreeBSD should work now. #(86)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 9 calendar days.
 - 9 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v3.1.1 ([`aa6fd20`](https://github.com/Byron/trash-rs/commit/aa6fd20ec75585f7cda5e8745a6cafd2c5b26e91))
    - Compilation on FreeBSD should work now. #(86) ([`1a5bc2d`](https://github.com/Byron/trash-rs/commit/1a5bc2de178ca76fe06631a09305e4f014764084))
    - Update freedesktop.rs ([`aa7b7fd`](https://github.com/Byron/trash-rs/commit/aa7b7fd66573631cf17b031b90e5e0139f0fdab6))
    - Restore statfs for FreeBSD & OpenBSD ([`1562113`](https://github.com/Byron/trash-rs/commit/1562113e12f9020a9c3f866e5adf5e913f4040e6))
    - Update version in README so it matches the latest published one ([`50e8030`](https://github.com/Byron/trash-rs/commit/50e80304845cbae953b4ecf370c715d728ea9958))
</details>

## 3.1.0 (2023-10-08)

<csr-id-554c2735c8dd924fd7cebe863b529d91bb0cac0d/>

### New Features

 - <csr-id-24e0cb6f9fe15a0db1609e04cda6446e3335f89b/> compatibility with OpenBSD and NetBSD
 - <csr-id-0789b23c6c8e21bc1493455beaca75d46e0aa575/> allow passing in items' ownership or reference

### Other

 - <csr-id-554c2735c8dd924fd7cebe863b529d91bb0cac0d/> describe how to retry restoring when encountering `RestoreCollision` error

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 88 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#84](https://github.com/Byron/trash-rs/issues/84)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#84](https://github.com/Byron/trash-rs/issues/84)**
    - Compatibility with OpenBSD and NetBSD ([`24e0cb6`](https://github.com/Byron/trash-rs/commit/24e0cb6f9fe15a0db1609e04cda6446e3335f89b))
 * **Uncategorized**
    - Release trash v3.1.0 ([`be17cd2`](https://github.com/Byron/trash-rs/commit/be17cd20bb32ab00ceb72cd9afc3ddaed01cacdb))
    - Bump minor version to indicate a feature change ([`ddb9917`](https://github.com/Byron/trash-rs/commit/ddb99171715727a3339d4a9e2f07a517037b01db))
    - Merge pull request #81 from TD-Sky/re-restore ([`c87a946`](https://github.com/Byron/trash-rs/commit/c87a9467235e6208e2268d392ac61f332b4d1d09))
    - Test edition bump ([`b77bd6d`](https://github.com/Byron/trash-rs/commit/b77bd6d32f8d44f59b9fe53806248d0b0860aa18))
    - Bump version ([`75cc270`](https://github.com/Byron/trash-rs/commit/75cc27093d01628fb79acb1432c8ccdd66d86b2f))
    - Update dependencies ([`7d1e2bb`](https://github.com/Byron/trash-rs/commit/7d1e2bb0a51d88033428aad62bf87e400c2a334d))
    - One step closer ([`aee3dce`](https://github.com/Byron/trash-rs/commit/aee3dceac5575e4a2a23633ec5f3da5da79d9e89))
    - Allow passing in items' ownership or reference ([`0789b23`](https://github.com/Byron/trash-rs/commit/0789b23c6c8e21bc1493455beaca75d46e0aa575))
    - Describe how to retry restoring when encountering `RestoreCollision` error ([`554c273`](https://github.com/Byron/trash-rs/commit/554c2735c8dd924fd7cebe863b529d91bb0cac0d))
</details>

## 3.0.6 (2023-07-12)

### Bug Fixes

 - <csr-id-3f5e8427cbf299322d66b358ec3fa61ca4a5d66c/> don't recurse into symlink when trashing a directory on windows.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 5 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v3.0.6 ([`450edc1`](https://github.com/Byron/trash-rs/commit/450edc1a0d372ae450daf3aec33aabedd3efde3d))
    - Merge branch 'fix-symlink-traversal' ([`43d44cb`](https://github.com/Byron/trash-rs/commit/43d44cbe0979c92cbc117723387d762ecd9d3191))
    - Don't recurse into symlink when trashing a directory on windows. ([`3f5e842`](https://github.com/Byron/trash-rs/commit/3f5e8427cbf299322d66b358ec3fa61ca4a5d66c))
    - Inform about reason for yanking v3.0.5 ([`112e99e`](https://github.com/Byron/trash-rs/commit/112e99ecfd485c5115323b185efc5979eae26edc))
</details>

## 3.0.5 (2023-07-06)

YANKED: It was discovered that symlinks aren't handled correctly, which can lead to removals of unrelated directory trees.

### Bug Fixes

 - <csr-id-c1feece952dcd70163ed06ac2af79fdbb3d692bc/> On **windows**, `delete()` will now delete recursively like on the other platforms.
   Note that the current implementation may consume a lot of memory as it will traverse the
   entire directory structure once while storing each path for later trashing.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 1 calendar day.
 - 4 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v3.0.5 ([`4655a07`](https://github.com/Byron/trash-rs/commit/4655a0723ab4209872d4037be89d2b0876a70731))
    - Upgrade serial-test crate ([`0354d36`](https://github.com/Byron/trash-rs/commit/0354d36b7f870317cf57711624fd31054ffc946e))
    - On **windows**, `delete()` will now delete recursively like on the other platforms. ([`c1feece`](https://github.com/Byron/trash-rs/commit/c1feece952dcd70163ed06ac2af79fdbb3d692bc))
    - Refactor ([`41edcdf`](https://github.com/Byron/trash-rs/commit/41edcdfc8bdeb410b45ae636da25e3c7275a8a8c))
    - Removed self as parameter only used in recurssion. ([`a7619c1`](https://github.com/Byron/trash-rs/commit/a7619c13215daaf88316f7e1876cf59c96491cf4))
    - Reorganized code for cross-platform compatibility. ([`1c09e48`](https://github.com/Byron/trash-rs/commit/1c09e48c7977704b1a8d67078c84ed30b17c983a))
    - Use recursive deletion on Windows by default. ([`46e0697`](https://github.com/Byron/trash-rs/commit/46e0697c649f9e8184654e47f18f6b2930b6bd67))
    - Removed Windows only restriction for recursive deletion test. ([`d363dd8`](https://github.com/Byron/trash-rs/commit/d363dd840a0d35348b427ff6d1f6def568e008ed))
    - Merge branch 'Byron:master' into bug/windows_nonempty_folder ([`0f4b2c8`](https://github.com/Byron/trash-rs/commit/0f4b2c81a209f70592b33675144c1d7922433741))
</details>

## 3.0.4 (2023-07-01)

### Bug Fixes

 - <csr-id-55b0d5c86e2608552836ec0bf3e9aa0ce8c303b8/> Don't use 'oldtime' feature of `chrono` by controlling exactly which features are enabled.
   That particular feature has [a rustsec advisory](https://rustsec.org/advisories/RUSTSEC-2020-0071) up
   against it.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 19 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#75](https://github.com/Byron/trash-rs/issues/75)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#75](https://github.com/Byron/trash-rs/issues/75)**
    - Don't use 'oldtime' feature of `chrono` by controlling exactly which features are enabled. ([`55b0d5c`](https://github.com/Byron/trash-rs/commit/55b0d5c86e2608552836ec0bf3e9aa0ce8c303b8))
 * **Uncategorized**
    - Release trash v3.0.4 ([`a2343c2`](https://github.com/Byron/trash-rs/commit/a2343c2692aa8d6b5fc8684a654349a14094486b))
    - Don't use `oldtime` feature of chrono ([`fad81a4`](https://github.com/Byron/trash-rs/commit/fad81a4992fe053e30113f9ab0c7001d12b1ec17))
</details>

## 3.0.3 (2023-06-11)

### Bug Fixes

 - <csr-id-aa8cd7b05f8f0641d7fd73328619c2c45c7e050c/> disallow empty paths from being deleted.
   Previously passing "" for deletion wuold delete the current working directory
   as it would canonicalize any input path without validating the path is non-empty.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 11 calendar days.
 - 25 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#73](https://github.com/Byron/trash-rs/issues/73)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#73](https://github.com/Byron/trash-rs/issues/73)**
    - Disallow empty paths from being deleted. ([`aa8cd7b`](https://github.com/Byron/trash-rs/commit/aa8cd7b05f8f0641d7fd73328619c2c45c7e050c))
 * **Uncategorized**
    - Release trash v3.0.3 ([`841bc13`](https://github.com/Byron/trash-rs/commit/841bc1388959ab3be4f05ad1a90b03aa6bcaea67))
    - Fix issue #70.Added recursive removal on Windows. ([`05e0cf4`](https://github.com/Byron/trash-rs/commit/05e0cf442354b3b2b9ecfb8ed2b165b8547bc794))
</details>

## 3.0.2 (2023-05-17)

### Bug Fixes

 - <csr-id-75daea606cbdbc4d15a514bb674591d986e57490/> broken symlinks won't cause failure anymore on freedesktop platforms.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v3.0.2 ([`e20fe6a`](https://github.com/Byron/trash-rs/commit/e20fe6ae94aa73d07ff31d911ad9ecf98b17f3a8))
    - Broken symlinks won't cause failure anymore on freedesktop platforms. ([`75daea6`](https://github.com/Byron/trash-rs/commit/75daea606cbdbc4d15a514bb674591d986e57490))
    - Make `virtually_exists` private ([`454a77e`](https://github.com/Byron/trash-rs/commit/454a77e667b00a0aeb492dab9a81e69e77178802))
    - Operate broken symbolic links is safe now ([`9198013`](https://github.com/Byron/trash-rs/commit/919801376bc44fa3c4948349690c7e912be2dd3a))
</details>

## 3.0.1 (2023-01-30)

<csr-id-865a7c6d688cc6dd00dc8b16cd0e4a4fd60d953c/>

### Chore

 - <csr-id-865a7c6d688cc6dd00dc8b16cd0e4a4fd60d953c/> bump `windows` crate to 0.44
   Merge branch 'bump-windows-0.44'

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 61 calendar days.
 - 64 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v3.0.1 ([`eef463a`](https://github.com/Byron/trash-rs/commit/eef463aca73d5c623dd7b52bcb8b01b3b3d76b15))
    - Bump `windows` crate to 0.44 ([`865a7c6`](https://github.com/Byron/trash-rs/commit/865a7c6d688cc6dd00dc8b16cd0e4a4fd60d953c))
    - Thanks clippy ([`37dedb3`](https://github.com/Byron/trash-rs/commit/37dedb35ed71e4c43af3af7d39ae5d722c8b5a94))
    - Update `windows` crate to `0.44` ([`1a347fc`](https://github.com/Byron/trash-rs/commit/1a347fcce57627dd71979ca8399dedba149f9569))
    - Add `Error::FileSystem` ([`575b8ed`](https://github.com/Byron/trash-rs/commit/575b8ed4c78b76e9ecdf4fe877b6e32cd74cf166))
</details>

## 3.0.0 (2022-11-27)

<csr-id-a024b44b6e1cd4a357ffabda8f31e82dcc7e78cb/>

### Chore (BREAKING)

 - <csr-id-a024b44b6e1cd4a357ffabda8f31e82dcc7e78cb/> Upgrade from `windows` v0.37 to v0.43.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v3.0.0 ([`1fb5ad6`](https://github.com/Byron/trash-rs/commit/1fb5ad628868f1480510efe10bdc021ce65b4f32))
    - Upgrade from `windows` v0.37 to v0.43. ([`a024b44`](https://github.com/Byron/trash-rs/commit/a024b44b6e1cd4a357ffabda8f31e82dcc7e78cb))
    - Fix Clippy failures on Linux ([`538dea0`](https://github.com/Byron/trash-rs/commit/538dea0e77af2ed70c6f8b17c86b956b8caa6459))
    - Upgrade windows crate from v0.37 to v0.43 ([`48cdc67`](https://github.com/Byron/trash-rs/commit/48cdc67d09e20f8d07438e45d3ceefd23da6af9a))
    - Derive Clone for TrashItem ([`fcf6bb5`](https://github.com/Byron/trash-rs/commit/fcf6bb5eded49de4fedb40513c949f11c6da0b12))
</details>

## 2.1.5 (2022-07-05)

### Bug Fixes

 - <csr-id-67244ba2e4c71135b0ab36331dc465615e23211a/> Make chrono a default-enabled optional feature.
   This allows to turn chrono support off without actually affecting the
   ability to trash and restore items.
   `chrono` still has issues to dubious local-time support which relies
   on a c-library function that can cause undefined behaviour as it
   accesses an environment variable in a non-threadsafe fashion.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 40 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#39](https://github.com/Byron/trash-rs/issues/39)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#39](https://github.com/Byron/trash-rs/issues/39)**
    - Make chrono a default-enabled optional feature. ([`67244ba`](https://github.com/Byron/trash-rs/commit/67244ba2e4c71135b0ab36331dc465615e23211a))
 * **Uncategorized**
    - Release trash v2.1.5 ([`266d780`](https://github.com/Byron/trash-rs/commit/266d7808d2309f0911ebc6c8a0189511c4e77835))
    - Improve CI stage names; fix feature configuration on windows ([`5591fda`](https://github.com/Byron/trash-rs/commit/5591fdab131de1f6fa5a04bef44d7b394d3f7f72))
    - Silence clippy ([`d13be48`](https://github.com/Byron/trash-rs/commit/d13be48c59a1a0df3e37aa676cda06cc1f48ece9))
    - Add rust-cache for faster builds ([`676a43f`](https://github.com/Byron/trash-rs/commit/676a43f7ec7c116a7b40dcf4236bf2156a88fd04))
</details>

## 2.1.4 (2022-05-25)

### Fixes

- upgrade the `windows` crate to v0.37 to resolve [a build issue](https://github.com/Byron/trash-rs/issues/39) and lay the foundation
  for more regular updates of the windows support.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 8 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#39](https://github.com/Byron/trash-rs/issues/39), [#51](https://github.com/Byron/trash-rs/issues/51)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#39](https://github.com/Byron/trash-rs/issues/39)**
    - Prepare changelog ([`7816e07`](https://github.com/Byron/trash-rs/commit/7816e07bab38a79aa6f5d705a4fb40f330ac155b))
 * **[#51](https://github.com/Byron/trash-rs/issues/51)**
    - Upgrade windows crate ([`d18f9d4`](https://github.com/Byron/trash-rs/commit/d18f9d435d2f76fb982f4bfcc98d5ccfe57c092c))
 * **Uncategorized**
    - Release trash v2.1.4 ([`17d162f`](https://github.com/Byron/trash-rs/commit/17d162fcf7a53d3d82961a448d4b70b4eb596825))
</details>

## 2.1.3 (2022-05-17)

### Fixes

- include `windows` crate only on windows for reduced CI build times from ~9s to ~4s.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#5050505050](https://github.com/Byron/trash-rs/issues/5050505050)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#5050505050](https://github.com/Byron/trash-rs/issues/5050505050)**
    - Update changelog ([`8e64f34`](https://github.com/Byron/trash-rs/commit/8e64f34bd6f1b823353fae61d60f765615be0024))
 * **Uncategorized**
    - Release trash v2.1.3 ([`f98bc45`](https://github.com/Byron/trash-rs/commit/f98bc45199cbb24525d2b41c748b9547f3c3ac44))
    - Merge pull request #50 from rgwood/windows-dep ([`883c5a4`](https://github.com/Byron/trash-rs/commit/883c5a48c8ad07bef4f7e1822a31761211cf304d))
    - Add names to CI steps ([`ef7003a`](https://github.com/Byron/trash-rs/commit/ef7003a4f83910f318b05a3f51960a33fd444915))
    - Only use `windows` crate on Windows ([`e088525`](https://github.com/Byron/trash-rs/commit/e088525047a14a531d414fe9cd098e08fe2ff79f))
</details>

## 2.1.2 (2022-05-13)

### Bug Fixes

 - <csr-id-367cf5f2616f1f49b115189b3bede3bb99f8324d/> avoid inconsistency when using relative paths in trashed file info.
   We use absolute paths now without trying to generate a relative path
   based on some top directory as the latter seems to be causing
   inconsistencies on some linux distros, as the restore path ends
   up being incorrect.
   
   Rather go with the absolute truth and don't fiddle with path
   transformations at all to make it work everywhere.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#39](https://github.com/Byron/trash-rs/issues/39)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#39](https://github.com/Byron/trash-rs/issues/39)**
    - Avoid inconsistency when using relative paths in trashed file info. ([`367cf5f`](https://github.com/Byron/trash-rs/commit/367cf5f2616f1f49b115189b3bede3bb99f8324d))
 * **Uncategorized**
    - Release trash v2.1.2 ([`e0746f0`](https://github.com/Byron/trash-rs/commit/e0746f0df91623231d13531ec33632f03f0588ac))
</details>

## 2.1.1 (2022-05-10)

### Bug Fixes

 - <csr-id-dcda6df8cefa06bf08e7eca7db2c34b050c2d913/> Properly reconstruct paths when restoring files on freedesktop if those were relative.
   
   Previously it would be unable to reconstruct original paths if the trash
   directory was on a mount point due to a 'split brain' of sorts.
   
   When trashing files it would create original path information based
   on them being relative to a mount point, but when restoring them
   it would reconstruct them to be relative to the trash top level
   directory.
   
   Now the reconstruction happens against to mount point itself which makes
   restoration match.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#47](https://github.com/Byron/trash-rs/issues/47)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#47](https://github.com/Byron/trash-rs/issues/47)**
    - Properly reconstruct paths when restoring files on freedesktop if those were relative ([`dcda6df`](https://github.com/Byron/trash-rs/commit/dcda6df8cefa06bf08e7eca7db2c34b050c2d913))
    - Somewhat hard-code special case for fedora ([`90f0f9b`](https://github.com/Byron/trash-rs/commit/90f0f9b035678efe51a20d4a47fd09158b8ef455))
    - Proper cleanup after potential assertion failure ([`1f3a600`](https://github.com/Byron/trash-rs/commit/1f3a6005eabd4629fe0743030a612a29fcb7d80c))
    - Remove unused trait ([`ac913d8`](https://github.com/Byron/trash-rs/commit/ac913d83ed9344d8ed8e18957b2e99136e0b29c1))
 * **Uncategorized**
    - Release trash v2.1.1 ([`50ab31a`](https://github.com/Byron/trash-rs/commit/50ab31afa9f641a16a1ab50bf1ea8f8bacb0330f))
    - Update changelog ([`98d32c8`](https://github.com/Byron/trash-rs/commit/98d32c88e85b2b40ea17d372c427ef168ad80b30))
    - More robust removal of test files in failure case on os specific tests ([`3f6502d`](https://github.com/Byron/trash-rs/commit/3f6502db02e09e36c2fbce2fea054a9a2b9229de))
</details>

## 2.1.0 (2022-05-06)

### Fixes

- Leading directories are now created on linux to avoid errors when trashing nested directories.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 103 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#45](https://github.com/Byron/trash-rs/issues/45), [#47](https://github.com/Byron/trash-rs/issues/47)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#45](https://github.com/Byron/trash-rs/issues/45)**
    - Reproduce issue with lack of leading directories and fix it ([`d5b6faa`](https://github.com/Byron/trash-rs/commit/d5b6faa81d59ccd6185261399bc7449432b9deb6))
 * **[#47](https://github.com/Byron/trash-rs/issues/47)**
    - Try to reproduce ([`8eba501`](https://github.com/Byron/trash-rs/commit/8eba50155e006cf923d8bb77fea88cde6395512e))
 * **Uncategorized**
    - Release trash v2.1.0 ([`b3a4547`](https://github.com/Byron/trash-rs/commit/b3a45471ce5fcd489a096145e06ac663ed854747))
    - Prepare upcoming release ([`e3bbb6b`](https://github.com/Byron/trash-rs/commit/e3bbb6be1072675c331176e8d0585cc67910d17b))
    - Merge branch 'refactor-tests' ([`0e90cac`](https://github.com/Byron/trash-rs/commit/0e90cace515344c68eead8e59180487561849289))
    - Assure tests don't race ([`d9778ba`](https://github.com/Byron/trash-rs/commit/d9778ba1912c5764cbfaa9c46b2bba5c3d1899eb))
    - Thanks clippy ([`220a216`](https://github.com/Byron/trash-rs/commit/220a2164e86bf7f0e1e636d24595b6ce4182de14))
    - Move all intergration tests into corresponding location ([`e5dc62e`](https://github.com/Byron/trash-rs/commit/e5dc62ee2b363a11e57e4aad2c1d128d2f8961e2))
</details>

## 2.0.4 (2022-01-23)

We detected the possibility of UB in the Linux and FreeBSD versions of `get_mount_points()` and reduced the likelihood
of it happening in a multi-threaded environment by synchronizing access. You can read more about the state of
a more permanent fix [in the tracking issue](https://github.com/Byron/trash-rs/issues/42).

All previous 2.0.* releases which contained this function were yanked from crates-io.

### Fixes

* Make internal `get_mount_points()` thread-safe to reduce chance of UB greatly. 
  This may reduce performance of crates that are using trash from multiple threads somewhat, as a part of the operation
  is now synchronized.
* Fix build on FreeBSD, handle UB similarly to the above.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 30 calendar days.
 - 30 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release trash v2.0.4 ([`c7edcb1`](https://github.com/Byron/trash-rs/commit/c7edcb175dd125bda5b15e726fc7b36eae3c89a4))
    - Prepare changelog for next release ([`b65f574`](https://github.com/Byron/trash-rs/commit/b65f574d5aeb8ea3a918e8288c8d13dd082b8f0a))
    - Add Mutex to linux version of get_mount_points(); document UB chance in lib.rs ([`c5c9c5e`](https://github.com/Byron/trash-rs/commit/c5c9c5e40d345736df7d078bf8e6991acc701e83))
    - Use Mutex to prevent concurrent access to getmntinfo ([`5c8e0ce`](https://github.com/Byron/trash-rs/commit/5c8e0ce1c700c68fc63c612cc0ea5b3191f6b0d1))
    - Merge pull request #43 from wezm/num-threads-freebsd ([`8f10c85`](https://github.com/Byron/trash-rs/commit/8f10c852bd9ec2e69353a0dd5397fab1c4ba089f))
    - Fix build on FreeBSD after refactor ([`f3d31e5`](https://github.com/Byron/trash-rs/commit/f3d31e54dd93c22605e8178958a1caa503be19f4))
    - Use `num_threads()` to avoid UB in FreeBSD version of get_mount_points() ([`3c153ae`](https://github.com/Byron/trash-rs/commit/3c153ae2f1ed92d8a240a742e90fcb0e483284b8))
    - Refactor ([`92ab7b9`](https://github.com/Byron/trash-rs/commit/92ab7b91adcde3305cc3e319fb0b59feff8f81cc))
    - Add BSD compatible implementation of get_mount_points ([`82d2132`](https://github.com/Byron/trash-rs/commit/82d2132f8e1323272f5d8e1f54112589f75c3202))
    - Run `cargo-diet` for a more minimal crates package ([`561f21d`](https://github.com/Byron/trash-rs/commit/561f21d9de2a56cb0f0c87002d2ead3dc8ca6ab2))
</details>

## 2.0.3 (2021-12-23)

### Bug Fixes

 - <csr-id-cb5b6176aa296853f7a6e3cfa177e1235acaa903/> let dependency specification in Cargo.toml match cfg directives in code
   This fixes [issue 40](https://github.com/Byron/trash-rs/issues/40).

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#37](https://github.com/Byron/trash-rs/issues/37), [#40](https://github.com/Byron/trash-rs/issues/40)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#37](https://github.com/Byron/trash-rs/issues/37)**
    - Fix some clippy warnings ([`3c566ef`](https://github.com/Byron/trash-rs/commit/3c566ef417350b75e02ea80be51165815014ec74))
 * **[#40](https://github.com/Byron/trash-rs/issues/40)**
    - Let dependency specification in Cargo.toml match cfg directives in code ([`cb5b617`](https://github.com/Byron/trash-rs/commit/cb5b6176aa296853f7a6e3cfa177e1235acaa903))
 * **Uncategorized**
    - Release trash v2.0.3 ([`6864e34`](https://github.com/Byron/trash-rs/commit/6864e340890f247f675982744396bae8ea856565))
    - Disable lint for platforms where it matters ([`b4add86`](https://github.com/Byron/trash-rs/commit/b4add8643cc0659b4318f3113a197794cb0032b0))
    - Update changelog with `cargo changelog` ([`932cea4`](https://github.com/Byron/trash-rs/commit/932cea48c6ceba2adf0b824c3236b330e232de12))
    - Add Rust CI status badge ([`b94fce2`](https://github.com/Byron/trash-rs/commit/b94fce2bf74dd5c1ee66735eca32d6ace5db83ea))
</details>

## v2.0.2 (2021-08-18)

### Changed

- Fix failing to delete files on some freedesktop (eg Linux) systems when the home was not mounted at the root.
- The `list` function now returns an empty list if there is no trash directory (it used to return an error).
- Fix for test failing on Linux environments that don't have a desktop environment (more specifically don't have a tool like `gio`)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 104 calendar days.
 - 108 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#34](https://github.com/Byron/trash-rs/issues/34), [#35](https://github.com/Byron/trash-rs/issues/35), [#36](https://github.com/Byron/trash-rs/issues/36)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#34](https://github.com/Byron/trash-rs/issues/34)**
    - Fix for failing to delete files on Freedesktop systems (eg Linux) ([`bd8679c`](https://github.com/Byron/trash-rs/commit/bd8679c39b163e87c33bec7a669cebdc9ff37358))
 * **[#35](https://github.com/Byron/trash-rs/issues/35)**
    - Fix for test failing on some Linux environments ([`9da7b59`](https://github.com/Byron/trash-rs/commit/9da7b590a23940693ad2809ca28c7ec904a574a6))
 * **[#36](https://github.com/Byron/trash-rs/issues/36)**
    - Avoid error from the list function ([`cb59c7e`](https://github.com/Byron/trash-rs/commit/cb59c7e09f6409881c24131bf25cb89930203655))
 * **Uncategorized**
    - Update version ([`600b59c`](https://github.com/Byron/trash-rs/commit/600b59c3422d5f6f51aca27b867a64650f06c865))
    - Update windows-rs ([`2b64f38`](https://github.com/Byron/trash-rs/commit/2b64f3832781b2715688c236194392ec31b2c5d3))
    - Some minor improvements ([`0e281bc`](https://github.com/Byron/trash-rs/commit/0e281bcbfe0bb50d8b68782cdd1da7d7e74355f7))
    - Merge pull request #29 from ArturKovacs/update-win-rs ([`2a1eaf8`](https://github.com/Byron/trash-rs/commit/2a1eaf8630b2c49b06e28d323d85e95dd0dd514a))
    - Revert the build script ([`1b4a501`](https://github.com/Byron/trash-rs/commit/1b4a501685fa02e80579fa825156ec1077a39519))
    - Ran cargo fmt ([`42884ae`](https://github.com/Byron/trash-rs/commit/42884aec20b1ad1b59213b465b34b600a8bf4cff))
    - Update windows-rs and fix for cross compilation ([`681d7b4`](https://github.com/Byron/trash-rs/commit/681d7b49140c0fd1db33628ee66bf432a5818eac))
</details>

## v2.0.1 (2021-05-02)

### Changed

- Fix not being able to trash any item on some systems.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 11 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update version number ([`6f11f8d`](https://github.com/Byron/trash-rs/commit/6f11f8dd58190afd00b15211584c15919477ad07))
    - Merge pull request #26 from ArturKovacs/fix-25 ([`13a36ce`](https://github.com/Byron/trash-rs/commit/13a36cec736c8127676f90f45f0c3941590aca1d))
    - Update changelog ([`812b574`](https://github.com/Byron/trash-rs/commit/812b574f08c73b3b26cd3c1b4b761e209f9544df))
    - Fix for error when trashing an item ([`a876d0f`](https://github.com/Byron/trash-rs/commit/a876d0f92e48cae89ac4815187b0bdff7634148d))
</details>

## v2.0.0 (2021-04-20)

### Changed

- The "Linux" implementation was replaced by a custom Freedesktop implementation.

### Added

- `list`, `purge_all`, and `restore_all` to Windows and Freedesktop

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 32 commits contributed to the release over the course of 4 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #11 from ArturKovacs/v2-dev ([`3dcac24`](https://github.com/Byron/trash-rs/commit/3dcac248a029a7f78d41fcdf1645f7ad6dc5bc4d))
    - Merge branch 'v2-dev' of https://github.com/ArturKovacs/trash-rs into v2-dev ([`e9047a3`](https://github.com/Byron/trash-rs/commit/e9047a364ba531c720d356867649d13dcac1f918))
    - Update the version number ([`28307a6`](https://github.com/Byron/trash-rs/commit/28307a662c99b150268d7b20d946ee9bd51baa75))
    - Add test for NsFileManager delete method ([`8aea6ef`](https://github.com/Byron/trash-rs/commit/8aea6ef92cde4daa6336424c91192d80ca62bde6))
    - Run cargo fmt ([`3ce2160`](https://github.com/Byron/trash-rs/commit/3ce2160a87de0b88901048063cc5d7b5aa8455f2))
    - Fix clippy error ([`2158550`](https://github.com/Byron/trash-rs/commit/21585507786f8ab0426afbe2dab7dd738b6c8c84))
    - More tweaks ([`ee2527a`](https://github.com/Byron/trash-rs/commit/ee2527a78134e408f73347b4f6bfaf43d2f9fb29))
    - Minor tweaks ([`1c43fe7`](https://github.com/Byron/trash-rs/commit/1c43fe7e7de12e1ecd551633b62c6105cfa4019d))
    - Update readme, add changelog ([`4c1ece3`](https://github.com/Byron/trash-rs/commit/4c1ece3db523de11546f487efc8ae39b01b35b5c))
    - Add more logging to the freedesktop implementation ([`a94b4ce`](https://github.com/Byron/trash-rs/commit/a94b4ce160a4926d3cf777517ee6768a364b8310))
    - Remove the Filesystem error kind ([`1138d8c`](https://github.com/Byron/trash-rs/commit/1138d8ccbc6e5bfd84daca86aacd9902326ecd3a))
    - Don't run the CI for the nightly Rust ([`afa33ba`](https://github.com/Byron/trash-rs/commit/afa33badbab2473f649881d78c9acc49de376697))
    - Fix clippy error ([`a182fbc`](https://github.com/Byron/trash-rs/commit/a182fbc7cd685151ff109c6a76623e27c2f666af))
    - Fix freedesktop errors ([`afd17c3`](https://github.com/Byron/trash-rs/commit/afd17c3efd939283d20d2e130cfeb4b609adad42))
    - Update the list example ([`a18d055`](https://github.com/Byron/trash-rs/commit/a18d055e684589eb2f9176ae6536ec433b023dc1))
    - Fix warnings on macOS ([`f12cea9`](https://github.com/Byron/trash-rs/commit/f12cea96221d52c3809408009894fa28ac3b8a0c))
    - Tweaked tests and documentation ([`330b1ec`](https://github.com/Byron/trash-rs/commit/330b1ec4f99376666ed48fb125e95e1928b1be0d))
    - Documentation improvements ([`18337bf`](https://github.com/Byron/trash-rs/commit/18337bf73c3e89a90e793ba9b6e9741d558a019a))
    - Rename `extra` to `os_limited` and other tweaks ([`29b6b11`](https://github.com/Byron/trash-rs/commit/29b6b113ffa4af0eeab6b147f5e19cee605e3274))
    - Update the macOS backend ([`eff82e4`](https://github.com/Byron/trash-rs/commit/eff82e4e11195eb6871a08b5f607e9a0ab921a4c))
    - Update the macos backend ([`e739014`](https://github.com/Byron/trash-rs/commit/e739014e08c4b883ff9350cbc1beef0e2da10797))
    - Removed the silly PlatformApi error ([`61fa667`](https://github.com/Byron/trash-rs/commit/61fa667246cc83960d68f6ccfe2f29080ddb4186))
    - Implement restore_all for windows ([`baa5171`](https://github.com/Byron/trash-rs/commit/baa5171c83a9f007e55dc2c2f412b7aad08815cc))
    - Implement purge_all on Windows ([`9fc224d`](https://github.com/Byron/trash-rs/commit/9fc224db47b3bd4c34d6a34c3251dc495a076fe9))
    - Remove the WinNull workaround ([`d8ab41f`](https://github.com/Byron/trash-rs/commit/d8ab41f97a25fdf81a20f0641c8a472e948a7f35))
    - Ran cargo fmt ([`cf13e78`](https://github.com/Byron/trash-rs/commit/cf13e78e303da0a99fb01c801a030a9f1ff9d8af))
    - Implement the `list` function for windows ([`6e77795`](https://github.com/Byron/trash-rs/commit/6e777954438acf6db2f0089d6a34fa0b77a60ab1))
    - Implemented the delete function using `windows-rs` ([`218d0d0`](https://github.com/Byron/trash-rs/commit/218d0d00492833fdb301aeaaf1164b837a2a3af4))
    - Fix example ([`69dbe38`](https://github.com/Byron/trash-rs/commit/69dbe386af48a1fb3d7a85fbb09235f263a9a5a2))
    - Don't track the lockfile ([`942108d`](https://github.com/Byron/trash-rs/commit/942108d378c92edf387788d572f91808683b0019))
    - Merge branch 'master' into v2-dev ([`c2d7a35`](https://github.com/Byron/trash-rs/commit/c2d7a35b7584f17e724853e6e3fdff9efeff5835))
    - Minor adjustments ([`314e808`](https://github.com/Byron/trash-rs/commit/314e80823fccbb2b78558e9bea2f086e77fba26a))
</details>

## v1.3.0 (2021-01-24)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fix for clippy error ([`a728dce`](https://github.com/Byron/trash-rs/commit/a728dce614add4f3aa10c2b2721a4eb2a9e57cca))
    - Increment version and update fmt ([`6d2270a`](https://github.com/Byron/trash-rs/commit/6d2270a0cbcebd8ebcc67a0278f81271e355bc63))
    - Ran fmt and fix for warning ([`ff7cf3b`](https://github.com/Byron/trash-rs/commit/ff7cf3b09916c04ff861047db2b5005621d0597a))
    - Fix for path canonicalization ([`5dfe5dc`](https://github.com/Byron/trash-rs/commit/5dfe5dc0beaa29a537808d017e5852ad976644e4))
    - Merge pull request #23 from cbr9/optimize--get-desktop-environment ([`c887b6b`](https://github.com/Byron/trash-rs/commit/c887b6bdbe707320aada2478e5033f101e86aba6))
    - Optimized get_desktop_environment() ([`a0a7fbb`](https://github.com/Byron/trash-rs/commit/a0a7fbbcd3e0e60b4b59066b65f3f4443ab57dbf))
    - Update readme ([`30427f0`](https://github.com/Byron/trash-rs/commit/30427f04121bfbd8526d06deaf1d04cc7db145b0))
    - Oops that Path wasn't completely unused after all ([`ba850ee`](https://github.com/Byron/trash-rs/commit/ba850eee27299e2aca0b1fc634f566f91a40e43b))
    - Fixed compile warning and ran rustfmt ([`c556d28`](https://github.com/Byron/trash-rs/commit/c556d284887ae72bf95b6fafba357a59f982204d))
    - Removed Cargo.lock from the gitignore. ([`cdde3a7`](https://github.com/Byron/trash-rs/commit/cdde3a7a34671b9ba26231361319f19459b75567))
    - Implement `delete` and `delete_all` for macOS ([`cb564ef`](https://github.com/Byron/trash-rs/commit/cb564ef6efcd770cf96c527624da38b14db4b6ff))
    - Updated readme ([`7a298be`](https://github.com/Byron/trash-rs/commit/7a298be45e22943206617eff9fbc2eca1234223c))
    - Implement `delete` and `delete_all` for windows. ([`d9a25c8`](https://github.com/Byron/trash-rs/commit/d9a25c8f6addf87eb177184f24a444835fad0b4a))
    - Add `delete` functions for Linux ([`fedeb83`](https://github.com/Byron/trash-rs/commit/fedeb8350625f252510ae5a2c5bb26fb74876b49))
    - Update to the readme, incorporating some suggestions by Caleb Bassi ([`9bddccc`](https://github.com/Byron/trash-rs/commit/9bddccc2e8e368f7278135e60d41da601fa20aa4))
    - Merge pull request #18 from cjbassi/rename-files ([`c49b496`](https://github.com/Byron/trash-rs/commit/c49b4961b1e83548777ea0a24cd99c7e6c6660fe))
    - Rename readme and license files ([`5a9a5a6`](https://github.com/Byron/trash-rs/commit/5a9a5a66b53803b037636febc9265b66bcfc7334))
    - Adds a deprecated attribute to the `is_implemented` function. ([`386db96`](https://github.com/Byron/trash-rs/commit/386db96e8eebed0b60d79ac055e8e312f01a605c))
</details>

## v1.1.0 (2020-08-12)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 87 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#17](https://github.com/Byron/trash-rs/issues/17)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#17](https://github.com/Byron/trash-rs/issues/17)**
    - Implement std::error::Error for trash::Error ([`8765acf`](https://github.com/Byron/trash-rs/commit/8765acf6ef7a93db322baabb40df1edfc405b437))
 * **Uncategorized**
    - Increment minor version number ([`281bb93`](https://github.com/Byron/trash-rs/commit/281bb931159f22da85f4f23fcee92cc96e8a28e7))
</details>

## v1.0.1 (2020-05-16)

<csr-id-576fad719cb240203dec030890d54fe416a42edd/>

### Refactor

 - <csr-id-576fad719cb240203dec030890d54fe416a42edd/> port mac implementation to work with v2
   Updates the existing Mac implementation to compile with v2 of the
   library. Does not add any new functionality other defining required
   methods.
   
   Tests fail for methods relating to `list`, `purge_all`, or
   `restore_all`, which are unimplemented.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 59 commits contributed to the release.
 - 218 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update readme and increment the patch field in the version number. ([`217b473`](https://github.com/Byron/trash-rs/commit/217b4739d84827744a0b23a23b344e2118ac6f5b))
    - Merge pull request #15 from myfreeweb/bsd ([`5af79ab`](https://github.com/Byron/trash-rs/commit/5af79aba26d767b1be1c816d18ff3ef7a7b3301d))
    - Build "linux" module on *BSD (any non-macOS unix) ([`9e38ff8`](https://github.com/Byron/trash-rs/commit/9e38ff8ea89a70c9c3c87369413b3f798baa727d))
    - Fix clippy warnings on linux. ([`0731a64`](https://github.com/Byron/trash-rs/commit/0731a6403f0854ee7098d4cd534bb435684ffd50))
    - Merge branch 'master' of https://github.com/ArturKovacs/trash ([`1e43dc1`](https://github.com/Byron/trash-rs/commit/1e43dc1682e95ff497d6ce5d4b3185336227506f))
    - Add .vscode to gitignore ([`f63f7ce`](https://github.com/Byron/trash-rs/commit/f63f7ce308dd3dad6c64e39d210891cc237704a8))
    - Fix for clippy warning. ([`80eba00`](https://github.com/Byron/trash-rs/commit/80eba00727a72e47b016cda028a6ede7d218085f))
    - Ran `cargo fmt`. ([`b753689`](https://github.com/Byron/trash-rs/commit/b7536891e17e796ab17a4070100024f1754e3ba0))
    - Add tests for Windows and MacOS as well as the nightly toolchain. ([`29876c7`](https://github.com/Byron/trash-rs/commit/29876c7f96144c82f8bee565c80d433beaba548d))
    - Default rust workflow (GitHub Actions) ([`cf7d22f`](https://github.com/Byron/trash-rs/commit/cf7d22fd2db44f52bbc48041edee66be391b2911))
    - Merge branch 'master' into v2-dev ([`32de332`](https://github.com/Byron/trash-rs/commit/32de3324618619ffefe0cc7ed3c16bb8df647caf))
    - Merge branch 'master' into v2-dev ([`b3ea819`](https://github.com/Byron/trash-rs/commit/b3ea819c48c619ff0b41df8357da0c4cd1da8d67))
    - Remove Azure test ([`3e7db4f`](https://github.com/Byron/trash-rs/commit/3e7db4f45f81232567b9beb65860dd4a8651a6fa))
    - Add GitHub Actions test ([`0c8b1fa`](https://github.com/Byron/trash-rs/commit/0c8b1fa561a574678d4db38fc3f0a10b470aca38))
    - Fix wording in Readme ([`7025e10`](https://github.com/Byron/trash-rs/commit/7025e102110a00a290619a32822a7bbd823e3925))
    - Update Readme to reflect the state of development. ([`a31a944`](https://github.com/Byron/trash-rs/commit/a31a94487c46e8ac2b886f59b79b5aa0be195fb8))
    - Merge pull request #9 from NilsIrl/patch-1 ([`02bb739`](https://github.com/Byron/trash-rs/commit/02bb73950db23156680a2273498a1e815ab6fa3d))
    - Fix typo ([`6c4d650`](https://github.com/Byron/trash-rs/commit/6c4d650fd8dd2346e20d37a17d6db9e4afd2ce77))
    - Added test cases and extra documentation. The test cases cover empty input for  `purge_all` and `restore_all`. ([`c275449`](https://github.com/Byron/trash-rs/commit/c275449adde03da9d7ba3064f4b72c9e816c2979))
    - Moved Linux and Windows specific features to a mod ([`0e2fc93`](https://github.com/Byron/trash-rs/commit/0e2fc93f8bb5c17a9c5c89849453b04344995cab))
    - Refined windows implementation. Added error kind `RestoreTwins`. ([`d105553`](https://github.com/Byron/trash-rs/commit/d1055538c16e318247b2817cce85ec522b2163a2))
    - Ran `cargo fmt` ([`743b2f3`](https://github.com/Byron/trash-rs/commit/743b2f3f889b0519275b9d52475b0b68c661382d))
    - Merge branch 'v2-mac' into v2-dev ([`62a7218`](https://github.com/Byron/trash-rs/commit/62a7218644511d75e7b4d162c70aac2f1a4625e9))
    - Merge branch 'v2-dev' of https://github.com/ArturKovacs/trash into v2-dev ([`90d3fa6`](https://github.com/Byron/trash-rs/commit/90d3fa62eb7ca525e6fc45c48cdeba95322a7416))
    - Removed the two previously added errors. Replaced `ZeroMountPointsFound` and `CantOpenMountPointsFile` with `panic!` after coming across https://lukaskalbertodt.github.io/2019/11/14/thoughts-on-error-handling-in-rust.html and reading http://joeduffyblog.com/2016/02/07/the-error-model/ ([`fa03282`](https://github.com/Byron/trash-rs/commit/fa0328204662ea871336087738642a6c9dece33e))
    - No need for those parentheses ([`534677e`](https://github.com/Byron/trash-rs/commit/534677ed4c23316f956dc43e84f16ca68f744331))
    - Merge branch 'v2-dev' of https://github.com/ArturKovacs/trash into v2-dev ([`00fc235`](https://github.com/Byron/trash-rs/commit/00fc235235d4f00e34ffb8cd04b975157037ab91))
    - Add missing error kinds. `ZeroMountPointsFound` and `CantOpenMountPointsFile` were added. ([`6c79f7d`](https://github.com/Byron/trash-rs/commit/6c79f7d95604c68363cd710f99150e655152dbc4))
    - Improve collision handling and add collision test. ([`6db249b`](https://github.com/Byron/trash-rs/commit/6db249bef9529eab4f8325dbc36e84a47000525b))
    - Merge pull request #7 from ArturKovacs/v2-linux ([`b957f38`](https://github.com/Byron/trash-rs/commit/b957f3894b6d06b42cc48824a125825b954496c4))
    - Remove debug lines. ([`3ab9217`](https://github.com/Byron/trash-rs/commit/3ab9217280b6656b4a861e61cc590c326c008d67))
    - Fix creating the home trash folder. ([`10364c6`](https://github.com/Byron/trash-rs/commit/10364c64508acd4fb564cc761bc746eb2d9dd4b1))
    - Create home trash if doesn't yet exist. Also added debug print line numbers. ([`e1b2aae`](https://github.com/Byron/trash-rs/commit/e1b2aaeb8fd00d07be250208767ba85217d55010))
    - Attempt to add RUST_BACKTRACE=1 again. ([`7bd6023`](https://github.com/Byron/trash-rs/commit/7bd6023710e60ddbf6f9f15bcf1bd714f6aaedd9))
    - Merge branch 'v2-dev' into v2-linux ([`303a274`](https://github.com/Byron/trash-rs/commit/303a274989d0a6b6faf7eab3f0ae9149e6e86d71))
    - Added RUST_BACKTRACE=1 to test. ([`c1cb106`](https://github.com/Byron/trash-rs/commit/c1cb10611be762cb13fce4fd38c39961ad84317a))
    - Merge branch 'v2-dev' into v2-linux ([`ecf521f`](https://github.com/Byron/trash-rs/commit/ecf521fbccaae27b4eb160598448341d5e8b7700))
    - Add ability to trash items from an external drive. ([`16f0ee1`](https://github.com/Byron/trash-rs/commit/16f0ee19beaf116b9dfd44de06b272f8b62fb3fd))
    - Added a partialy implementaiton of `remove_all`. Can't remove from non-root devices or partitions. ([`1e03167`](https://github.com/Byron/trash-rs/commit/1e031679d6a3c0cd8a780014d3331bd4fd8bcd1d))
    - Steps towards implementing `remove_all`. ([`20ba354`](https://github.com/Byron/trash-rs/commit/20ba354399d2ce96e7ee624bbf2a03407df163db))
    - Fix for `list` failing on Linux. This happened because `list` on Linux didn't handle paralell threads manipulating the trash correctly. ([`d6cb6ba`](https://github.com/Byron/trash-rs/commit/d6cb6bac6758a0020b88ef5632bf9f06f748f7ca))
    - Merge pull request #6 from ayazhafiz/refactor/mac2 ([`adf0ea4`](https://github.com/Byron/trash-rs/commit/adf0ea4fab0ace99b443c5498ba5495c89abcd30))
    - Remove the Cirrus CI config. ([`fd597fc`](https://github.com/Byron/trash-rs/commit/fd597fc852eddb23472276be6c638e6e40281f67))
    - Port mac implementation to work with v2 ([`576fad7`](https://github.com/Byron/trash-rs/commit/576fad719cb240203dec030890d54fe416a42edd))
    - Add MacOS and Linux as targets for CI tests. ([`e409a98`](https://github.com/Byron/trash-rs/commit/e409a983b64d36c2b585ecdb5374357a34f5da53))
    - Fix OS setup in Azure's config. ([`08db817`](https://github.com/Byron/trash-rs/commit/08db8172080832727bd5002e394054c34c5147ea))
    - Update Azure's target operating systems. ([`cfb25b6`](https://github.com/Byron/trash-rs/commit/cfb25b6e0d9e8fd00379871760430009c52289cd))
    - Add Azure Pipelines CI. ([`760dfa6`](https://github.com/Byron/trash-rs/commit/760dfa64e53a2e1228230c073bd551acb868b286))
    - Add Cirrus CI test. ([`e5c22c4`](https://github.com/Byron/trash-rs/commit/e5c22c4567e2f289a918affc46fa923a962799cf))
    - Added implementation of purge_all for Linux. Also ran rustfmt and created a rustfmt config. ([`a90f9bf`](https://github.com/Byron/trash-rs/commit/a90f9bfa0d19fd6fad88c91bbd0e6a46c4661a0e))
    - Now using the url crate for parsing the original location on Linux. ([`02ffe0b`](https://github.com/Byron/trash-rs/commit/02ffe0b336136c730213670d4c5b6eb04addaa55))
    - Add `list` implementation for linux. ([`5c73fea`](https://github.com/Byron/trash-rs/commit/5c73fea22341c520103564953535c84b7271fc4e))
    - Add note about coming features in version 2 to the Readme. ([`dddbe25`](https://github.com/Byron/trash-rs/commit/dddbe25171e6f93ffd2b80627d25e8313ff21498))
    - Improve the Error type and add `create_remove_empty_folder` test. ([`a940b66`](https://github.com/Byron/trash-rs/commit/a940b66abea7769aba8e0b1d99995b8174239877))
    - Changed `std::mem::uninit` and `std::mem::zeroed` to `std::mem::MaybeUninit`. Plus ran Rustfmt. ([`632a6fb`](https://github.com/Byron/trash-rs/commit/632a6fb31fc4c4bee751f0537ca317fe9f933f5c))
    - Now `purge_all` doesn't show a dialog on windows. ([`07e3bc2`](https://github.com/Byron/trash-rs/commit/07e3bc25832a49af13ee3c2a1fdc1f425fce8805))
    - Fix `purge_all` and `restore_all` reading invvalid memory and not executing the operation on the requested items. Add test cases for `purge_all` and `restore_all`. Test are now thread safe. ([`22d5181`](https://github.com/Byron/trash-rs/commit/22d51813759c129e87625eef5d068a1481bfbdb8))
    - Implement `purge_all` and `restore_all` for Windows. ([`e06c825`](https://github.com/Byron/trash-rs/commit/e06c825e93ce9774733cfcf539e3c7e928cdb8cc))
    - Run rust fmt. Implement `list` for Windows. ([`3f29636`](https://github.com/Byron/trash-rs/commit/3f29636a978fd5a462db1588040d794d81648be7))
</details>

## v1.0.0 (2019-10-11)

<csr-id-576fad719cb240203dec030890d54fe416a42edd/>

### Refactor

 - <csr-id-576fad719cb240203dec030890d54fe416a42edd/> port mac implementation to work with v2
   Updates the existing Mac implementation to compile with v2 of the
   library. Does not add any new functionality other defining required
   methods.
   
   Tests fail for methods relating to `list`, `purge_all`, or
   `restore_all`, which are unimplemented.

### New Features

 - <csr-id-d68cc2aedee5e8316117bec257975da30cbd7483/> implementation for macOS
   Moves files to trash on macOS by executing an AppleScript command to
   delete all requested paths.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 19 commits contributed to the release over the course of 99 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Updated version number and readme ([`79ee69e`](https://github.com/Byron/trash-rs/commit/79ee69e3e12a9a66146897ab432f29eaa8ac2d28))
    - Merge pull request #1 from ayazhafiz/feat/mac ([`48a6b11`](https://github.com/Byron/trash-rs/commit/48a6b11cae520ca1b60c42270912402c1d51c018))
    - Implementation for macOS ([`d68cc2a`](https://github.com/Byron/trash-rs/commit/d68cc2aedee5e8316117bec257975da30cbd7483))
    - Fix wrong code references in the linux implementation. ([`037fed8`](https://github.com/Byron/trash-rs/commit/037fed8ae6b5ed76cec00037cdc8340d7787d7cb))
    - Add docs badge to readme ([`88261d5`](https://github.com/Byron/trash-rs/commit/88261d5af0b165a06483ea07e5aa378d2223d067))
    - Increment version number ([`f758543`](https://github.com/Byron/trash-rs/commit/f75854358b7c8dea23aec6f40362fab4039d9659))
    - Improve readme. Add remove_all function to mac as unimplemented. ([`2850270`](https://github.com/Byron/trash-rs/commit/2850270004cea47718b18aa3d3b290263ba7b8e3))
    - Merge branch 'master' of https://github.com/ArturKovacs/trash ([`a651d0f`](https://github.com/Byron/trash-rs/commit/a651d0f3b7c261da9bd2fd65f16166b43d63abf3))
    - Add folder remove test ([`b7bb22f`](https://github.com/Byron/trash-rs/commit/b7bb22f7b21027ad73d53eded480921f3346a14c))
    - Updated the Cargo.toml ([`1ec1ef9`](https://github.com/Byron/trash-rs/commit/1ec1ef96b941bbab6672a86b24b0e23afdfe2165))
    - Add license ([`1725e61`](https://github.com/Byron/trash-rs/commit/1725e612e103dda2e574543ea90821934ed46ae6))
    - Add doc comments ([`b16a1d3`](https://github.com/Byron/trash-rs/commit/b16a1d3dff8c3b4d9b24b6dc87c9e3781b667c58))
    - Fix Windows compile error. ([`15e801e`](https://github.com/Byron/trash-rs/commit/15e801e1a242e1b0263fe854e6c9d58a68774dd0))
    - Add the `remove_all` function. ([`f033dc3`](https://github.com/Byron/trash-rs/commit/f033dc308ee061adc579f7426697c5cb3c280956))
    - Minor refactoring. ([`9c7363d`](https://github.com/Byron/trash-rs/commit/9c7363dbeae19edb8079a57fcc93d012c8064ef0))
    - Add Linux support. ([`0429d3b`](https://github.com/Byron/trash-rs/commit/0429d3bf1e7f09e97eae4aefde0cb8c8e283b235))
    - Fixed platform specific compilation ([`84cba5b`](https://github.com/Byron/trash-rs/commit/84cba5b11acf02d5ba99d776529b93bac54f8094))
    - Changed required winapi version. ([`d49bce8`](https://github.com/Byron/trash-rs/commit/d49bce8d346e10c08910520383ed4054a3948535))
    - Initial ([`4c23314`](https://github.com/Byron/trash-rs/commit/4c233148288711419a04fdfa96e36dcb77f0469f))
</details>


# Changelog

## 5.3.2 (2025-01-05)

### Bug Fixes

 - <csr-id-c452a8c4e56c3726431d8a4a77ad910bc8ae3ecb/> fix `that_detached` for UNC path of a directory

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 51 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge pull request #107 from amrbashir/fix/windows/remove-unc-and-fallback-on-error ([`472ce26`](https://github.com/Byron/open-rs/commit/472ce262c8f3c02f089f881387616df2303f48f8))
    - Fix `that_detached` for UNC path of a directory ([`c452a8c`](https://github.com/Byron/open-rs/commit/c452a8c4e56c3726431d8a4a77ad910bc8ae3ecb))
    - Merge pull request #79 from Byron/better-docs ([`2646ff8`](https://github.com/Byron/open-rs/commit/2646ff820c2ab965a636d57753a10c56501f4163))
</details>

## 5.3.1 (2024-11-14)

On Windows, respect the `expand to open folder` setting.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v5.3.1 ([`10c3f0d`](https://github.com/Byron/open-rs/commit/10c3f0d8c9a229522c217b38ecb6c257d925105a))
    - Prepare changelog prior to release ([`6e565e2`](https://github.com/Byron/open-rs/commit/6e565e2271f05a80b4360d5e0dbff8b05483efb0))
    - Merge pull request #104 from Legend-Master/shopenfolderandselectitems ([`d149825`](https://github.com/Byron/open-rs/commit/d14982521423e67c3dd0fde115bc64bee7f7fb45))
    - Fix: use SHOpenFolderAndSelectItems for folders To respect 'expand to open folder' setting Fixes #103 ([`7595da7`](https://github.com/Byron/open-rs/commit/7595da7d655e910f3cb9d26b96d3947f2acf2522))
</details>

## 5.3.0 (2024-07-10)

### New Features

 - <csr-id-58142a695d50460e439f85be3f0bc010936520e6/> add GNU/Hurd support
   Handle it like most of the other Unix platforms (e.g. Linux, BSDs, etc).

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 7 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v5.3.0 ([`c26d98c`](https://github.com/Byron/open-rs/commit/c26d98cc66979f153682690201a4748026012224))
    - Merge pull request #101 from pinotree/hurd ([`a060608`](https://github.com/Byron/open-rs/commit/a0606084dd4a18ca67bef84fa217529cb858ded5))
    - Add GNU/Hurd support ([`58142a6`](https://github.com/Byron/open-rs/commit/58142a695d50460e439f85be3f0bc010936520e6))
</details>

## 5.2.0 (2024-07-02)

### New Features

 - <csr-id-d4e66dc1332e49672190060de6b39b13784c5de9/> Add AIX support

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 27 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v5.2.0 ([`3cf72fd`](https://github.com/Byron/open-rs/commit/3cf72fd426442ed856f1b9cddea61b5413299c45))
    - Add AIX support ([`d4e66dc`](https://github.com/Byron/open-rs/commit/d4e66dc1332e49672190060de6b39b13784c5de9))
    - Add AIX support ([`b414215`](https://github.com/Byron/open-rs/commit/b41421579aa68b484d57814acd2ef33e5b1b9b5a))
</details>

## 5.1.4 (2024-06-04)

This release adds vision-os support, by means of `ios` simulation.
See [the PR](https://github.com/Byron/open-rs/pull/99) for a little more context.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 19 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v5.1.4 ([`644f186`](https://github.com/Byron/open-rs/commit/644f186e6b4e870ca91c1eceeebe8fe9760eca35))
    - Prepare changelog prior to release ([`5967b66`](https://github.com/Byron/open-rs/commit/5967b66212097facead9f16bc56905992eb6650e))
    - Merge pull request #99 from eugenehp/visionos ([`1efa4cb`](https://github.com/Byron/open-rs/commit/1efa4cbb8ed7c1abac95299dd6af3c17727b27af))
    - Added visionos support ([`df28e8b`](https://github.com/Byron/open-rs/commit/df28e8bdce30cd1a8e930c947566edfd8835cef8))
</details>

## 5.1.3 (2024-05-16)

### Bug Fixes

 - <csr-id-43d8d88a2d5949114cdebcc08a0aad68f2141e54/> use `ShellExecuteExW` to avoid freeze when opening directories

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#94](https://github.com/Byron/open-rs/issues/94)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#94](https://github.com/Byron/open-rs/issues/94)**
    - Clarify docs of `shellexecute-on-windows` feature ([`7652a30`](https://github.com/Byron/open-rs/commit/7652a3016e3b243604f38d40eab6097c2d816fde))
 * **Uncategorized**
    - Release open v5.1.3 ([`6e396da`](https://github.com/Byron/open-rs/commit/6e396da65e3483866a75a405b67f495d43cceb06))
    - Merge pull request #98 from amrbashir/fix/windows/shell-execute-freeze ([`d152c8e`](https://github.com/Byron/open-rs/commit/d152c8e09225b23195dbee243e8bd508e07686e3))
    - Use cfg_attr ([`c8840af`](https://github.com/Byron/open-rs/commit/c8840afb1550cef2c9897130c7d05b72bfd55d4a))
    - Update src/windows.rs ([`75c7ea0`](https://github.com/Byron/open-rs/commit/75c7ea085116502d4dae61bc752cb27e7663a914))
    - Use `ShellExecuteExW` to avoid freeze when opening directories ([`43d8d88`](https://github.com/Byron/open-rs/commit/43d8d88a2d5949114cdebcc08a0aad68f2141e54))
</details>

## 5.1.2 (2024-03-07)

### Bug Fixes

 - <csr-id-2a62e84a234274b53cb77ffcef4ad76630ba7c2f/> link using "system" ABI

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 4 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v5.1.2 ([`a65b733`](https://github.com/Byron/open-rs/commit/a65b7332e998ce0d61be8bf6786d0911b3334539))
    - Merge branch 'fix/shell-execute-w-link' ([`50b7a96`](https://github.com/Byron/open-rs/commit/50b7a96a964987fc4a82f6633930384c6d8e5fb0))
    - Link using "system" ABI ([`2a62e84`](https://github.com/Byron/open-rs/commit/2a62e84a234274b53cb77ffcef4ad76630ba7c2f))
</details>

## 5.1.1 (2024-03-03)

### Bug Fixes

 - <csr-id-74fd8ec005d9bd24e6cb604e3239730b0b414b84/> add `shellexecute-on-windows` feature.
   That way, it's possible to toggle on a feature that might
   cause issues in some dependency trees that contain `flate2`
   with `zlib-ng` backend.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v5.1.1 ([`0c916ae`](https://github.com/Byron/open-rs/commit/0c916aefe11391d029635ad4e4a284a9b7f0d90a))
    - Merge branch 'validate-linkage' ([`59886df`](https://github.com/Byron/open-rs/commit/59886df5db59f6874ff24401e32a3e90b0d27a53))
    - Add `shellexecute-on-windows` feature. ([`74fd8ec`](https://github.com/Byron/open-rs/commit/74fd8ec005d9bd24e6cb604e3239730b0b414b84))
    - Try to validate linkage on all platforms ([`8f26da4`](https://github.com/Byron/open-rs/commit/8f26da4ff13be60c69f7b144b185d6166f6d4af6))
</details>

## 5.1.0 (2024-03-01)

### New Features

 - <csr-id-191cb0e2201c911d1bf0df3ba03062c6d9b6e738/> use `ShellExecuteW` for detached spawning on Windows

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v5.1.0 ([`21a73ee`](https://github.com/Byron/open-rs/commit/21a73ee19d2d976076cefa8bdda639f510278420))
    - Merge pull request #91 from amrbashir/feat/windows/detachded-using-shellexecutew ([`b268647`](https://github.com/Byron/open-rs/commit/b268647bd2684ee13f55eb3c976e71949c321bc3))
    - Split into two functions for better readability ([`4506b2f`](https://github.com/Byron/open-rs/commit/4506b2f8ac51579932b76884a11133ce5c49c21f))
    - Use `ShellExecuteW` for detached spawning on Windows ([`191cb0e`](https://github.com/Byron/open-rs/commit/191cb0e2201c911d1bf0df3ba03062c6d9b6e738))
</details>

## 5.0.2 (2024-02-28)

<csr-id-a583658a2f2cfea64c3be6e12cef159f5cbc7fbf/>

### Other

 - <csr-id-a583658a2f2cfea64c3be6e12cef159f5cbc7fbf/> use PATH to find launcher
   Redox has moved the launcher from /ui/bin to /usr/bin. Just use the PATH to locate it, so any future changes in location don't break this crate.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 97 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v5.0.2 ([`f4ef7c9`](https://github.com/Byron/open-rs/commit/f4ef7c9de95fd503871028f100db28ef0287fbc4))
    - Merge pull request #89 from jackpot51/patch-1 ([`0a25651`](https://github.com/Byron/open-rs/commit/0a25651fa04b9a696356a5980ede105f6923ce91))
    - Use PATH to find launcher ([`a583658`](https://github.com/Byron/open-rs/commit/a583658a2f2cfea64c3be6e12cef159f5cbc7fbf))
</details>

## 5.0.1 (2023-11-22)

<csr-id-dca688bf473169b6d779695e35cb05596d094ba1/>

### Bug Fixes

 - <csr-id-15eedee81d851dd26e78984030020ba252812419/> open paths with given applications with whitespace in their paths on windows.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#85](https://github.com/Byron/open-rs/issues/85)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#85](https://github.com/Byron/open-rs/issues/85)**
    - Open paths with given applications with whitespace in their paths on windows. ([`15eedee`](https://github.com/Byron/open-rs/commit/15eedee81d851dd26e78984030020ba252812419))
 * **Uncategorized**
    - Release open v5.0.1 ([`4a2a5b1`](https://github.com/Byron/open-rs/commit/4a2a5b104bb0aa8f2a42dc3e505c773fd8f5f45c))
    - Prepare changelog ([`2735009`](https://github.com/Byron/open-rs/commit/2735009c771056fbb65120a439e34c6a3f2038f3))
    - Wrap provided app in quotes on Windows ([`e1b74d8`](https://github.com/Byron/open-rs/commit/e1b74d8e171ce63ddf4cad212a94b8238ae66be7))
    - Merge pull request #84 from nomyfan/readme-version ([`a33caba`](https://github.com/Byron/open-rs/commit/a33caba441354a14bd55a16ad4a5e8bc793819f0))
    - Update version to v5 ([`dca688b`](https://github.com/Byron/open-rs/commit/dca688bf473169b6d779695e35cb05596d094ba1))
</details>

## 5.0.0 (2023-06-25)

### Bug Fixes (BREAKING)

 - <csr-id-ddf4842dc78389e87615f5c2ddf9e859b11277b2/> `with()` on windows to be able to lookup `program` in registry/PATH.
   The underlying invocation of `with()` changed slightly on windows to make it more useful
   as it can now find application names like `chrome` in the registry, but that change may also
   be breaking for some who previously worked around the previous behaviour.
   
   Please let us know if this truly works better, or if more changes are needed to launch something
   with a program on Windows.
   
   See https://github.com/Byron/open-rs/pull/82 for details.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#80](https://github.com/Byron/open-rs/issues/80)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#80](https://github.com/Byron/open-rs/issues/80)**
    - Fix open::with() on Windows. ([`5607cd6`](https://github.com/Byron/open-rs/commit/5607cd69f4f53aa02c67bccb60f7d4f9be6b74ea))
    - `open` application now reads `OPEN_WITH` environment variable to obtain the program to open with. ([`659b8a0`](https://github.com/Byron/open-rs/commit/659b8a0a6580fb89376daf27822fc08557ccf477))
 * **Uncategorized**
    - Release open v5.0.0 ([`92695f6`](https://github.com/Byron/open-rs/commit/92695f6cfb397e84f13b997df06c5bf036305185))
    - `with()` on windows to be able to lookup `program` in registry/PATH. ([`ddf4842`](https://github.com/Byron/open-rs/commit/ddf4842dc78389e87615f5c2ddf9e859b11277b2))
    - Cleanup `main` program to support `--with` on all platforms. ([`b5528b6`](https://github.com/Byron/open-rs/commit/b5528b60d49f4dd48449d9ff5b75edbea71ce248))
</details>

## 4.2.0 (2023-06-21)

### Other

 - <csr-id-323b8ea2aba9b0661bf3af6bd48ccef53197b0bf/> Improve documentation about shortcomings particularly on console-only UNIX platforms.

### New Features

 - <csr-id-7ad9cb373eb7e999f7e79705db891fd24a4d4bd2/> add `that_detached()` and `with_detached()` functions.
   
   They allow to open the program in a completely independent fashion so that it can out-live the program that spawned it.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 28 calendar days.
 - 55 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#8](https://github.com/Byron/open-rs/issues/8)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#8](https://github.com/Byron/open-rs/issues/8)**
    - Improve documentation about shortcomings particularly on console-only UNIX platforms. ([`323b8ea`](https://github.com/Byron/open-rs/commit/323b8ea2aba9b0661bf3af6bd48ccef53197b0bf))
 * **Uncategorized**
    - Release open v4.2.0 ([`b4592a4`](https://github.com/Byron/open-rs/commit/b4592a4367a8b743e89812f73a274e5e2b0c0d42))
    - Prepare changelog prior to release ([`4ad428e`](https://github.com/Byron/open-rs/commit/4ad428e4944a066bbd2d3646d0c338532956001d))
    - Merge branch 'feat/detached-process' ([`d21c48e`](https://github.com/Byron/open-rs/commit/d21c48e6682341e34ca37369460eb5247daaf71c))
    - Add `that_detached` and `with_detached` ([`7ad9cb3`](https://github.com/Byron/open-rs/commit/7ad9cb373eb7e999f7e79705db891fd24a4d4bd2))
</details>

## 4.1.0 (2023-04-26)

### New Features

 - <csr-id-b8089a6403394a56e9d023cee85e839f91ddc9aa/> Add Redox support.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 9 calendar days.
 - 10 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#74](https://github.com/Byron/open-rs/issues/74)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#74](https://github.com/Byron/open-rs/issues/74)**
    - Add Redox support. ([`b8089a6`](https://github.com/Byron/open-rs/commit/b8089a6403394a56e9d023cee85e839f91ddc9aa))
 * **Uncategorized**
    - Release open v4.1.0 ([`8e47876`](https://github.com/Byron/open-rs/commit/8e47876d5d11ac31f3fbc4cf015cb865fc72bf3a))
    - Add Redox support ([`4c31d9c`](https://github.com/Byron/open-rs/commit/4c31d9cb55a60881a84c81454c3f433b2c215809))
    - Fix version in readme document ([`b076fc4`](https://github.com/Byron/open-rs/commit/b076fc44a984b55b0ca986530e9f4a18f1a43ba4))
</details>

## 4.0.2 (2023-04-16)

### Bug Fixes

 - <csr-id-5f1f80f9fb63e6c63b48ef97370711fd8cebcfcc/> now works within WSL if `gio` is installed.
   `gio` would fail on WSL which is fixed by detecting that WSL is present
   which is when `wslview` will be prioritized.
   
   Note that the binary size inceases by ~13kb as we try to avoid running `wslview`
   first and fail everywhere, so prefer runtime performance on linux over binary
   size. The binary size changes only on unix.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#71](https://github.com/Byron/open-rs/issues/71)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#71](https://github.com/Byron/open-rs/issues/71)**
    - Now works within WSL if `gio` is installed. ([`5f1f80f`](https://github.com/Byron/open-rs/commit/5f1f80f9fb63e6c63b48ef97370711fd8cebcfcc))
 * **Uncategorized**
    - Release open v4.0.2 ([`370f03b`](https://github.com/Byron/open-rs/commit/370f03b28910afae32d2e3054848068bea271a9b))
    - See if using the latest ubuntu provides CI runners faster ([`b7009ac`](https://github.com/Byron/open-rs/commit/b7009acf2d9149db13096815708588873403142b))
    - Only pull `is-wsl` on unix. ([`1912915`](https://github.com/Byron/open-rs/commit/1912915cd541381a8184f574e13c3b31ba6f0a06))
    - Check whether running in WSL before trying wslview ([`b150494`](https://github.com/Byron/open-rs/commit/b150494e70cfea1fb6aec87d1100c4426d13a16a))
    - Prioritize wslview so it's always launched when present ([`9048e63`](https://github.com/Byron/open-rs/commit/9048e63181626ca9c165f8e85682465bbb985554))
</details>

## 4.0.1 (2023-03-20)

In the previous releases 3.3 and 4.0 on windows the console windows would show.
This was unintentional and is now fixed.

Thanks so much for [the contribution](https://github.com/Byron/open-rs/pull/69).

### Bug Fixes

 - <csr-id-bc75d847c6c89cdd835f4220d6a0ed9a6985b41e/> hide the console window in Windows

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 10 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v4.0.1 ([`c5af111`](https://github.com/Byron/open-rs/commit/c5af111792417c604c9b188aa2d627556a40b119))
    - Prepare changelog prior to release ([`8b4c259`](https://github.com/Byron/open-rs/commit/8b4c2596b1ac5d08bffb58784d1760ce47df14a5))
    - Merge pull request #69 from amrbashir/hide-console ([`8cf21e0`](https://github.com/Byron/open-rs/commit/8cf21e0d23f0d18e503f4a056762d1efd733f99d))
    - Hide the console window in Windows  ([`bc75d84`](https://github.com/Byron/open-rs/commit/bc75d847c6c89cdd835f4220d6a0ed9a6985b41e))
</details>

## 4.0.0 (2023-03-10)

### Bug Fixes

 - <csr-id-9ceb0a4b707022a6ce8046d3512be7c7a51d77a4/> Windows URL encoding for multiple query params
   Previously, passing a URL with multiple query parameters on Windows would result in
   improper escaping, causing unexpected behavior when using it with "cmd /c".
   This commit addresses this issue by properly escaping all query parameters in the URL.
   Now, URLs with multiple query parameters are handled correctly on Windows.
   
   See also:
   * https://doc.rust-lang.org/std/os/windows/process/trait.CommandExt.html#tymethod.raw_arg
* https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/cmd
* https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/start

### New Features (BREAKING)

 - <csr-id-0cd6f98d0a97ac3ddcecefc5d33952e7632298a5/> set minimal required Rust version of 1.62.
   This version is motivated only when compiling on windows (but we can't differentiate that)
   due to the recent addition of `CommandExt::raw_arg()`.
   
   This might be a breaking change as previously there was no minimal supported Rust version.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 3 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v4.0.0 ([`f36f37a`](https://github.com/Byron/open-rs/commit/f36f37a7a2ac8f417cd14f900de795a144748264))
    - Windows URL encoding for multiple query params ([`9ceb0a4`](https://github.com/Byron/open-rs/commit/9ceb0a4b707022a6ce8046d3512be7c7a51d77a4))
    - Refactor ([`9a1178a`](https://github.com/Byron/open-rs/commit/9a1178a9d32c2b4da12cdb8542b3b36806825d7d))
    - Set minimal required Rust version of 1.62. ([`0cd6f98`](https://github.com/Byron/open-rs/commit/0cd6f98d0a97ac3ddcecefc5d33952e7632298a5))
    - Fix Windows URL encoding for multiple query params ([`e4856ac`](https://github.com/Byron/open-rs/commit/e4856acd170cdea5b06a947c7e7e9cdd36f0b392))
</details>

## 3.4.0 (2023-03-06)

<csr-id-7e2a9c645cd4ff5f86ece7cdc220e18c1b4ac1b5/>

### New Features

 - <csr-id-adf99e91af0f1c034123170e6dca77489484b004/> un-deprecate `that_in_background()`.
   In some configurations launchers may block, thus it's better to provide an easy mechanism to unblock
   an otherwise blocking call. This is alongside being able to use `command()` directly and simply spawn
   the command without waiting for it.
 - <csr-id-66e0d7c295317aed27e89af8e323dde3b761c0e3/> `with_command()` to obtain the command that opens a program with a given application.
 - <csr-id-a501d65388562724471b5493fe54c88fe23df31e/> improved error messages that will list the invoked command.
 - <csr-id-245c95ede24adc6694d935993d6045d19a935035/> `commands()` function to obtain a list of launchers to open the given path.
   This allows async applications to control the application launch in an async way,
   for instance with `tokio`.

### Other

 - <csr-id-7e2a9c645cd4ff5f86ece7cdc220e18c1b4ac1b5/> improve documentation around how to use the library.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 11 commits contributed to the release.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Thanks Clippy

<csr-read-only-do-not-edit/>

[Clippy](https://github.com/rust-lang/rust-clippy) helped 1 time to make code idiomatic. 

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v3.4.0 ([`031c44b`](https://github.com/Byron/open-rs/commit/031c44b82b518757530568055db2f0833ba1ac5b))
    - Improve README ([`bfededa`](https://github.com/Byron/open-rs/commit/bfededa42cfa50f6fb88540cbce54a2318af5655))
    - Improve documentation around how to use the library. ([`7e2a9c6`](https://github.com/Byron/open-rs/commit/7e2a9c645cd4ff5f86ece7cdc220e18c1b4ac1b5))
    - Modernize generics of API using `impl` instead of 'the other way'. ([`cb322bf`](https://github.com/Byron/open-rs/commit/cb322bf72733a210bd41654bd3152f19dbb31059))
    - Thanks clippy ([`2f6fb47`](https://github.com/Byron/open-rs/commit/2f6fb473c53538544d3693a6057049b2ccdee6e2))
    - Un-deprecate `that_in_background()`. ([`adf99e9`](https://github.com/Byron/open-rs/commit/adf99e91af0f1c034123170e6dca77489484b004))
    - `with_command()` to obtain the command that opens a program with a given application. ([`66e0d7c`](https://github.com/Byron/open-rs/commit/66e0d7c295317aed27e89af8e323dde3b761c0e3))
    - Refactor ([`c2e0eb2`](https://github.com/Byron/open-rs/commit/c2e0eb2d08b54472485bbc552e91043dbfb163bb))
    - Improved error messages that will list the invoked command. ([`a501d65`](https://github.com/Byron/open-rs/commit/a501d65388562724471b5493fe54c88fe23df31e))
    - `commands()` function to obtain a list of launchers to open the given path. ([`245c95e`](https://github.com/Byron/open-rs/commit/245c95ede24adc6694d935993d6045d19a935035))
    - Remove dbg! statement ([`5644f17`](https://github.com/Byron/open-rs/commit/5644f17195244ca6d137f68f0f51a77d0be1fd99))
</details>

## 3.3.0 (2023-03-06)

### New Features

 - <csr-id-fdd2ef013fe7dee1767694e229f4653f16d4ab8a/> add command function to return std::Command
   Adds new `command` function that returns the underlying `std::Command`
   that would be called to open given path.
 - <csr-id-056cb003d19a6deb58214a041ee76d62716524c2/> use std Command
   Use std `Command` instead of `ShellExecuteW` from windows sys crate.
   
   This change was already attempted in: https://github.com/Byron/open-rs/issues/25
   and later reverted in: https://github.com/Byron/open-rs/pull/27
   and it it seems that it didn't work due to incorrect usage of
   `explorer` instead of `cmd /c start`.
   (see https://github.com/helix-editor/helix/pull/5820#issuecomment-1416796024
   for detailed explanation).

### Bug Fixes

 - <csr-id-55dc10dd1b3d04dbb0c7acb2b0a5efe7fc97dee6/> don't unnecessarily reject launchers on linux.
   Previously it probably went through 5 launers assuming they don't work
   before falling back to `xdg-open` which is usually present.
   
   Now it will use the first launcher that it could invoke.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 105 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v3.3.0 ([`6600c23`](https://github.com/Byron/open-rs/commit/6600c2363759dff3d932c2f2f0a762253c4d50b7))
    - Don't unnecessarily reject launchers on linux. ([`55dc10d`](https://github.com/Byron/open-rs/commit/55dc10dd1b3d04dbb0c7acb2b0a5efe7fc97dee6))
    - Try commands without args ([`8aa9bce`](https://github.com/Byron/open-rs/commit/8aa9bce4adbb66aefff4992b3ee7ee1d02774339))
    - Add command function to return std::Command ([`fdd2ef0`](https://github.com/Byron/open-rs/commit/fdd2ef013fe7dee1767694e229f4653f16d4ab8a))
    - Merge branch 'md/windows-command' ([`0d09f28`](https://github.com/Byron/open-rs/commit/0d09f2805d4055579727aef412bd0921c32dfbcc))
    - Remove win32-msvc from build matrix as it fails due to permission-denied with rustup.exe ([`1f4a9f9`](https://github.com/Byron/open-rs/commit/1f4a9f925329649fe0929936eef07d8e6f557ec4))
    - Refactor ([`7ab725a`](https://github.com/Byron/open-rs/commit/7ab725a2c6776a8f45305f71aec53650b45d9df7))
    - Use std Command ([`056cb00`](https://github.com/Byron/open-rs/commit/056cb003d19a6deb58214a041ee76d62716524c2))
</details>

## 3.2.0 (2022-11-21)

### New Features

 - <csr-id-c3d2819d121ede284ba12d26ac3272c1f664c4ed/> upgrade `windows-sys` to more recent version.
   This mainly reduces build times for some, and may increase them for
   others, on windows only. If build times increase, try to upgrade
   `windows-sys` across the dependency tree.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v3.2.0 ([`eea4f27`](https://github.com/Byron/open-rs/commit/eea4f27eef8d83614e99cbe332cdb69d81a2f43e))
    - Prepare changelog prior to release ([`20c6ee4`](https://github.com/Byron/open-rs/commit/20c6ee456d400416673d0e98058f55d50c78115a))
    - Upgrade `windows-sys` to more recent version. ([`c3d2819`](https://github.com/Byron/open-rs/commit/c3d2819d121ede284ba12d26ac3272c1f664c4ed))
    - Upgrade to windows-sys v0.42 ([`4de95c7`](https://github.com/Byron/open-rs/commit/4de95c73503b19f810d7e669b73e261b1004e689))
    - Revert "Upgrade to windows-sys v0.42.0" ([`2aff3bd`](https://github.com/Byron/open-rs/commit/2aff3bd2a2e917377ef10dcc4104c6aaf5895bd4))
</details>

## 3.1.0 (2022-11-20)

**YANKED**

### New Features

 - <csr-id-a1c8dd79eb6c4f91a92aa631fd0d8bc163d1a05c/> upgrade `windows-sys` to more recent version.
   This mainly reduces build times for some, and may increase them for
   others, on windows only. If build times increase, try to upgrade
   `windows-sys` across the dependency tree.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v3.1.0 ([`37bf011`](https://github.com/Byron/open-rs/commit/37bf011b68a5579254341de92e4d9b27ce71803b))
    - Upgrade `windows-sys` to more recent version. ([`a1c8dd7`](https://github.com/Byron/open-rs/commit/a1c8dd79eb6c4f91a92aa631fd0d8bc163d1a05c))
    - Upgrade to windows-sys v0.42.0 ([`aba0a62`](https://github.com/Byron/open-rs/commit/aba0a628b1bf1be365fdbe2bc0200d0c98c7a2bf))
    - Update listed version number. ([`7a1cc83`](https://github.com/Byron/open-rs/commit/7a1cc838d5fe0218e8d1422b42a32023fd140e67))
    - Merge branch 'fmt' ([`f4dfeab`](https://github.com/Byron/open-rs/commit/f4dfeabf43b2ede234892e1204248a85313b51b5))
    - Point docs link to docs.rs rather than an outdated copy ([`52f96fc`](https://github.com/Byron/open-rs/commit/52f96fc20f9a9c0db3464b3f8f1a24f8045145f2))
    - Update Readme ([`98316c4`](https://github.com/Byron/open-rs/commit/98316c42a236018d51fdc3c65afa7338237fe964))
</details>

## 3.0.3 (2022-09-16)

### Bug Fixes

 - <csr-id-4c0fdb3bacd73c881c6e8178248c588932ec6196/> quote paths on windows to allow spaces in paths not be treated as multiple paths.
   Note that paths that are already quoted will also be quoted, as the
   current quoting implementation is unconditional.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 60 calendar days.
 - 60 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v3.0.3 ([`9c69785`](https://github.com/Byron/open-rs/commit/9c697852432e5e34d5475706107e2f418b9296de))
    - Quote paths on windows to allow spaces in paths not be treated as multiple paths. ([`4c0fdb3`](https://github.com/Byron/open-rs/commit/4c0fdb3bacd73c881c6e8178248c588932ec6196))
    - Refactor ([`e0d5968`](https://github.com/Byron/open-rs/commit/e0d596880cd1d746d80927155092827614a7a3ef))
    - Fixed issue on Windows where a space in a path could cause problems with specific programs. ([`1ab9bc3`](https://github.com/Byron/open-rs/commit/1ab9bc37a0fc04d9fa033245d0c44392f2a2912a))
    - Try to fix CI by not using nightly toolchains on windows ([`b20e01c`](https://github.com/Byron/open-rs/commit/b20e01cf590d82a05841af1c92428249fe21d838))
</details>

## 3.0.2 (2022-07-18)

### Bug Fixes

 - <csr-id-fe70aad1ee0c792b83e1c5faabda8d2c142cdabe/> Improve documentation about blocking behaviour.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 35 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#51](https://github.com/Byron/open-rs/issues/51)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#51](https://github.com/Byron/open-rs/issues/51)**
    - Improve documentation about blocking behaviour. ([`fe70aad`](https://github.com/Byron/open-rs/commit/fe70aad1ee0c792b83e1c5faabda8d2c142cdabe))
 * **Uncategorized**
    - Release open v3.0.2 ([`c7ea529`](https://github.com/Byron/open-rs/commit/c7ea5291ac6a26da7346f995fad5b3121b02f488))
</details>

## 3.0.1 (2022-06-12)

### Bug Fixes

 - <csr-id-df358d296fc40801e970654bf2b689577637db5e/> deprecate `that_in_background()` as `that()` is definitely non-blocking now.
   Note that we keep `with_in_background()` as it's unclear if a custom
   launcher blocks or not.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v3.0.1 ([`757f773`](https://github.com/Byron/open-rs/commit/757f773a6d7e3afa35c2cab6f3f4a44c7c8facee))
    - Deprecate `that_in_background()` as `that()` is definitely non-blocking now. ([`df358d2`](https://github.com/Byron/open-rs/commit/df358d296fc40801e970654bf2b689577637db5e))
</details>

## 3.0.0 (2022-06-12)

A major release which simplifies the error type to resolve a significant problems that surfaced on
linux (and was present from day one).

### Bug Fixes (BREAKING)

 - <csr-id-0bdc6d64ed425b2627a7ba17614f44ba686536fb/> Assure `that(…)` is non-blocking on linux
   This change goes hand in hand with removing additional information
   from the error case which was the reason for the blocking issue
   on linux.
   
   Note that the top-level `Result` type was also removed.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 9 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v3.0.0 ([`3f51fb2`](https://github.com/Byron/open-rs/commit/3f51fb2e95a1f54c3ba54f349edefec34c25c7dc))
    - Update changelog and docs ([`10b92f5`](https://github.com/Byron/open-rs/commit/10b92f55de77c508a6cbd95c344a3d923b9207c4))
    - Refactor ([`475f002`](https://github.com/Byron/open-rs/commit/475f0021071fa1498a0fb5ca7d7336a3f4a35b7f))
    - Assure `that(…)` is non-blocking on linux ([`0bdc6d6`](https://github.com/Byron/open-rs/commit/0bdc6d64ed425b2627a7ba17614f44ba686536fb))
</details>

## 2.1.3 (2022-06-03)

A maintenance release which reduces compile times on windows by switching from `winapi` to the
`windows` crate.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 35 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v2.1.3 ([`bf6e99c`](https://github.com/Byron/open-rs/commit/bf6e99cb578cd3c90eed3ff4fddd712b26982e21))
    - Prepare new release ([`c1844c7`](https://github.com/Byron/open-rs/commit/c1844c7557b5e2d3c96cc19f4bc7e3fa7f2ef7d3))
    - Merge branch 'windows-sys' ([`246ddc8`](https://github.com/Byron/open-rs/commit/246ddc837d19760e9ad255ce31fbb6dfdac71738))
    - Switch to windows-sys ([`a95a288`](https://github.com/Byron/open-rs/commit/a95a2881064ec1a348031b2050d2873df2def31e))
</details>

## 2.1.2 (2022-04-29)

<csr-id-85f4dfdafe6119af5b3a5d8f079279818d3d61ee/>

### Other

 - <csr-id-85f4dfdafe6119af5b3a5d8f079279818d3d61ee/> add Heiku platform support

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 54 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v2.1.2 ([`ee25446`](https://github.com/Byron/open-rs/commit/ee25446e872c18253bfe4c974b534ea8dd993cc2))
    - Update changelog ([`45e0388`](https://github.com/Byron/open-rs/commit/45e0388e3c0a1b255b5868d6e0c3a540b75c33e9))
    - Add platform support ([`85f4dfd`](https://github.com/Byron/open-rs/commit/85f4dfdafe6119af5b3a5d8f079279818d3d61ee))
</details>

## 2.1.1 (2022-03-05)

A maintenance release which allows boxed values in parameter position.

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
    - Release open v2.1.1 ([`18f730d`](https://github.com/Byron/open-rs/commit/18f730d7d40f7e27002479adb41b342413235ce7))
    - Prepare changelog ([`d569761`](https://github.com/Byron/open-rs/commit/d569761a7c6c57f92e48fc6ac195baf13df8666d))
    - Revert rust edition version ([`9441d6c`](https://github.com/Byron/open-rs/commit/9441d6c87419f94e0ebaffdf69f9b01f0aec4ddb))
    - Update to 2021 edition and remove Sized bound ([`2601e4e`](https://github.com/Byron/open-rs/commit/2601e4eff11a77a7ccd5acfa3215eb76450fe18c))
</details>

## 2.1.0 (2022-02-22)

* add support for illumnos

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 8 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v2.1.0 ([`a49e9cc`](https://github.com/Byron/open-rs/commit/a49e9ccac9ea89dabc19b1a0215378ede887260b))
    - Update changelog ([`b56050f`](https://github.com/Byron/open-rs/commit/b56050f41fc04a2d5ec61f20451df534315f7d74))
    - Add Illumos support ([`5d43c13`](https://github.com/Byron/open-rs/commit/5d43c13e5418f1d34b44cab71ee7306402fe5823))
</details>

## 2.0.3 (2022-02-13)

On MacOS, specify the `open` program explicitly by path, instead of relying on a similarly named program to be available
in the `PATH`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 74 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release open v2.0.3 ([`3b5e74d`](https://github.com/Byron/open-rs/commit/3b5e74dbab169ee2a22c9de0b3a5923dc7e6937e))
    - Prepare changelog ([`1c7e10f`](https://github.com/Byron/open-rs/commit/1c7e10f94c30598bdc2e4ae482d38b2f46928ebf))
    - Use full path for `open` command on macOS ([`8f7c92a`](https://github.com/Byron/open-rs/commit/8f7c92ab1adf936cd43e4ba0eb1934e2c73763f7))
</details>

## 2.0.2 (2021-12-01)

### Bug Fixes

 - <csr-id-30a144ac15acffbc78005cd67d3f783aa2526498/> Prevent deadlocks due to filled pipe on stderr

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 128 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#85](https://github.com/Byron/open-rs/issues/85)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#85](https://github.com/Byron/open-rs/issues/85)**
    - Prevent deadlocks due to filled pipe on stderr ([`30a144a`](https://github.com/Byron/open-rs/commit/30a144ac15acffbc78005cd67d3f783aa2526498))
 * **Uncategorized**
    - Release open v2.0.2 ([`1d94593`](https://github.com/Byron/open-rs/commit/1d94593fa7be75ffdafcb7614c0f68fe4485f07a))
    - Update changelog ([`e9a2f05`](https://github.com/Byron/open-rs/commit/e9a2f05ec8248b3723779dfead6fbd4827a2f929))
    - Release open v2.0.1 ([`066a591`](https://github.com/Byron/open-rs/commit/066a591823ddebb2904959b6395bc945c22ba213))
    - Merge pull request #36 from apogeeoak/documentation ([`fc755d3`](https://github.com/Byron/open-rs/commit/fc755d343cede927c06e1735e8d14ed3858d2582))
    - Add no_run to documentation examples. ([`7c97658`](https://github.com/Byron/open-rs/commit/7c9765891b86d5d6168556e8f5363641f57e130d))
    - Update documentation. ([`5dd987f`](https://github.com/Byron/open-rs/commit/5dd987f3d25ebf3c82394d1225b836aefaf93b5d))
</details>

## v2.0.1 (2021-08-15)

Update documentation. No functionality changes.

## v2.0.0 (2021-07-25)

**Breaking**: Change result from `io::Result<ExitStatus>` to `io::Result<()>`.
Commands that exit with a successful exit status result in `Ok`, otherwise an `Err` variant is created.
Previously it was easy to receive an `Ok(ExitStatus)` but forget to actually check the status. Along with
issues with particular programs reporting success even on error, doing error handling correctly was
close to impossible.
This releases alleviates most of the issues.

## Notes

`wslview` always reports a 0 exit status, even if the path does not exist, which results in false positives.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 8 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 2.0.0 ([`bc2e36f`](https://github.com/Byron/open-rs/commit/bc2e36f5d61b81974420cd62650d743afd4b6824))
    - Update changelog ([`6659519`](https://github.com/Byron/open-rs/commit/665951968a2d99cbebaf41bc2dd564ea9d6dc93c))
    - Merge branch 'result_type' ([`0226df6`](https://github.com/Byron/open-rs/commit/0226df6be4abd85f0c8f8001532d0c67ad231a49))
    - Merge pull request #34 from apogeeoak/rustfmt ([`05f02be`](https://github.com/Byron/open-rs/commit/05f02be302377d669350f30991c2f80e6a729bc7))
    - Encode unsuccessful exit status in Err. ([`668734e`](https://github.com/Byron/open-rs/commit/668734ee8d4a3b3c48c9d3ad892280ce8e71f943))
    - Add empty rustfmt.toml file to enforce defaults. ([`1faabe3`](https://github.com/Byron/open-rs/commit/1faabe36fcaa4986b25bbc91a08d41759d1b8b88))
</details>

## v1.7.1 (2021-07-17)

* Improved support for [Windows Subsystem for Linux](https://github.com/Byron/open-rs/pull/33#issue-691044025)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release.
 - 89 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 1.7.1 ([`c5c0bf7`](https://github.com/Byron/open-rs/commit/c5c0bf7ca070fb81fbbb1bd06d51b490a8f8bf1a))
    - Prepare release ([`866740b`](https://github.com/Byron/open-rs/commit/866740b10e1f5b03fc4a3aab847546b7c378b6d9))
    - Be bold and assert ([`1dfb789`](https://github.com/Byron/open-rs/commit/1dfb7892554087ab07c7c0da8ef863d368e109e3))
    - Cargo fmt ([`5bc5e8e`](https://github.com/Byron/open-rs/commit/5bc5e8e739915d4850d4a973d9cf13591aa337cc))
    - Improve support for wsl. ([`428ff97`](https://github.com/Byron/open-rs/commit/428ff979979760132d7c583df6834c3349132350))
    - Merge pull request #32 from apogeeoak/exit_status ([`81d8c40`](https://github.com/Byron/open-rs/commit/81d8c406cdf9405e31965a5aea9a5d21da812433))
    - Cargo fmt ([`215227a`](https://github.com/Byron/open-rs/commit/215227a3385aa2624d32567eebb08af49e258b60))
    - Clarify what the error handler does ([`4f87a78`](https://github.com/Byron/open-rs/commit/4f87a7888049b182ede9e00a057c2cc625152ef9))
    - Handle unsuccessful exit status. ([`d2d35af`](https://github.com/Byron/open-rs/commit/d2d35af2f582249030fc569854450ac12e3c08d4))
</details>

## v1.7.0 (2021-04-18)

* Add `gio` support on unix platforms

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 38 calendar days.
 - 38 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 1.7.0 ([`ac09da1`](https://github.com/Byron/open-rs/commit/ac09da180256c5974427313b845b928199faf913))
    - Update changelog ([`e39f357`](https://github.com/Byron/open-rs/commit/e39f357627288d971b6436f873ac2949fa534548))
    - Re-enable CI after branch renaming ([`0db1b1a`](https://github.com/Byron/open-rs/commit/0db1b1ad11853750b8c22a701438d0e3e149821b))
    - Merge pull request #31 from City-busz/patch-1 ([`10fd4a7`](https://github.com/Byron/open-rs/commit/10fd4a7183c9137bb1afee5a9a6d3dcc87eb821a))
    - Remove unnecessary allocation ([`6a1766a`](https://github.com/Byron/open-rs/commit/6a1766a602fa3354827b06d7b5dbf8f694b86690))
    - Add support for gio open on Linux ([`90bc634`](https://github.com/Byron/open-rs/commit/90bc6348e00e2e42cc0f7ed3eb7746d6e749749e))
    - Update changelog to reflect 1.5.1 is also yanked ([`ccbae5d`](https://github.com/Byron/open-rs/commit/ccbae5d122cb0b8cff58d9125ced2d0211e82ec9))
</details>

## v1.6.0 (2021-03-10)

* Add IOS support
* Restore Android support

## v1.5.1 (2021-03-03) - YANKED

YANKED as it would erroneously exclude Android from the list of supported platforms, making it a breaking release for some despite
the minor version change.

* Use shell instead of explorer on windows, reverting the original behaviour.

## v1.5.0 (2021-02-28) - YANKED

YANKED to avoid potential for breakage by using 'explorer.exe' to open URLs.

* Use 'explorer' on Windows instead of a shell.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 7 calendar days.
 - 7 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 1.6.0 ([`68613a5`](https://github.com/Byron/open-rs/commit/68613a5cabc1e650ab36ce5c2802c1b29e6af569))
    - More coherent ordering of target_os attributes ([`c058966`](https://github.com/Byron/open-rs/commit/c058966e5ec4cbd52c7cb50e5ee29afdac08cc15))
    - Restore android support ([`9e20f22`](https://github.com/Byron/open-rs/commit/9e20f22453955e5d7adba94cea8321961fac30ed))
    - Adjust changelog in preparation for release ([`9bfefd0`](https://github.com/Byron/open-rs/commit/9bfefd0e38ccce6f898ac895b10ab5555606744f))
    - Merge pull request #28 from aspenluxxxy/ios ([`049f698`](https://github.com/Byron/open-rs/commit/049f698714cacfad9142db492d9f309af567d26a))
    - Bring back Android support ([`ef91705`](https://github.com/Byron/open-rs/commit/ef9170527d6e9eb58e2b11e73e2756ccbc6b170b))
    - Add iOS support ([`00119a7`](https://github.com/Byron/open-rs/commit/00119a7e5b00889828ab9d38dd5959a519f22b1d))
    - Run cargo-fmt ([`330c2d0`](https://github.com/Byron/open-rs/commit/330c2d02f92e3660a86158713a9a9c3aba542094))
</details>

## v1.5.1 (2021-03-03)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 3 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - (cargo-release) version 1.5.1 ([`147f428`](https://github.com/Byron/open-rs/commit/147f428762c84d0353644b5b846756ac38b29670))
    - Changelog for patch ([`9400b1a`](https://github.com/Byron/open-rs/commit/9400b1a67ccf02ff757ecb392d179440ddb98eac))
    - Minor refactor ([`67ea295`](https://github.com/Byron/open-rs/commit/67ea2950aa2c478c8cd63764145ad53ad55bdd11))
    - Merge pull request #27 from hybras/master ([`b58fa52`](https://github.com/Byron/open-rs/commit/b58fa52eb8ee46a789c864b7132e8375fe7efa77))
    - Keep Fork up to date with upstream ([`f113b80`](https://github.com/Byron/open-rs/commit/f113b80374ed1412d2d86e79b79f7ac9ef39a2fc))
    - Revert "Add missing Command import" ([`7ff85da`](https://github.com/Byron/open-rs/commit/7ff85da679de7cd17155c4ea27d0f89fda6dff0a))
    - Revert "Use the file explorer to open windows url's" ([`b2a79f6`](https://github.com/Byron/open-rs/commit/b2a79f6b93feef3a59ce57d865334d757e642540))
</details>

## v1.5.0 (2021-02-28)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 321 calendar days.
 - 356 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Minor version bump ([`831d440`](https://github.com/Byron/open-rs/commit/831d4404ee3ab9d76a416f69ee586f7d037f4840))
    - Merge branch 'hybras/master' ([`e8bb206`](https://github.com/Byron/open-rs/commit/e8bb20654ac8b8ffbdfebf70f9aa1f0d3cdc0a33))
    - Delete completed TODO file ([`6c6bad0`](https://github.com/Byron/open-rs/commit/6c6bad075a5dcdc12670ec885000e26810bcf4fc))
    - Remove unneeded pub ([`3507b55`](https://github.com/Byron/open-rs/commit/3507b55dcaa30db5673cbe8b7a405db4f00245ac))
    - Remove user specific dir from gitgnore ([`021bb15`](https://github.com/Byron/open-rs/commit/021bb150d6066b111bdb04d2c4340dc9172db562))
    - Add missing Command import ([`c910278`](https://github.com/Byron/open-rs/commit/c9102785d58cc955595eb189bd89a2ff82a539f0))
    - Use the file explorer to open windows url's ([`4545425`](https://github.com/Byron/open-rs/commit/45454254b6e07fd88e398e8de86b55863f369373))
    - Mark completed todo items ([`db518e9`](https://github.com/Byron/open-rs/commit/db518e9063933df824c4bb0e0c560bc73ef1b700))
    - Use which in non-macOS unix ([`ef8ab99`](https://github.com/Byron/open-rs/commit/ef8ab99d65ce7baf03d43304b3c0cb48e816e411))
    - Change cfg(not(any(bad)) to cfg(any(good)) ([`204f0ca`](https://github.com/Byron/open-rs/commit/204f0ca89f522ca4e6dc31b0cdefc3bcd434909b))
    - Modularize Code ([`cb5bbd3`](https://github.com/Byron/open-rs/commit/cb5bbd3287bf2ca66e6ea3afefb149e4fe12bdd8))
    - Add todo's ([`311ad44`](https://github.com/Byron/open-rs/commit/311ad44c50ddba910c13f3cd85326522accc8e23))
    - Optimize manifest includes ([`c3d8262`](https://github.com/Byron/open-rs/commit/c3d826220e59040d6d08d707ac771ba817165a07))
    - See if we can run cargo clippy and rustfmt as well ([`c90687d`](https://github.com/Byron/open-rs/commit/c90687de90eb3731ec508c8d3df639de582fb163))
    - Actually link to the correct workflow when clicking the badge ([`6765b42`](https://github.com/Byron/open-rs/commit/6765b424010b55e23568924786700a3795e694dc))
    - Bye bye travis, we had a great time ([`aa28a85`](https://github.com/Byron/open-rs/commit/aa28a858dfe8be9c34e3fd6a6df67722baec4df1))
    - Rename workflow in file as well ([`6bfc6d2`](https://github.com/Byron/open-rs/commit/6bfc6d2e9efdbd656a37531fe43cca6ab443a2b9))
    - Try cross-platform testing based on cross-platform binary builds ([`d62e50d`](https://github.com/Byron/open-rs/commit/d62e50d7b1944597468b2c983047e236ae9ff08f))
</details>

## v1.4.0 (2020-03-08)

* add `open::with(path, app)` and `open::with_in_background(…)`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 25 calendar days.
 - 25 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Adjust doc; cut release ([`ead2494`](https://github.com/Byron/open-rs/commit/ead2494b783ffd0e393972aeb1f82adaf39fe0d3))
    - Cargo fmt ([`94b129a`](https://github.com/Byron/open-rs/commit/94b129ad998729967a856f19f74e4628957ea99b))
    - Fixed import bug ([`e98ec3d`](https://github.com/Byron/open-rs/commit/e98ec3d79ef199dc16f3ce65b766aa0110abaaf0))
    - Update README.md ([`9efaee0`](https://github.com/Byron/open-rs/commit/9efaee0b5402c725e2c152643d448182881a2898))
    - Add with function ([`9b83669`](https://github.com/Byron/open-rs/commit/9b83669e8c463648b6f4149e84fcb1e00d68f49b))
    - (cargo-release) start next development iteration 1.3.5-alpha.0 ([`d3db8c7`](https://github.com/Byron/open-rs/commit/d3db8c748be2e65865aed7246cd8eaeaacd4ef8a))
</details>

## v1.3.4 (2020-02-11)

<csr-id-5c1497c6d09a829d4be19e9bd3eec5557efce370/>

* Add LICENSE.md and README.md into the crates.io tarball.

### Chore

 - <csr-id-5c1497c6d09a829d4be19e9bd3eec5557efce370/> Include README/LICENSE into a release tarball

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 9 calendar days.
 - 184 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version ([`547fd28`](https://github.com/Byron/open-rs/commit/547fd283e684470e5e981a658d98c31208da1e8b))
    - Include README/LICENSE into a release tarball ([`5c1497c`](https://github.com/Byron/open-rs/commit/5c1497c6d09a829d4be19e9bd3eec5557efce370))
    - Further simplification ([`9f285e5`](https://github.com/Byron/open-rs/commit/9f285e559878f3da2eb54f50aa88632385618f7c))
    - Update to edition 2018 ([`dfca673`](https://github.com/Byron/open-rs/commit/dfca6736f69555e3285786bb10719adb0ae1d0c7))
</details>

## v1.3.3 (2020-02-01)

* update code and crate to Edition 2018

## v1.3.2 (2019-08-11)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 13 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version ([`9d9e40c`](https://github.com/Byron/open-rs/commit/9d9e40cc9b68266652a5ac21915b558b812ee444))
    - Improve documentation ([`d45e4c6`](https://github.com/Byron/open-rs/commit/d45e4c6103f95027b3ef397d5f03a8b75bcdb03d))
    - Add that_in_background ([`5927784`](https://github.com/Byron/open-rs/commit/5927784721174259af5e6f3d07f724f5b6e89501))
</details>

## v1.3.1 (2019-07-29)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 25 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - New minor release with wslview support ([`cb41dce`](https://github.com/Byron/open-rs/commit/cb41dce487b9936c3bf93d242dc6ff70a7924f0a))
    - Use wslview on WSL, try it as last binary ([`0babfd0`](https://github.com/Byron/open-rs/commit/0babfd0abfa266739a8aaadf8fc936c8c061ac0b))
    - Add support for Linux in WSL through wslu/wslview ([`0a43537`](https://github.com/Byron/open-rs/commit/0a4353764a17579e92ae97ea08ea226ace5cc86a))
</details>

## v1.2.3 (2019-07-03)

<csr-id-c2908176e2bb982a679d7097584e584a53deaf15/>

### Chore

 - <csr-id-c2908176e2bb982a679d7097584e584a53deaf15/> Exclude unneeded files from crates.io

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 16 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version ([`2e8d245`](https://github.com/Byron/open-rs/commit/2e8d2453d801cb27b311b27bf49b06791a35958a))
    - Supress stdout and stderr for non-windows platforms ([`4e3574a`](https://github.com/Byron/open-rs/commit/4e3574a20a84c8a0d681e11ec351d20e35b73ea4))
    - Exclude unneeded files from crates.io ([`c290817`](https://github.com/Byron/open-rs/commit/c2908176e2bb982a679d7097584e584a53deaf15))
</details>

## v1.2.2 (2018-08-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 314 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version: better way to open things on windows ([`78e94aa`](https://github.com/Byron/open-rs/commit/78e94aa545e531f99aabcd0a328adc44e4ed06a6))
    - Use ShellExecute rather than start.exe on windows ([`e2fc4b1`](https://github.com/Byron/open-rs/commit/e2fc4b1061ef105e237b4dda1ffa03eaf3a1cdb4))
    - Small optimizations and stylistic improvements ([`88ddb6f`](https://github.com/Byron/open-rs/commit/88ddb6febe811fa8f5ab12b32cbf2a716676fb53))
    - Adjust code style ([`dd9dde6`](https://github.com/Byron/open-rs/commit/dd9dde6aa4cd47ca57378ac018a66dbbcd661d44))
    - Add crates version badge ([`4e41d8b`](https://github.com/Byron/open-rs/commit/4e41d8bdf0c3bbca84efc1de9759e06839208c86))
    - Run latest rustfmt ([`ec5c7ab`](https://github.com/Byron/open-rs/commit/ec5c7ab817f3978212b0230512b75a1a8b5374f1))
</details>

## v1.2.1 (2017-09-24)

<csr-id-79bc73b7ca0927f0594670bcc23de989693275c0/>

### Other

 - <csr-id-79bc73b7ca0927f0594670bcc23de989693275c0/> improve example

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 178 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Version bump - fix windows 'start' invocation ([`d52cfb2`](https://github.com/Byron/open-rs/commit/d52cfb252e3feee3d3793bc91d28e9cf6f193947))
    - Merge pull request #11 from reiner-dolp/master ([`59dd0bd`](https://github.com/Byron/open-rs/commit/59dd0bd64921594ba3ed59ddb373aac55d33c95d))
    - Merge pull request #9 from skade/patch-1 ([`c017217`](https://github.com/Byron/open-rs/commit/c017217f4b975a297d2116e35fb230b3b370c9cf))
    - Fix filenames with spaces on windows ([`a631235`](https://github.com/Byron/open-rs/commit/a631235c285b5f48ce63a52cbf7d70f51439db06))
    - Fix a small typo ([`89caa59`](https://github.com/Byron/open-rs/commit/89caa594bf2e16929dc74565e197ba2d3cbd8390))
    - Merge pull request #7 from tshepang/misc ([`0ccdbd0`](https://github.com/Byron/open-rs/commit/0ccdbd08f450f364ce3538fe28a05f41c8188ae6))
    - Improve example ([`79bc73b`](https://github.com/Byron/open-rs/commit/79bc73b7ca0927f0594670bcc23de989693275c0))
</details>

## v1.2.0 (2017-01-31)

<csr-id-37a253c89b1241b6f6ca0d3cafc8baa936aa274f/>

* **windows**: escape '&' in URLs. On windows, a shell is used to execute the command, which
  requires certain precautions for the URL to open to get through the interpreter.

### Chore

 - <csr-id-37a253c89b1241b6f6ca0d3cafc8baa936aa274f/> v1.2.0

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - V1.2.0 ([`37a253c`](https://github.com/Byron/open-rs/commit/37a253c89b1241b6f6ca0d3cafc8baa936aa274f))
    - Merge pull request #6 from DenisKolodin/win-escape-fix ([`d0b3b35`](https://github.com/Byron/open-rs/commit/d0b3b35b4881da297cc44847dc5f3500c25eef61))
    - Escape GET parameters for Windows ([`3f4319c`](https://github.com/Byron/open-rs/commit/3f4319c79e293fb8141e6574db710a7f97e0f1d4))
</details>

## v1.1.1 (2016-04-10)

<csr-id-da45d9bad33fd9ed9659ec56ffe3b31d310253ca/>

### Bug Fixes

* **cargo:**  no docs for open ([31605e0e](https://github.com/Byron/open-rs/commit/31605e0eddfb0cf8db635dd4d86131bc46beae78))
 - <csr-id-31605e0eddfb0cf8db635dd4d86131bc46beae78/> no docs for open
   And I thought I did that, but disabled tests only ... .

### Improvements

* **api:**  allow OSStrings instead of &str ([1d13a671](https://github.com/Byron/open-rs/commit/1d13a671f2c9bd9616bf185fac77b32da1dcf8ee))

### Other

 - <csr-id-da45d9bad33fd9ed9659ec56ffe3b31d310253ca/> allow OSStrings instead of &str
   Actually I can only hope that ordinary &str will still be fine.
   Technically, I think they should ... but we shall see.

## 25c0e398 (2015-07-08)

### Features

* **open**  added 'open' program ([a4c3a352](https://github.com/Byron/open-rs/commit/a4c3a352c8f912211d5ab48daaf41cb847ebcc0c))

### Bug Fixes

* **cargo**  description added ([0fcafb56](https://github.com/Byron/open-rs/commit/0fcafb56cdb5d154b3e983d17c93a1dd7c665426))
* **open**
  * use result ([25c0e398](https://github.com/Byron/open-rs/commit/25c0e398856c24a2daf0444640567ed3fd2f4307))
  * don't use 'open' on linux ([30c96b1c](https://github.com/Byron/open-rs/commit/30c96b1cb95c1e03bede218b8fb03bbd9ada9317))
  * linux uses open before anything else ([4696d1a5](https://github.com/Byron/open-rs/commit/4696d1a5ec80691e97bb1be4261d4f79ee0ade4d))
* don't use 'open' on linux ([30c96b1c](https://github.com/Byron/open-rs/commit/30c96b1cb95c1e03bede218b8fb03bbd9ada9317))
* linux uses open before anything else ([4696d1a5](https://github.com/Byron/open-rs/commit/4696d1a5ec80691e97bb1be4261d4f79ee0ade4d))
 - <csr-id-31605e0eddfb0cf8db635dd4d86131bc46beae78/> no docs for open
   And I thought I did that, but disabled tests only ... .

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 276 calendar days.
 - 276 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Allow OSStrings instead of &str ([`da45d9b`](https://github.com/Byron/open-rs/commit/da45d9bad33fd9ed9659ec56ffe3b31d310253ca))
    - Merge pull request #2 from hoodie/master ([`ff32bea`](https://github.com/Byron/open-rs/commit/ff32beac235d5702a14752a9166ce3a168c6b250))
    - Taking T:AsRef<OsStr> instead of &str ([`2540a0a`](https://github.com/Byron/open-rs/commit/2540a0a6abc4b27d6553400e7aef62e3ef94020d))
    - No docs for open ([`31605e0`](https://github.com/Byron/open-rs/commit/31605e0eddfb0cf8db635dd4d86131bc46beae78))
</details>

<csr-unknown>
don’t use ‘open’ on linux (https://github.com/Byron/open-rs/commit/30c96b1cb95c1e03bede218b8fb03bbd9ada931730c96b1c)linux uses open before anything else (https://github.com/Byron/open-rs/commit/4696d1a5ec80691e97bb1be4261d4f79ee0ade4d4696d1a5)<csr-unknown/>

## v1.1.0 (2015-07-08)

<csr-id-a5557d5c096983cf70f59b1807cb6fbe2b6dab5e/>
<csr-id-8db67f5874b007ea3710ed9670e88ad3f49b6d7d/>
<csr-id-d816380f9680a9d56e22a79e025dc6c2073fb439/>
<csr-id-bf8c9a11f4c1b1ac17d684a31c90d2a38255045e/>
<csr-id-210ec6ef37ba7d230a0cc367e979173a555fa092/>

### Chore

 - <csr-id-a5557d5c096983cf70f59b1807cb6fbe2b6dab5e/> v1.1.0
   * added clog configuration and changelog
 - <csr-id-8db67f5874b007ea3710ed9670e88ad3f49b6d7d/> use stable instead of beta
 - <csr-id-d816380f9680a9d56e22a79e025dc6c2073fb439/> switch to travis-cargo
 - <csr-id-bf8c9a11f4c1b1ac17d684a31c90d2a38255045e/> added sublime-rustc-linter cfg
   [skip ci]

### Other

 - <csr-id-210ec6ef37ba7d230a0cc367e979173a555fa092/> start is a cmd command, not an executable

### Documentation

 - <csr-id-c2e31d55da439e30639da2d014951e2eb2b851ff/> added travis badge
   [skip ci]

### New Features

 - <csr-id-a4c3a352c8f912211d5ab48daaf41cb847ebcc0c/> added 'open' program
   Which uses the `open` library to open any path or url.

### Bug Fixes

<csr-id-30c96b1cb95c1e03bede218b8fb03bbd9ada9317/>
<csr-id-4696d1a5ec80691e97bb1be4261d4f79ee0ade4d/>
<csr-id-0fcafb56cdb5d154b3e983d17c93a1dd7c665426/>

 - <csr-id-25c0e398856c24a2daf0444640567ed3fd2f4307/> use result
   I wonder why that was not shown when I compiled it
 - <csr-id-8b4e1558f09937c555ab381ea6399a2c0758c23d/> (07560d233 2015-04-20) (built 2015-04-19)
   * use std::io consistently
* adjust to improved `Command` API

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release.
 - 12 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - V1.1.0 ([`a5557d5`](https://github.com/Byron/open-rs/commit/a5557d5c096983cf70f59b1807cb6fbe2b6dab5e))
    - Use result ([`25c0e39`](https://github.com/Byron/open-rs/commit/25c0e398856c24a2daf0444640567ed3fd2f4307))
    - Added 'open' program ([`a4c3a35`](https://github.com/Byron/open-rs/commit/a4c3a352c8f912211d5ab48daaf41cb847ebcc0c))
    - Merge pull request #1 from oli-obk/patch-1 ([`dee0000`](https://github.com/Byron/open-rs/commit/dee00005fa1089e97fc4e193c934f6e7b3104333))
    - Start is a cmd command, not an executable ([`210ec6e`](https://github.com/Byron/open-rs/commit/210ec6ef37ba7d230a0cc367e979173a555fa092))
    - Use stable instead of beta ([`8db67f5`](https://github.com/Byron/open-rs/commit/8db67f5874b007ea3710ed9670e88ad3f49b6d7d))
    - Switch to travis-cargo ([`d816380`](https://github.com/Byron/open-rs/commit/d816380f9680a9d56e22a79e025dc6c2073fb439))
    - Added sublime-rustc-linter cfg ([`bf8c9a1`](https://github.com/Byron/open-rs/commit/bf8c9a11f4c1b1ac17d684a31c90d2a38255045e))
    - (07560d233 2015-04-20) (built 2015-04-19) ([`8b4e155`](https://github.com/Byron/open-rs/commit/8b4e1558f09937c555ab381ea6399a2c0758c23d))
    - Don't use 'open' on linux ([`30c96b1`](https://github.com/Byron/open-rs/commit/30c96b1cb95c1e03bede218b8fb03bbd9ada9317))
    - Linux uses open before anything else ([`4696d1a`](https://github.com/Byron/open-rs/commit/4696d1a5ec80691e97bb1be4261d4f79ee0ade4d))
    - Description added ([`0fcafb5`](https://github.com/Byron/open-rs/commit/0fcafb56cdb5d154b3e983d17c93a1dd7c665426))
    - Added travis badge ([`c2e31d5`](https://github.com/Byron/open-rs/commit/c2e31d55da439e30639da2d014951e2eb2b851ff))
</details>

## v1.0.0 (2015-02-27)

### New Features

 - <csr-id-6fbf79011577d465d9fed94a07a5f75b63199609/> from zero to 1.0.0
   Contains everything, including
   
   * API docs
* usage
* CI

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - From zero to 1.0.0 ([`6fbf790`](https://github.com/Byron/open-rs/commit/6fbf79011577d465d9fed94a07a5f75b63199609))
</details>


# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.9 (2026-01-30)

* Add support for delegating setters for generic types. (Thanks @MrSubidubi)
* Documentation has been moved from the `README.md` to a proper rustdoc.
* MSRV is increased to 1.68+ due to dependency updates.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add proper changelog entries. ([`87037bf`](https://github.com/Lymia/derive_setters/commit/87037bf65042bf127aa0c1d3986285731ccbfb2b))
    - Adjusting changelogs prior to release of derive_setters v0.1.9 ([`f8a3c46`](https://github.com/Lymia/derive_setters/commit/f8a3c46737f229cb9472f0fafe790d5f0d115609))
    - Add tests for generics in compile-pass ([`8d37502`](https://github.com/Lymia/derive_setters/commit/8d375020487594b888ebd0f6e8e38007916eb663))
    - Add trybuild tests. ([`74e353f`](https://github.com/Lymia/derive_setters/commit/74e353f6a9413047383e28898962fc3ecbe8d90d))
    - Update Rust version. ([`1ca0910`](https://github.com/Lymia/derive_setters/commit/1ca091031ec85d3e7dbeca33f234986677d037f4))
    - Update the rustfmt options and run rustfmt. ([`0ab982f`](https://github.com/Lymia/derive_setters/commit/0ab982f01254c1e17b7745f54aebe6a7f1ece22b))
    - Move the documentation into rustdoc. ([`722f3fc`](https://github.com/Lymia/derive_setters/commit/722f3fc029f56fdf31c8e838a4a8a05a609d0a36))
    - Code style improvements. ([`da6237b`](https://github.com/Lymia/derive_setters/commit/da6237bccf4f56519e38e0f3d4a51cd6c050dae0))
    - Remove an unused field. ([`8a112f3`](https://github.com/Lymia/derive_setters/commit/8a112f31abc90a511c92595422fc946cd6d0006b))
    - Update MSRV to 1.68.0 due to dependency updates. ([`474375d`](https://github.com/Lymia/derive_setters/commit/474375d304cbafa4b8bd8b0d27060a71c0588e71))
    - Update documentation and adds a changelog. ([`66c390a`](https://github.com/Lymia/derive_setters/commit/66c390ae72f2644d89c6a6706f5b3fd523e30fee))
    - Update darling version. ([`30ea90e`](https://github.com/Lymia/derive_setters/commit/30ea90e7d7c0be674eb9ccc9324e4d020b671299))
    - Implement tests for generic delegates. ([`bd14002`](https://github.com/Lymia/derive_setters/commit/bd1400267cee1e30d216eff0ac0cd9d20dea3c2c))
    - Run rustfmt. ([`ccdc14e`](https://github.com/Lymia/derive_setters/commit/ccdc14e939ac9cd3b098015c9851b21685d077c9))
    - Merge pull request #19 from MrSubidubi/delegate-generic-support ([`284500e`](https://github.com/Lymia/derive_setters/commit/284500e046344bb9a407d954cc44066a4b641c45))
    - Add support for generics in `generate_delegates` ([`d777525`](https://github.com/Lymia/derive_setters/commit/d7775259dfb21564b2c5cee4d5b3a692158c5b6b))
</details>

## v0.1.8 (2025-07-08)

* Fixes the logic for `#[setters(must_use)]`. (Thanks @eakoli)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 63 calendar days.
 - 63 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version to 0.1.8 ([`7398bc1`](https://github.com/Lymia/derive_setters/commit/7398bc189ac22b1bca214445a74949fe051df34a))
    - Merge pull request #17 from eakoli/must_use_fix ([`2b50b7f`](https://github.com/Lymia/derive_setters/commit/2b50b7f75343f910452390e37d94e4be563a47d6))
    - Correct the logic for #[must_use] (was inverted) ([`66e2729`](https://github.com/Lymia/derive_setters/commit/66e2729c11f2f08f8437e326045d6fc042417207))
    - Fix unused variable warning. ([`6c5ff44`](https://github.com/Lymia/derive_setters/commit/6c5ff44f8f9d05b9f954fea01fb5f8cb443904f1))
</details>

## v0.1.7 (2025-05-05)

* Adds license texts to the documentation for packaging reasons. (Thanks @krasimirgg)
* Adds `#[setters(doc = "…")]` to allow overriding the setter documentation on a per-field basis.
* Adds `#[setters(prefix = "…")]` to allow prefixing generated functions with something like `with_`. (Thanks @someone13574)
* Adds `#[setters(must_use)]` to generate a `#[must_use]` annotation on consuming setters. (Thanks @eakoli)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 13 commits contributed to the release.
 - 709 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version to 0.1.7 ([`bc58434`](https://github.com/Lymia/derive_setters/commit/bc58434513ce20ba0b520dcc7c34af114bd5b50f))
    - Merge branch 'must_use' of https://github.com/eakoli/derive_setters ([`e6f7c7c`](https://github.com/Lymia/derive_setters/commit/e6f7c7ceb92d2e260f8ed0a4a62d6a6f49133ea4))
    - Update copyright and license. ([`abff3f3`](https://github.com/Lymia/derive_setters/commit/abff3f3c0337d907a24ae2e84c6b2d0f9d74613a))
    - Support for adding #[must_use] on consuming setters ([`5b4adf6`](https://github.com/Lymia/derive_setters/commit/5b4adf64609960125cc2d3297be160f07c2dad9e))
    - Merge pull request #12 from redforks/master ([`5512c2e`](https://github.com/Lymia/derive_setters/commit/5512c2ee4eaaf05fdac2cc1ada14fe9bdfb38d6c))
    - Fix clippy::needless_update ([`e12e428`](https://github.com/Lymia/derive_setters/commit/e12e428ee3640ca845c2f0e76de50c4fe3ae9cd4))
    - Add support for overriding the setter documentation on a per-field basis. (Fixes #10) ([`553387b`](https://github.com/Lymia/derive_setters/commit/553387b7895eb62d4ce0a16fa9f103cd92b26662))
    - Merge branch 'master' of github.com:Lymia/derive_setters ([`8de3bd3`](https://github.com/Lymia/derive_setters/commit/8de3bd3e17c8c817e3b65e40495918bc073de71e))
    - Merge commit '2f036eb' from 'someone13574:delegate-prefixes' ([`d49d2d7`](https://github.com/Lymia/derive_setters/commit/d49d2d763b999f0486b942391e9cc87c378ff833))
    - Merge pull request #7 from krasimirgg/lics ([`79b3482`](https://github.com/Lymia/derive_setters/commit/79b3482d018a19b27afa32848125cc7c6c5f2bf6))
    - Update README.md for prefixing delegates ([`2f036eb`](https://github.com/Lymia/derive_setters/commit/2f036ebc7d7e29f61f82f65cb1a207f1cc18fa91))
    - Add prefix field to delegate ([`2bece62`](https://github.com/Lymia/derive_setters/commit/2bece62804e0e12ba0f1257caf75e85e957d7a31))
    - Add license texts ([`4ed9cd7`](https://github.com/Lymia/derive_setters/commit/4ed9cd7dba1dbb0db25d5eab12c7ff73faa2805e))
</details>

## v0.1.6 (2023-05-26)

* Updates dependency versions.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version to 0.1.6 ([`5238cc2`](https://github.com/Lymia/derive_setters/commit/5238cc21e40052097ccbf2cd9330f0af59445341))
    - Update derive_setters for syn 2.0 ([`b9cc558`](https://github.com/Lymia/derive_setters/commit/b9cc55852b53dd0cffe671a0b212d3d328f50e01))
    - Update README to point at travis-ci.com ([`5348035`](https://github.com/Lymia/derive_setters/commit/534803529093532c4c5a6a37bd7e6ebd27768cfb))
</details>

## v0.1.5 (2021-01-25)

* Fixes an bug where `derive_setters` accidentally used an internal type from `syn`. (Thanks @nn1ks)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release 0.1.5 ([`41f4527`](https://github.com/Lymia/derive_setters/commit/41f452772884f1a393d2fb3022c52bfe5813e317))
    - Merge pull request #3 from yuqio/proc-macro2-span ([`a6d5ad5`](https://github.com/Lymia/derive_setters/commit/a6d5ad58f05a6a16b5614e717285ca35331200cc))
    - Use `proc_macro2::Span` instead of `syn::export::Span` ([`a174da6`](https://github.com/Lymia/derive_setters/commit/a174da67a4cdeb9f52cf9b6206b512e2f5e25d7b))
</details>

## v0.1.4 (2020-03-29)

* Adds `#[setters(borrow_self)]` to allow referencing `self` through a mutable reference. (Thanks @ishehadeh)
* Makes `#[setters(strip_option)]` work better with code that uses the full path rather than an import. (Thanks @TheVova)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version. ([`b1e964a`](https://github.com/Lymia/derive_setters/commit/b1e964a9d98e0477f5210e7dd5c27e300e0d2c0d))
    - Document borrow_self ([`2193c03`](https://github.com/Lymia/derive_setters/commit/2193c03d532f19163bc2b40572388973f2a9d0a5))
    - Split tests slightly. ([`06749b5`](https://github.com/Lymia/derive_setters/commit/06749b5c3365cbac58f7bd3e83a3d20f54a79d7d))
    - Implement borrow_self on a per-field basis. ([`c730956`](https://github.com/Lymia/derive_setters/commit/c730956d73c6266eae3b1762f0ec308ac36a3acd))
    - Minor code style fixes. ([`fcec09d`](https://github.com/Lymia/derive_setters/commit/fcec09d691fdf26c879cb323c3ae98334aab2b4e))
    - Merge pull request #2 from TheVova/master ([`900be74`](https://github.com/Lymia/derive_setters/commit/900be7440fdc59f606afb74ef172373e9e0ec924))
    - Added tests for changed 'strip_option' variant. ([`6de1566`](https://github.com/Lymia/derive_setters/commit/6de15667db7c5ce4ffc1b11f9e8e15e90e38203f))
    - Make the 'strip_option' option work on full paths. ([`bdb8cfb`](https://github.com/Lymia/derive_setters/commit/bdb8cfb7cd10759d788f81d5b08cf28bbbb011d6))
    - Merge pull request #1 from IanS5/feature/borrow-self ([`68550c5`](https://github.com/Lymia/derive_setters/commit/68550c5e8aaf31e156c3b1dcd9c249db79573723))
    - Add option (borrow_self) to take a mutable reference to self in setters ([`25c71b1`](https://github.com/Lymia/derive_setters/commit/25c71b160bb2fa680a0a558dda115c66a3f76a94))
</details>

## v0.1.3 (2020-03-04)

* Add support for generating a delegate setters on other types.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version. ([`2221092`](https://github.com/Lymia/derive_setters/commit/222109277622a2384b4b2e34565210a7092d2c05))
    - Document generate_delegates. ([`8434f7d`](https://github.com/Lymia/derive_setters/commit/8434f7d6d9b431a0e1a6c44bde81bd333ebcc3e8))
    - Adds support for generating delegates on other types. ([`23ba7f1`](https://github.com/Lymia/derive_setters/commit/23ba7f133c28a8bd213a764000eaaf379fa972b6))
    - Fix example in readme. ([`ac7a291`](https://github.com/Lymia/derive_setters/commit/ac7a291b84dd8073c76e6664929635c149b724a1))
    - Minor fix to README. ([`a232706`](https://github.com/Lymia/derive_setters/commit/a232706e77b35a26fc05c10565af76922ca20100))
</details>

## v0.1.2 (2019-10-27)

* Adds `#[setters(bool)]` to allow shorthand functions for boolean fields.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version to 0.1.2 ([`2cefb37`](https://github.com/Lymia/derive_setters/commit/2cefb37005e67127e0163d25adb4c95169a87566))
    - Move documentation to the README, as docs.rs isn't cooperating. ([`042ff0e`](https://github.com/Lymia/derive_setters/commit/042ff0e6ad92ceccde10ab9d91f12a1390774092))
    - Implement `#[setters(bool)]` option. ([`6e40754`](https://github.com/Lymia/derive_setters/commit/6e407543cad3f25f81c8ab2bc405c029fb737d23))
</details>

## v0.1.1 (2019-10-27)

* Remove an internal mechanism that caused `derive_setters` to work inconsistently for some structs.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version to 0.1.1 ([`8e6eebf`](https://github.com/Lymia/derive_setters/commit/8e6eebf014e2d261a1db4674c4f2167d8f0dc6d0))
    - Remove relax_lt mechanism, it doesn't work in some cases. ([`e3a852e`](https://github.com/Lymia/derive_setters/commit/e3a852ebe0a79f37dd81f8e4fd53680723058e9d))
</details>

## v0.1.0 (2019-10-27)

* Initial release of `derive_setters`. 

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fix Travis build. ([`286d85b`](https://github.com/Lymia/derive_setters/commit/286d85bd2bb4925e9b3a3c636247009ef240dda9))
    - Make Travis start a build. ([`61f0a79`](https://github.com/Lymia/derive_setters/commit/61f0a79a69f96ad499b530b56cf72fb152c787f2))
    - Set up CI. ([`ed349aa`](https://github.com/Lymia/derive_setters/commit/ed349aad8507daec9683fa1a4f79a09ae878be4d))
    - Rename to derive_setters. ([`52778dc`](https://github.com/Lymia/derive_setters/commit/52778dc79bedb80c3369ece9b64991c2121bef07))
    - Add documentation, add prefix feature. ([`f91f5b8`](https://github.com/Lymia/derive_setters/commit/f91f5b80a012381a7685e87008c2d502e771840f))
    - Add tests. ([`2ab5585`](https://github.com/Lymia/derive_setters/commit/2ab55858d5b9e31d5ed4c0b57a6905079bf6d651))
    - Various minor bug fixes. ([`45ba8ff`](https://github.com/Lymia/derive_setters/commit/45ba8ff7af70cc37b909310d5f6a8ebd46bebd52))
    - Initial implementation. ([`8967379`](https://github.com/Lymia/derive_setters/commit/896737931f46ee6afe1c26ab752c68a6f79353a2))
</details>


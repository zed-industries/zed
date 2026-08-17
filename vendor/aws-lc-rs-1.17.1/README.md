# AWS Libcrypto for Rust (aws-lc-rs)

[![Crates.io](https://img.shields.io/crates/v/aws-lc-rs.svg)](https://crates.io/crates/aws-lc-rs)
[![GitHub](https://img.shields.io/badge/GitHub-aws%2Faws--lc--rs-blue)](https://github.com/aws/aws-lc-rs)

A [*ring*](https://github.com/briansmith/ring)-compatible crypto library using the cryptographic
operations provided by [*AWS-LC*](https://github.com/aws/aws-lc). It uses either the
auto-generated [*aws-lc-sys*](https://crates.io/crates/aws-lc-sys) or
[*aws-lc-fips-sys*](https://crates.io/crates/aws-lc-fips-sys)
Foreign Function Interface (FFI) crates found in this repository for invoking *AWS-LC*.

## Build

`aws-lc-rs` is available through [crates.io](https://crates.io/crates/aws-lc-rs). It can
be added to your project in the [standard way](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
using `Cargo.toml`:

```toml
[dependencies]
aws-lc-rs = "1"
```
Consuming projects will need a C/C++ compiler to build.

**Non-FIPS builds (default):**
* CMake is **never** required
* Bindgen is **never** required (pre-generated bindings are provided)
* Go is **never** required

**FIPS builds:** Require **CMake**, **Go**, and potentially **bindgen** depending on the target platform.

See our [User Guide](https://aws.github.io/aws-lc-rs/) for guidance on installing build requirements.

## Feature Flags

##### alloc (default)

Allows implementation to allocate values of arbitrary size. (The meaning of this feature differs
from the "alloc" feature of *ring*.) Currently, this is required by the `io::writer` module.

##### ring-io (default)

Enable feature to access the  `io`  module.

##### ring-sig-verify (default)

Enable feature to preserve compatibility with ring's `signature::VerificationAlgorithm::verify`
function. This adds a requirement on `untrusted = "0.7.1"`.

##### fips

Enable this feature to have aws-lc-rs use the [*aws-lc-fips-sys*](https://crates.io/crates/aws-lc-fips-sys)
crate for the cryptographic implementations. The aws-lc-fips-sys crate provides bindings to the
latest version of the AWS-LC-FIPS module that has completed FIPS validation testing by an
accredited lab and has been submitted to NIST for certification. This will continue to be the
case as we periodically submit new versions of the AWS-LC-FIPS module to NIST for certification.
Currently, aws-lc-fips-sys binds to
[AWS-LC-FIPS 3.0.x](https://github.com/aws/aws-lc/tree/fips-2024-09-27).

Consult with your local FIPS compliance team to determine the version of AWS-LC-FIPS module that you require. Consumers
needing to remain on a previous version of the AWS-LC-FIPS module should pin to specific versions of aws-lc-rs to avoid
automatically being upgraded to a newer module version.
(See [cargo's documentation](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
on how to specify dependency versions.)

| AWS-LC-FIPS module | aws-lc-rs |
|--------------------|-----------|
| 2.0.x              | \<1.12.0  |
| 3.0.x              | *latest*  |

Refer to the
[NIST Cryptographic Module Validation Program's Modules In Progress List](https://csrc.nist.gov/Projects/cryptographic-module-validation-program/modules-in-process/Modules-In-Process-List)
for the latest status of the static or dynamic AWS-LC Cryptographic Module. Please see the
[FIPS.md in the aws-lc repository](https://github.com/aws/aws-lc/blob/main/crypto/fipsmodule/FIPS.md)
for relevant security policies and information on supported operating environments.
We will also update our release notes and documentation to reflect any changes in FIPS certification status.

##### non-fips

Enable this feature to guarantee that the non-FIPS [*aws-lc-sys*](https://crates.io/crates/aws-lc-sys)
crate is used for cryptographic implementations. This feature is mutually exclusive with the `fips`
feature - enabling both will result in a compile-time error. Use this feature when you need a
compile-time guarantee that your build is using the non-FIPS cryptographic module.

##### asan

Performs an "address sanitizer" build. This can be used to help detect memory leaks. See the
["Address Sanitizer" section](https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html#addresssanitizer)
of the [Rust Unstable Book](https://doc.rust-lang.org/beta/unstable-book/).

##### bindgen

Causes `aws-lc-sys` or `aws-lc-fips-sys` to generates fresh bindings for AWS-LC instead of using
the pre-generated bindings. This feature requires `libclang` to be installed. See the
[requirements](https://rust-lang.github.io/rust-bindgen/requirements.html)
for [rust-bindgen](https://github.com/rust-lang/rust-bindgen)

##### prebuilt-nasm

Enables the use of crate provided prebuilt NASM objects under certain conditions. This only affects builds for
Windows x86-64 platforms. This feature is ignored if the "fips" feature is also enabled.

Use of prebuilt NASM objects is prevented if either of the following conditions are true:
* The NASM assembler is detected in the build environment
* `AWS_LC_SYS_PREBUILT_NASM` environment variable is set with a value of `0`

Be aware that [features are additive](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification);
by enabling this feature, it is enabled for all crates within the same build.

##### dev-tests-only

Enables the `rand::unsealed` module, which re-exports the normally sealed `SecureRandom` trait.
This allows consumers to provide their own implementations of `SecureRandom` (e.g., a
deterministic RNG) for testing purposes. When enabled, a `mut_fill` method is also available on
`SecureRandom`.

This feature is restricted to **dev/debug profile builds only** — attempting to use it in a
release build will result in a compile-time error.

It can be enabled in two ways:
* **Feature flag:** `cargo test --features dev-tests-only`
* **Environment variable:** `AWS_LC_RS_DEV_TESTS_ONLY=1 cargo test`

**⚠️ Warning:** This feature is intended **only** for development and testing. It must not be
used in production builds. The `rand::unsealed` module and `mut_fill` method are not part of the
stable public API and may change without notice.

##### legacy-des

Enables single DES and Triple DES as opt-in symmetric ciphers under the `cipher`
module, exposing `DES_FOR_LEGACY_USE_ONLY` (single DES),
`DES_EDE_FOR_LEGACY_USE_ONLY` (2-key Triple DES / DES-EDE) and
`DES_EDE3_FOR_LEGACY_USE_ONLY` (3-key Triple DES / DES-EDE3), together with the
supporting key-length and IV-length constants. Only CBC and ECB operating modes are
supported.

**⚠️ Warning:** Single DES and Triple DES are legacy algorithms. Single DES
provides only 56 bits of effective security and has been considered insecure for
decades. Triple DES has been disallowed for encryption by
[NIST SP 800-131A Rev. 2](https://csrc.nist.gov/publications/detail/sp/800-131a/rev-2/final):
2-key Triple DES after 2015, and 3-key Triple DES after 2023. This feature exists
solely to support interoperability. All exposed items are marked `#[deprecated]`
so that any use site produces a compiler warning. **Do not use single DES or Triple
DES in new designs.** If you only need confidentiality, prefer AES-GCM or another
AEAD from the `aead` module; if you specifically need a block cipher, prefer one
of the AES algorithms in `cipher`.

**Build-time impact:** The default `aws-lc-sys` bindings do not expose the low-level `DES_*`
symbols this feature relies on, so enabling `legacy-des` also enables `aws-lc-sys?/all-bindings`.
Concretely:

* On platforms with pre-generated per-target bindings, the build switches from the universal
  bindings to the per-target bindings (no extra tooling required).
* On platforms *without* pre-generated bindings for the selected target, `bindgen` is invoked at
  build time, which requires `libclang` to be installed in the build environment.

Because [features are additive](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification),
enabling `legacy-des` anywhere in your dependency graph applies the above to the entire build.

## Use of prebuilt NASM objects

Prebuilt NASM objects are **only** applicable to Windows x86-64 platforms. They are **never** used on any other platform (Linux, macOS, etc.).

For Windows x86 and x86-64, NASM is required for assembly code compilation. On these platforms,
we recommend that you install [the NASM assembler](https://www.nasm.us/). **If NASM is
detected in the build environment, it is always used** to compile the assembly files. Prebuilt NASM objects are only used as a fallback.

If a NASM assembler is not available, and the "fips" feature is not enabled, then the build fails unless one of the following conditions are true:

* You are building for `x86-64` and either:
   * The `AWS_LC_SYS_PREBUILT_NASM` environment variable is found and has a value of "1"; OR
   * `AWS_LC_SYS_PREBUILT_NASM` is *not found* in the environment AND the "prebuilt-nasm" feature has been enabled.

If the above cases apply, then the crate provided prebuilt NASM objects will be used for the build. To prevent usage of prebuilt NASM
objects, install NASM in the build environment and/or set the variable `AWS_LC_SYS_PREBUILT_NASM` to `0` in the build environment to prevent their use.

### About prebuilt NASM objects

Prebuilt NASM objects are generated using automation similar to the crate provided pregenerated bindings. See the repository's
[GitHub workflow configuration](https://github.com/aws/aws-lc-rs/blob/main/.github/workflows/sys-bindings-generator.yml) for more information.
The prebuilt NASM objects are checked into the repository
and are [available for inspection](https://github.com/aws/aws-lc-rs/tree/main/aws-lc-sys/builder/prebuilt-nasm).
For each PR submitted,
[CI verifies](https://github.com/aws/aws-lc-rs/blob/main/.github/workflows/tests.yml)
that the NASM objects newly built from source match the NASM objects currently in the repository.

## *ring*-compatibility

Although this library attempts to be fully compatible with *ring* (v0.16.x), there are a few places where our
behavior is observably different.

* Our implementation requires the `std` library. We currently do not support a
  [`#![no_std]`](https://docs.rust-embedded.org/book/intro/no-std.html) build.
* `aws-lc-rs` supports the platforms supported by `aws-lc-sys` and AWS-LC. See the
  [Platform Support](https://aws.github.io/aws-lc-rs/platform_support.html) page in our User Guide.
* `Ed25519KeyPair::from_pkcs8` and `Ed25519KeyPair::from_pkcs8_maybe_unchecked` both support
  parsing of v1 or v2 PKCS#8 documents. If a v2 encoded key is provided to either function,
  public key component, if present, will be verified to match the one derived from the encoded
  private key.

## Post-Quantum Cryptography

Details on the post-quantum algorithms supported by aws-lc-rs can be found at
[PQREADME](https://github.com/aws/aws-lc/tree/main/crypto/fipsmodule/PQREADME.md).

## Motivation

Rust developers increasingly need to deploy applications that meet US and Canadian government
cryptographic requirements. We evaluated how to deliver FIPS validated cryptography in idiomatic
and performant Rust, built around our AWS-LC offering. We found that the popular ring (v0.16)
library fulfilled much of the cryptographic needs in the Rust community, but it did not meet the
needs of developers with FIPS requirements. Our intention is to contribute a drop-in replacement
for ring that provides FIPS support and is compatible with the ring API. Rust developers with
prescribed cryptographic requirements can seamlessly integrate aws-lc-rs into their applications
and deploy them into AWS Regions.

### Contributor Quickstart for Amazon Linux 2023

For those who would like to contribute to our project or build it directly from our repository,
a few more packages may be needed. The listing below shows the steps needed for you to begin
building and testing our project locally.

```shell
# Install dependencies needed for build and testing
sudo yum install -y cmake3 clang git clang-libs golang openssl-devel perl-FindBin

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Clone and initialize a local repository
git clone https://github.com/aws/aws-lc-rs.git
cd aws-lc-rs
git submodule update --init --recursive

# Build and test the project
cargo test

```

## Questions, Feedback and Contributing

* [Submit an non-security Bug/Issue/Request](https://github.com/aws/aws-lc-rs/issues/new/choose)
* [API documentation](https://docs.rs/aws-lc-rs/)
* [Fork our repo](https://github.com/aws/aws-lc-rs/fork)

We use [GitHub Issues](https://github.com/aws/aws-lc-rs/issues/new/choose) for managing feature requests, bug
reports, or questions about aws-lc-rs API usage.

Otherwise, if you think you might have found a security impacting issue, please instead
follow our *Security Notification Process* below.

## Security Notification Process

If you discover a potential security issue in *AWS-LC* or *aws-lc-rs*, we ask that you notify AWS
Security via our
[vulnerability reporting page](https://aws.amazon.com/security/vulnerability-reporting/).
Please do **not** create a public GitHub issue.

If you package or distribute *aws-lc-rs*, or use *aws-lc-rs* as part of a large multi-user service,
you may be eligible for pre-notification of future *aws-lc-rs* releases.
Please contact aws-lc-pre-notifications@amazon.com.

## License

This library is licensed under the Apache-2.0 or the ISC License.

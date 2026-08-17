//! # rustls-platform-verifier-android
//!
//! This crate is an implementation detail of the actual [rustls-platform-verifier](https://github.com/rustls/rustls-platform-verifier) crate.
//!
//! It contains no Rust code and is solely intended as a convenient delivery mechanism for the supporting Kotlin code that the main crate
//! requires to perform TLS certificate validation using Android's APIs.
//!
//! Other crates should not directly depend on this crate in any way, as nothing about it is considered stable and it is probably useless elsewhere.
//!
//! ## Details
//!
//! Note: Everything in this section is subject to change at any time. Semver may not be followed.
//!
//! ### Why?
//!
//! It was the best middle ground between several tradeoffs. The important ones, in priority order, are:
//! - Automatically keeping component versions in sync
//! - Allowing well-tested and well-known `cargo` dependency management patterns to apply everywhere
//! - Providing a smooth developer experience as an Android consumer of `rustls-platform-verifier`
//!
//! Firstly, what alternatives are available for distributing the component? The other two known are source distribution in some form (here, it will be through crates.io)
//! and Maven Central. Starting with the first, its become infeasible due to toolchain syncing requirements. If the Android component is
//! built as part of the host app's Gradle build, then it becomes subject to any Gradle or Android Gradle Plugin incompatibilities/requirements. In practice this means
//! the AGP version between this project and the main application have to match all the time. Sometimes this works, but it becomes challenging/unfeasible
//! during yearly toolchain/SDK upgrades and is not maintainable long term. Note that this is the _only_ option in this section which retains compatibility
//! with Cargo's Git dependency patching.
//!
//! Next, Maven Central. This is considered the standard way of distributing public Android dependencies. There are two downsides to this
//! approach: version synchronization and publishing overhead. Version syncing is the hardest part: There's not a good way to know what version
//! a crate is that doesn't hurt the Cargo part of the build or damage functionality. So instead of making assumptions at runtime, we would need to do
//! clunky and manual version counting with an extra error case. Less importantly, the admin overhead of Maven Central is non-zero so its good to avoid
//! if possible for such a small need.
//!
//! It is also worth calling out a third set of much worse options: requiring users to manually download and install the Android component
//! on each update, which magnifies the version syncing problem with lots of user overhead and then deleting the component outright. A rewrite
//! could be done with raw JNI calls, but this would easily be 3x the size of the existing implementation and require huge amounts of `unsafe`
//! to review then audit.
//!
//! ### The solution
//!
//! The final design was built to avoid the pitfalls the previous two options mentioned. To build it, we rely on CI and packaging scripts to build
//! the Android component into a prebuilt AAR file before creating a release. Next, a [on-disk Maven repository](https://maven.apache.org/repositories/local.html)
//! is hosted inside of this repository. Only the unchanging file structure of it is kept checked-in, to avoid churn. The remaining parts are filled in
//! during the packaging/release process, before being included in `cargo package` via an `include` Cargo.toml directive. Finally, once the repository has had
//! its artifacts added the crate containing the Maven repository is published to crates.io. Then, the main crate ensures it's downloaded when an Android target
//! is compiled via a platform-specific dependency.
//!
//! On [the Gradle side](https://github.com/rustls/rustls-platform-verifier/tree/main#gradle-setup), we include a very small snippet of code for users to include in their `settings.gradle` file
//! to dynamically locate the local maven repository on disk automatically based off Cargo's current version of it. The script is configuration cache friendly and
//! doesn't impact performance either. When the script is run, it finds the cargo-cached download of the crate and tells Gradle it can find the Android component there
//! when it gets sourced into the hosting application's build tree.
//!
//! Assuming a properly configured Gradle project, the slow (~500ms) script should only run once per Gradle sync while the `android-release-support` crate
//! remains untouched. This is due to the configuration cache previously mentioned and is what ensures performance on-par with a "normal" Maven repository.
//! Upon any version updates (semver, Git refs, etc), the change will be detected as-intended by Gradle, break the cache, and the project will update the dependency reference to the new AAR file.
//!
//! ### Precompiled artifacts?
//!
//! For some, the notion of shipping something pre-compiled with an existing source distribution might seem incorrect, or insecure. However in this specific case,
//! putting aside the fact shipping Kotlin code doesn't work (see above), there are many reasons this isn't the case:
//! - Shipping pre-compiled artifacts is normal in the Java ecosystem. Maven Central and other package repositories do the same thing and serve `.jar` downloads.
//! - Those not using Android will never download the pre-compiled AAR file.
//! - The artifacts are incredibly easy to reproduce given an identical compilation toolchain.
//! - The artifacts are not native executables, or raw `.jar` files, so they can't be accidentally executed on a host system.
//!
//! ## Summary
//!
//! In summary, the selected distribution method avoids most of the previous pitfalls while still balancing a good experience for `cargo` and Gradle users. Some of its
//! positive properties include:
//! - Full compatibility with Cargo's dependency management, including Git patching[^1]
//! - No version checking or synchronization required
//! - Painless and harmless to integrate into an Android app's build system
//! - Low maintenance for the main crate maintainers'
//!
//! [^1]: The Git reference being used must have the local maven repository built and checked-in first.

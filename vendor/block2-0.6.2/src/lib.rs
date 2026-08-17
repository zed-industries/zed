//! # Apple's C language extension of blocks
//!
//! C Blocks are functions which capture their environments, i.e. the
//! C-equivalent of Rust's [`Fn`] closures. As they were originally developed
//! by Apple, they're often used in Objective-C code. This crate provides
//! capabilities to create, manage and invoke these blocks, in an ergonomic,
//! "Rust-centric" fashion.
//!
//! At a high level, this crate contains four types, each representing
//! different kinds of blocks, and different kinds of ownership.
//!
//! | `block2` type                            | Equivalent Rust type  |
//! | ---------------------------------------- | --------------------- |
//! | `&Block<dyn Fn() + 'a>`                  | `&dyn Fn() + 'a`      |
//! | `RcBlock<dyn Fn() + 'a>`                 | `Arc<dyn Fn() + 'a>`  |
//! | `StackBlock<'a, (), (), impl Fn() + 'a>` | `impl Fn() + 'a`      |
//! | `GlobalBlock<dyn Fn()>`                  | [`fn` item]           |
//!
//! For more information on the specifics of the block implementation, see the
//! [C language specification][lang] and the [ABI specification][ABI].
//!
//! [lang]: https://clang.llvm.org/docs/BlockLanguageSpec.html
//! [ABI]: http://clang.llvm.org/docs/Block-ABI-Apple.html
//! [`fn` item]: https://doc.rust-lang.org/reference/types/function-item.html
//!
//!
//! ## Using blocks
//!
//! You can create a new block from a closure using [`RcBlock::new`]. This can
//! then be used to call functions or Objective-C methods that takes a block:
//!
//! ```
//! use block2::RcBlock;
//! #
//! # struct ExampleObject;
//! #
//! # impl ExampleObject {
//! #     fn someMethod(&self, block: &block2::Block<dyn Fn(i32, i32) -> i32>) {
//! #         assert_eq!(block.call((5, 8)), 18);
//! #     }
//! # }
//! #
//! # let obj = ExampleObject;
//!
//! let val = 5;
//! let block = RcBlock::new(move |a, b| a + b + val);
//! obj.someMethod(&block);
//! ```
//!
//!
//! ## My block isn't being run?
//!
//! Most of the time, blocks are used to do asynchronous work; but just like
//! futures in Rust don't do anything unless polled, a lot of Apple APIs won't
//! call your block unless a [run loop][run_loop] is active, see that link for
//! more information on how to do so.
//!
//! [run_loop]: objc2::topics::run_loop
//!
//!
//! ## Lifetimes
//!
//! When dealing with blocks, there can be quite a few lifetimes to keep in
//! mind.
//!
//! The most important one is the lifetime of the block's data, i.e. the
//! lifetime of the data in the closure contained in the block. This lifetime
//! can be specified as `'f` in `&Block<dyn Fn() + 'f>`.
//!
//! Note that `&Block<dyn Fn()>`, without any lifetime specifier, can be a bit
//! confusing, as the default depends on where it is typed. In function/method
//! signatures, it defaults to `'static`, but as the type of e.g. a `let`
//! binding, the lifetime may be inferred to be something smaller, see [the
//! reference][ref-dyn-lifetime] for details. If in doubt, either add a
//! `+ 'static` or `+ '_` to force an escaping or non-escaping block.
//!
//! Another lifetime is the lifetime of the currently held pointer, i.e. `'b`
//! in `&'b Block<dyn Fn()>`. This lifetime can be safely extended using
//! [`Block::copy`], so should prove to be little trouble (of course the
//! lifetime still can't be extended past the lifetime of the captured data
//! above).
//!
//! Finally, the block's parameter and return types can also contain
//! lifetimes, as `'a` and `'r` in `&Block<dyn Fn(&'a i32) -> &'r u32>`.
//! Unfortunately, these lifetimes are quite problematic and unsupported at
//! the moment, due to Rust trait limitations regarding higher-ranked trait
//! bounds. If you run into problems with this in a block that takes or
//! returns a reference, consider using the ABI-compatible `NonNull<T>`, or
//! transmute to a `'static` lifetime.
//!
//! [ref-dyn-lifetime]: https://doc.rust-lang.org/reference/lifetime-elision.html#default-trait-object-lifetimes
//!
//!
//! ## Thread safety
//!
//! Thread-safe blocks are not yet representable in `block2`, and as such any
//! function that requires a thread-safe block must be marked `unsafe`.
//!
//!
//! ## Mutability
//!
//! Blocks are generally assumed to be shareable, and as such can only very
//! rarely be made mutable.
//!
//! You will likely have to use interior mutability helpers like [`RefCell`]
//! or [`Cell`] instead, see below.
//!
//! [`RefCell`]: core::cell::RefCell
//! [`Cell`]: core::cell::Cell
//!
//!
//! ### Transforming [`FnMut`] to a block
//!
//! Mutable closures differs from immutable ones in part in that they need to
//! avoid re-entrancy.
//!
//! The below example transforms [`FnMut`] to [`Fn`] using a [`RefCell`]. We
//! do not include this function as part of the public API of `block2`, as the
//! specifics are very dependent on your use-case, and can be optimized with
//! e.g. a [`Cell`] if your closure is [`Copy`] or if you do not care about
//! unwind safety, or with [`UnsafeCell`] if you are able to unsafely
//! guarantee the absence of re-entrancy.
//!
//! [`UnsafeCell`]: core::cell::UnsafeCell
//!
//! ```
//! use std::cell::RefCell;
//! use block2::RcBlock;
//!
//! fn fnmut_to_fn(closure: impl FnMut()) -> impl Fn() {
//!     let cell = RefCell::new(closure);
//!
//!     move || {
//!         let mut closure = cell.try_borrow_mut().expect("re-entrant call");
//!         (closure)()
//!     }
//! }
//!
//! let mut x = 0;
//! let b = RcBlock::new(fnmut_to_fn(|| {
//!     x += 1;
//! }));
//! b.call(());
//! b.call(());
//! drop(b);
//! assert_eq!(x, 2);
//! ```
//!
//!
//! ### Transforming [`FnOnce`] to a block
//!
//! [`FnOnce`] is similar to [`FnMut`] in that we must protect against
//! re-entrancy, with the addition that it can also only be called once.
//!
//! Ensuring that it can be called once can be done by taking the closure
//! out of an [`Option`] as shown in the example below. We can use [`Cell`]
//! instead of [`RefCell`] here, since we never need to put the closure "back"
//! for later use (like we need to do with `FnMut` above).
//!
//! In certain cases you may be able to do micro-optimizations, namely to use
//! a [`ManuallyDrop`], if you wanted to optimize with the assumption that the
//! block is always called, or [`unwrap_unchecked`] if you wanted to optimize
//! with the assumption that it is only called once.
//!
//! [`Cell`]: core::cell::Cell
//! [`ManuallyDrop`]: core::mem::ManuallyDrop
//! [`unwrap_unchecked`]: core::option::Option::unwrap_unchecked
//!
//! ```
//! use std::cell::Cell;
//! use block2::RcBlock;
//!
//! fn fnonce_to_fn(closure: impl FnOnce()) -> impl Fn() {
//!     let cell = Cell::new(Some(closure));
//!     move || {
//!         let closure = cell.take().expect("called twice");
//!         closure()
//!     }
//! }
//!
//! let v = vec![1, 2, 3];
//! let b = RcBlock::new(fnonce_to_fn(move || {
//!     drop(v);
//! }));
//! b.call(());
//! ```
//!
//!
//! ## External functions using blocks
//!
//! To declare external functions or methods that takes blocks, use
//! `&Block<dyn Fn(Params) -> R>` or `Option<&Block<dyn Fn(Args) -> R>>`,
//! where `Params` is the parameter types, and `R` is the return type.
//!
//! For this example, consider the function `check_addition` which takes a
//! single parameter, namely a block that adds two integers, and then checks
//! that the addition is correct.
//!
//! Such a function could be written in C like in the following.
//!
//! ```objc
//! #include <cassert>
//! #include <stdint.h>
//! #include <Block.h>
//!
//! void check_addition(int32_t (^block)(int32_t, int32_t)) {
//!     assert(block(5, 8) == 13);
//! }
//! ```
//!
//! An `extern "C" { ... }` declaration for that function would then be:
//!
//! ```
//! use block2::Block;
//!
//! extern "C" {
//!     fn check_addition(block: &Block<dyn Fn(i32, i32) -> i32>);
//! }
//! ```
//!
//! This can similarly be done for Objcective-C methods declared with
//! [`objc2::extern_methods!`] (though most of the time, the [framework
//! crates][framework-crates] will take care of that for you).
//!
//! ```
//! use block2::Block;
//! use objc2::extern_methods;
//! #
//! # use objc2::ClassType;
//! # objc2::extern_class!(
//! #     #[unsafe(super(objc2::runtime::NSObject))]
//! #     #[name = "NSObject"]
//! #     struct MyClass;
//! # );
//!
//! impl MyClass {
//!     extern_methods!(
//!         #[unsafe(method(checkAddition:))]
//!         pub fn checkAddition(&self, block: &Block<dyn Fn(i32, i32) -> i32>);
//!     );
//! }
//! ```
//!
//! If the function/method allows passing `NULL` blocks, the type should be
//! `Option<&Block<dyn Fn(i32, i32) -> i32>>` instead.
//!
//! [framework-crates]: objc2::topics::about_generated
//!
//!
//! ## Invoking blocks
//!
//! We can also define the external function in Rust, and expose it to
//! Objective-C. To do this, we can use [`Block::call`] to invoke the block
//! inside the function.
//!
//! ```
//! use block2::Block;
//!
//! #[no_mangle]
//! extern "C" fn check_addition(block: &Block<dyn Fn(i32, i32) -> i32>) {
//!     assert_eq!(block.call((5, 8)), 13);
//! }
//! ```
//!
//! Note the extra parentheses in the `call` method, since the arguments must
//! be passed as a tuple.
//!
//!
//! ## Specifying a runtime
//!
//! Different runtime implementations exist and act in slightly different ways
//! (and have several different helper functions), the most important aspect
//! being that the libraries are named differently, so we must take that into
//! account when linking.
//!
//! You can choose the desired runtime by using the relevant cargo feature
//! flags, see the following sections.
//!
//!
//! ### Apple's [`libclosure`](https://github.com/apple-oss-distributions/libclosure)
//!
//! This is the most common and most sophisticated runtime, and it has quite a
//! lot more features than the specification mandates.
//!
//! The minimum required operating system versions are as follows (though in
//! practice Rust itself requires higher versions than this):
//!
//! - macOS: `10.6`
//! - iOS/iPadOS: `3.2`
//! - tvOS: `1.0`
//! - watchOS: `1.0`
//!
//! **This is used by default**, so you do not need to specify a runtime if
//! you're using this crate on of these platforms.
//!
//!
//! ### LLVM `compiler-rt`'s [`libBlocksRuntime`](https://github.com/llvm/llvm-project/tree/release/13.x/compiler-rt/lib/BlocksRuntime)
//!
//! - Feature flag: `compiler-rt`.
//!
//! This is a copy of Apple's older (around macOS 10.6) runtime, and is now
//! used in [Swift's `libdispatch`] and [Swift's Foundation] as well.
//!
//! The runtime and associated headers can be installed on many Linux systems
//! with the `libblocksruntime-dev` package.
//!
//! Using this runtime probably won't work together with `objc2` crate.
//!
//! [Swift's `libdispatch`]: https://github.com/apple/swift-corelibs-libdispatch/tree/swift-5.5.1-RELEASE/src/BlocksRuntime
//! [Swift's Foundation]: https://github.com/apple/swift-corelibs-foundation/tree/swift-5.5.1-RELEASE/Sources/BlocksRuntime
//!
//!
//! ### GNUStep's [`libobjc2`](https://github.com/gnustep/libobjc2)
//!
//! - Feature flag: `gnustep-1-7`, `gnustep-1-8`, `gnustep-1-9`, `gnustep-2-0`
//!   and `gnustep-2-1` depending on the version you're using.
//!
//! GNUStep is a bit odd, because it bundles blocks support into its
//! Objective-C runtime. This means we have to link to `libobjc`, and this is
//! done by depending on the `objc2` crate. A bit unorthodox, yes, but it
//! works.
//!
//! Sources:
//!
//! - [`Block.h`](https://github.com/gnustep/libobjc2/blob/v2.1/objc/blocks_runtime.h)
//! - [`Block_private.h`](https://github.com/gnustep/libobjc2/blob/v2.1/objc/blocks_private.h)
//!
//!
//! ### Microsoft's [`WinObjC`](https://github.com/microsoft/WinObjC)
//!
//! - Feature flag: `unstable-winobjc`.
//!
//! **Unstable: Hasn't been tested on Windows yet!**
//!
//! [A fork](https://github.com/microsoft/libobjc2) based on GNUStep's `libobjc2`
//! version 1.8.
//!
//!
//! ### [`ObjFW`](https://github.com/ObjFW/ObjFW)
//!
//! - Feature flag: `unstable-objfw`.
//!
//! **Unstable: Doesn't work yet!**
//!
//!
//! ## C Compiler configuration
//!
//! To our knowledge, only Clang supports blocks. To compile C or Objective-C
//! sources using these features, you should set [the `-fblocks` flag][flag].
//!
//! [flag]: https://clang.llvm.org/docs/ClangCommandLineReference.html#cmdoption-clang-fblocks

#![no_std]
#![allow(rustdoc::broken_intra_doc_links)] // FIXME link to objc2::topics
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/block2/0.6.2")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(feature = "unstable-coerce-pointee", feature(derive_coerce_pointee))]

#[cfg(not(feature = "alloc"))]
compile_error!("The `alloc` feature currently must be enabled.");

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(all(
    not(docsrs),
    not(any(
        target_vendor = "apple",
        feature = "compiler-rt",
        feature = "gnustep-1-7",
        feature = "unstable-objfw",
    ))
))]
compile_error!("`block2` only works on Apple platforms. Pass `--target aarch64-apple-darwin` or similar to compile for macOS.\n(If you're absolutely certain that you want to use `block2` on Linux/Windows, you can specify that with the `gnustep-x-y`/`compiler-rt` Cargo features instead).");

#[cfg(any(
    all(feature = "compiler-rt", feature = "gnustep-1-7"),
    all(feature = "gnustep-1-7", feature = "unstable-objfw"),
    all(feature = "compiler-rt", feature = "unstable-objfw"),
))]
compile_error!("Only one runtime may be selected");

#[cfg(feature = "unstable-objfw")]
compile_error!("ObjFW is not yet supported");

// Link to `libclosure` (internally called `libsystem_blocks.dylib`), which is
// exported by `libSystem.dylib`.
//
// Note: Don't get confused by the presence of `System.framework`, it is a
// deprecated wrapper over the dynamic library, so we'd rather use the latter.
//
// Alternative: Only link to `libsystem_blocks.dylib`:
// println!("cargo:rustc-link-search=native=/usr/lib/system");
// println!("cargo:rustc-link-lib=dylib=system_blocks");
#[cfg_attr(
    all(
        target_vendor = "apple",
        not(feature = "compiler-rt"),
        not(feature = "gnustep-1-7"),
        not(feature = "unstable-objfw"),
        not(feature = "std"), // `libSystem` is already linked by `libstd`.
    ),
    link(name = "System", kind = "dylib")
)]
// Link to `libBlocksRuntime`.
#[cfg_attr(feature = "compiler-rt", link(name = "BlocksRuntime", kind = "dylib"))]
// Link to `libobjfw`, which provides the blocks implementation.
#[cfg_attr(feature = "unstable-objfw", link(name = "objfw", kind = "dylib"))]
extern "C" {}

// Don't link to anything on GNUStep; objc2 already does that for us!
//
// We do want to ensure linkage actually happens, though.
#[cfg(feature = "gnustep-1-7")]
extern crate objc2 as _;

mod abi;
mod block;
mod debug;
mod encoding;
pub mod ffi;
mod global;
mod rc_block;
mod stack;
mod traits;

pub use self::block::Block;
pub use self::global::GlobalBlock;
pub use self::rc_block::RcBlock;
pub use self::stack::StackBlock;
pub use self::traits::{BlockFn, IntoBlock, ManualBlockEncoding};

/// Deprecated alias for a `'static` `StackBlock`.
#[deprecated = "renamed to `StackBlock`"]
pub type ConcreteBlock<A, R, Closure> = StackBlock<'static, A, R, Closure>;

// Note: We could use `_Block_object_assign` and `_Block_object_dispose` to
// implement a `ByRef<T>` wrapper, which would behave like `__block` marked
// variables and have roughly the same memory management scheme as blocks.
//
// But I've yet to see the value in such a wrapper in Rust code compared to
// just using `Box`, `Rc` or `Arc`, and since `__block` variables are
// basically never exposed as part of a (public) function's API, we won't
// implement such a thing yet.

/// Helper type to allow changing [`Block`] in the future without affecting
/// framework crates.
///
/// Tracked in [#572](https://github.com/madsmtm/objc2/issues/572).
pub type DynBlock<F> = crate::Block<F>;

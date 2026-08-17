Recent Changes (arrayvec)
=========================

## 0.7.6

- Fix no-std build [#274](https://github.com/bluss/arrayvec/pull/274)

## 0.7.5

- Add `as_ptr` and `as_mut_ptr` to `ArrayString` [@YuhanLiin](https://github.com/YuhanLiin) [#260](https://github.com/bluss/arrayvec/pull/260)
- Add borsh serialization support by @honzasp and @Fuuzetsu [#259](https://github.com/bluss/arrayvec/pull/259)
- Move length field before before data in ArrayVec and ArrayString by @JakkuSakura [#255](https://github.com/bluss/arrayvec/pull/255)
- Fix miri error for ZST case in extend by @bluss
- implement AsRef<Path> for ArrayString by [@Zoybean](https://github.com/Zoybean) [#218](https://github.com/bluss/arrayvec/pull/218)
- Fix typos in changelog by [@striezel](https://github.com/striezel) [#241](https://github.com/bluss/arrayvec/pull/241)
- Add `as_slice`, `as_mut_slice` methods to `IntoIter` by [@clarfonthey](https://github.com/clarfonthey) [#224](https://github.com/bluss/arrayvec/pull/224)


## 0.7.4

- Add feature zeroize to support the `Zeroize` trait by @elichai

## 0.7.3

- Use track_caller on multiple methods like push and similar, for capacity
  overflows by @kornelski
- impl BorrowMut for ArrayString by @msrd0
- Fix stacked borrows violations by @clubby789
- Update Miri CI by @RalfJung

## 0.7.2

- Add `.as_mut_str()` to `ArrayString` by @clarfonthey
- Add `remaining_capacity` to `ArrayString` by @bhgomes
- Add `zero_filled` constructor by @c410-f3r
- Optimize `retain` by @TennyZhuang and @niklasf
- Make the following methods `const` by @bhgomes:
  - len
  - is_empty
  - capacity
  - is_full
  - remaining_capacity
  - CapacityError::new

## 0.7.1

- Add new ArrayVec methods `.take()` and `.into_inner_unchecked()` by @conradludgate
- `clone_from` now uses `truncate` when needed by @a1phyr

## 0.7.0

- `fn new_const` is now the way to const-construct arrayvec and arraystring,
  and `fn new` has been reverted to a regular "non-const" function.
  This works around performance issue #182, where the const fn version did not
  optimize well. Change by @bluss with thanks to @rodrimati1992 and @niklasf
  for analyzing the problem.

- The deprecated feature flag `unstable-const-fn` was removed, since it's not needed

- Optimize `.retain()` by using the same algorithm as in std, change by @niklasf,
  issue #174. Original optimization in Rust std by @oxalica in rust-lang/rust/pull/81126

## 0.6.1

- The ``ArrayVec::new`` and ``ArrayString::new`` constructors are properly
  const fns on stable and the feature flag ``unstable-const-fn`` is now deprecated.
  by @rodrimati1992

- Small fix to the capacity check macro by @Xaeroxe
- Typo fix in documentation by @cuviper
- Small code cleanup by @bluss

## 0.6.0

- The **const generics** release 🎉. Arrayvec finally implements what it
  wanted to implement, since its first version: a vector backed by an array,
  with generic parameters for the arbitrary element type *and* backing array
  capacity.

  The New type syntax is `ArrayVec<T, CAP>` where `CAP` is the arrayvec capacity.
  For arraystring the syntax is `ArrayString<CAP>`.

  Length is stored internally as u32; this limits the maximum capacity. The size
  of the `ArrayVec` or `ArrayString` structs for the same capacity may grow
  slightly compared with the previous version (depending on padding requirements
  for the element type). Change by @bluss.

- Arrayvec's `.extend()` and `FromIterator`/`.collect()` to arrayvec now
  **panic** if the capacity of the arrayvec is exceeded. Change by @bluss.

- Arraystring now implements `TryFrom<&str>` and `TryFrom<fmt::Arguments>` by
  @c410-f3r

- Minimum supported rust version is Rust 1.51

## 0.5.2

- Add `is_empty` methods for ArrayVec and ArrayString by @nicbn
- Implement `TryFrom<Slice>` for ArrayVec by @paulkernfeld
- Add `unstable-const-fn` to make `new` methods const by @m-ou-se
- Run miri in CI and a few related fixes by @RalfJung
- Fix outdated comment by @Phlosioneer
- Move changelog to a separate file by @Luro02
- Remove deprecated `Error::description` by @AnderEnder
- Use pointer method `add` by @hbina

## 0.5.1

- Add `as_ptr`, `as_mut_ptr` accessors directly on the `ArrayVec` by @tbu-
  (matches the same addition to `Vec` which happened in Rust 1.37).
- Add method `ArrayString::len` (now available directly, not just through deref to str).
- Use raw pointers instead of `&mut [u8]` for encoding chars into `ArrayString`
  (uninit best practice fix).
- Use raw pointers instead of `get_unchecked_mut` where the target may be
  uninitialized everywhere relevant in the ArrayVec implementation
  (uninit best practice fix).
- Changed inline hints on many methods, mainly removing inline hints
- `ArrayVec::dispose` is now deprecated (it has no purpose anymore)

## 0.4.12

- Use raw pointers instead of `get_unchecked_mut` where the target may be
  uninitialized everywhere relevant in the ArrayVec implementation.

## 0.5.0

- Use `MaybeUninit` (now unconditionally) in the implementation of
  `ArrayVec`
- Use `MaybeUninit` (now unconditionally) in the implementation of
  `ArrayString`
- The crate feature for serde serialization is now named `serde`.
- Updated the `Array` trait interface, and it is now easier to use for
  users outside the crate.
- Add `FromStr` impl for `ArrayString` by @despawnerer
- Add method `try_extend_from_slice` to `ArrayVec`, which is always
  efficient by @Thomasdezeeuw.
- Add method `remaining_capacity` by @Thomasdezeeuw
- Improve performance of the `extend` method.
- The index type of zero capacity vectors is now itself zero size, by
  @clarfon
- Use `drop_in_place` for truncate and clear methods. This affects drop order
  and resume from panic during drop.
- Use Rust 2018 edition for the implementation
- Require Rust 1.36 or later, for the unconditional `MaybeUninit`
  improvements.

## Older releases

- 0.4.11

  - In Rust 1.36 or later, use newly stable `MaybeUninit`. This extends the
    soundness work introduced in 0.4.9, we are finally able to use this in
    stable. We use feature detection (build script) to enable this at build
    time.

- 0.4.10

  - Use `repr(C)` in the `union` version that was introduced in 0.4.9, to
    allay some soundness concerns.

- 0.4.9

  - Use `union` in the implementation on when this is detected to be supported
    (nightly only for now). This is a better solution for treating uninitialized
    regions correctly, and we'll use it in stable Rust as soon as we are able.
    When this is enabled, the `ArrayVec` has no space overhead in its memory
    layout, although the size of the vec should not be relied upon. (See [#114](https://github.com/bluss/arrayvec/pull/114))
  - `ArrayString` updated to not use uninitialized memory, it instead zeros its
    backing array. This will be refined in the next version, since we
    need to make changes to the user visible API.
  - The `use_union` feature now does nothing (like its documentation foretold).


- 0.4.8

  - Implement Clone and Debug for `IntoIter` by @clarcharr
  - Add more array sizes under crate features. These cover all in the range
    up to 128 and 129 to 255 respectively (we have a few of those by default):

    - `array-size-33-128`
    - `array-size-129-255`

- 0.4.7

  - Fix future compat warning about raw pointer casts
  - Use `drop_in_place` when dropping the arrayvec by-value iterator
  - Decrease minimum Rust version (see docs) by @jeehoonkang

- 0.3.25

  - Fix future compat warning about raw pointer casts

- 0.4.6

  - Fix compilation on 16-bit targets. This means, the 65536 array size is not
    included on these targets.

- 0.3.24

  - Fix compilation on 16-bit targets. This means, the 65536 array size is not
    included on these targets.
  - Fix license files so that they are both included (was fixed in 0.4 before)

- 0.4.5

  - Add methods to `ArrayString` by @DenialAdams:

    - `.pop() -> Option<char>`
    - `.truncate(new_len)`
    - `.remove(index) -> char`

  - Remove dependency on crate odds
  - Document debug assertions in unsafe methods better

- 0.4.4

  - Add method `ArrayVec::truncate()` by @niklasf

- 0.4.3

  - Improve performance for `ArrayVec::extend` with a lower level
    implementation (#74)
  - Small cleanup in dependencies (use no std for crates where we don't need more)

- 0.4.2

  - Add constructor method `new` to `CapacityError`.

- 0.4.1

  - Add `Default` impl to `ArrayString` by @tbu-

- 0.4.0

  - Reformed signatures and error handling by @bluss and @tbu-:

    - `ArrayVec`'s `push, insert, remove, swap_remove` now match `Vec`'s
      corresponding signature and panic on capacity errors where applicable.
    - Add fallible methods `try_push, insert` and checked methods
      `pop_at, swap_pop`.
    - Similar changes to `ArrayString`'s push methods.

  - Use a local version of the `RangeArgument` trait
  - Add array sizes 50, 150, 200 by @daboross
  - Support serde 1.0 by @daboross
  - New method `.push_unchecked()` by @niklasf
  - `ArrayString` implements `PartialOrd, Ord` by @tbu-
  - Require Rust 1.14
  - crate feature `use_generic_array` was dropped.

- 0.3.23

  - Implement `PartialOrd, Ord` as well as `PartialOrd<str>` for
    `ArrayString`.

- 0.3.22

  - Implement `Array` for the 65536 size

- 0.3.21

  - Use `encode_utf8` from crate odds
  - Add constructor `ArrayString::from_byte_string`

- 0.3.20

  - Simplify and speed up `ArrayString`’s `.push(char)`-

- 0.3.19

  - Add new crate feature `use_generic_array` which allows using their
    `GenericArray` just like a regular fixed size array for the storage
    of an `ArrayVec`.

- 0.3.18

  - Fix bounds check in `ArrayVec::insert`!
    It would be buggy if `self.len() < index < self.capacity()`. Take note of
    the push out behavior specified in the docs.

- 0.3.17

  - Added crate feature `use_union` which forwards to the nodrop crate feature
  - Added methods `.is_full()` to `ArrayVec` and `ArrayString`.

- 0.3.16

  - Added method `.retain()` to `ArrayVec`.
  - Added methods `.as_slice(), .as_mut_slice()` to `ArrayVec` and `.as_str()`
    to `ArrayString`.

- 0.3.15

  - Add feature std, which you can opt out of to use `no_std` (requires Rust 1.6
    to opt out).
  - Implement `Clone::clone_from` for ArrayVec and ArrayString

- 0.3.14

  - Add `ArrayString::from(&str)`

- 0.3.13

  - Added `DerefMut` impl for `ArrayString`.
  - Added method `.simplify()` to drop the element for `CapacityError`.
  - Added method `.dispose()` to `ArrayVec`

- 0.3.12

  - Added ArrayString, a fixed capacity analogy of String

- 0.3.11

  - Added trait impls Default, PartialOrd, Ord, Write for ArrayVec

- 0.3.10

  - Go back to using external NoDrop, fixing a panic safety bug (issue #3)

- 0.3.8

  - Inline the non-dropping logic to remove one drop flag in the
    ArrayVec representation.

- 0.3.7

  - Added method .into_inner()
  - Added unsafe method .set_len()

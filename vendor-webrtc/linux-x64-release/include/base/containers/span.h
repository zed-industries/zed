// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// This file intentionally uses the `CHECK()` macro instead of the `CHECK_op()`
// macros, as `CHECK()` generates significantly less code and is more likely to
// optimize reasonably, even in non-official release builds. Please do not
// change the `CHECK()` calls back to `CHECK_op()` calls.

#ifndef BASE_CONTAINERS_SPAN_H_
#define BASE_CONTAINERS_SPAN_H_

#include <stddef.h>
#include <stdint.h>
#include <string.h>

#include <algorithm>
#include <array>
#include <concepts>
#include <functional>
#include <initializer_list>
#include <iosfwd>
#include <iterator>
#include <limits>
#include <memory>
#include <optional>
#include <ranges>
#include <span>
#include <type_traits>
#include <utility>
#include <vector>

#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/containers/checked_iterators.h"
#include "base/containers/span_forward_internal.h"
#include "base/numerics/integral_constant_like.h"
#include "base/numerics/safe_conversions.h"
#include "base/strings/cstring_view.h"
#include "base/strings/to_string.h"
#include "base/types/to_address.h"

// A span is a view of contiguous elements that can be accessed like an array,
// intended for use as a parameter or local. Unlike direct use of pointers and
// sizes, it enforces safe usage (and simplifies callers); unlike container
// refs, it is agnostic to the element container, expressing only "access to
// some sequence of elements". It is similar to `std::string_view`, but for
// arbitrary elements instead of just characters, and additionally allowing
// mutation if the element type is non-`const`.
//
// Spans can be constructed from arrays, range-like objects (generally, objects
// which expose `begin()`, `end()`, `data()`, and `size()`), and initializer
// lists. As with all view types, spans do not own the underlying memory, so
// users must ensure they do not outlive their backing stores; storing a span as
// a member object is usually incorrect. (For the rare case this is useful,
// prefer `raw_span<>` so the underlying storage pointer will be protected by
// MiraclePtr.)
//
// Since spans only consist of a pointer and (for dynamic-extent spans) a size,
// they are lightweight; constructing and copying spans is cheap and they should
// be passed by value.
//
// Scopes which only need read access to the underlying data should use
// `span<const T>`, which can be implicitly constructed from `span<T>`.
// Habitually using `span<const T>` also avoids confusing compile errors when
// trying to construct spans from compile-time constants or non-borrowed ranges,
// which won't convert to `span<T>`.
//
// Without span:
// ```
//   /* Read-only usage */
//
//   // Implementation must avoid OOB reads.
//   std::string HexEncode(const uint8_t* data, size_t size) { ... }
//
//   // Must use a separate variable to avoid repeated generation calls below.
//   std::vector<uint8_t> data_buffer = GenerateData();
//   // Prone to accidentally passing the wrong size.
//   std::string r = HexEncode(data_buffer.data(), data_buffer.size());
//
//   /* Mutable usage */
//
//   // Same concerns apply in this example.
//   ssize_t SafeSNPrintf(char* buf, size_t N, const char* fmt, Args...) { ... }
//
//   char str_buffer[100];
//   SafeSNPrintf(str_buffer, sizeof(str_buffer), "Pi ~= %lf", 3.14);
// ```
//
// With span:
// ```
//   /* Read-only usage */
//
//   // Automatically `CHECK()`s on attempted OOB accesses.
//   std::string HexEncode(span<const uint8_t> data) { ... }
//
//   // Can pass return value directly, since it lives until the end of the full
//   // expression, outlasting the function call. Can't pass wrong size.
//   std::string r = HexEncode(GenerateData());
//
//   /* Mutable usage */
//
//   // Can write to `buf`, but only within bounds.
//   ssize_t SafeSNPrintf(span<char> buf, const char* fmt, Args...) { ... }
//
//   char str_buffer[100];
//   // Automatically infers span size as array size (i.e. 100).
//   SafeSNPrintf(str_buffer, "Pi ~= %lf", 3.14);
// ```
//
// Dynamic-extent vs. fixed-extent spans
// -------------------------------------
// By default spans have dynamic extent, which means that the size is available
// at runtime via `size()`, a la other containers and views. By using a second
// template parameter or passing a `std::integral_constant` to the second (size)
// constructor arg, a span's extent can be fixed at compile time; this can move
// some constraint checks to compile time and slightly improve codegen, at the
// cost of verbosity and more template instantiations. Methods like `first()` or
// `subspan()` also provide templated overloads that produce fixed-extent spans;
// these are preferred when the size is known at compile time, in part because
// e.g. `first(1)` is a compile-error (the `int` arg is not compatible with the
// `StrictNumeric<size_t>` param; use `first(1u)` instead), but `first<1>()` is
// not.
//
// A fixed-extent span implicitly converts to a dynamic-extent span (e.g.
// `span<int, 6>` is implicitly convertible to `span<int>`), so most code that
// operates on spans of arbitrary length can just accept a `span<T>`; there is
// no need to add an additional overload for specially handling the `span<T, N>`
// case.
//
// There are several ways to go from a dynamic-extent span to a fixed-extent
// span:
// - Explicit construction of `span<T, N>`, which `CHECK()`s if the size doesn't
//   match.
// - Construction of `span(T*, fixed_extent<N>)`, which is equivalent to the
//   above.
// - `to_fixed_extent<N>()`, which returns `std::nullopt` if the size doesn't
//   match.
// - `first<N>()`, `last<N>()`, and `subspan<Index, N>()`, which `CHECK()` if
//   the size is insufficient.
//
// Spans, `const`, and pointer-type element types
// ----------------------------------------------
// Pointer-type elements can make translating `const` from container types to
// spans confusing. Fundamentally, if you analogize types this way:
//   `std::vector<T>`       => `span<T>`
// Then this would be const version:
//   `const std::vector<T>` => `span<const T>`
//    (or, more verbosely:) => `span<std::add_const_t<T>>`
//
// However, note that if `T` is `int*`, then `const T` is `int* const`. So:
//   `const std::vector<int*>`       => `span<int* const>`
//   `std::vector<const int*>`       => `span<const int*>`
//   `const std::vector<const int*>` => `span<const int* const>`
//
// (N.B. There is no entry above for `std::vector<int* const>`, since per the
// C++ standard, `std::vector`'s element type must be non-const.)
//
// Byte spans, `std::has_unique_object_representations_v<>`, and conversions
// -------------------------------------------------------------------------
// Because byte spans are often used to copy and hash objects, the byte span
// conversion functions (e.g. `as_bytes()`, `as_byte_span()`) require the
// element type to meet `std::has_unique_object_representations_v<>`. For types
// which do not meet this requirement but need conversion to a byte span, there
// are two workarounds:
//   1. If the type is safe to convert to a byte span in general, specialize
//      `kCanSafelyConvertToByteSpan<T>` to be true for it. For example, Blink's
//      `AtomicString` is not trivially copyable, but it is interned, so hashing
//      and comparing the hashed values is safe.
//   2. If the type is not safe in general but is safe for a particular use
//      case, pass `base::allow_nonunique_obj` as the first arg to the byte span
//      conversion functions. For example, floating-point values are not unique
//      (among other reasons, because `+0` and `-0` are distinct but compare
//      equal), but they are trivially copyable, so serializing them to disk and
//      then deserializing is OK.
//
// Spans using `raw_ptr<T>` for internal storage
// ---------------------------------------------
// Provided via the type alias `raw_span<T[, N]>` (see base/memory/raw_span.h).
// Use only for the uncommon case when a span should be a data member of an
// object; for locals and params, use `span` (similarly to where you'd use a
// `raw_ptr<T>` vs. a `T*`).
//
// Beware the risk of dangling pointers! The object owning the member span must
// not access that span's data after the backing storage's lifetime ends. This
// is the same risk as with all spans, but members tend to be longer-lived than
// params/locals, and thus more prone to dangerous use.
//
// Differences from `std::span`
// ----------------------------
// https://eel.is/c++draft/views contains the latest C++ draft of `std::span`
// and related utilities. Chromium aims to follow the draft except where noted
// below; please report other divergences you find.
//
// Differences from [span.syn]:
// - For convenience, provides `fixed_extent<N>` as an alias to
//   `std::integral_constant<size_t, N>`, to aid in constructing fixed-extent
//   spans from pointers.
//
// Differences from [span.overview]:
// - `span` takes an optional third template argument that can be used to
//   customize the underlying storage pointer type. This allows implementing
//   `raw_span` as a specialization.
//
// Differences from [span.cons]:
// - The constructor which takes an iterator and a count uses
//   `StrictNumeric<size_type>` instead of `size_type` to prevent unsafe type
//   conversions.
// - Omits constructors from `std::array`, since separating these from the range
//   constructor is only useful to mark them `noexcept`, and Chromium doesn't
//   care about that.
// - Fixed-extent constructor from range is only `explicit` for ranges whose
//   extent cannot be statically computed. This matches the spirit of
//   `std::span`, which handles these (so far as it is aware) via other
//   overloads. Without this, we would not only need the dedicated constructors
//   from `std::array`, we would also need dedicated constructors from
//   fixed-extent `std::span`.
// - Adds move construction and assignment. These can avoid refcount churn when
//   the storage pointer is not `T*`. Not necessary for `std::span` since it
//   does not allow customizing the storage pointer type.
// - Provides implicit conversion in both directions between fixed-extent `span`
//   and `std::span`. The general-purpose range constructors that would
//   otherwise handle these cases are explicit for both fixed-extent span types.
// - For convenience, provides `span::copy_from[_nonoverlapping]()` as wrappers
//   around `std::ranges::copy()` that enforce equal-size spans.
// - For convenience, provides `span::copy_prefix_from()` to allow copying into
//   the beginning of the current span.
//
// Differences from [span.deduct]:
// - The deduction guide from a range creates fixed-extent spans if the source
//   extent is available at compile time.
//
// Differences from [span.sub]:
// - As in [span.cons], `size_t` parameters are changed to
//   `StrictNumeric<size_type>`.
// - There are separate overloads for one-arg and two-arg forms of subspan,
//   and the two-arg form does not accept dynamic_extent as a count.
// - For convenience, provides `span::split_at()` to split a single span into
//   two at a given offset.
// - For convenience, provides `span::take_first[_elem]()` to remove the first
//   portion of a dynamic-extent span and return it.
//
// Differences from [span.obs]:
// - For convenience, provides `span::operator==()` to check whether two spans
//   refer to equal-sized ranges of equal objects. This was intentionally
//   removed from `std::span` because it makes the type non-Regular; see
//   http://wg21.link/p1085 for details.
// - Similarly, provides `span::operator<=>()`, which performs lexicographic
//   comparison between spans.
//
// Differences from [span.elem]:
// - Because Chromium does not use exceptions, `span::at()` behaves identically
//   to `span::operator[]()` (i.e. it `CHECK()`s on out-of-range indexes rather
//   than throwing).
// - For convenience, provides `span::get_at()` to return a pointer (rather than
//   reference) to an element. This is necessary if the backing memory may be
//   uninitialized, since forming a reference would be UB.
//
// Differences from [span.objectrep]:
// - For convenience, provides `span::to_fixed_extent<N>()` to attempt
//   conversion to a fixed-extent span, and return null on failure.
// - Because Chromium bans `std::byte`, `as_[writable_]bytes()` use `uint8_t`
//   instead of `std::byte` as the returned element type.
// - For convenience, provides `as_[writable_]chars()` and `as_string_view()`
//   to convert to other "view of bytes"-like objects.
// - For convenience, provides an `operator<<()` overload that accepts a span
//   and prints a byte representation. Also provides a `PrintTo()` overload to
//   convince GoogleTest to use this operator to print.
// - For convenience, provides `[byte_]span_from_ref()` to convert single
//   (non-range) objects to spans.
// - For convenience, provides `[byte_]span_[with_nul_]from_cstring()` to
//   convert `const char[]` literals to spans.
// - For convenience, provides `[byte_]span_with_nul_from_cstring_view()` to
//   convert `basic_cstring_view<T>` to spans, preserving the null terminator.
// - For convenience, provides `as_[writable_]byte_span()` to convert
//   spanifiable objects directly to byte spans.
// - For safety, bans types which do not meet
//   `std::has_unique_object_representations_v<>` from all byte span conversion
//   functions by default. See more detailed comments above for workarounds.

namespace base {

// Provides a compile-time fixed extent to the `count` argument of the span
// constructor.
//
// (Not in `std::`.)
template <size_t N>
using fixed_extent = std::integral_constant<size_t, N>;

}  // namespace base

// Mark `span` as satisfying the `view` and `borrowed_range` concepts. This
// should be done before the definition of `span`, so that any inlined calls to
// range functionality use the correct specializations.
template <typename ElementType, size_t Extent, typename InternalPtrType>
inline constexpr bool
    std::ranges::enable_view<base::span<ElementType, Extent, InternalPtrType>> =
        true;
template <typename ElementType, size_t Extent, typename InternalPtrType>
inline constexpr bool std::ranges::enable_borrowed_range<
    base::span<ElementType, Extent, InternalPtrType>> = true;

namespace base {

// Allows global use of a type for conversion to byte spans.
template <typename T>
inline constexpr bool kCanSafelyConvertToByteSpan =
    std::has_unique_object_representations_v<T>;
template <typename T, typename U>
inline constexpr bool kCanSafelyConvertToByteSpan<std::pair<T, U>> =
    kCanSafelyConvertToByteSpan<std::remove_cvref_t<T>> &&
    kCanSafelyConvertToByteSpan<std::remove_cvref_t<U>>;

// Type tag to provide to byte span conversion functions to bypass
// `std::has_unique_object_representations_v<>` check.
struct allow_nonunique_obj_t {
  explicit allow_nonunique_obj_t() = default;
};
inline constexpr allow_nonunique_obj_t allow_nonunique_obj{};

namespace internal {

// Exposition-only concept from [span.syn]
template <typename T>
inline constexpr size_t MaybeStaticExt = dynamic_extent;
template <typename T>
  requires IntegralConstantLike<T>
inline constexpr size_t MaybeStaticExt<T> = {T::value};

template <typename From, typename To>
concept LegalDataConversion = std::is_convertible_v<From (*)[], To (*)[]>;

// Akin to `std::constructible_from<span, T>`, but meant to be used in a
// type-deducing context where we don't know what args would be deduced;
// `std::constructible_from` can't be directly used in such a case since the
// type parameters must be fully-specified (e.g. `span<int>`), requiring us to
// have that knowledge already.
template <typename T>
concept SpanConstructibleFrom = requires(T&& t) { span(std::forward<T>(t)); };

// Returns the element type of `span(T)`.
template <typename T>
  requires SpanConstructibleFrom<T>
using ElementTypeOfSpanConstructedFrom =
    typename decltype(span(std::declval<T>()))::element_type;

template <typename T, typename It>
concept CompatibleIter =
    std::contiguous_iterator<It> &&
    LegalDataConversion<std::remove_reference_t<std::iter_reference_t<It>>, T>;

// True when `T` is a `span`.
template <typename T>
inline constexpr bool kIsSpan = false;
template <typename ElementType, size_t Extent, typename InternalPtrType>
inline constexpr bool kIsSpan<span<ElementType, Extent, InternalPtrType>> =
    true;

template <typename T, typename R>
concept CompatibleRange =
    std::ranges::contiguous_range<R> && std::ranges::sized_range<R> &&
    (std::ranges::borrowed_range<R> || (std::is_const_v<T>)) &&
    // `span`s should go through the copy constructor.
    (!kIsSpan<std::remove_cvref_t<R>> &&
     // Arrays should go through the array constructors.
     (!std::is_array_v<std::remove_cvref_t<R>>)) &&
    LegalDataConversion<
        std::remove_reference_t<std::ranges::range_reference_t<R>>,
        T>;

// Whether source object extent `X` will work to create a span of fixed extent
// `N`. This is not intended for use in dynamic-extent spans.
template <size_t N, size_t X>
concept FixedExtentConstructibleFromExtent = X == N || X == dynamic_extent;

// Computes a fixed extent if possible from a source container type `T`.
template <typename T>
inline constexpr size_t kComputedExtentImpl = dynamic_extent;
template <typename T>
  requires requires { std::tuple_size<T>(); }
inline constexpr size_t kComputedExtentImpl<T> = std::tuple_size_v<T>;
template <typename T, size_t N>
inline constexpr size_t kComputedExtentImpl<T[N]> = N;
template <typename T, size_t N>
inline constexpr size_t kComputedExtentImpl<std::span<T, N>> = N;
template <typename T, size_t N, typename InternalPtrType>
inline constexpr size_t kComputedExtentImpl<span<T, N, InternalPtrType>> = N;
template <typename T>
inline constexpr size_t kComputedExtent =
    kComputedExtentImpl<std::remove_cvref_t<T>>;

template <typename T>
concept CanSafelyConvertToByteSpan =
    kCanSafelyConvertToByteSpan<std::remove_cvref_t<T>>;

template <typename T>
concept ByteSpanConstructibleFrom =
    SpanConstructibleFrom<T> &&
    CanSafelyConvertToByteSpan<ElementTypeOfSpanConstructedFrom<T>>;

// Allows one-off use of a type that wouldn't normally convert to a byte span.
template <typename T>
concept CanSafelyConvertNonUniqueToByteSpan =
    // Non-trivially-copyable elements usually aren't safe even to serialize;
    // when they are that's normally unconditionally true and can be handled
    // using `kCanSafelyConvertToByteSpan`.
    std::is_trivially_copyable_v<T> &&
    // If this fails, `allow_nonunique_obj` wasn't necessary.
    !std::has_unique_object_representations_v<T>;

template <typename T>
concept ByteSpanConstructibleFromNonUnique =
    SpanConstructibleFrom<T> &&
    CanSafelyConvertNonUniqueToByteSpan<ElementTypeOfSpanConstructedFrom<T>>;

template <typename ByteType,
          typename ElementType,
          size_t Extent,
          typename InternalPtrType>
  requires((std::same_as<std::remove_const_t<ByteType>, char> ||
            std::same_as<std::remove_const_t<ByteType>, unsigned char>) &&
           (std::is_const_v<ByteType> || !std::is_const_v<ElementType>))
constexpr auto as_byte_span(
    span<ElementType, Extent, InternalPtrType> s) noexcept {
  constexpr size_t kByteExtent =
      Extent == dynamic_extent ? dynamic_extent : sizeof(ElementType) * Extent;
  // SAFETY: `s.data()` points to at least `s.size_bytes()` bytes' worth of
  // valid elements, so the size computed below must only contain valid
  // elements. Since `ByteType` is an alias to a character type, it has a size
  // of 1 byte, the resulting pointer has no alignment concerns, and it is not
  // UB to access memory contents inside the allocation through it.
  return UNSAFE_BUFFERS(span<ByteType, kByteExtent>(
      reinterpret_cast<ByteType*>(s.data()), s.size_bytes()));
}

}  // namespace internal

// [span]: class `span` (non-dynamic `Extent`s)
template <typename ElementType, size_t Extent, typename InternalPtrType>
class GSL_POINTER span {
 public:
  using element_type = ElementType;
  using value_type = std::remove_cv_t<element_type>;
  using size_type = size_t;
  using difference_type = ptrdiff_t;
  using pointer = element_type*;
  using const_pointer = const element_type*;
  using reference = element_type&;
  using const_reference = const element_type&;
  using iterator = CheckedContiguousIterator<element_type>;
  using const_iterator = CheckedContiguousConstIterator<element_type>;
  using reverse_iterator = std::reverse_iterator<iterator>;
  // TODO(C++23): When `std::const_iterator<>` is available, switch to
  // `std::const_iterator<reverse_iterator>` as the standard specifies.
  using const_reverse_iterator = std::reverse_iterator<const_iterator>;
  static constexpr size_type extent = Extent;

  // [span.cons]: Constructors, copy, and assignment
  // Default constructor.
  constexpr span() noexcept
    requires(extent == 0)
  = default;

  // Iterator + count.
  template <typename It>
    requires(internal::CompatibleIter<element_type, It>)
  // SAFETY: `first` must point to the first of at least `count` contiguous
  // valid elements, or the span will allow access to invalid elements,
  // resulting in UB.
  UNSAFE_BUFFER_USAGE constexpr explicit span(It first,
                                              StrictNumeric<size_type> count)
      : data_(to_address(first)) {
    CHECK(size_type{count} == extent);

    // Non-zero `count` implies non-null `data_`. Use `SpanOrSize<T>` to
    // represent a size that might not be accompanied by the actual data.
    DCHECK(count == 0 || !!data_);
  }

  // Iterator + sentinel.
  template <typename It, typename End>
    requires(internal::CompatibleIter<element_type, It> &&
             std::sized_sentinel_for<End, It> &&
             !std::is_convertible_v<End, size_t>)
  // SAFETY: `first` and `last` must be for the same allocation and all elements
  // in the range [first, last) must be valid, or the span will allow access to
  // invalid elements, resulting in UB.
  UNSAFE_BUFFER_USAGE constexpr explicit span(It first, End last)
      // SAFETY: The caller must guarantee that `first` and `last` point into
      // the same allocation. In this case, the extent will be the number of
      // elements between the iterators and thus a valid size for the pointer to
      // the element at `first`.
      //
      // It is safe to check for underflow after subtraction because the
      // underflow itself is not UB and `size_` is not converted to an invalid
      // pointer (which would be UB) before the check.
      : UNSAFE_BUFFERS(span(first, static_cast<size_type>(last - first))) {
    // Verify `last - first` did not underflow.
    CHECK(first <= last);
  }

  // Array of size `extent`.
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr span(
      std::type_identity_t<element_type> (&arr LIFETIME_BOUND)[extent]) noexcept
      // SAFETY: The type signature guarantees `arr` contains `extent` elements.
      : UNSAFE_BUFFERS(span(arr, extent)) {}

  // Range.
  template <typename R, size_t N = internal::kComputedExtent<R>>
    requires(internal::CompatibleRange<element_type, R> &&
             internal::FixedExtentConstructibleFromExtent<extent, N>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr explicit(N != extent) span(R&& range LIFETIME_BOUND)
      // SAFETY: `std::ranges::size()` returns the number of elements
      // `std::ranges::data()` will point to, so accessing those elements will
      // be safe.
      : UNSAFE_BUFFERS(
            span(std::ranges::data(range), std::ranges::size(range))) {}
  template <typename R, size_t N = internal::kComputedExtent<R>>
    requires(internal::CompatibleRange<element_type, R> &&
             internal::FixedExtentConstructibleFromExtent<extent, N> &&
             std::ranges::borrowed_range<R>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr explicit span(R&& range)
      // SAFETY: `std::ranges::size()` returns the number of elements
      // `std::ranges::data()` will point to, so accessing those elements will
      // be safe.
      : UNSAFE_BUFFERS(
            span(std::ranges::data(range), std::ranges::size(range))) {}

  // Initializer list.
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr explicit span(std::initializer_list<value_type> il LIFETIME_BOUND)
    requires(std::is_const_v<element_type>)
      // SAFETY: `size()` is exactly the number of elements in the initializer
      // list, so accessing that many will be safe.
      : UNSAFE_BUFFERS(span(il.begin(), il.size())) {}

  // Copy and move.
  constexpr span(const span& other) noexcept = default;
  template <typename OtherElementType,
            size_t OtherExtent,
            typename OtherInternalPtrType>
    requires((OtherExtent == dynamic_extent || extent == OtherExtent) &&
             internal::LegalDataConversion<OtherElementType, element_type>)
  constexpr explicit(OtherExtent == dynamic_extent)
      span(const span<OtherElementType, OtherExtent, OtherInternalPtrType>&
               other) noexcept
      // SAFETY: `size()` is the number of elements that can be safely accessed
      // at `data()`.
      : UNSAFE_BUFFERS(span(other.data(), other.size())) {}
  constexpr span(span&& other) noexcept = default;

  // Copy and move assignment.
  constexpr span& operator=(const span& other) noexcept = default;
  constexpr span& operator=(span&& other) noexcept = default;

  // Performs a deep copy of the elements referenced by `other` to those
  // referenced by `this`. The spans must be the same size.
  //
  // If it's known the spans can not overlap, `copy_from_nonoverlapping()`
  // provides an unsafe alternative that avoids intermediate copies.
  //
  // (Not in `std::`; inspired by Rust's `slice::copy_from_slice()`.)
  constexpr void copy_from(span<const element_type, extent> other)
    requires(!std::is_const_v<element_type>)
  {
    if (std::is_constant_evaluated()) {
      // Comparing pointers to different objects at compile time yields
      // unspecified behavior, which would halt compilation. Instead,
      // unconditionally use a separate buffer in the constexpr context. This
      // would be inefficient at runtime, but that's irrelevant.
      std::vector<element_type> vec(other.begin(), other.end());
      std::ranges::copy(vec, begin());
    } else {
      // Using `<=` to compare pointers to different allocations is UB, but
      // using `std::less_equal` is well-defined ([comparisons.general]).
      if (std::less_equal{}(to_address(begin()), to_address(other.begin()))) {
        std::ranges::copy(other, begin());
      } else {
        std::ranges::copy_backward(other, end());
      }
    }
  }
  template <typename R, size_t N = internal::kComputedExtent<R>>
    requires(!std::is_const_v<element_type> &&
             // Fixed-extent ranges should implicitly convert to use the
             // overload above; if they don't, it's because the extent doesn't
             // match. Rejecting this here improves the resulting errors.
             N == dynamic_extent &&
             std::convertible_to<R &&, span<const element_type>>)
  constexpr void copy_from(R&& other) {
    // Note: The constructor `CHECK()`s that a dynamic-extent `other` has the
    // right size.
    copy_from(span<const element_type, extent>(std::forward<R>(other)));
  }

  // Like `copy_from()`, but may be more performant; however, the caller must
  // guarantee the spans do not overlap, or this will invoke UB.
  //
  // (Not in `std::`; inspired by Rust's `slice::copy_from_slice()`.)
  constexpr void copy_from_nonoverlapping(
      span<const element_type, extent> other)
    requires(!std::is_const_v<element_type>)
  {
    // Comparing pointers to different objects at compile time yields
    // unspecified behavior, which would halt compilation. Instead implement in
    // terms of the guaranteed-safe behavior; performance is irrelevant in the
    // constexpr context.
    if (std::is_constant_evaluated()) {
      copy_from(other);
      return;
    }

    // See comments in `copy_from()` re: use of templated comparison objects.
    DCHECK(std::less_equal{}(to_address(end()), to_address(other.begin())) ||
           std::greater_equal{}(to_address(begin()), to_address(other.end())));
    std::ranges::copy(other, begin());
  }
  template <typename R, size_t N = internal::kComputedExtent<R>>
    requires(!std::is_const_v<element_type> && N == dynamic_extent &&
             std::convertible_to<R &&, span<const element_type>>)
  constexpr void copy_from_nonoverlapping(R&& other) {
    // Note: The constructor `CHECK()`s that a dynamic-extent `other` has the
    // right size.
    copy_from_nonoverlapping(
        span<const element_type, extent>(std::forward<R>(other)));
  }

  // Like `copy_from()`, but allows the source to be smaller than this span, and
  // will only copy as far as the source size, leaving the remaining elements of
  // this span unwritten.
  //
  // (Not in `std::`; allows caller code to elide repeated size information and
  // makes it easier to preserve fixed-extent spans in the process.)
  template <typename R, size_t N = internal::kComputedExtent<R>>
    requires(!std::is_const_v<element_type> &&
             (N <= extent || N == dynamic_extent) &&
             std::convertible_to<R &&, span<const element_type>>)
  constexpr void copy_prefix_from(R&& other) {
    if constexpr (N == dynamic_extent) {
      return first(other.size()).copy_from(other);
    } else {
      return first<N>().copy_from(other);
    }
  }

  // Implicit conversion to fixed-extent `std::span<>`. (The fixed-extent
  // `std::span` range constructor is explicit.)
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator std::span<element_type, extent>() const {
    return std::span<element_type, extent>(*this);
  }
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator std::span<const element_type, extent>() const
    requires(!std::is_const_v<element_type>)
  {
    return std::span<const element_type, extent>(*this);
  }

  // [span.sub]: Subviews
  // First `count` elements.
  template <size_t Count>
  constexpr auto first() const
    requires(Count <= extent)
  {
    // SAFETY: `data()` points to at least `extent` elements, so the new data
    // scope is a strict subset of the old.
    return UNSAFE_BUFFERS(span<element_type, Count>(data(), Count));
  }
  constexpr auto first(StrictNumeric<size_type> count) const {
    CHECK(size_type{count} <= extent);
    // SAFETY: `data()` points to at least `extent` elements, so the new data
    // scope is a strict subset of the old.
    return UNSAFE_BUFFERS(span<element_type>(data(), count));
  }

  // Last `count` elements.
  template <size_t Count>
  constexpr auto last() const
    requires(Count <= extent)
  {
    // SAFETY: `data()` points to at least `extent` elements, so the new data
    // scope is a strict subset of the old.
    return UNSAFE_BUFFERS(
        span<element_type, Count>(data() + (extent - Count), Count));
  }
  constexpr auto last(StrictNumeric<size_type> count) const {
    CHECK(size_type{count} <= extent);
    // SAFETY: `data()` points to at least `extent` elements, so the new data
    // scope is a strict subset of the old.
    return UNSAFE_BUFFERS(
        span<element_type>(data() + (extent - size_type{count}), count));
  }

  // `count` elements beginning at `offset`.
  template <size_t Offset, size_t Count = dynamic_extent>
  constexpr auto subspan() const
    requires(Offset <= extent &&
             (Count == dynamic_extent || Count <= extent - Offset))
  {
    if constexpr (Count == dynamic_extent) {
      constexpr size_t kRemaining = extent - Offset;
      // SAFETY: `data()` points to at least `extent` elements, so `Offset`
      // specifies a valid element index or the past-the-end index, and
      // `kRemaining` cannot index past-the-end elements.
      return UNSAFE_BUFFERS(
          span<element_type, kRemaining>(data() + Offset, kRemaining));
    } else {
      // SAFETY: `data()` points to at least `extent` elements, so `Offset`
      // specifies a valid element index or the past-the-end index, and `Count`
      // is no larger than the number of remaining valid elements.
      return UNSAFE_BUFFERS(span<element_type, Count>(data() + Offset, Count));
    }
  }
  constexpr auto subspan(StrictNumeric<size_type> offset) const {
    CHECK(size_type{offset} <= extent);
    const size_type remaining = extent - size_type{offset};
    // SAFETY: `data()` points to at least `extent` elements, so `offset`
    // specifies a valid element index or the past-the-end index, and
    // `remaining` cannot index past-the-end elements.
    return UNSAFE_BUFFERS(
        span<element_type>(data() + size_type{offset}, remaining));
  }
  constexpr auto subspan(StrictNumeric<size_type> offset,
                         StrictNumeric<size_type> count) const {
    DCHECK(size_type{count} != dynamic_extent)
        << "base does not allow dynamic_extent in two-arg subspan()";
    // Deliberately combine tests to minimize code size.
    CHECK(size_type{offset} <= size() &&
          size_type{count} <= size() - size_type{offset});
    // SAFETY: `data()` points to at least `extent` elements, so `offset`
    // specifies a valid element index or the past-the-end index, and `count` is
    // no larger than the number of remaining valid elements.
    return UNSAFE_BUFFERS(
        span<element_type>(data() + size_type{offset}, count));
  }

  // Splits a span a given offset, returning a pair of spans that cover the
  // ranges strictly before the offset and starting at the offset, respectively.
  //
  // (Not in `std::span`; inspired by Rust's `slice::split_at()` and
  // `split_at_mut()`.)
  template <size_t Offset>
    requires(Offset <= extent)
  constexpr auto split_at() const {
    return std::pair(first<Offset>(), subspan<Offset, extent - Offset>());
  }
  constexpr auto split_at(StrictNumeric<size_type> offset) const {
    return std::pair(first(offset), subspan(offset));
  }

  // [span.obs]: Observers
  // Size.
  constexpr size_type size() const noexcept { return extent; }
  constexpr size_type size_bytes() const noexcept {
    return extent * sizeof(element_type);
  }

  // Empty.
  [[nodiscard]] constexpr bool empty() const noexcept { return extent == 0; }

  // Returns true if `lhs` and `rhs` are equal-sized and are per-element equal.
  //
  // (Not in `std::span`; improves both ergonomics and safety.)
  //
  // NOTE: Using non-members here intentionally allows comparing types that
  // implicitly convert to `span`.
  friend constexpr bool operator==(span lhs, span rhs)
    requires(std::is_const_v<element_type> &&
             std::equality_comparable<const element_type>)
  {
    return std::ranges::equal(span<const element_type, extent>(lhs),
                              span<const element_type, extent>(rhs));
  }
  friend constexpr bool operator==(span lhs,
                                   span<const element_type, extent> rhs)
    requires(!std::is_const_v<element_type> &&
             std::equality_comparable<const element_type>)
  {
    return std::ranges::equal(span<const element_type, extent>(lhs), rhs);
  }
  template <typename OtherElementType,
            size_t OtherExtent,
            typename OtherInternalPtrType>
    requires((OtherExtent == dynamic_extent || extent == OtherExtent) &&
             std::equality_comparable_with<const element_type,
                                           const OtherElementType>)
  friend constexpr bool operator==(
      span lhs,
      span<OtherElementType, OtherExtent, OtherInternalPtrType> rhs) {
    return std::ranges::equal(span<const element_type, extent>(lhs),
                              span<const OtherElementType, OtherExtent>(rhs));
  }

  // Performs lexicographical comparison of `lhs` and `rhs`.
  //
  // (Not in `std::span`; improves both ergonomics and safety.)
  //
  // NOTE: Using non-members here intentionally allows comparing types that
  // implicitly convert to `span`.
  friend constexpr auto operator<=>(span lhs, span rhs)
    requires(std::is_const_v<element_type> &&
             std::three_way_comparable<const element_type>)
  {
    const auto const_lhs = span<const element_type>(lhs);
    const auto const_rhs = span<const element_type>(rhs);
    return std::lexicographical_compare_three_way(
        const_lhs.begin(), const_lhs.end(), const_rhs.begin(), const_rhs.end());
  }
  friend constexpr auto operator<=>(span lhs,
                                    span<const element_type, extent> rhs)
    requires(!std::is_const_v<element_type> &&
             std::three_way_comparable<const element_type>)
  {
    return span<const element_type>(lhs) <=> rhs;
  }
  template <typename OtherElementType,
            size_t OtherExtent,
            typename OtherInternalPtrType>
    requires((OtherExtent == dynamic_extent || extent == OtherExtent) &&
             std::three_way_comparable_with<const element_type,
                                            const OtherElementType>)
  friend constexpr auto operator<=>(
      span lhs,
      span<OtherElementType, OtherExtent, OtherInternalPtrType> rhs) {
    const auto const_lhs = span<const element_type>(lhs);
    const auto const_rhs = span<const OtherElementType, OtherExtent>(rhs);
    return std::lexicographical_compare_three_way(
        const_lhs.begin(), const_lhs.end(), const_rhs.begin(), const_rhs.end());
  }

  // [span.elem]: Element access
  // Reference to specific element.
  // When `idx` is outside the span, the underlying call will `CHECK()`.
  //
  // Intentionally does not take `StrictNumeric<size_t>`, unlike all other APIs.
  // There are far too many false positives on integer literals (e.g. `s[0]`),
  // and while `ENABLE_IF_ATTR` can be used to work around those for Clang, that
  // would leave the gcc build broken. The consequence of not upgrading this is
  // that some errors will only be detected at runtime instead of compile time.
  constexpr reference operator[](size_type idx) const
    requires(extent > 0)
  {
    return at(idx);
  }
  // When `idx` is outside the span, the underlying call will `CHECK()`.
  constexpr reference at(StrictNumeric<size_type> idx) const
    requires(extent > 0)
  {
    return *get_at(idx);
  }

  // Returns a pointer to an element in the span.
  //
  // (Not in `std::`; necessary when underlying memory is not yet initialized.)
  constexpr pointer get_at(StrictNumeric<size_type> idx) const
    requires(extent > 0)
  {
    CHECK(size_type{idx} < extent);
    // SAFETY: `data()` points to at least `extent` elements, so `idx` must be
    // the index of a valid element.
    return UNSAFE_BUFFERS(data() + size_type{idx});
  }

  // Reference to first/last elements.
  // When `empty()`, the underlying call will `CHECK()`.
  constexpr reference front() const
    requires(extent > 0)
  {
    return operator[](0);
  }
  // When `empty()`, the underlying call will `CHECK()`.
  constexpr reference back() const
    requires(extent > 0)
  {
    return operator[](size() - 1);
  }

  // Underlying memory.
  constexpr pointer data() const noexcept { return data_; }

  // [span.iter]: Iterator support
  // Forward iterators.
  constexpr iterator begin() const noexcept {
    // SAFETY: `data()` points to at least `extent` elements, so `data() +
    // extent` is no larger than just past the end of the corresponding
    // allocation, which is a legal pointer to construct and compare to (though
    // not dereference).
    //
    // Use `AssumeValid()` to elide unnecessary precondition `CHECK()`'s in the
    // iterator constructor: `data() + extent` must not overflow given the above
    // constraints, so the iterator's requirement that begin <= current <= end
    // is guaranteed to be true.
    return UNSAFE_BUFFERS(iterator(
        typename iterator::AssumeValid(data(), data(), data() + extent)));
  }
  constexpr const_iterator cbegin() const noexcept {
    return const_iterator(begin());
  }
  constexpr iterator end() const noexcept {
    // SAFETY: `data()` points to at least `extent` elements, so `data() +
    // extent` is no larger than just past the end of the corresponding
    // allocation, which is a legal pointer to construct and compare to (though
    // not dereference).
    //
    // Use `AssumeValid()` to elide unnecessary precondition `CHECK()`'s in the
    // iterator constructor: `data() + extent` must not overflow given the above
    // constraints, so the iterator's requirement that begin <= current <= end
    // is guaranteed to be true.
    return UNSAFE_BUFFERS(iterator(typename iterator::AssumeValid(
        data(), data() + extent, data() + extent)));
  }
  constexpr const_iterator cend() const noexcept {
    return const_iterator(end());
  }

  // Reverse iterators.
  constexpr reverse_iterator rbegin() const noexcept {
    return reverse_iterator(end());
  }
  constexpr const_reverse_iterator crbegin() const noexcept {
    return const_iterator(rbegin());
  }
  constexpr reverse_iterator rend() const noexcept {
    return reverse_iterator(begin());
  }
  constexpr const_reverse_iterator crend() const noexcept {
    return const_iterator(rend());
  }

 private:
  InternalPtrType data_ = nullptr;
};

// [span]: class <span> (dynamic `Extent`)
template <typename ElementType, typename InternalPtrType>
class GSL_POINTER span<ElementType, dynamic_extent, InternalPtrType> {
 public:
  using element_type = ElementType;
  using value_type = std::remove_cv_t<element_type>;
  using size_type = size_t;
  using difference_type = ptrdiff_t;
  using pointer = element_type*;
  using const_pointer = const element_type*;
  using reference = element_type&;
  using const_reference = const element_type&;
  using iterator = CheckedContiguousIterator<element_type>;
  using const_iterator = CheckedContiguousConstIterator<element_type>;
  using reverse_iterator = std::reverse_iterator<iterator>;
  // TODO(C++23): When `std::const_iterator<>` is available, switch to
  // `std::const_iterator<reverse_iterator>` as the standard specifies.
  using const_reverse_iterator = std::reverse_iterator<const_iterator>;
  static constexpr size_type extent = dynamic_extent;

  // [span.cons]: Constructors, copy, and assignment
  // Default constructor.
  constexpr span() noexcept = default;

  // Iterator + count.
  template <typename It>
    requires(internal::CompatibleIter<element_type, It>)
  // SAFETY: `first` must point to the first of at least `count` contiguous
  // valid elements, or the span will allow access to invalid elements,
  // resulting in UB.
  UNSAFE_BUFFER_USAGE constexpr span(It first, StrictNumeric<size_type> count)
      : data_(to_address(first)), size_(count) {
    // Non-zero `count` implies non-null `data_`. Use `SpanOrSize<T>` to
    // represent a size that might not be accompanied by the actual data.
    DCHECK(count == 0 || !!data_);
  }

  // Iterator + sentinel.
  template <typename It, typename End>
    requires(internal::CompatibleIter<element_type, It> &&
             std::sized_sentinel_for<End, It> &&
             !std::is_convertible_v<End, size_t>)
  // SAFETY: `first` and `last` must be for the same allocation and all elements
  // in the range [first, last) must be valid, or the span will allow access to
  // invalid elements, resulting in UB.
  UNSAFE_BUFFER_USAGE constexpr span(It first, End last)
      // SAFETY: The caller must guarantee that `first` and `last` point into
      // the same allocation. In this case, `size_` will be the number of
      // elements between the iterators and thus a valid size for the pointer to
      // the element at `first`.
      //
      // It is safe to check for underflow after subtraction because the
      // underflow itself is not UB and `size_` is not converted to an invalid
      // pointer (which would be UB) before the check.
      : UNSAFE_BUFFERS(span(first, static_cast<size_type>(last - first))) {
    // Verify `last - first` did not underflow.
    CHECK(first <= last);
  }

  // Array of size N.
  template <size_t N>
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr span(
      std::type_identity_t<element_type> (&arr LIFETIME_BOUND)[N]) noexcept
      // SAFETY: The type signature guarantees `arr` contains `N` elements.
      : UNSAFE_BUFFERS(span(arr, N)) {}

  // Range.
  template <typename R>
    requires(internal::CompatibleRange<element_type, R>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr span(R&& range LIFETIME_BOUND)
      // SAFETY: `std::ranges::size()` returns the number of elements
      // `std::ranges::data()` will point to, so accessing those elements will
      // be safe.
      : UNSAFE_BUFFERS(
            span(std::ranges::data(range), std::ranges::size(range))) {}
  template <typename R>
    requires(internal::CompatibleRange<element_type, R> &&
             std::ranges::borrowed_range<R>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr span(R&& range)
      // SAFETY: `std::ranges::size()` returns the number of elements
      // `std::ranges::data()` will point to, so accessing those elements will
      // be safe.
      : UNSAFE_BUFFERS(
            span(std::ranges::data(range), std::ranges::size(range))) {}

  // Initializer list.
  constexpr span(std::initializer_list<value_type> il LIFETIME_BOUND)
    requires(std::is_const_v<element_type>)
      // SAFETY: `size()` is exactly the number of elements in the initializer
      // list, so accessing that many will be safe.
      : UNSAFE_BUFFERS(span(il.begin(), il.size())) {}

  // Copy and move.
  constexpr span(const span& other) noexcept = default;
  template <typename OtherElementType,
            size_t OtherExtent,
            typename OtherInternalPtrType>
    requires(internal::LegalDataConversion<OtherElementType, element_type>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr span(
      const span<OtherElementType, OtherExtent, OtherInternalPtrType>&
          other) noexcept
      : data_(other.data()), size_(other.size()) {}
  constexpr span(span&& other) noexcept = default;

  // Copy and move assignment.
  constexpr span& operator=(const span& other) noexcept = default;
  constexpr span& operator=(span&& other) noexcept = default;

  // Performs a deep copy of the elements referenced by `other` to those
  // referenced by `this`. The spans must be the same size.
  //
  // If it's known the spans can not overlap, `copy_from_nonoverlapping()`
  // provides an unsafe alternative that avoids intermediate copies.
  //
  // (Not in `std::`; inspired by Rust's `slice::copy_from_slice()`.)
  constexpr void copy_from(span<const element_type> other)
    requires(!std::is_const_v<element_type>)
  {
    CHECK(size() == other.size());
    if (std::is_constant_evaluated()) {
      // Comparing pointers to different objects at compile time yields
      // unspecified behavior, which would halt compilation. Instead,
      // unconditionally use a separate buffer in the constexpr context. This
      // would be inefficient at runtime, but that's irrelevant.
      std::vector<element_type> vec(other.begin(), other.end());
      std::ranges::copy(vec, begin());
    } else {
      // Using `<=` to compare pointers to different allocations is UB, but
      // using `std::less_equal` is well-defined ([comparisons.general]).
      if (std::less_equal{}(to_address(begin()), to_address(other.begin()))) {
        std::ranges::copy(other, begin());
      } else {
        std::ranges::copy_backward(other, end());
      }
    }
  }

  // Like `copy_from()`, but may be more performant; however, the caller must
  // guarantee the spans do not overlap, or this will invoke UB.
  //
  // (Not in `std::`; inspired by Rust's `slice::copy_from_slice()`.)
  constexpr void copy_from_nonoverlapping(span<const element_type> other)
    requires(!std::is_const_v<element_type>)
  {
    // Comparing pointers to different objects at compile time yields
    // unspecified behavior, which would halt compilation. Instead implement in
    // terms of the guaranteed-safe behavior; performance is irrelevant in the
    // constexpr context.
    if (std::is_constant_evaluated()) {
      copy_from(other);
      return;
    }

    CHECK(size() == other.size());
    // See comments in `copy_from()` re: use of templated comparison objects.
    DCHECK(std::less_equal{}(to_address(end()), to_address(other.begin())) ||
           std::greater_equal{}(to_address(begin()), to_address(other.end())));
    std::ranges::copy(other, begin());
  }

  // Like `copy_from()`, but allows the source to be smaller than this span, and
  // will only copy as far as the source size, leaving the remaining elements of
  // this span unwritten.
  //
  // (Not in `std::`; allows caller code to elide repeated size information and
  // makes it easier to preserve fixed-extent spans in the process.)
  constexpr void copy_prefix_from(span<const element_type> other)
    requires(!std::is_const_v<element_type>)
  {
    return first(other.size()).copy_from(other);
  }

  // [span.sub]: Subviews
  // First `count` elements.
  template <size_t Count>
  constexpr auto first() const {
    CHECK(Count <= size());
    // SAFETY: `data()` points to at least `size()` elements, so the new data
    // scope is a strict subset of the old.
    return UNSAFE_BUFFERS(span<element_type, Count>(data(), Count));
  }
  constexpr auto first(StrictNumeric<size_t> count) const {
    CHECK(size_type{count} <= size());
    // SAFETY: `data()` points to at least `size()` elements, so the new data
    // scope is a strict subset of the old.
    return UNSAFE_BUFFERS(span<element_type>(data(), count));
  }

  // Last `count` elements.
  template <size_t Count>
  constexpr auto last() const {
    CHECK(Count <= size());
    // SAFETY: `data()` points to at least `size()` elements, so the new data
    // scope is a strict subset of the old.
    return UNSAFE_BUFFERS(
        span<element_type, Count>(data() + (size() - Count), Count));
  }
  constexpr auto last(StrictNumeric<size_type> count) const {
    CHECK(size_type{count} <= size());
    // SAFETY: `data()` points to at least `size()` elements, so the new data
    // scope is a strict subset of the old.
    return UNSAFE_BUFFERS(
        span<element_type>(data() + (size() - size_type{count}), count));
  }

  // `count` elements beginning at `offset`.
  template <size_t Offset, size_t Count = dynamic_extent>
  constexpr auto subspan() const {
    CHECK(Offset <= size());
    const size_type remaining = size() - Offset;
    if constexpr (Count == dynamic_extent) {
      // SAFETY: `data()` points to at least `size()` elements, so `Offset`
      // specifies a valid element index or the past-the-end index, and
      // `remaining` cannot index past-the-end elements.
      return UNSAFE_BUFFERS(
          span<element_type, Count>(data() + Offset, remaining));
    }
    CHECK(Count <= remaining);
    // SAFETY: `data()` points to at least `size()` elements, so `Offset`
    // specifies a valid element index or the past-the-end index, and `Count` is
    // no larger than the number of remaining valid elements.
    return UNSAFE_BUFFERS(span<element_type, Count>(data() + Offset, Count));
  }
  constexpr auto subspan(StrictNumeric<size_type> offset) const {
    CHECK(size_type{offset} <= size());
    const size_type remaining = size() - size_type{offset};
    // SAFETY: `data()` points to at least `size()` elements, so `offset`
    // specifies a valid element index or the past-the-end index, and
    // `remaining` cannot index past-the-end elements.
    return UNSAFE_BUFFERS(
        span<element_type>(data() + size_type{offset}, remaining));
  }
  constexpr auto subspan(StrictNumeric<size_type> offset,
                         StrictNumeric<size_type> count) const {
    DCHECK(size_type{count} != dynamic_extent)
        << "base does not allow dynamic_extent in two-arg subspan()";
    // Deliberately combine tests to minimize code size.
    CHECK(size_type{offset} <= size() &&
          size_type{count} <= size() - size_type{offset});
    // SAFETY: `data()` points to at least `size()` elements, so `offset`
    // specifies a valid element index or the past-the-end index, and `count` is
    // no larger than the number of remaining valid elements.
    return UNSAFE_BUFFERS(
        span<element_type>(data() + size_type{offset}, count));
  }

  // Splits a span a given offset, returning a pair of spans that cover the
  // ranges strictly before the offset and starting at the offset, respectively.
  //
  // (Not in `std::span`; inspired by Rust's `slice::split_at()` and
  // `split_at_mut()`.)
  template <size_t Offset>
  constexpr auto split_at() const {
    CHECK(Offset <= size());
    return std::pair(first<Offset>(), subspan<Offset>());
  }
  constexpr auto split_at(StrictNumeric<size_type> offset) const {
    return std::pair(first(offset), subspan(offset));
  }

  // Returns a span of the first N elements, removing them.
  // When `Offset` is outside the span, the underlying call will `CHECK()`. For
  // a non-fatal alternative, consider `SpanReader`.
  //
  // (Not in `std::span`; convenient for processing a stream of disparate
  // objects or looping over elements.)
  template <size_t Offset>
  constexpr auto take_first() {
    const auto [first, rest] = split_at<Offset>();
    *this = rest;
    return first;
  }
  // When `offset` is outside the span, the underlying call will `CHECK()`.
  constexpr auto take_first(StrictNumeric<size_type> offset) {
    const auto [first, rest] = split_at(offset);
    *this = rest;
    return first;
  }

  // Returns the first element, removing it.
  // When `empty()`, the underlying call will `CHECK()`. For a non-fatal
  // alternative, consider `SpanReader`.
  //
  // (Not in `std::span`; convenient for processing a stream of disparate
  // objects or looping over elements.)
  constexpr auto take_first_elem() { return take_first<1>().front(); }

  // [span.obs]: Observers
  // Size.
  constexpr size_type size() const noexcept { return size_; }
  constexpr size_type size_bytes() const noexcept {
    return size() * sizeof(element_type);
  }

  // Empty.
  [[nodiscard]] constexpr bool empty() const noexcept { return size() == 0; }

  // Returns true if `lhs` and `rhs` are equal-sized and are per-element equal.
  //
  // (Not in `std::span`; improves both ergonomics and safety.)
  //
  // NOTE: Using non-members here intentionally allows comparing types that
  // implicitly convert to `span`.
  friend constexpr bool operator==(span lhs, span rhs)
    requires(std::is_const_v<element_type> &&
             std::equality_comparable<const element_type>)
  {
    return std::ranges::equal(span<const element_type>(lhs),
                              span<const element_type>(rhs));
  }
  friend constexpr bool operator==(span lhs,
                                   span<const element_type, extent> rhs)
    requires(!std::is_const_v<element_type> &&
             std::equality_comparable<const element_type>)
  {
    return std::ranges::equal(span<const element_type>(lhs), rhs);
  }
  template <typename OtherElementType,
            size_t OtherExtent,
            typename OtherInternalPtrType>
    requires(std::equality_comparable_with<const element_type,
                                           const OtherElementType>)
  friend constexpr bool operator==(
      span lhs,
      span<OtherElementType, OtherExtent, OtherInternalPtrType> rhs) {
    return std::ranges::equal(span<const element_type>(lhs),
                              span<const OtherElementType, OtherExtent>(rhs));
  }

  // Performs lexicographical comparison of `lhs` and `rhs`.
  //
  // (Not in `std::span`; improves both ergonomics and safety.)
  //
  // NOTE: Using non-members here intentionally allows comparing types that
  // implicitly convert to `span`.
  friend constexpr auto operator<=>(span lhs, span rhs)
    requires(std::is_const_v<element_type> &&
             std::three_way_comparable<const element_type>)
  {
    const auto const_lhs = span<const element_type>(lhs);
    const auto const_rhs = span<const element_type>(rhs);
    return std::lexicographical_compare_three_way(
        const_lhs.begin(), const_lhs.end(), const_rhs.begin(), const_rhs.end());
  }
  friend constexpr auto operator<=>(span lhs,
                                    span<const element_type, extent> rhs)
    requires(!std::is_const_v<element_type> &&
             std::three_way_comparable<const element_type>)
  {
    return span<const element_type>(lhs) <=> rhs;
  }
  template <typename OtherElementType,
            size_t OtherExtent,
            typename OtherInternalPtrType>
    requires(std::three_way_comparable_with<const element_type,
                                            const OtherElementType>)
  friend constexpr auto operator<=>(
      span lhs,
      span<OtherElementType, OtherExtent, OtherInternalPtrType> rhs) {
    const auto const_lhs = span<const element_type>(lhs);
    const auto const_rhs = span<const OtherElementType, OtherExtent>(rhs);
    return std::lexicographical_compare_three_way(
        const_lhs.begin(), const_lhs.end(), const_rhs.begin(), const_rhs.end());
  }

  // [span.elem]: Element access
  // Reference to a specific element.
  // When `idx` is outside the span, the underlying call will `CHECK()`.
  //
  // Intentionally does not take `StrictNumeric<size_type>`; see comments on
  // fixed-extent version for rationale.
  constexpr reference operator[](size_type idx) const { return at(idx); }

  // When `idx` is outside the span, the underlying call will `CHECK()`.
  constexpr reference at(StrictNumeric<size_type> idx) const {
    return *get_at(idx);
  }

  // Returns a pointer to an element in the span.
  //
  // (Not in `std::`; necessary when underlying memory is not yet initialized.)
  constexpr pointer get_at(StrictNumeric<size_type> idx) const {
    CHECK(size_type{idx} < size());
    // SAFETY: `data()` points to at least `size()` elements, so `idx` must be
    // the index of a valid element.
    return UNSAFE_BUFFERS(data() + size_type{idx});
  }

  // Reference to first/last elements.
  // When `empty()`, the underlying call will `CHECK()`.
  constexpr reference front() const { return operator[](0); }
  // When `empty()`, the underlying call will `CHECK()`.
  constexpr reference back() const { return operator[](size() - 1); }

  // Underlying memory.
  constexpr pointer data() const noexcept { return data_; }

  // [span.iter]: Iterator support
  // Forward iterators.
  constexpr iterator begin() const noexcept {
    // SAFETY: `data()` points to at least `size()` elements, so `data() +
    // size()` is no larger than just past the end of the corresponding
    // allocation, which is a legal pointer to construct and compare to (though
    // not dereference).
    //
    // Use `AssumeValid()` to elide unnecessary precondition `CHECK()`'s in the
    // iterator constructor: `data() + size()` must not overflow given the above
    // constraints, so the iterator's requirement that begin <= current <= end
    // is guaranteed to be true.
    return UNSAFE_BUFFERS(iterator(
        typename iterator::AssumeValid(data(), data(), data() + size())));
  }
  constexpr const_iterator cbegin() const noexcept {
    return const_iterator(begin());
  }
  constexpr iterator end() const noexcept {
    // SAFETY: `data()` points to at least `size()` elements, so `data() +
    // size()` is no larger than just past the end of the corresponding
    // allocation, which is a legal pointer to construct and compare to (though
    // not dereference).
    //
    // Use `AssumeValid()` to elide unnecessary precondition `CHECK()`'s in the
    // iterator constructor: `data() + size()` must not overflow given the above
    // constraints, so the iterator's requirement that begin <= current <= end
    // is guaranteed to be true.
    return UNSAFE_BUFFERS(iterator(typename iterator::AssumeValid(
        data(), data() + size(), data() + size())));
  }
  constexpr const_iterator cend() const noexcept {
    return const_iterator(end());
  }

  // Reverse iterators.
  constexpr reverse_iterator rbegin() const noexcept {
    return reverse_iterator(end());
  }
  constexpr const_reverse_iterator crbegin() const noexcept {
    return const_iterator(rbegin());
  }
  constexpr reverse_iterator rend() const noexcept {
    return reverse_iterator(begin());
  }
  constexpr const_reverse_iterator crend() const noexcept {
    return const_iterator(rend());
  }

  // [span.objectrep]: Views of object representation
  // Converts a dynamic-extent span to a fixed-extent span. Returns a
  // `span<element_type, Extent>` iff `size() == Extent`; otherwise, returns
  // `std::nullopt`.
  //
  // (Not in `std::`; provides a conditional conversion path.)
  template <size_t Extent>
  constexpr std::optional<span<element_type, Extent>> to_fixed_extent() const {
    return size() == Extent ? std::optional(span<element_type, Extent>(*this))
                            : std::nullopt;
  }

 private:
  InternalPtrType data_ = nullptr;
  size_t size_ = 0;
};

// [span.deduct]: Deduction guides
template <typename It, typename EndOrSize>
  requires(std::contiguous_iterator<It>)
span(It, EndOrSize) -> span<std::remove_reference_t<std::iter_reference_t<It>>,
                            internal::MaybeStaticExt<EndOrSize>>;

template <typename T, size_t N>
span(T (&)[N]) -> span<T, N>;

template <typename R>
  requires(std::ranges::contiguous_range<R>)
span(R&&) -> span<std::remove_reference_t<std::ranges::range_reference_t<R>>,
                  internal::kComputedExtent<R>>;

// [span.objectrep]: Views of object representation
template <typename ElementType, size_t Extent, typename InternalPtrType>
  requires(internal::CanSafelyConvertToByteSpan<ElementType>)
constexpr auto as_bytes(span<ElementType, Extent, InternalPtrType> s) {
  return internal::as_byte_span<const uint8_t>(s);
}
template <typename ElementType, size_t Extent, typename InternalPtrType>
  requires(internal::CanSafelyConvertNonUniqueToByteSpan<ElementType>)
constexpr auto as_bytes(allow_nonunique_obj_t,
                        span<ElementType, Extent, InternalPtrType> s) {
  return internal::as_byte_span<const uint8_t>(s);
}
template <typename ElementType, size_t Extent, typename InternalPtrType>
  requires(internal::CanSafelyConvertToByteSpan<ElementType> &&
           !std::is_const_v<ElementType>)
constexpr auto as_writable_bytes(span<ElementType, Extent, InternalPtrType> s) {
  return internal::as_byte_span<uint8_t>(s);
}
template <typename ElementType, size_t Extent, typename InternalPtrType>
  requires(internal::CanSafelyConvertNonUniqueToByteSpan<ElementType> &&
           !std::is_const_v<ElementType>)
constexpr auto as_writable_bytes(allow_nonunique_obj_t,
                                 span<ElementType, Extent, InternalPtrType> s) {
  return internal::as_byte_span<uint8_t>(s);
}

// Like `as_[writable_]bytes()`, but uses `[const] char` rather than `[const]
// uint8_t`.
//
// (Not in `std::`; eases span adoption in Chromium, which uses `char` in many
// cases that rightfully should be `uint8_t`.)
template <typename ElementType, size_t Extent, typename InternalPtrType>
  requires(internal::CanSafelyConvertToByteSpan<ElementType>)
constexpr auto as_chars(span<ElementType, Extent, InternalPtrType> s) {
  return internal::as_byte_span<const char>(s);
}
template <typename ElementType, size_t Extent, typename InternalPtrType>
  requires(internal::CanSafelyConvertNonUniqueToByteSpan<ElementType>)
constexpr auto as_chars(allow_nonunique_obj_t,
                        span<ElementType, Extent, InternalPtrType> s) {
  return internal::as_byte_span<const char>(s);
}
template <typename ElementType, size_t Extent, typename InternalPtrType>
  requires(internal::CanSafelyConvertToByteSpan<ElementType> &&
           !std::is_const_v<ElementType>)
constexpr auto as_writable_chars(span<ElementType, Extent, InternalPtrType> s) {
  return internal::as_byte_span<char>(s);
}
template <typename ElementType, size_t Extent, typename InternalPtrType>
  requires(internal::CanSafelyConvertNonUniqueToByteSpan<ElementType> &&
           !std::is_const_v<ElementType>)
constexpr auto as_writable_chars(allow_nonunique_obj_t,
                                 span<ElementType, Extent, InternalPtrType> s) {
  return internal::as_byte_span<char>(s);
}

// Converts a span over byte-like elements to `std::string_view`.
//
// (Not in `std::`; eases span adoption in Chromium, which uses `string`s and
// `string_view`s in many cases that rightfully should be containers of
// `uint8_t`.)
//
// TODO(C++23): Replace with direct use of the `std::string_view` range
// constructor.
constexpr auto as_string_view(span<const char> s) {
  return std::string_view(s.begin(), s.end());
}
constexpr auto as_string_view(span<const unsigned char> s) {
  return as_string_view(as_chars(s));
}
constexpr auto as_string_view(span<const char16_t> s) {
  return std::u16string_view(s.begin(), s.end());
}
constexpr auto as_string_view(span<const wchar_t> s) {
  return std::wstring_view(s.begin(), s.end());
}

namespace internal {

template <typename T>
concept SpanConvertsToStringView = requires {
  { as_string_view(span<T>()) };
};

}  // namespace internal

// Stream output that prints a byte representation.
//
// (Not in `std::`; convenient for debugging.)
template <typename ElementType, size_t Extent, typename InternalPtrType>
  requires(internal::SpanConvertsToStringView<ElementType> ||
           requires(const ElementType& t) {
             { ToString(t) };
           })
constexpr std::ostream& operator<<(
    std::ostream& l,
    span<ElementType, Extent, InternalPtrType> r) {
  l << '[';
  if constexpr (internal::SpanConvertsToStringView<ElementType>) {
    const auto sv = as_string_view(r);
    if constexpr (requires { l << sv; }) {
      using T = std::remove_cvref_t<ElementType>;
      if constexpr (std::same_as<wchar_t, T>) {
        l << 'L';
      } else if constexpr (std::same_as<char16_t, T>) {
        l << 'u';
      } else if constexpr (std::same_as<char32_t, T>) {
        l << 'U';
      }
      l << '\"' << sv << '\"';
    } else {
      // base/strings/utf_ostream_operators.h provides streaming support for
      // wchar_t/char16_t, so branching on whether streaming is available will
      // give different results depending on whether code has included that,
      // which can lead to UB due to violating the ODR. We don't want to
      // unconditionally include this header above for compile time reasons, so
      // instead force the rare caller that wants this to do it themselves.
      static_assert(
          requires { l << sv; },
          "include base/strings/utf_ostream_operators.h when streaming spans "
          "of wide chars");
    }
  } else if constexpr (Extent != 0) {
    // It would be nice to use `JoinString()` here, but making that `constexpr`
    // is more trouble than it's worth.
    if (!r.empty()) {
      l << ToString(r.front());
      for (const ElementType& e : r.template subspan<1>()) {
        l << ", " << ToString(e);
      }
    }
  }
  return l << ']';
}

// Because `span` meets the GoogleTest "container" criteria, explicitly
// overloading `PrintTo()` is necessary to make GoogleTest print spans using the
// `operator<<()` overload above, and not its own container printer.
template <typename ElementType, size_t Extent, typename InternalPtrType>
constexpr void PrintTo(span<ElementType, Extent, InternalPtrType> s,
                       std::ostream* os) {
  *os << s;
}

// Converts a `T&` to a `span<T, 1>`.
//
// (Not in `std::`; inspired by Rust's `slice::from_ref()`.)
template <typename T>
constexpr auto span_from_ref(const T& t LIFETIME_BOUND) {
  // SAFETY: It's safe to read the memory at `t`'s address as long as the
  // provided reference is valid.
  return UNSAFE_BUFFERS(span<const T, 1>(std::addressof(t), 1u));
}
template <typename T>
constexpr auto span_from_ref(T& t LIFETIME_BOUND) {
  // SAFETY: It's safe to read the memory at `t`'s address as long as the
  // provided reference is valid.
  return UNSAFE_BUFFERS(span<T, 1>(std::addressof(t), 1u));
}

// Converts a `T&` to a `span<[const] uint8_t, sizeof(T)>`.
//
// (Not in `std::`.)
template <typename T>
  requires(internal::CanSafelyConvertToByteSpan<T>)
constexpr auto byte_span_from_ref(const T& t LIFETIME_BOUND) {
  return as_bytes(span_from_ref(t));
}
template <typename T>
  requires(internal::CanSafelyConvertNonUniqueToByteSpan<T>)
constexpr auto byte_span_from_ref(allow_nonunique_obj_t,
                                  const T& t LIFETIME_BOUND) {
  return as_bytes(allow_nonunique_obj, span_from_ref(t));
}
template <typename T>
  requires(internal::CanSafelyConvertToByteSpan<T>)
constexpr auto byte_span_from_ref(T& t LIFETIME_BOUND) {
  return as_writable_bytes(span_from_ref(t));
}
template <typename T>
  requires(internal::CanSafelyConvertNonUniqueToByteSpan<T>)
constexpr auto byte_span_from_ref(allow_nonunique_obj_t, T& t LIFETIME_BOUND) {
  return as_writable_bytes(allow_nonunique_obj, span_from_ref(t));
}

// Converts a `const CharT[]` literal to a `span<const CharT>`, omitting the
// trailing '\0' (internal '\0's, if any, are preserved). For comparison:
//   `span("hi")`                  => `span<const char, 3>({'h', 'i', '\0'})`
//   `span(std::string_view("hi")) => `span<const char>({'h', 'i'})`
//   `span_from_cstring("hi")`     => `span<const char, 2>({'h', 'i'})`
//
// (Not in `std::`; useful when reading and writing character subsequences in
// larger files.)
template <typename CharT, size_t Extent>
constexpr auto span_from_cstring(const CharT (&str LIFETIME_BOUND)[Extent])
    ENABLE_IF_ATTR(str[Extent - 1u] == CharT{0},
                   "requires string literal as input") {
  return span(str).template first<Extent - 1>();
}

// Converts a `const CharT[]` literal to a `span<const CharT>`, preserving the
// trailing '\0'.
//
// (Not in `std::`; identical to constructor behavior, but more explicit.)
template <typename CharT, size_t Extent>
constexpr auto span_with_nul_from_cstring(
    const CharT (&str LIFETIME_BOUND)[Extent])
    ENABLE_IF_ATTR(str[Extent - 1u] == CharT{0},
                   "requires string literal as input") {
  return span(str);
}

// Converts a `basic_cstring_view` instance to a `span<const CharT>`, preserving
// the trailing '\0'.
//
// (Not in `std::`; explicitly includes the trailing nul, which would be omitted
// by calling the range constructor.)
template <typename CharT>
constexpr auto span_with_nul_from_cstring_view(basic_cstring_view<CharT> str) {
  // SAFETY: It is safe to read the guaranteed null-terminator in `str`.
  return UNSAFE_BUFFERS(span(str.data(), str.size() + 1));
}

// Like `span_from_cstring()`, but returns a byte span.
//
// (Not in `std::`.)
template <typename CharT, size_t Extent>
constexpr auto byte_span_from_cstring(const CharT (&str LIFETIME_BOUND)[Extent])
    ENABLE_IF_ATTR(str[Extent - 1u] == CharT{0},
                   "requires string literal as input") {
  // Cannot call `span_from_cstring()` here, since the array contents do not
  // carry through the function call, so the `ENABLE_IF_ATTR` will not be
  // satisfied.
  return as_bytes(span(str).template first<Extent - 1>());
}

// Like `span_with_nul_from_cstring()`, but returns a byte span.
//
// (Not in `std::`.)
template <typename CharT, size_t Extent>
constexpr auto byte_span_with_nul_from_cstring(
    const CharT (&str LIFETIME_BOUND)[Extent])
    ENABLE_IF_ATTR(str[Extent - 1u] == CharT{0},
                   "requires string literal as input") {
  // Cannot call `span_with_nul_from_cstring()` here, since the array contents
  // do not carry through the function call, so the `ENABLE_IF_ATTR` will not be
  // satisfied.
  return as_bytes(span(str));
}

// Like `span_with_nul_from_cstring_view()`, but returns a byte span.
//
// (Not in `std::`.)
template <typename CharT>
constexpr auto byte_span_with_nul_from_cstring_view(
    basic_cstring_view<CharT> str) {
  return as_bytes(span_with_nul_from_cstring_view(str));
}

// Converts an object which can already explicitly convert to some kind of span
// directly into a byte span.
//
// (Not in `std::`.)
template <int&... ExplicitArgumentBarrier, typename T>
  requires(internal::ByteSpanConstructibleFrom<const T&>)
constexpr auto as_byte_span(const T& t LIFETIME_BOUND) {
  return as_bytes(span(t));
}
template <int&... ExplicitArgumentBarrier, typename T>
  requires(internal::ByteSpanConstructibleFromNonUnique<const T&>)
constexpr auto as_byte_span(allow_nonunique_obj_t, const T& t LIFETIME_BOUND) {
  return as_bytes(allow_nonunique_obj, span(t));
}
template <int&... ExplicitArgumentBarrier, typename T>
  requires(internal::ByteSpanConstructibleFrom<const T&> &&
           std::ranges::borrowed_range<T>)
constexpr auto as_byte_span(const T& t) {
  return as_bytes(span(t));
}
template <int&... ExplicitArgumentBarrier, typename T>
  requires(internal::ByteSpanConstructibleFromNonUnique<const T&> &&
           std::ranges::borrowed_range<T>)
constexpr auto as_byte_span(allow_nonunique_obj_t, const T& t) {
  return as_bytes(allow_nonunique_obj, span(t));
}
// Array arguments require dedicated specializations because if only the
// generalized functions are available, the compiler cannot deduce the template
// parameter.
template <int&... ExplicitArgumentBarrier, typename ElementType, size_t Extent>
  requires(internal::CanSafelyConvertToByteSpan<ElementType>)
constexpr auto as_byte_span(const ElementType (&arr LIFETIME_BOUND)[Extent]) {
  return as_bytes(span<const ElementType, Extent>(arr));
}
template <int&... ExplicitArgumentBarrier, typename ElementType, size_t Extent>
  requires(internal::CanSafelyConvertNonUniqueToByteSpan<ElementType>)
constexpr auto as_byte_span(allow_nonunique_obj_t,
                            const ElementType (&arr LIFETIME_BOUND)[Extent]) {
  return as_bytes(allow_nonunique_obj, span<const ElementType, Extent>(arr));
}
template <int&... ExplicitArgumentBarrier, typename T>
  requires(internal::ByteSpanConstructibleFrom<T &&> &&
           !std::is_const_v<internal::ElementTypeOfSpanConstructedFrom<T>>)
// NOTE: `t` is not marked as lifetimebound because the "non-const
// `element_type`" requirement above will in turn require `T` to be a borrowed
// range.
constexpr auto as_writable_byte_span(T&& t) {
  return as_writable_bytes(span(t));
}
template <int&... ExplicitArgumentBarrier, typename T>
  requires(internal::ByteSpanConstructibleFromNonUnique<T &&> &&
           !std::is_const_v<internal::ElementTypeOfSpanConstructedFrom<T>>)
constexpr auto as_writable_byte_span(allow_nonunique_obj_t, T&& t) {
  return as_writable_bytes(allow_nonunique_obj, span(t));
}
template <int&... ExplicitArgumentBarrier, typename ElementType, size_t Extent>
  requires(internal::CanSafelyConvertToByteSpan<ElementType> &&
           !std::is_const_v<ElementType>)
constexpr auto as_writable_byte_span(
    ElementType (&arr LIFETIME_BOUND)[Extent]) {
  return as_writable_bytes(span<ElementType, Extent>(arr));
}
template <int&... ExplicitArgumentBarrier, typename ElementType, size_t Extent>
  requires(internal::CanSafelyConvertNonUniqueToByteSpan<ElementType> &&
           !std::is_const_v<ElementType>)
constexpr auto as_writable_byte_span(
    allow_nonunique_obj_t,
    ElementType (&arr LIFETIME_BOUND)[Extent]) {
  return as_writable_bytes(allow_nonunique_obj, span<ElementType, Extent>(arr));
}

}  // namespace base

#endif  // BASE_CONTAINERS_SPAN_H_

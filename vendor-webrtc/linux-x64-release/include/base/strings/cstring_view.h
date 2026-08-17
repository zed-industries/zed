// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STRINGS_CSTRING_VIEW_H_
#define BASE_STRINGS_CSTRING_VIEW_H_

#include <algorithm>
#include <concepts>
#include <cstddef>
#include <ostream>
#include <string>
#include <string_view>

#include "base/check.h"
#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/containers/checked_iterators.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/numerics/safe_conversions.h"
#include "build/build_config.h"

namespace base {

// A CString is a NUL-terminated character array, which is the C programming
// language representation of a string. This class (and its aliases below)
// provides a non-owning and bounds-safe view of a CString, and can replace all
// use of native pointers (such as `const char*`) for this purpose in C++ code.
//
// The basic_cstring_view class is followed by aliases for the various char
// types:
// * cstring_view provides a view of a `const char*`.
// * u16cstring_view provides a view of a `const char16_t*`.
// * u32cstring_view provides a view of a `const char32_t*`.
// * wcstring_view provides a view of a `const wchar_t*`.
template <class Char>
class basic_cstring_view final {
  static_assert(!std::is_const_v<Char>);
  static_assert(!std::is_reference_v<Char>);

 public:
  using value_type = Char;
  using pointer = Char*;
  using const_pointer = const Char*;
  using reference = Char&;
  using const_reference = const Char&;
  using iterator = CheckedContiguousIterator<const Char>;
  using const_iterator = CheckedContiguousIterator<const Char>;
  using reverse_iterator = std::reverse_iterator<iterator>;
  using const_reverse_iterator = std::reverse_iterator<iterator>;
  using size_type = size_t;
  using difference_type = ptrdiff_t;

  // The `npos` constant represents a non-existent position in the cstring view.
  constexpr static auto npos = static_cast<size_t>(-1);

  // Constructs an empty cstring view, which points to an empty string with a
  // terminating NUL.
  constexpr basic_cstring_view() noexcept : ptr_(kEmpty), len_(0u) {}

  // cstring views are trivially copyable, moveable, and destructible.

  // Constructs a cstring view that points at the contents of a string literal.
  //
  // Example:
  // ```
  // const char kLiteral[] = "hello world";
  // auto s = base::cstring_view(kLiteral);
  // CHECK(s == "hello world");
  // auto s2 = base::cstring_view("this works too");
  // CHECK(s == "this works too");
  // ```
  //
  // The string will end at the first NUL character in the given array.
  //
  // Example:
  // ```
  // auto s = base::cstring_view("hello\0world");
  // CHECK(s == "hello");
  // ```
  template <int&..., size_t M>
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr basic_cstring_view(const Char (&lit LIFETIME_BOUND)[M]) noexcept
      ENABLE_IF_ATTR(lit[M - 1u] == Char{0}, "requires string literal as input")
      : ptr_(lit), len_(std::char_traits<Char>::length(lit)) {
    // For non-clang compilers. On clang, the function is not even callable
    // without this being known to pass at compile time.
    //
    // SAFETY: lit is an array of size M, so M-1 is in bounds.
    DCHECK_EQ(UNSAFE_BUFFERS(lit[M - 1u]), Char{0});
  }

  // Constructs a cstring view from a std::string (or other std::basic_string
  // type). The string parameter must outlive the cstring view, including that
  // it must not be moved-from or destroyed.
  //
  // This conversion is implicit, which matches the conversion from std::string
  // to std::string_view (through string's `operator string_view()`).
  //
  // # Interaction with SSO
  // std::string stores its contents inline when they fit (which is an
  // implementation defined length), instead of in a heap-allocated buffer. This
  // is referred to as the Small String Optimization. This means that moving or
  // destring a std::string will invalidate a cstring view and leave it with
  // dangling pointers. This differs from the behaviour of std::vector and span,
  // since pointers into a std::vector remain valid after moving the std::vector
  // and destroying the original.
  //
  // # Preventing implicit temporaries
  // Because std::string can be implicitly constructed, the string constructor
  // may unintentionally be called with a temporary `std::string` when called
  // with values that convert to `std::string`. We prevent this templating this
  // constructor and requiring the incoming type to actually be a `std::string`
  // (or other `std::basic_string`). This also improves compiler errors,
  // compared to deleting a string&& overload, when passed an array that does
  // not match the `ENABLE_IF_ATTR` constructor condition by not sending it to a
  // deleted overload receiving `std::string`.
  template <std::same_as<std::basic_string<Char>> String>
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr basic_cstring_view(const String& s LIFETIME_BOUND) noexcept
      : ptr_(s.c_str()), len_(s.size()) {}

  // Unsafe construction from a NUL-terminated cstring, primarily for use with C
  // APIs. Prefer to construct cstring view from a string literal, std::string,
  // or another cstring view.
  //
  // # Safety
  // The `ptr` must point to a NUL-terminated string or Undefined Behaviour will
  // result.
  //
  // # Implementation note
  // We use a `String&&` template to ensure the input is a pointer and not an
  // array that decayed to a pointer. This ensures the ctor will not act as a
  // fallback for the string literal ctor when the enable_if condition fails.
  template <class String>
    requires(std::same_as<std::remove_cvref_t<String>, Char*> ||
             std::same_as<std::remove_cvref_t<String>, const Char*>)
  UNSAFE_BUFFER_USAGE explicit constexpr basic_cstring_view(
      String&& ptr LIFETIME_BOUND) noexcept
      : ptr_(ptr), len_(std::char_traits<Char>::length(ptr)) {}

  // Returns a pointer to the NUL-terminated string, for passing to C-style APIs
  // that require `const char*` (or whatever the `Char` type is).
  //
  // This is never null.
  PURE_FUNCTION constexpr const Char* c_str() const noexcept { return ptr_; }

  // Returns a pointer to underlying buffer. To get a string pointer, use
  // `c_str()`.
  //
  // Pair with `size()` to construct a bounded non-NUL-terminated view, such as
  // by `base::span`. This is never null.
  PURE_FUNCTION constexpr const Char* data() const noexcept { return ptr_; }

  // Returns the number of characters in the string, not including the
  // terminating NUL.
  PURE_FUNCTION constexpr size_t size() const noexcept { return len_; }
  // An alias for `size()`, returning the number of characters in the string.
  PURE_FUNCTION constexpr size_t length() const noexcept { return len_; }

  // Returns whether the cstring view is for an empty string. When empty, it is
  // pointing to a cstring that contains only a NUL character.
  PURE_FUNCTION constexpr bool empty() const noexcept { return len_ == 0u; }

  // Returns the maximum number of characters that can be represented inside the
  // cstring view for character type `Char`.
  //
  // This is the number of `Char` objects that can fit inside an addressable
  // byte array. Since the number of bytes allowed is fixed, the number returned
  // is smaller when the `Char` is a larger type.
  PURE_FUNCTION constexpr size_t max_size() const noexcept {
    return static_cast<size_t>(-1) / sizeof(Char);
  }

  // Returns the number of bytes in the string, not including the terminating
  // NUL. To include the NUL, add `sizeof(Char)` where `Char` is the character
  // type of the cstring view (accessible as the `value_type` alias).
  PURE_FUNCTION constexpr size_t size_bytes() const noexcept {
    return len_ * sizeof(Char);
  }

  // Produces an iterator over the cstring view, excluding the terminating NUL.
  PURE_FUNCTION constexpr iterator begin() const noexcept {
    // SAFETY: `ptr_ + len_` for a cstring view always gives a pointer in
    // the same allocation as `ptr_` based on the precondition of
    // the type.
    return UNSAFE_BUFFERS(iterator(ptr_, ptr_ + len_));
  }
  // Produces an iterator over the cstring view, excluding the terminating NUL.
  PURE_FUNCTION constexpr iterator end() const noexcept {
    // SAFETY: `ptr_ + len_` for a cstring view always gives a pointer in
    // the same allocation as `ptr_` based on the precondition of
    // the type.
    return UNSAFE_BUFFERS(iterator(ptr_, ptr_ + len_, ptr_ + len_));
  }
  // Produces an iterator over the cstring view, excluding the terminating NUL.
  PURE_FUNCTION constexpr const_iterator cbegin() const noexcept {
    return begin();
  }
  // Produces an iterator over the cstring view, excluding the terminating NUL.
  PURE_FUNCTION constexpr const_iterator cend() const noexcept { return end(); }

  // Produces a reverse iterator over the cstring view, excluding the
  // terminating NUL.
  PURE_FUNCTION constexpr reverse_iterator rbegin() const noexcept {
    return std::reverse_iterator(end());
  }
  // Produces a reverse iterator over the cstring view, excluding the
  // terminating NUL.
  PURE_FUNCTION constexpr reverse_iterator rend() const noexcept {
    return std::reverse_iterator(begin());
  }
  // Produces a reverse iterator over the cstring view, excluding the
  // terminating NUL.
  PURE_FUNCTION constexpr const_reverse_iterator rcbegin() const noexcept {
    return std::reverse_iterator(cend());
  }
  // Produces a reverse iterator over the cstring view, excluding the
  // terminating NUL.
  PURE_FUNCTION constexpr const_reverse_iterator rcend() const noexcept {
    return std::reverse_iterator(cbegin());
  }

  // Returns the character at offset `idx`.
  //
  // This can be used to access any character in the ctring, as well as the NUL
  // terminator.
  //
  // # Checks
  // The function CHECKs that the `idx` is inside the cstring (including at its
  // NUL terminator) and will terminate otherwise.
  PURE_FUNCTION constexpr const Char& operator[](size_t idx) const noexcept {
    CHECK_LE(idx, len_);
    // SAFETY: `ptr_` points `len_` many elements plus a NUL terminator, and
    // `idx <= len_`, so `idx` is in range for `ptr_`.
    return UNSAFE_BUFFERS(ptr_[idx]);
  }

  // A named function that performs the same as `operator[]`.
  PURE_FUNCTION constexpr const Char& at(size_t idx) const noexcept {
    return (*this)[idx];
  }

  // Returns the first character in the cstring view.
  //
  // # Checks
  // The function CHECKs that the string is non-empty, and will terminate
  // otherwise.
  PURE_FUNCTION constexpr const Char& front() const noexcept {
    CHECK(len_);
    // Since `len_ > 0`, 0 is a valid offset into the string contents.
    return UNSAFE_BUFFERS(ptr_[0u]);
  }

  // Returns the last (non-NUL) character in the cstring view.
  //
  // # Checks
  // The function CHECKs that the string is non-empty, and will terminate
  // otherwise.
  PURE_FUNCTION constexpr const Char& back() const noexcept {
    CHECK(len_);
    // Since `len_ > 0`, `len - 1` will not underflow. There are `len_` many
    // chars in the string before a NUL, so `len_ - 1` is in range of the string
    // contents.
    return UNSAFE_BUFFERS(ptr_[len_ - 1u]);
  }

  // Modifies the cstring view in place, moving the front ahead by `n`
  // characters.
  //
  // # Checks
  // The function CHECKs that `n <= size()`, and will terminate otherwise.
  constexpr void remove_prefix(size_t n) noexcept {
    CHECK_LE(n, len_);
    // SAFETY: Since `n <= len_`, the pointer at offset `n` is inside the string
    // (or at the terminating NUL) and the `len_ - n` value will not underflow.
    // Thus the resulting pointer is still a NUL- terminated string of length
    // `len_ - n`.
    ptr_ = UNSAFE_BUFFERS(ptr_ + n);
    len_ = len_ - n;
  }

  // No `remove_suffix()` method exists as it would remove the terminating NUL
  // character. Convert to a `std::string_view` (either by construction or with
  // a `substr(0u)` call) to construct arbitrary substrings that are not
  // NUL-terminated.
  void remove_suffix(size_t n) = delete;

  // Modifies the cstring view in place, swapping its contents with another view
  // of the same type.
  constexpr void swap(basic_cstring_view& other) noexcept {
    std::swap(ptr_, other.ptr_);
    std::swap(len_, other.len_);
  }

  // Returns a string view of the subrange starting as `pos` and including
  // `count` characters. If `count` is not specified, or exceeds the length of
  // the string after `pos`, the subrange returned will include all characters
  // up to the terminating NUL.
  //
  // # Checks
  // The function CHECKs that `pos` is in range for the string (or at the
  // terminating NULL), and will terminate otherwise.
  PURE_FUNCTION constexpr std::basic_string_view<Char> substr(
      size_t pos,
      size_t count = npos) const noexcept {
    // Ensure `ptr_ + pos` is valid. and `len_ - pos` does not underflow.
    CHECK_LE(pos, len_);
    // SAFETY: We require that:
    // * `ptr_ + pos` is a pointer in the string.
    // * `pos + count <= len_` so that resulting substring's end is in range.
    //
    // The first follows directly from the CHECK above that `pos <= len_`. The
    // second follows from clamping `count` to at most `len_ - pos`.
    return UNSAFE_BUFFERS(
        std::basic_string_view<Char>(ptr_ + pos, std::min(count, len_ - pos)));
  }

  // Returns whether the cstring view starts with the given `prefix`. Will
  // always return false if `prefix` is larger than the current cstring view.
  constexpr bool starts_with(
      std::basic_string_view<Char> prefix) const noexcept {
    return std::basic_string_view<Char>(*this).starts_with(prefix);
  }

  // Returns whether the cstring view starts with the given `character`.
  constexpr bool starts_with(Char character) const noexcept {
    return std::basic_string_view<Char>(*this).starts_with(character);
  }

  // Returns whether the cstring view ends with the given `suffix`. Will
  // always return false if `suffix` is larger than the current cstring view.
  constexpr bool ends_with(std::basic_string_view<Char> suffix) const noexcept {
    return std::basic_string_view<Char>(*this).ends_with(suffix);
  }

  // Returns whether the cstring view starts with the given `character`.
  constexpr bool ends_with(Char character) const noexcept {
    return std::basic_string_view<Char>(*this).ends_with(character);
  }

  // Returns the first position in the cstring view at which `search` is found,
  // starting from the offset `pos`. If `pos` is not specified, the entire
  // cstring view is searched. Returns `npos` if `search` is not found or if
  // `pos` is out of range.
  constexpr size_t find(std::basic_string_view<Char> search,
                        size_t pos = 0u) const noexcept {
    return std::basic_string_view<Char>(*this).find(search, pos);
  }
  constexpr size_t find(Char search, size_t pos = 0u) const noexcept {
    return std::basic_string_view<Char>(*this).find(search, pos);
  }

  // Returns the last position in the cstring view at which `search` is found,
  // starting from the offset `pos`. If `pos` is not specified or is out of
  // range, the entire cstring view is searched. Returns `npos` if `search` is
  // not found.
  constexpr size_t rfind(std::basic_string_view<Char> search,
                         size_t pos = npos) const noexcept {
    return std::basic_string_view<Char>(*this).rfind(search, pos);
  }
  constexpr size_t rfind(Char search, size_t pos = npos) const noexcept {
    return std::basic_string_view<Char>(*this).rfind(search, pos);
  }

  // Returns the first position in the cstring view at any character in the
  // `search` is found, starting from the offset `pos`. If `pos` is not
  // specified, the entire cstring view is searched. Returns `npos` if `search`
  // is not found or if `pos` is out of range.
  constexpr size_t find_first_of(std::basic_string_view<Char> search,
                                 size_t pos = 0u) const noexcept {
    return std::basic_string_view<Char>(*this).find_first_of(search, pos);
  }
  constexpr size_t find_first_of(Char search, size_t pos = 0u) const noexcept {
    return std::basic_string_view<Char>(*this).find_first_of(search, pos);
  }

  // Returns the last position in the cstring view at any character in the
  // `search` is found, starting from the offset `pos`. If `pos` is not
  // specified or is out of range, the entire cstring view is searched. Returns
  // `npos` if `search` is not found.
  constexpr size_t find_last_of(std::basic_string_view<Char> search,
                                size_t pos = npos) const noexcept {
    return std::basic_string_view<Char>(*this).find_last_of(search, pos);
  }
  constexpr size_t find_last_of(Char search, size_t pos = npos) const noexcept {
    return std::basic_string_view<Char>(*this).find_last_of(search, pos);
  }

  // Returns the first position in the cstring view that is not equal to any
  // character in the `search`, starting from the offset `pos`. If `pos` is not
  // specified, the entire cstring view is searched. Returns `npos` if every
  // character is part of `search` or if `pos` is out of range.
  constexpr size_t find_first_not_of(std::basic_string_view<Char> search,
                                     size_t pos = 0u) const noexcept {
    return std::basic_string_view<Char>(*this).find_first_not_of(search, pos);
  }
  constexpr size_t find_first_not_of(Char search,
                                     size_t pos = 0u) const noexcept {
    return std::basic_string_view<Char>(*this).find_first_not_of(search, pos);
  }

  // Returns the last position in the cstring view that is not equal to any
  // character in the `search`, starting from the offset `pos`. If `pos` is not
  // specified or is out of range, the entire cstring view is searched.  Returns
  // `npos` if every character is part of `search`.
  constexpr size_t find_last_not_of(std::basic_string_view<Char> search,
                                    size_t pos = npos) const noexcept {
    return std::basic_string_view<Char>(*this).find_last_not_of(search, pos);
  }
  constexpr size_t find_last_not_of(Char search,
                                    size_t pos = npos) const noexcept {
    return std::basic_string_view<Char>(*this).find_last_not_of(search, pos);
  }

  // Compare two cstring views for equality, comparing the string contents.
  friend constexpr bool operator==(basic_cstring_view l, basic_cstring_view r) {
    return std::ranges::equal(l, r);
  }

  // Return an ordering between two cstring views, comparing the string
  // contents.
  //
  // cstring views are weakly ordered, since string views pointing into
  // different strings can compare as equal.
  friend constexpr std::weak_ordering operator<=>(basic_cstring_view l,
                                                  basic_cstring_view r) {
    return std::lexicographical_compare_three_way(l.begin(), l.end(), r.begin(),
                                                  r.end());
  }

  // Implicitly converts from cstring_view to a non-NUL-terminated
  // std::string_view. The std::string_view type implicitly constructs from
  // `const char*` and cstring view is meant to replace the latter, so this acts
  // like an implicit constructor on `std::string_view` for cstring views.
  //
  // This operator also avoids a requirement on having overloads for both
  // std::string_view and cstring_view. Such overloads are ambiguous because
  // both can construct from a character array.
  //
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr operator std::basic_string_view<Char>() const noexcept {
    // SAFETY: The cstring view provides that `ptr_ + len_` to be valid.
    return UNSAFE_BUFFERS(std::basic_string_view<Char>(ptr_, len_));
  }

  // Converts from cstring_view to std::string. This allocates a new string
  // backing and copies into it.
  //
  // The std::string type implicitly constructs from `const char*` however it
  // does not implicitly construct from std::string_view. This type sits between
  // these two, and opts towards making heap allocations explicit by requiring
  // an explicit conversion.
  constexpr explicit operator std::basic_string<Char>() const noexcept {
    // SAFETY: The cstring view provides that `ptr_ + len_` to be valid.
    return UNSAFE_BUFFERS(std::basic_string<Char>(ptr_, len_));
  }

  // Concatenate a std::string with a cstring_view to produce another
  // std::string.
  //
  // These act like overloads on `std::string` that work for concatenating
  // `std::string` and `const char*`.
  //
  // The rvalue overloads allow `std::string` to reuse existing capacity, by
  // calling through to the rvalue overloads on `std::string`.
  template <class Traits, class Alloc>
  friend constexpr std::basic_string<Char, Traits, Alloc> operator+(
      basic_cstring_view lhs,
      const std::basic_string<Char, Traits, Alloc>& rhs) {
    return lhs.c_str() + rhs;
  }
  template <class Traits, class Alloc>
  friend constexpr std::basic_string<Char, Traits, Alloc> operator+(
      basic_cstring_view lhs,
      std::basic_string<Char, Traits, Alloc>&& rhs) {
    return lhs.c_str() + std::move(rhs);
  }
  template <class Traits, class Alloc>
  friend constexpr std::basic_string<Char, Traits, Alloc> operator+(
      const std::basic_string<Char, Traits, Alloc>& lhs,
      basic_cstring_view rhs) {
    return lhs + rhs.c_str();
  }
  template <class Traits, class Alloc>
  friend constexpr std::basic_string<Char, Traits, Alloc> operator+(
      std::basic_string<Char, Traits, Alloc>&& lhs,
      basic_cstring_view rhs) {
    return std::move(lhs) + rhs.c_str();
  }

 private:
  // An empty string literal for the `Char` type.
  static constexpr Char kEmpty[] = {Char{0}};

  // An always-valid pointer (never null) to a NUL-terminated string.
  //
  // RAW_PTR_EXCLUSION: cstring_view is typically used on the stack as a local
  // variable/function parameter, so no raw_ptr is used here.
  RAW_PTR_EXCLUSION const Char* ptr_;
  // The number of characters between `ptr_` and the NUL terminator.
  //
  // SAFETY: `ptr_ + len_` is always valid since `len_` must not exceed the
  // number of characters in the allocation, or it would no longer indicate the
  // position of the NUL terminator in the string allocation.
  size_t len_;
};

// cstring_view provides a view of a NUL-terminated string. It is a replacement
// for all use of `const char*`, in order to provide bounds checks and prevent
// unsafe pointer usage (otherwise prevented by `-Wunsafe-buffer-usage`).
//
// See basic_cstring_view for more.
using cstring_view = basic_cstring_view<char>;

// u16cstring_view provides a view of a NUL-terminated string. It is a
// replacement for all use of `const char16_t*`, in order to provide bounds
// checks and prevent unsafe pointer usage (otherwise prevented by
// `-Wunsafe-buffer-usage`).
//
// See basic_cstring_view for more.
using u16cstring_view = basic_cstring_view<char16_t>;

// u32cstring_view provides a view of a NUL-terminated string. It is a
// replacement for all use of `const char32_t*`, in order to provide bounds
// checks and prevent unsafe pointer usage (otherwise prevented by
// `-Wunsafe-buffer-usage`).
//
// See basic_cstring_view for more.
using u32cstring_view = basic_cstring_view<char32_t>;

#if BUILDFLAG(IS_WIN)
// wcstring_view provides a view of a NUL-terminated string. It is a
// replacement for all use of `const wchar_t*`, in order to provide bounds
// checks and prevent unsafe pointer usage (otherwise prevented by
// `-Wunsafe-buffer-usage`).
//
// See basic_cstring_view for more.
using wcstring_view = basic_cstring_view<wchar_t>;
#endif

// Writes the contents of the cstring view to the stream.
template <class Char, class Traits>
std::basic_ostream<Char, Traits>& operator<<(
    std::basic_ostream<Char, Traits>& os,
    basic_cstring_view<Char> view) {
  return os << std::basic_string_view<Char>(view);
}

// Explicitly define PrintTo to avoid gtest printing these as containers
// rather than strings.
inline void PrintTo(cstring_view view, std::ostream* os) {
  *os << view;
}

}  // namespace base

template <class Char>
struct std::hash<base::basic_cstring_view<Char>> {
  size_t operator()(const base::basic_cstring_view<Char>& t) const noexcept {
    return std::hash<std::basic_string_view<Char>>()(t);
  }
};

template <class Char>
inline constexpr bool
    std::ranges::enable_borrowed_range<base::basic_cstring_view<Char>> = true;

template <class Char>
inline constexpr bool std::ranges::enable_view<base::basic_cstring_view<Char>> =
    true;

#endif  // BASE_STRINGS_CSTRING_VIEW_H_

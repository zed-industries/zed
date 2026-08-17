//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBCXX_TEST_STD_UTILITIES_TUPLE_CNSTR_TYPES_H
#define LIBCXX_TEST_STD_UTILITIES_TUPLE_CNSTR_TYPES_H

#include "test_allocator.h"
#include <type_traits>
#include <tuple>

// Types that can be used to test copy/move operations

struct MutableCopy {
  int val;
  bool alloc_constructed{false};

  constexpr MutableCopy() = default;
  constexpr MutableCopy(int _val) : val(_val) {}
  constexpr MutableCopy(MutableCopy&) = default;
  constexpr MutableCopy(const MutableCopy&) = delete;

  constexpr MutableCopy(std::allocator_arg_t, const test_allocator<int>&, MutableCopy& o)
      : val(o.val), alloc_constructed(true) {}
};

template <>
struct std::uses_allocator<MutableCopy, test_allocator<int>> : std::true_type {};

struct ConstCopy {
  int val;
  bool alloc_constructed{false};

  constexpr ConstCopy() = default;
  constexpr ConstCopy(int _val) : val(_val) {}
  constexpr ConstCopy(const ConstCopy&) = default;
  constexpr ConstCopy(ConstCopy&) = delete;

  constexpr ConstCopy(std::allocator_arg_t, const test_allocator<int>&, const ConstCopy& o)
      : val(o.val), alloc_constructed(true) {}
};

template <>
struct std::uses_allocator<ConstCopy, test_allocator<int>> : std::true_type {};

struct MutableMove {
  int val;
  bool alloc_constructed{false};

  constexpr MutableMove() = default;
  constexpr MutableMove(int _val) : val(_val) {}
  constexpr MutableMove(MutableMove&&) = default;
  constexpr MutableMove(const MutableMove&&) = delete;

  constexpr MutableMove(std::allocator_arg_t, const test_allocator<int>&, MutableMove&& o)
      : val(o.val), alloc_constructed(true) {}
};

template <>
struct std::uses_allocator<MutableMove, test_allocator<int>> : std::true_type {};

struct ConstMove {
  int val;
  bool alloc_constructed{false};

  constexpr ConstMove() = default;
  constexpr ConstMove(int _val) : val(_val) {}
  constexpr ConstMove(const ConstMove&& o) : val(o.val) {}
  constexpr ConstMove(ConstMove&&) = delete;

  constexpr ConstMove(std::allocator_arg_t, const test_allocator<int>&, const ConstMove&& o)
      : val(o.val), alloc_constructed(true) {}
};

template <>
struct std::uses_allocator<ConstMove, test_allocator<int>> : std::true_type {};

template <class T>
struct ConvertibleFrom {
  T v;
  bool alloc_constructed{false};

  constexpr ConvertibleFrom() = default;
  constexpr ConvertibleFrom(T& _v)
    requires(std::is_constructible_v<T, T&>)
  : v(_v) {}
  constexpr ConvertibleFrom(const T& _v)
    requires(std::is_constructible_v<T, const T&> && !std::is_const_v<T>)
  : v(_v) {}
  constexpr ConvertibleFrom(T&& _v)
    requires(std::is_constructible_v<T, T &&>)
  : v(std::move(_v)) {}
  constexpr ConvertibleFrom(const T&& _v)
    requires(std::is_constructible_v<T, const T &&> && !std::is_const_v<T>)
  : v(std::move(_v)) {}

  template <class U>
    requires std::is_constructible_v<ConvertibleFrom, U&&>
  constexpr ConvertibleFrom(std::allocator_arg_t, const test_allocator<int>&, U&& _u)
      : ConvertibleFrom{std::forward<U>(_u)} {
    alloc_constructed = true;
  }
};

template <class T>
struct std::uses_allocator<ConvertibleFrom<T>, test_allocator<int>> : std::true_type {};

template <class T>
struct ExplicitConstructibleFrom {
  T v;
  bool alloc_constructed{false};

  constexpr explicit ExplicitConstructibleFrom() = default;
  constexpr explicit ExplicitConstructibleFrom(T& _v)
    requires(std::is_constructible_v<T, T&>)
  : v(_v) {}
  constexpr explicit ExplicitConstructibleFrom(const T& _v)
    requires(std::is_constructible_v<T, const T&> && !std::is_const_v<T>)
  : v(_v) {}
  constexpr explicit ExplicitConstructibleFrom(T&& _v)
    requires(std::is_constructible_v<T, T &&>)
  : v(std::move(_v)) {}
  constexpr explicit ExplicitConstructibleFrom(const T&& _v)
    requires(std::is_constructible_v<T, const T &&> && !std::is_const_v<T>)
  : v(std::move(_v)) {}

  template <class U>
    requires std::is_constructible_v<ExplicitConstructibleFrom, U&&>
  constexpr ExplicitConstructibleFrom(std::allocator_arg_t, const test_allocator<int>&, U&& _u)
      : ExplicitConstructibleFrom{std::forward<U>(_u)} {
    alloc_constructed = true;
  }
};

template <class T>
struct std::uses_allocator<ExplicitConstructibleFrom<T>, test_allocator<int>> : std::true_type {};

struct TracedCopyMove {
  int nonConstCopy = 0;
  int constCopy = 0;
  int nonConstMove = 0;
  int constMove = 0;
  bool alloc_constructed = false;

  constexpr TracedCopyMove() = default;
  constexpr TracedCopyMove(const TracedCopyMove& other)
      : nonConstCopy(other.nonConstCopy), constCopy(other.constCopy + 1), nonConstMove(other.nonConstMove),
        constMove(other.constMove) {}
  constexpr TracedCopyMove(TracedCopyMove& other)
      : nonConstCopy(other.nonConstCopy + 1), constCopy(other.constCopy), nonConstMove(other.nonConstMove),
        constMove(other.constMove) {}

  constexpr TracedCopyMove(TracedCopyMove&& other)
      : nonConstCopy(other.nonConstCopy), constCopy(other.constCopy), nonConstMove(other.nonConstMove + 1),
        constMove(other.constMove) {}

  constexpr TracedCopyMove(const TracedCopyMove&& other)
      : nonConstCopy(other.nonConstCopy), constCopy(other.constCopy), nonConstMove(other.nonConstMove),
        constMove(other.constMove + 1) {}

  template <class U>
    requires std::is_constructible_v<TracedCopyMove, U&&>
  constexpr TracedCopyMove(std::allocator_arg_t, const test_allocator<int>&, U&& _u)
      : TracedCopyMove{std::forward<U>(_u)} {
    alloc_constructed = true;
  }
};

template <>
struct std::uses_allocator<TracedCopyMove, test_allocator<int>> : std::true_type {};

// If the constructor tuple(tuple<UTypes...>&) is not available,
// the fallback call to `tuple(const tuple&) = default;` or any other
// constructor that takes const ref would increment the constCopy.
inline constexpr bool nonConstCopyCtrCalled(const TracedCopyMove& obj) {
  return obj.nonConstCopy == 1 && obj.constCopy == 0 && obj.constMove == 0 && obj.nonConstMove == 0;
}

// If the constructor tuple(const tuple<UTypes...>&&) is not available,
// the fallback call to `tuple(const tuple&) = default;` or any other
// constructor that takes const ref would increment the constCopy.
inline constexpr bool constMoveCtrCalled(const TracedCopyMove& obj) {
  return obj.nonConstMove == 0 && obj.constMove == 1 && obj.constCopy == 0 && obj.nonConstCopy == 0;
}

struct NoConstructorFromInt {};

struct CvtFromTupleRef : TracedCopyMove {
  constexpr CvtFromTupleRef() = default;
  constexpr CvtFromTupleRef(std::tuple<CvtFromTupleRef>& other)
      : TracedCopyMove(static_cast<TracedCopyMove&>(std::get<0>(other))) {}
};

struct ExplicitCtrFromTupleRef : TracedCopyMove {
  constexpr explicit ExplicitCtrFromTupleRef() = default;
  constexpr explicit ExplicitCtrFromTupleRef(std::tuple<ExplicitCtrFromTupleRef>& other)
      : TracedCopyMove(static_cast<TracedCopyMove&>(std::get<0>(other))) {}
};

struct CvtFromConstTupleRefRef : TracedCopyMove {
  constexpr CvtFromConstTupleRefRef() = default;
  constexpr CvtFromConstTupleRefRef(const std::tuple<CvtFromConstTupleRefRef>&& other)
      : TracedCopyMove(static_cast<const TracedCopyMove&&>(std::get<0>(other))) {}
};

struct ExplicitCtrFromConstTupleRefRef : TracedCopyMove {
  constexpr explicit ExplicitCtrFromConstTupleRefRef() = default;
  constexpr explicit ExplicitCtrFromConstTupleRefRef(std::tuple<const ExplicitCtrFromConstTupleRefRef>&& other)
      : TracedCopyMove(static_cast<const TracedCopyMove&&>(std::get<0>(other))) {}
};

template <class T>
void conversion_test(T);

template <class T, class... Args>
concept ImplicitlyConstructible = requires(Args&&... args) { conversion_test<T>({std::forward<Args>(args)...}); };

struct CopyAssign {
  int val;

  constexpr CopyAssign() = default;
  constexpr CopyAssign(int v) : val(v) {}

  constexpr CopyAssign& operator=(const CopyAssign&) = default;

  constexpr const CopyAssign& operator=(const CopyAssign&) const = delete;
  constexpr CopyAssign& operator=(CopyAssign&&) = delete;
  constexpr const CopyAssign& operator=(CopyAssign&&) const = delete;
};

struct ConstCopyAssign {
  mutable int val;

  constexpr ConstCopyAssign() = default;
  constexpr ConstCopyAssign(int v) : val(v) {}

  constexpr const ConstCopyAssign& operator=(const ConstCopyAssign& other) const {
    val = other.val;
    return *this;
  }

  constexpr ConstCopyAssign& operator=(const ConstCopyAssign&) = delete;
  constexpr ConstCopyAssign& operator=(ConstCopyAssign&&) = delete;
  constexpr const ConstCopyAssign& operator=(ConstCopyAssign&&) const = delete;
};

struct MoveAssign {
  int val;

  constexpr MoveAssign() = default;
  constexpr MoveAssign(int v) : val(v) {}

  constexpr MoveAssign& operator=(MoveAssign&&) = default;

  constexpr MoveAssign& operator=(const MoveAssign&) = delete;
  constexpr const MoveAssign& operator=(const MoveAssign&) const = delete;
  constexpr const MoveAssign& operator=(MoveAssign&&) const = delete;
};

struct ConstMoveAssign {
  mutable int val;

  constexpr ConstMoveAssign() = default;
  constexpr ConstMoveAssign(int v) : val(v) {}

  constexpr const ConstMoveAssign& operator=(ConstMoveAssign&& other) const {
    val = other.val;
    return *this;
  }

  constexpr ConstMoveAssign& operator=(const ConstMoveAssign&) = delete;
  constexpr const ConstMoveAssign& operator=(const ConstMoveAssign&) const = delete;
  constexpr ConstMoveAssign& operator=(ConstMoveAssign&&) = delete;
};

template <class T>
struct AssignableFrom {
  T v;

  constexpr AssignableFrom() = default;

  template <class U>
  constexpr AssignableFrom(U&& u)
    requires std::is_constructible_v<T, U&&>
  : v(std::forward<U>(u)) {}

  constexpr AssignableFrom& operator=(const T& t)
    requires std::is_copy_assignable_v<T>
  {
    v = t;
    return *this;
  }

  constexpr AssignableFrom& operator=(T&& t)
    requires std::is_move_assignable_v<T>
  {
    v = std::move(t);
    return *this;
  }

  constexpr const AssignableFrom& operator=(const T& t) const
    requires std::is_assignable_v<const T&, const T&>
  {
    v = t;
    return *this;
  }

  constexpr const AssignableFrom& operator=(T&& t) const
    requires std::is_assignable_v<const T&, T&&>
  {
    v = std::move(t);
    return *this;
  }
};

struct TracedAssignment {
  int copyAssign = 0;
  mutable int constCopyAssign = 0;
  int moveAssign = 0;
  mutable int constMoveAssign = 0;

  constexpr TracedAssignment() = default;

  constexpr TracedAssignment& operator=(const TracedAssignment&) {
    copyAssign++;
    return *this;
  }
  constexpr const TracedAssignment& operator=(const TracedAssignment&) const {
    constCopyAssign++;
    return *this;
  }
  constexpr TracedAssignment& operator=(TracedAssignment&&) {
    moveAssign++;
    return *this;
  }
  constexpr const TracedAssignment& operator=(TracedAssignment&&) const {
    constMoveAssign++;
    return *this;
  }
};
#endif

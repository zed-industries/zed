/*
 * Copyright (C) 2006, 2007, 2008 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_VECTOR_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_VECTOR_TRAITS_H_

#include <memory>
#include <type_traits>
#include <utility>
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

namespace WTF {

// Vector traits serve two purposes:
// 1. Faster bulk operations: Instead of invoking proper constructors,
//    destructors, copy operators, and move operators on individual elements,
//    the vector implementation will just use `memcpy()` and friends where
//    possible.
// 2. Garbage collection support: HeapVector requires certain traits to be set
//    which is used to acknowledge that semantics are different from regular
//    `WTF::Vector` and `std::vector`.
template <typename T>
struct VectorTraitsBase {
  using TraitType = T;

  // When true, T will be destroyed using `~T`.
  static const bool kNeedsDestruction =
      !std::is_trivially_destructible<T>::value;

  // When true, allows initializing a value for a range with `memset()` instead
  // of invoking constructors.
  static constexpr bool kCanInitializeWithMemset =
      std::is_trivially_default_constructible<T>::value;

  // When true, allows setting a value for a range with `memset(0)` instead of
  // invoking constructors.
  static constexpr bool kCanFillWithMemset =
      std::is_default_constructible<T>::value && (sizeof(T) == sizeof(char));

  // When true, allows comparing T with `memcmp()`. instead of `std::equals()`.
  static constexpr bool kCanCompareWithMemcmp =
      std::is_scalar<T>::value;  // Types without padding.

  // When true, allows moving vector backings with memcpy instead of invoking
  // move operations on every vector slot. There's no move operator being
  // invoked.
  //
  // Garbage collection support: When true, the GC may move vector backings when
  // they are not referred to from the native stack. This prohibits keeping
  // inner pointers to vector elements and backing stores across event loop
  // turns. When elements are moved using copy, their old storage is is just
  // freed. In essence, no constructor/destructor/move operations will be
  // invoked.
  static constexpr bool kCanMoveWithMemcpy =
      std::is_trivially_move_assignable<T>::value;

  // When true, allows copying vector backings with memcpy instead of invoking
  // copy operations on every vector slot. There's no copy operator being
  // invoked.
  static constexpr bool kCanCopyWithMemcpy =
      std::is_trivially_copy_assignable<T>::value;

  // Garbage collection support: Must be true for types used in `HeapVector`.
  // The reason is that GCed vector backings are initialized to zeroed memory.
  // The GC assumes that invoking (a) `T::Trace()`, and (b) `T::~T` are no ops
  // on zeroed memory.
  static constexpr bool kCanClearUnusedSlotsWithMemset =
      std::is_trivially_destructible<T>::value &&
      (!IsTraceable<T>::value || (std::is_trivially_constructible<T>::value &&
                                  std::is_trivially_copyable<T>::value));

  // Garbage collection support: When true, the vector invokes `Trace()` methods
  // concurrently from the non-owning thread.
  static constexpr bool kCanTraceConcurrently = false;
};

template <typename T>
struct VectorTraits : VectorTraitsBase<T> {};

// Classes marked with SimpleVectorTraits will use memmov, memcpy, memcmp
// instead of constructors, copy operators, etc for initialization, move and
// comparison.
template <typename T>
struct SimpleClassVectorTraits : VectorTraitsBase<T> {
  static const bool kCanInitializeWithMemset = true;
  static const bool kCanClearUnusedSlotsWithMemset = true;
  static const bool kCanMoveWithMemcpy = true;
  static const bool kCanCompareWithMemcmp = true;
};

// We know std::unique_ptr and RefPtr are simple enough that initializing to 0
// and moving with memcpy (and then not destructing the original) will totally
// work.
template <typename P>
struct VectorTraits<scoped_refptr<P>>
    : SimpleClassVectorTraits<scoped_refptr<P>> {
  // scoped_refptr cannot be copied using memcpy as the internals (e.g. ref
  // count) depend on properly constructing the object.
  static const bool kCanCopyWithMemcpy = false;
};

template <typename P>
struct VectorTraits<std::unique_ptr<P>>
    : SimpleClassVectorTraits<std::unique_ptr<P>> {
  // std::unique_ptr -> std::unique_ptr has a very particular structure that
  // tricks the normal type traits into thinking that the class is "trivially
  // copyable".
  static const bool kCanCopyWithMemcpy = false;
};
static_assert(VectorTraits<scoped_refptr<int>>::kCanInitializeWithMemset,
              "inefficient RefPtr Vector");
static_assert(VectorTraits<scoped_refptr<int>>::kCanMoveWithMemcpy,
              "inefficient RefPtr Vector");
static_assert(VectorTraits<scoped_refptr<int>>::kCanCompareWithMemcmp,
              "inefficient RefPtr Vector");
static_assert(VectorTraits<std::unique_ptr<int>>::kCanInitializeWithMemset,
              "inefficient std::unique_ptr Vector");
static_assert(VectorTraits<std::unique_ptr<int>>::kCanMoveWithMemcpy,
              "inefficient std::unique_ptr Vector");
static_assert(VectorTraits<std::unique_ptr<int>>::kCanCompareWithMemcmp,
              "inefficient std::unique_ptr Vector");

template <typename First, typename Second>
struct VectorTraits<std::pair<First, Second>> {
  using TraitType = std::pair<First, Second>;

  typedef VectorTraits<First> FirstTraits;
  typedef VectorTraits<Second> SecondTraits;

  static_assert(!IsWeak<First>::value,
                "Weak references are not allowed in Vector");
  static_assert(!IsWeak<Second>::value,
                "Weak references are not allowed in Vector");

  static const bool kNeedsDestruction =
      FirstTraits::kNeedsDestruction || SecondTraits::kNeedsDestruction;
  static const bool kCanInitializeWithMemset =
      FirstTraits::kCanInitializeWithMemset &&
      SecondTraits::kCanInitializeWithMemset;
  static const bool kCanMoveWithMemcpy =
      FirstTraits::kCanMoveWithMemcpy && SecondTraits::kCanMoveWithMemcpy;
  static const bool kCanCopyWithMemcpy =
      FirstTraits::kCanCopyWithMemcpy && SecondTraits::kCanCopyWithMemcpy;
  static const bool kCanFillWithMemset = false;
  static const bool kCanCompareWithMemcmp =
      FirstTraits::kCanCompareWithMemcmp && SecondTraits::kCanCompareWithMemcmp;
  static const bool kCanClearUnusedSlotsWithMemset =
      FirstTraits::kCanClearUnusedSlotsWithMemset &&
      SecondTraits::kCanClearUnusedSlotsWithMemset;

  static constexpr bool kCanTraceConcurrently = false;
};

}  // namespace WTF

#define WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(ClassName)         \
  namespace WTF {                                                             \
  static_assert(!std::is_trivially_default_constructible<ClassName>::value || \
                    !std::is_trivially_move_assignable<ClassName>::value ||   \
                    !std::is_scalar<ClassName>::value,                        \
                "macro not needed");                                          \
  template <>                                                                 \
  struct VectorTraits<ClassName> : SimpleClassVectorTraits<ClassName> {};     \
  }

#define WTF_ALLOW_MOVE_AND_INIT_WITH_MEM_FUNCTIONS(ClassName)                 \
  namespace WTF {                                                             \
  static_assert(!std::is_trivially_default_constructible<ClassName>::value || \
                    !std::is_trivially_move_assignable<ClassName>::value,     \
                "macro not needed");                                          \
  template <>                                                                 \
  struct VectorTraits<ClassName> : VectorTraitsBase<ClassName> {              \
    static const bool kCanInitializeWithMemset = true;                        \
    static const bool kCanClearUnusedSlotsWithMemset = true;                  \
    static const bool kCanMoveWithMemcpy = true;                              \
  };                                                                          \
  }

#define WTF_ALLOW_INIT_WITH_MEM_FUNCTIONS(ClassName)                        \
  namespace WTF {                                                           \
  static_assert(!std::is_trivially_default_constructible<ClassName>::value, \
                "macro not needed");                                        \
  template <>                                                               \
  struct VectorTraits<ClassName> : VectorTraitsBase<ClassName> {            \
    static const bool kCanInitializeWithMemset = true;                      \
    static const bool kCanClearUnusedSlotsWithMemset = true;                \
  };                                                                        \
  }

#define WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(ClassName)          \
  namespace WTF {                                                           \
  static_assert(!std::is_trivially_default_constructible<ClassName>::value, \
                "macro not needed");                                        \
  template <>                                                               \
  struct VectorTraits<ClassName> : VectorTraitsBase<ClassName> {            \
    static const bool kCanClearUnusedSlotsWithMemset = true;                \
  };                                                                        \
  }

using WTF::VectorTraits;
using WTF::SimpleClassVectorTraits;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_VECTOR_TRAITS_H_

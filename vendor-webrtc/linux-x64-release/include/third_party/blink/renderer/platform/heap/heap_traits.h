// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_TRAITS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_TRAITS_H_

#include <type_traits>

#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

namespace blink {

namespace internal {
template <typename T>
inline constexpr bool IsHeapVector = false;

template <typename T>
inline constexpr bool IsHeapVector<HeapVector<T>> = true;
}  // namespace internal

// Given a type T, returns a type that is either Member<T> or just T depending
// on whether T is a garbage-collected type.
template <typename T>
using AddMemberIfNeeded =
    std::conditional_t<WTF::IsGarbageCollectedType<T>::value &&
                           !internal::IsHeapVector<T>,
                       Member<T>,
                       T>;

// Given a type T, returns a type that is either HeapVector<T>,
// HeapVector<Member<T>> or Vector<T> depending on T.
template <typename T>
using VectorOf = std::conditional_t<WTF::IsTraceable<T>::value,
                                    HeapVector<AddMemberIfNeeded<T>>,
                                    Vector<T>>;

// Given types T and U, returns a type that is one of the following:
// - HeapVector<std::pair<V, X>>
//   (where V is either T or Member<T> and X is either U or Member<U>)
// - Vector<std::pair<T, U>>
template <typename T, typename U>
using VectorOfPairs = std::conditional_t<
    WTF::IsTraceable<T>::value || WTF::IsTraceable<U>::value,
    HeapVector<std::pair<AddMemberIfNeeded<T>, AddMemberIfNeeded<U>>>,
    Vector<std::pair<T, U>>>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_HEAP_TRAITS_H_

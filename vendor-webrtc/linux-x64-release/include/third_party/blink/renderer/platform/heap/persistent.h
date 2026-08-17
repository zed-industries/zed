// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_PERSISTENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_PERSISTENT_H_

#include "third_party/blink/renderer/platform/heap/heap_buildflags.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"
#include "third_party/blink/renderer/platform/wtf/hash_functions.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"
#include "third_party/blink/renderer/platform/wtf/vector_traits.h"
#include "v8/include/cppgc/cross-thread-persistent.h"
#include "v8/include/cppgc/persistent.h"  // IWYU pragma: export
#include "v8/include/cppgc/source-location.h"

// Required to optimize away locations for builds that do not need them to avoid
// binary size blowup.
#if BUILDFLAG(VERBOSE_PERSISTENT)
#define PERSISTENT_LOCATION_FROM_HERE blink::PersistentLocation::Current()
#else  // !BUILDFLAG(VERBOSE_PERSISTENT)
#define PERSISTENT_LOCATION_FROM_HERE blink::PersistentLocation()
#endif  // !BUILDFLAG(VERBOSE_PERSISTENT)

namespace blink {

template <typename T>
using Persistent = cppgc::Persistent<T>;

template <typename T>
using WeakPersistent = cppgc::WeakPersistent<T>;

using PersistentLocation = cppgc::SourceLocation;

template <typename T>
Persistent<T> WrapPersistent(
    T* value,
    const PersistentLocation& loc = PERSISTENT_LOCATION_FROM_HERE) {
  return Persistent<T>(value, loc);
}

template <typename T>
WeakPersistent<T> WrapWeakPersistent(
    T* value,
    const PersistentLocation& loc = PERSISTENT_LOCATION_FROM_HERE) {
  return WeakPersistent<T>(value, loc);
}

template <typename U, typename T, typename weakness>
cppgc::internal::BasicPersistent<U, weakness> DownCast(
    const cppgc::internal::BasicPersistent<T, weakness>& p) {
  return p.template To<U>();
}

template <typename U, typename T, typename weakness>
cppgc::internal::BasicCrossThreadPersistent<U, weakness> DownCast(
    const cppgc::internal::BasicCrossThreadPersistent<T, weakness>& p) {
  return p.template To<U>();
}

template <typename T,
          typename = std::enable_if_t<WTF::IsGarbageCollectedType<T>::value>>
Persistent<T> WrapPersistentIfNeeded(T* value) {
  return Persistent<T>(value);
}

template <typename T>
T& WrapPersistentIfNeeded(T& value) {
  return value;
}

}  // namespace blink

namespace WTF {

template <typename T>
struct PersistentVectorTraitsBase : VectorTraitsBase<T> {
  STATIC_ONLY(PersistentVectorTraitsBase);
  static const bool kCanInitializeWithMemset = true;
};

template <typename T>
struct VectorTraits<blink::Persistent<T>>
    : PersistentVectorTraitsBase<blink::Persistent<T>> {};

template <typename T>
struct VectorTraits<blink::WeakPersistent<T>>
    : PersistentVectorTraitsBase<blink::WeakPersistent<T>> {};

template <typename T, typename PersistentType>
struct BasePersistentHashTraits : SimpleClassHashTraits<PersistentType> {
  template <typename U>
  static unsigned GetHash(const U& key) {
    return WTF::GetHash<T*>(key);
  }

  template <typename U, typename V>
  static bool Equal(const U& a, const V& b) {
    return a == b;
  }

  // TODO: Implement proper const'ness for iterator types. Requires support
  // in the marking Visitor.
  using PeekInType = T*;
  using IteratorGetType = PersistentType*;
  using IteratorConstGetType = const PersistentType*;
  using IteratorReferenceType = PersistentType&;
  using IteratorConstReferenceType = const PersistentType&;

  using PeekOutType = T*;

  template <typename U>
  static void Store(const U& value, PersistentType& storage) {
    storage = value;
  }

  static PeekOutType Peek(const PersistentType& value) { return value; }

  static void ConstructDeletedValue(PersistentType& slot) {
    new (&slot) PersistentType(cppgc::kSentinelPointer);
  }

  static bool IsDeletedValue(const PersistentType& value) {
    return value.Get() == cppgc::kSentinelPointer;
  }
};

template <typename T>
struct HashTraits<blink::Persistent<T>>
    : BasePersistentHashTraits<T, blink::Persistent<T>> {};

template <typename T>
struct HashTraits<blink::WeakPersistent<T>>
    : BasePersistentHashTraits<T, blink::WeakPersistent<T>> {};

}  // namespace WTF

namespace base {

template <typename T>
struct IsWeakReceiver;

template <typename T>
struct IsWeakReceiver<blink::WeakPersistent<T>> : std::true_type {};

template <typename>
struct MaybeValidTraits;

// TODO(https://crbug.com/653394): Consider returning a thread-safe best
// guess of validity. MaybeValid() can be invoked from an arbitrary thread.
template <typename T>
struct MaybeValidTraits<blink::WeakPersistent<T>> {
  static bool MaybeValid(const blink::WeakPersistent<T>& p) { return true; }
};

}  // namespace base

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_PERSISTENT_H_

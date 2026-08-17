// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_CROSS_THREAD_PERSISTENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_CROSS_THREAD_PERSISTENT_H_

#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "v8/include/cppgc/cross-thread-persistent.h"

namespace blink {

// CrossThreadPersistent allows retaining objects from threads other than the
// thread that owns the heap of the corresponding object.
//
// Strongly prefer using `CrossThreadHandle` if the object must only be held
// from a different thread.
//
// Caveats:
// - Does not protect the heap owning an object from terminating. E.g., posting
//   a task with a CrossThreadPersistent for `this` will result in a
//   use-after-free in case the heap owning `this` is terminated before the task
//   is invoked.
// - Reaching transitively through the graph is unsupported as objects may be
//   moved concurrently on the thread owning the object.
template <typename T>
using CrossThreadPersistent = cppgc::subtle::CrossThreadPersistent<T>;

// CrossThreadWeakPersistent allows weakly retaining objects from threads other
// than the thread that owns the heap of the corresponding object.
//
// Strongly prefer using `CrossThreadWeakHandle` if the object must only be held
// from a different thread.
//
// Caveats:
// - Does not protect the heap owning an object from termination, as the
//   reference is weak.
// - In order to access the underlying object
//   `CrossThreadWeakPersistent<T>::Lock()` must be used which returns a
//   `CrossThreadPersistent<T>` which in turn also does not protect the heap
//   owning the object from terminating (see above).
// - Reaching transitively through the graph is unsupported as objects may be
//   moved concurrently on the thread owning the object.
template <typename T>
using CrossThreadWeakPersistent = cppgc::subtle::WeakCrossThreadPersistent<T>;

template <typename T>
CrossThreadPersistent<T> WrapCrossThreadPersistent(
    T* value,
    const PersistentLocation& loc = PERSISTENT_LOCATION_FROM_HERE) {
  return CrossThreadPersistent<T>(value, loc);
}

template <typename T>
CrossThreadWeakPersistent<T> WrapCrossThreadWeakPersistent(
    T* value,
    const PersistentLocation& loc = PERSISTENT_LOCATION_FROM_HERE) {
  return CrossThreadWeakPersistent<T>(value, loc);
}

}  // namespace blink

namespace WTF {

template <typename T>
struct HashTraits<blink::CrossThreadPersistent<T>>
    : BasePersistentHashTraits<T, blink::CrossThreadPersistent<T>> {};

template <typename T>
struct HashTraits<blink::CrossThreadWeakPersistent<T>>
    : BasePersistentHashTraits<T, blink::CrossThreadWeakPersistent<T>> {};

template <typename T>
struct CrossThreadCopier<blink::CrossThreadPersistent<T>>
    : public CrossThreadCopierPassThrough<blink::CrossThreadPersistent<T>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <typename T>
struct CrossThreadCopier<blink::CrossThreadWeakPersistent<T>>
    : public CrossThreadCopierPassThrough<blink::CrossThreadWeakPersistent<T>> {
  STATIC_ONLY(CrossThreadCopier);
};

}  // namespace WTF

namespace base {

template <typename T>
struct IsWeakReceiver<blink::CrossThreadWeakPersistent<T>> : std::true_type {};

template <typename>
struct BindUnwrapTraits;

template <typename T>
struct BindUnwrapTraits<blink::CrossThreadWeakPersistent<T>> {
  static blink::CrossThreadPersistent<T> Unwrap(
      const blink::CrossThreadWeakPersistent<T>& wrapped) {
    return wrapped.Lock();
  }
};

template <typename T>
struct MaybeValidTraits<blink::CrossThreadWeakPersistent<T>> {
  static bool MaybeValid(const blink::CrossThreadWeakPersistent<T>& p) {
    return true;
  }
};

}  // namespace base

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_CROSS_THREAD_PERSISTENT_H_

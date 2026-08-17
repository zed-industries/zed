// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_CROSS_THREAD_HANDLE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_CROSS_THREAD_HANDLE_H_

#include "third_party/blink/renderer/platform/heap/cross_thread_handle_internal.h"

namespace blink {

// A handle object that may be used to hold a garbage collection object from a
// different thread than the object was created on. The handle supports
// thread-safe copy/move/destruction. The underlying object may only be used
// from the creation thread.
//
// Example posting to a static method that forwards back to an instance method
// on the main thread:
// ```
// class PingPong final : public GarbageCollected<PingPong> {
//  public:
//   void Ping() {
//    DCHECK(IsMainThread());
//    worker_pool::PostTask(FROM_HERE,
//                          CrossThreadBindOnce(&PingPong::PongOnBackground,
//                                              MakeCrossThreadHandle(this),
//                                              std::move(task_runner_)));
//   }
//
//  private:
//   const scoped_refptr<base::SingleThreadTaskRunner> task_runner_ =
//       GetTaskRunner();
//
//   void DoneOnMainThread() { DCHECK(IsMainThread()); }
//
//   static void PongOnBackground(
//      CrossThreadHandle<PingPong> ping_pong,
//      scoped_refptr<base::SingleThreadTaskRunner> task_runner) {
//    DCHECK(!IsMainThread());
//    PostCrossThreadTask(
//        *task_runner, FROM_HERE,
//        CrossThreadBindOnce(&PingPong::DoneOnMainThread,
//                            MakeUnwrappingCrossThreadWeakHandle(
//                                std::move(ping_pong))));
//   }
// };
// ```
template <typename T>
using CrossThreadHandle = internal::
    BasicCrossThreadHandle<T, internal::StrongCrossThreadHandleWeaknessPolicy>;

// Utility function creating a `CrossThreadHandle` tracking the current source
// location position in debugging configurations.
template <typename T>
CrossThreadHandle<T> MakeCrossThreadHandle(
    T* value,
    const CrossThreadHandleLocation& loc =
        CROSS_THREAD_HANDLE_LOCATION_FROM_HERE) {
  return CrossThreadHandle<T>(value);
}

// A weak handle object with similar restrictions as `CrossThreadHandle`.
// The object is only held alive weakly, meaning that the object may be
// reclaimed by the garbage collector. As a consequence, any value retrieved
// from this handle must be checked against nullptr.
template <typename T>
using CrossThreadWeakHandle = internal::
    BasicCrossThreadHandle<T, internal::WeakCrossThreadHandleWeaknessPolicy>;

// Utility function creating a `CrossThreadWeakHandle` tracking the current
// source location position in debugging configurations.
template <typename T>
CrossThreadWeakHandle<T> MakeCrossThreadWeakHandle(
    T* value,
    const CrossThreadHandleLocation& loc =
        CROSS_THREAD_HANDLE_LOCATION_FROM_HERE) {
  return CrossThreadWeakHandle<T>(value);
}

// A version of `CrossThreadHandle` that automatically unwraps into `T*` on
// invocation of a bound function. This is useful for binding against regular
// instance methods of a type.
template <typename T>
using UnwrappingCrossThreadHandle = internal::BasicUnwrappingCrossThreadHandle<
    T,
    internal::StrongCrossThreadHandleWeaknessPolicy>;

// Utility function creating an `UnwrappingCrossThreadHandle`.
template <typename T>
UnwrappingCrossThreadHandle<T> MakeUnwrappingCrossThreadHandle(
    T* value,
    const CrossThreadHandleLocation& loc =
        CROSS_THREAD_HANDLE_LOCATION_FROM_HERE) {
  return UnwrappingCrossThreadHandle<T>(value, loc);
}

// Utility function creating an `UnwrappingCrossThreadHandle`.
template <typename T>
UnwrappingCrossThreadHandle<T> MakeUnwrappingCrossThreadHandle(
    const CrossThreadHandle<T>& handle) {
  return UnwrappingCrossThreadHandle<T>(handle);
}

// Utility function creating an `UnwrappingCrossThreadHandle`.
template <typename T>
UnwrappingCrossThreadHandle<T> MakeUnwrappingCrossThreadHandle(
    CrossThreadHandle<T>&& handle) {
  return UnwrappingCrossThreadHandle<T>(std::move(handle));
}

// A version of `CrossThreadWeakHandle` that automatically unwraps into `T*`
// on invocation of a bound function. This is useful for binding against regular
// instance methods of a type.
template <typename T>
using UnwrappingCrossThreadWeakHandle =
    internal::BasicUnwrappingCrossThreadHandle<
        T,
        internal::WeakCrossThreadHandleWeaknessPolicy>;

// Utility function creating an `UnwrappingCrossThreadHandle`.
template <typename T>
UnwrappingCrossThreadWeakHandle<T> MakeUnwrappingCrossThreadWeakHandle(
    T* value,
    const CrossThreadHandleLocation& loc =
        CROSS_THREAD_HANDLE_LOCATION_FROM_HERE) {
  return UnwrappingCrossThreadWeakHandle<T>(value, loc);
}

// Utility function creating an `UnwrappingCrossThreadHandle`.
template <typename T>
UnwrappingCrossThreadWeakHandle<T> MakeUnwrappingCrossThreadWeakHandle(
    const CrossThreadWeakHandle<T>& handle) {
  return UnwrappingCrossThreadWeakHandle<T>(handle);
}

// Utility function creating an `UnwrappingCrossThreadHandle`.

template <typename T>
UnwrappingCrossThreadWeakHandle<T> MakeUnwrappingCrossThreadWeakHandle(
    CrossThreadWeakHandle<T>&& handle) {
  return UnwrappingCrossThreadWeakHandle<T>(std::move(handle));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_CROSS_THREAD_HANDLE_H_

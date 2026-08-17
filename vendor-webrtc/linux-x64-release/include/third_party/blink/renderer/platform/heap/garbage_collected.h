// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_GARBAGE_COLLECTED_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_GARBAGE_COLLECTED_H_

#include <type_traits>

#include "base/functional/unretained_traits.h"
#include "third_party/blink/renderer/platform/heap/thread_state_storage.h"
#include "v8/include/cppgc/allocation.h"
#include "v8/include/cppgc/garbage-collected.h"
#include "v8/include/cppgc/liveness-broker.h"
#include "v8/include/cppgc/type-traits.h"

namespace cppgc {
class LivenessBroker;
class Visitor;
}  // namespace cppgc

namespace blink {

template <typename T>
using GarbageCollected = cppgc::GarbageCollected<T>;

using GarbageCollectedMixin = cppgc::GarbageCollectedMixin;

using LivenessBroker = cppgc::LivenessBroker;

using Visitor = cppgc::Visitor;

// Default MakeGarbageCollected: Constructs an instance of T, which is a garbage
// collected type.
template <typename T, typename... Args>
T* MakeGarbageCollected(Args&&... args) {
  return cppgc::MakeGarbageCollected<T>(
      ThreadStateStorageFor<ThreadingTrait<T>::kAffinity>::GetState()
          ->allocation_handle(),
      std::forward<Args>(args)...);
}

using AdditionalBytes = cppgc::AdditionalBytes;

// Constructs an instance of T, which is a garbage collected type. This special
// version takes size which enables constructing inline objects.
template <typename T, typename... Args>
T* MakeGarbageCollected(AdditionalBytes additional_bytes, Args&&... args) {
  return cppgc::MakeGarbageCollected<T>(
      ThreadStateStorageFor<ThreadingTrait<T>::kAffinity>::GetState()
          ->allocation_handle(),
      std::forward<AdditionalBytes>(additional_bytes),
      std::forward<Args>(args)...);
}

}  // namespace blink

namespace base::internal {

// v8 lives outside the Chromium repository and cannot rely on //base concepts
// like `DISALLOW_UNRETAINED()`.
template <typename T>
  requires cppgc::IsGarbageCollectedOrMixinTypeV<T>
inline constexpr bool kCustomizeSupportsUnretained<T> = false;

}  // namespace base::internal

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_GARBAGE_COLLECTED_H_

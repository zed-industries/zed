// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_GC_OBJECT_LIVENESS_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_GC_OBJECT_LIVENESS_OBSERVER_H_

#include <memory>

#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Observer that can be used to track whether an object has been reclaimed by
// the garbage collector.
template <typename T>
class GCObjectLivenessObserver {
  STACK_ALLOCATED();

 public:
  GCObjectLivenessObserver() = default;

  explicit GCObjectLivenessObserver(T* object)
      : holder_(
            std::unique_ptr<WeakPersistent<T>>{new WeakPersistent<T>(object)}) {
  }

  void Observe(T* object) {
    DCHECK(!holder_.get());
    holder_.reset(new WeakPersistent<T>(object));
  }

  bool WasCollected() const { return !holder_->Get(); }

 private:
  // WeakPersistent needs to be allocated separately to allow using
  // GCObjectLivenessObserver from stack. Otherwise, Oilpan could treat pointers
  // reachable from stack as strong.
  std::unique_ptr<WeakPersistent<T>> holder_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_GC_OBJECT_LIVENESS_OBSERVER_H_

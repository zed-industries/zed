// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_DETACHABLE_USE_COUNTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_DETACHABLE_USE_COUNTER_H_

#include "third_party/blink/renderer/platform/instrumentation/use_counter.h"

namespace blink {

class DetachableUseCounter final
    : public GarbageCollected<DetachableUseCounter>,
      public UseCounter {
 public:
  // |use_counter| can be null, and in that case |this| is already detached.
  explicit DetachableUseCounter(UseCounter* use_counter)
      : use_counter_(use_counter) {}
  ~DetachableUseCounter() override = default;

  // UseCounter
  void CountUse(mojom::WebFeature feature) override {
    if (use_counter_) {
      use_counter_->CountUse(feature);
    }
  }
  void CountDeprecation(mojom::WebFeature feature) override {
    if (use_counter_) {
      use_counter_->CountDeprecation(feature);
    }
  }
  void CountWebDXFeature(mojom::blink::WebDXFeature feature) override {
    if (use_counter_) {
      use_counter_->CountWebDXFeature(feature);
    }
  }
  void Trace(Visitor* visitor) const override { visitor->Trace(use_counter_); }

  void Detach() { use_counter_ = nullptr; }

 private:
  Member<UseCounter> use_counter_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_DETACHABLE_USE_COUNTER_H_

// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_LOADING_BEHAVIOR_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_LOADING_BEHAVIOR_OBSERVER_H_

#include "third_party/blink/public/common/loader/loading_behavior_flag.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Used for notifying loading behavior from the platform loader layer.
class PLATFORM_EXPORT LoadingBehaviorObserver : public GarbageCollectedMixin {
 public:
  virtual void DidObserveLoadingBehavior(
      blink::LoadingBehaviorFlag behavior) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_LOADING_BEHAVIOR_OBSERVER_H_

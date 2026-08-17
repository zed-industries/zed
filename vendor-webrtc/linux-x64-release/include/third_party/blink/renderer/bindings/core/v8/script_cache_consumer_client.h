// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_CACHE_CONSUMER_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_CACHE_CONSUMER_CLIENT_H_

#include "third_party/blink/renderer/platform/bindings/parkable_string.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "v8/include/v8-script.h"

namespace blink {

// Interface for clients of ScriptCacheConsumer which are notified when the
// cache consume completes.
class ScriptCacheConsumerClient : public GarbageCollectedMixin {
 public:
  virtual void NotifyCacheConsumeFinished() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SCRIPT_CACHE_CONSUMER_CLIENT_H_

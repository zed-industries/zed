// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_ACCEPT_LANGUAGES_WATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_ACCEPT_LANGUAGES_WATCHER_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// Helper class allowing DedicatedOrSharedWorkerFetchContextImpl to notify blink
// upon an accept languages update. This class will be extended by
// WorkerNavigator.
class AcceptLanguagesWatcher : public GarbageCollectedMixin {
 public:
  virtual void NotifyUpdate() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_ACCEPT_LANGUAGES_WATCHER_H_

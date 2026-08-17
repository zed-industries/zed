// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_MOCK_CONTEXT_LIFECYCLE_NOTIFIER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_MOCK_CONTEXT_LIFECYCLE_NOTIFIER_H_

#include "third_party/blink/renderer/platform/context_lifecycle_notifier.h"
#include "third_party/blink/renderer/platform/context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"

namespace blink {

class MockContextLifecycleNotifier final
    : public GarbageCollected<MockContextLifecycleNotifier>,
      public ContextLifecycleNotifier {
  USING_PRE_FINALIZER(MockContextLifecycleNotifier, Dispose);

 public:
  MockContextLifecycleNotifier() = default;

  void Dispose() {
    if (!notified_destruction_) {
      NotifyContextDestroyed();
    }
  }

  void NotifyContextDestroyed() {
    ContextLifecycleNotifier::NotifyContextDestroyed();
    notified_destruction_ = true;
  }

  void Trace(Visitor* visitor) const override {
    ContextLifecycleNotifier::Trace(visitor);
  }

 private:
  bool notified_destruction_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_MOCK_CONTEXT_LIFECYCLE_NOTIFIER_H_

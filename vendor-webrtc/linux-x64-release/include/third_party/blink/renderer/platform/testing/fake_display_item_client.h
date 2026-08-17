// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FAKE_DISPLAY_ITEM_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FAKE_DISPLAY_ITEM_CLIENT_H_

#include "third_party/blink/renderer/platform/graphics/paint/display_item_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

// A simple DisplayItemClient implementation suitable for use in unit tests.
class FakeDisplayItemClient : public GarbageCollected<FakeDisplayItemClient>,
                              public DisplayItemClient {
 public:
  explicit FakeDisplayItemClient(const String& name = "FakeDisplayItemClient")
      : name_(name) {}

  String DebugName() const final { return name_; }

  // This simulates a paint without needing a PaintController.
  using DisplayItemClient::Validate;
  void Trace(Visitor* visitor) const override {
    DisplayItemClient::Trace(visitor);
  }

 private:
  String name_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FAKE_DISPLAY_ITEM_CLIENT_H_

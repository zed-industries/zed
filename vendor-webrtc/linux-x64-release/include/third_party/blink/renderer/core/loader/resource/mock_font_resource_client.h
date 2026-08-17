// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_MOCK_FONT_RESOURCE_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_MOCK_FONT_RESOURCE_CLIENT_H_

#include "third_party/blink/renderer/core/loader/resource/font_resource.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_client.h"

namespace blink {

class MockFontResourceClient final
    : public GarbageCollected<MockFontResourceClient>,
      public FontResourceClient {
 public:
  MockFontResourceClient();
  ~MockFontResourceClient() override;

  void FontLoadShortLimitExceeded(FontResource*) override;
  void FontLoadLongLimitExceeded(FontResource*) override;

  bool FontLoadShortLimitExceededCalled() const {
    return font_load_short_limit_exceeded_called_;
  }

  bool FontLoadLongLimitExceededCalled() const {
    return font_load_long_limit_exceeded_called_;
  }

  String DebugName() const override { return "MockFontResourceClient"; }

 private:
  bool font_load_short_limit_exceeded_called_;
  bool font_load_long_limit_exceeded_called_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_MOCK_FONT_RESOURCE_CLIENT_H_

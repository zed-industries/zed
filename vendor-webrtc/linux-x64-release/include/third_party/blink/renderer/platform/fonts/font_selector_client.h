// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_SELECTOR_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_SELECTOR_CLIENT_H_

#include "third_party/blink/renderer/platform/fonts/font_invalidation_reason.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class FontSelector;

class FontSelectorClient : public GarbageCollectedMixin {
 public:
  virtual ~FontSelectorClient() = default;

  virtual void FontsNeedUpdate(FontSelector*, FontInvalidationReason) = 0;

  void Trace(Visitor* visitor) const override {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_SELECTOR_CLIENT_H_

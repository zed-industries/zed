// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_PREWARMER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_PREWARMER_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_string.h"

namespace blink {

// The renderer can request to prewarm the font cache.
class WebFontPrewarmer {
 public:
  virtual void PrewarmFamily(const WebString& family_name) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_PREWARMER_H_

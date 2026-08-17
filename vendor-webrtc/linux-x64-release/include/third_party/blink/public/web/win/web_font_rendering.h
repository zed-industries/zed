// Copyright 2014 Google Inc. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WIN_WEB_FONT_RENDERING_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WIN_WEB_FONT_RENDERING_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_font_prewarmer.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/skia/include/core/SkRefCnt.h"

class SkFontMgr;

namespace blink {

class WebFontRenderingClient;

class BLINK_EXPORT WebFontRendering {
 public:
  static void SetSkiaFontManager(sk_sp<SkFontMgr>);
  // Set an instance of |WebFontPrewarmer|. The instance must be kept alive
  // until the process exits.
  static void SetFontPrewarmer(WebFontPrewarmer*);
  // Set an instance of `WebFontRenderingClient`. The instance must be kept
  // alive until the process exits.
  static void SetFontRenderingClient(WebFontRenderingClient*);
  static WebFontPrewarmer* GetFontPrewarmer();
  static void SetMenuFontMetrics(const WebString& family_name,
                                 int32_t font_height);
  static void SetSmallCaptionFontMetrics(const WebString& family_name,
                                         int32_t font_height);
  static void SetStatusFontMetrics(const WebString& family_name,
                                   int32_t font_height);
  static void SetAntialiasedTextEnabled(bool);
  static void SetLCDTextEnabled(bool);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WIN_WEB_FONT_RENDERING_H_

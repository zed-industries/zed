/* Copyright (c) 2010, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_RENDER_STYLE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_RENDER_STYLE_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/skia/include/core/SkFontStyle.h"
#include "third_party/skia/include/core/SkFontTypes.h"
#include "third_party/skia/include/core/SkPaint.h"
#include "third_party/skia/include/core/SkRefCnt.h"

class SkFont;
class SkFontMgr;

namespace blink {

// WebFontRenderStyle describes the user's preferences for rendering a font at a
// given size.
struct BLINK_PLATFORM_EXPORT WebFontRenderStyle {
  enum {
    kNoPreference = 2,
  };

  WebFontRenderStyle() = default;

  bool operator==(const WebFontRenderStyle& a) const {
    return use_bitmaps == a.use_bitmaps && use_auto_hint == a.use_auto_hint &&
           use_hinting == a.use_hinting && hint_style == a.hint_style &&
           use_anti_alias == a.use_anti_alias &&
           use_subpixel_rendering == a.use_subpixel_rendering &&
           use_subpixel_positioning == a.use_subpixel_positioning;
  }

  static void SetSkiaFontManager(sk_sp<SkFontMgr>);
  static void SetHinting(SkFontHinting);
  static void SetAutoHint(bool);
  static void SetUseBitmaps(bool);
  static void SetAntiAlias(bool);
  static void SetSubpixelRendering(bool);
  static void SetSubpixelPositioning(bool);
  static void SetSystemFontFamily(const WebString& name);

  static WebFontRenderStyle GetDefault();

  // Overrides fields with those from |other|, except the fields that are set to
  // kNoPreference in |other|.
  void OverrideWith(const WebFontRenderStyle& other);

  void ApplyToSkFont(SkFont*) const;

  // Each of the use* members below can take one of three values:
  //   0: off
  //   1: on
  //   NoPreference: no preference expressed
  char use_bitmaps = kNoPreference;    // use embedded bitmap strike if possible
  char use_auto_hint = kNoPreference;  // use 'auto' hinting (FreeType specific)
  char use_hinting = kNoPreference;    // hint glyphs to the pixel grid
  char hint_style = 0;                 // level of hinting, 0..3
  char use_anti_alias = kNoPreference;  // antialias glyph shapes
  // use subpixel rendering (partially-filled pixels)
  char use_subpixel_rendering = kNoPreference;
  // use subpixel positioning (fractional X positions for glyphs)
  char use_subpixel_positioning = kNoPreference;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FONT_RENDER_STYLE_H_

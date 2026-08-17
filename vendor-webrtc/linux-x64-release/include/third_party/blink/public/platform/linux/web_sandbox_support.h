/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_LINUX_WEB_SANDBOX_SUPPORT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_LINUX_WEB_SANDBOX_SUPPORT_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_string.h"

namespace gfx {
struct FallbackFontData;
}

namespace blink {

struct WebFontRenderStyle;

// Put methods here that are required due to sandbox restrictions.
// These are currently only implemented only on Linux:
// https://chromium.googlesource.com/chromium/src/+/main/docs/linux/sandbox_ipc.md
class WebSandboxSupport {
 public:
  virtual ~WebSandboxSupport() {}

  // Get information to instantiate a font which contains glyphs for the given
  // Unicode code-point.
  //   character: a UTF-32 codepoint
  //   preferred_locale: preferred locale identifier for the |characters|
  //                     (e.g. "en", "ja", "zh-CN")
  //
  // fallback_font will be filled with the font name and filename, among other
  // data. Returns false if the request could not be satisfied.
  virtual bool GetFallbackFontForCharacter(
      WebUChar32 character,
      const char* preferred_locale,
      gfx::FallbackFontData* fallback_font) = 0;

  // Get a FallbackFontData specification for a font uniquely identified by full
  // font name or postscript name.  Specify full font name or postscript name as
  // argument in UTF-8.
  //
  // The FallbackFontData out parameter will contain a filename, ttc index and
  // fontconfig interface id, with the italic and bold members set always
  // initialised to false. If a match is not found, return false.
  virtual bool MatchFontByPostscriptNameOrFullFontName(
      const char* font_unique_name,
      gfx::FallbackFontData*) = 0;

  // Fill out the given WebFontRenderStyle with the user's preferences for
  // rendering the given font at the given size (in pixels), given weight and
  // given slant and given device scale factor. The device scale factor is
  // needed in gfx::GetFontRenderParams for determining subpixel and hinting
  // settings.
  virtual void GetWebFontRenderStyleForStrike(const char* family,
                                              int size,
                                              bool is_bold,
                                              bool is_italic,
                                              float device_scale_factor,
                                              WebFontRenderStyle*) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_LINUX_WEB_SANDBOX_SUPPORT_H_

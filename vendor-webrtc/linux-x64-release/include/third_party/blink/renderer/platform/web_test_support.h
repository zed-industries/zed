/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEB_TEST_SUPPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEB_TEST_SUPPORT_H_

#include "base/auto_reset.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class WebTestSupport {
  STATIC_ONLY(WebTestSupport);

 public:
  PLATFORM_EXPORT static bool IsRunningWebTest();
  PLATFORM_EXPORT static bool IsFontAntialiasingEnabledForTest();
  PLATFORM_EXPORT static bool IsTextSubpixelPositioningAllowedForTest();

 private:
  // In harfbuzz_shaper_test.cc. It knows how to restore the settings.
  friend class ScopedSubpixelOverride;
  PLATFORM_EXPORT static void SetFontAntialiasingEnabledForTest(bool);
  PLATFORM_EXPORT static void SetTextSubpixelPositioningAllowedForTest(bool);
};

// Web test mode is enabled by default in blink_unittests, while disabled by
// default in blink_platform_unittests. This class is for unit tests needing a
// specific web test mode. See the callers of WebTestSupport::IsRunningWebTest()
// for what are different in the mode.
class PLATFORM_EXPORT ScopedWebTestMode {
 public:
  explicit ScopedWebTestMode(bool enable_web_test_mode);

 private:
  base::AutoReset<bool> auto_reset_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEB_TEST_SUPPORT_H_

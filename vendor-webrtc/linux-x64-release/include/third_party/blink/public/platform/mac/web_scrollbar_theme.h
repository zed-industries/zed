/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MAC_WEB_SCROLLBAR_THEME_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MAC_WEB_SCROLLBAR_THEME_H_

#include <optional>

#include "third_party/blink/public/platform/web_common.h"

namespace blink {

// This enum must match NSScrollerStyle in the 10.7 SDK.
enum ScrollerStyle { kScrollerStyleLegacy = 0, kScrollerStyleOverlay = 1 };

class WebScrollbarTheme {
 public:
  // Updates the scrollbar appearance and behavior.
  // |initialButtonDelay| is the current value of NSScrollerButtonDelay from
  // NSUserDefaults.
  // |autoscrollButtonDelay| is the current value of NSScrollerButtonPeriod from
  // NSUserDefaults.
  // |preferredScrollerStyle| is the current value of +[NSScroller
  // preferredScrollerStyle].
  // |redraw| is true if the update requires a redraw to include the change.
  // |jump_on_track_click| is the current value of AppleScrollerPagingBehavior.
  BLINK_EXPORT static void UpdateScrollbarsWithNSDefaults(
      std::optional<float> initial_button_delay,
      std::optional<float> autoscroll_button_delay,
      ScrollerStyle preferred_scroller_style,
      bool redraw,
      bool jump_on_track_click);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MAC_WEB_SCROLLBAR_THEME_H_

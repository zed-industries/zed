// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_LCD_TEXT_PREFERENCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_LCD_TEXT_PREFERENCE_H_

namespace blink {

enum class LCDTextPreference {
  // LCD text is preferred when making decisions. For example, we may
  // sacrifice composited scrolling if we would lose LCD text.
  kStronglyPreferred,
  // LCD text is good to have, but don't affect important decisions. We can
  // use composited scrolling even if we will lose LCD text, but we will try
  // to preserve LCD text when possible.
  kWeaklyPreferred,
  // LCD text is disabled (e.g. on Android) or doesn't matter very much (e.g.
  // on high-dpi screens).
  kIgnored,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_LCD_TEXT_PREFERENCE_H_

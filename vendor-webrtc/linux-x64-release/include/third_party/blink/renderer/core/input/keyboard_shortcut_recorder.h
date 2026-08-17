// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_KEYBOARD_SHORTCUT_RECORDER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_KEYBOARD_SHORTCUT_RECORDER_H_

#include "build/build_config.h"

namespace blink {
// Used for histograms. See PhysicalKeyboardShortcut in
// tools/metrics/histograms/enums.xml &
// content/public/android/java/src/org/chromium/content_public/browser/KeyboardShortcutRecorder.java
enum class KeyboardShortcut {
  // The values should not be reordered or deleted and new entries should only
  // be added at the end (otherwise it will cause problems interpreting logs)
  kZoomIn = 0,
  kZoomOut = 1,
  kZoomReset = 2,
  kDeleteLine = 3,
  kPageUp = 4,
  kPageDown = 5,
  kMaxValue = kPageDown,
};

#if BUILDFLAG(IS_ANDROID)
void RecordKeyboardShortcutForAndroid(const KeyboardShortcut keyboard_shortcut);
#endif  // BUILDFLAG(IS_ANDROID)

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INPUT_KEYBOARD_SHORTCUT_RECORDER_H_

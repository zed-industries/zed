// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FULLSCREEN_VIDEO_STATUS_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FULLSCREEN_VIDEO_STATUS_H_

namespace blink {

enum class WebFullscreenVideoStatus {
  // Video is not effectively fullscreen.
  kNotEffectivelyFullscreen = 0,
  // Video is fullscreen and allowed to enter Picture-in-Picture.
  kFullscreenAndPictureInPictureEnabled,
  // Video is fullscreen and is not allowed to enter Picture-in-Picture.
  kFullscreenAndPictureInPictureDisabled,
  // The maximum number of fullscreen status.
  kMaxValue = kFullscreenAndPictureInPictureDisabled,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_FULLSCREEN_VIDEO_STATUS_H_

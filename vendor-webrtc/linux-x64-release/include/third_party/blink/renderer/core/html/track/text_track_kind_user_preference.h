// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_KIND_USER_PREFERENCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_KIND_USER_PREFERENCE_H_

namespace blink {

// Defines user preference for text track kind.
enum class TextTrackKindUserPreference {
  // Display only tracks marked as default.
  kDefault,
  // If available, display captions track in preferred language, else display
  // subtitles.
  kCaptions,
  // If available, display subtitles track in preferred language, else display
  // captions.
  kSubtitles
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_KIND_USER_PREFERENCE_H_

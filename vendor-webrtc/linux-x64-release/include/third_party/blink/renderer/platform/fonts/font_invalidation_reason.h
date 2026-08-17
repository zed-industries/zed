// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_INVALIDATION_REASON_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_INVALIDATION_REASON_H_

namespace blink {

// Notifies FontSelectorClient of detailed reason of FontSelection invalidation.
enum class FontInvalidationReason {
  // The default reason without any specific details.
  kGeneralInvalidation,
  // A custom font has finished loading and is ready for use.
  kFontFaceLoaded,
  // A @font-face rule has been deleted.
  kFontFaceDeleted,
  // TODO(xiaochengh): Add more detailed entries for different callers, and
  // implement different behaviors on FontSelectorClient.
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_INVALIDATION_REASON_H_

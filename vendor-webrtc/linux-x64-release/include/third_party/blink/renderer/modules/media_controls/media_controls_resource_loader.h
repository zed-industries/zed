// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_RESOURCE_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_RESOURCE_LOADER_H_

#include "third_party/blink/renderer/core/css/css_default_style_sheets.h"

namespace blink {

// Builds the UA stylesheet for the Media Controls based on feature flags
// and platform. There is an Android specific stylesheet that we will include
// on Android or if devtools has enable mobile emulation.
class MediaControlsResourceLoader
    : public CSSDefaultStyleSheets::UAStyleSheetLoader {
 public:
  static void InjectMediaControlsUAStyleSheet();

  // The loading specific stylesheet is inserted into the loading panel DOM
  // tree and contains styles specific to the loading panel.
  static String GetShadowLoadingStyleSheet();

  // Returns the jump SVG image as a string.
  static String GetJumpSVGImage();

  // Returns the arrow right SVG image as a string.
  static String GetArrowRightSVGImage();

  // Returns the arrow left SVG image as a string.
  static String GetArrowLeftSVGImage();

  // Returns the scrubbing message stylesheet content as a string.
  static String GetScrubbingMessageStyleSheet();

  // Returns the animated arrow stylesheet content as a string.
  static String GetAnimatedArrowStyleSheet();

  // Returns the specific stylesheet used for media related interstitials.
  static String GetMediaInterstitialsStyleSheet();

  String GetUAStyleSheet() override;

  MediaControlsResourceLoader();

  MediaControlsResourceLoader(const MediaControlsResourceLoader&) = delete;
  MediaControlsResourceLoader& operator=(const MediaControlsResourceLoader&) =
      delete;

  ~MediaControlsResourceLoader() override;

 private:
  String GetMediaControlsCSS() const;

  String GetMediaControlsAndroidCSS() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_RESOURCE_LOADER_H_

// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_DETAILS_SCREEN_DETAILED_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_DETAILS_SCREEN_DETAILED_H_

#include "third_party/blink/renderer/core/frame/screen.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class LocalDOMWindow;

// Interface exposing additional per-screen information.
// https://w3c.github.io/window-management/
class MODULES_EXPORT ScreenDetailed final : public Screen {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ScreenDetailed(LocalDOMWindow* window, int64_t display_id);

  static bool AreWebExposedScreenDetailedPropertiesEqual(
      const display::ScreenInfo& prev,
      const display::ScreenInfo& current);

  // Web-exposed interface (additional per-screen information):
  int left() const;
  int top() const;
  bool isPrimary() const;
  bool isInternal() const;
  float devicePixelRatio() const;
  String label() const;

  // Attributes exposed for HDR canvas.
  // https://github.com/w3c/ColorWeb-CG/blob/main/hdr_html_canvas_element.md
  float highDynamicRangeHeadroom() const;
  float redPrimaryX() const;
  float redPrimaryY() const;
  float greenPrimaryX() const;
  float greenPrimaryY() const;
  float bluePrimaryX() const;
  float bluePrimaryY() const;
  float whitePointX() const;
  float whitePointY() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SCREEN_DETAILS_SCREEN_DETAILED_H_

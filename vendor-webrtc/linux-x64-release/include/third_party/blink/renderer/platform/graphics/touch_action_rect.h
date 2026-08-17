// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TOUCH_ACTION_RECT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TOUCH_ACTION_RECT_H_

#include "third_party/blink/renderer/platform/graphics/touch_action.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

struct PLATFORM_EXPORT TouchActionRect {
  gfx::Rect rect;
  TouchAction allowed_touch_action = TouchAction::kNone;

  bool operator==(const TouchActionRect& rhs) const {
    return rect == rhs.rect && allowed_touch_action == rhs.allowed_touch_action;
  }

  bool operator!=(const TouchActionRect& rhs) const { return !(*this == rhs); }

  String ToString() const;
};

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, const TouchActionRect&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TOUCH_ACTION_RECT_H_

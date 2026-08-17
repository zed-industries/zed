// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_VALUE_CHANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_VALUE_CHANGE_H_

namespace blink {

enum class MediaValueChange {
  // Viewport or device size changed. width/height/device-width/device-height.
  kSize,
  // dv* unit evaluation changed.
  kDynamicViewport,
  // Any other value which affect media query evaluations changed.
  kOther,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_MEDIA_VALUE_CHANGE_H_

// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_VALUE_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_VALUE_FACTORY_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CompositorKeyframeValue;
class ComputedStyle;
class PropertyHandle;

class CompositorKeyframeValueFactory {
  STATIC_ONLY(CompositorKeyframeValueFactory);

 public:
  static CompositorKeyframeValue* Create(const PropertyHandle&,
                                         const ComputedStyle&,
                                         double offset);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_CSS_COMPOSITOR_KEYFRAME_VALUE_FACTORY_H_

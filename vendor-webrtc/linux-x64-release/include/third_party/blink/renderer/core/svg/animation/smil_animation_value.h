// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_ANIMATION_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_ANIMATION_VALUE_H_

#include "third_party/blink/renderer/platform/transforms/affine_transform.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class SVGPropertyBase;

// Representation of value being computed and applied. Different fields will be
// used depending on which target is being animated (attribute/property
// vs. motion path) but always the same field within the same sandwich.
struct SMILAnimationValue {
  STACK_ALLOCATED();

 public:
  SVGPropertyBase* property_value = nullptr;
  AffineTransform motion_transform;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_ANIMATION_VALUE_H_

// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_NATIVE_PAINT_IMAGE_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_NATIVE_PAINT_IMAGE_GENERATOR_H_

#include "third_party/blink/renderer/core/animation/animation.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

// See README.md in modules/csspaint/nativepaint for more detail in how this
// fits in the overall design. This class is used as a proxy for native
// paint definitions, with its subclasses *PaintImageGenerator.
class CORE_EXPORT NativePaintImageGenerator
    : public GarbageCollected<NativePaintImageGenerator> {
 public:
  virtual ~NativePaintImageGenerator() = default;

  // See comment in NativeCssPaintDefinition::GetAnimationForProperty
  virtual Animation* GetAnimationIfCompositable(const Element* element) = 0;

  virtual void Shutdown() = 0;

  virtual void Trace(Visitor* visitor) const {}

  static bool NativePaintWorkletAnimationsEnabled() {
    return RuntimeEnabledFeatures::CompositeClipPathAnimationEnabled() ||
           RuntimeEnabledFeatures::CompositeBGColorAnimationEnabled();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_NATIVE_PAINT_IMAGE_GENERATOR_H_

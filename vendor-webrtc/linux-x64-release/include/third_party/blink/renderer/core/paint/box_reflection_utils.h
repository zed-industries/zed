// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_REFLECTION_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_REFLECTION_UTILS_H_

namespace blink {

class BoxReflection;
class ComputedStyle;
class PaintLayer;

// Utilities for manipulating box reflections in terms of core concepts, like
// PaintLayer.
BoxReflection BoxReflectionForPaintLayer(const PaintLayer&,
                                         const ComputedStyle&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BOX_REFLECTION_UTILS_H_

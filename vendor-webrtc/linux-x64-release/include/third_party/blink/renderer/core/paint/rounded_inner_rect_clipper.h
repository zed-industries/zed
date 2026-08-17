// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_ROUNDED_INNER_RECT_CLIPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_ROUNDED_INNER_RECT_CLIPPER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ContouredRect;
class GraphicsContext;
struct PhysicalRect;

class RoundedInnerRectClipper {
  STACK_ALLOCATED();

 public:
  RoundedInnerRectClipper(GraphicsContext&,
                          const PhysicalRect&,
                          const ContouredRect& clip_rect);
  ~RoundedInnerRectClipper();

 private:
  GraphicsContext& context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_ROUNDED_INNER_RECT_CLIPPER_H_

// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BLOCK_PAINT_INVALIDATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BLOCK_PAINT_INVALIDATOR_H_

#include "third_party/blink/renderer/platform/graphics/paint_invalidation_reason.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

struct PaintInvalidatorContext;
class LayoutBlock;

class BlockPaintInvalidator {
  STACK_ALLOCATED();

 public:
  BlockPaintInvalidator(const LayoutBlock& block) : block_(block) {}

  void InvalidatePaint(const PaintInvalidatorContext&);

 private:
  const LayoutBlock& block_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_BLOCK_PAINT_INVALIDATOR_H_

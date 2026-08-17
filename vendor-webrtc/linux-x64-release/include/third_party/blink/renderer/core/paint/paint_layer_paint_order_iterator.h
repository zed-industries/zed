/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_PAINT_ORDER_ITERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_PAINT_ORDER_ITERATOR_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/paint/paint_layer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// This iterator walks the PaintLayer descendants in the following paint order:
// NegativeZOrderChildren -> NormalFlowChildren -> PositiveZOrderChildren.
class CORE_EXPORT PaintLayerPaintOrderIterator {
  STACK_ALLOCATED();

 public:
  PaintLayerPaintOrderIterator(const PaintLayer* root,
                               PaintLayerIteration which_children)
      : root_(root),
        remaining_children_(which_children),
        index_(0),
        current_normal_flow_child_(root->FirstChild())
#if DCHECK_IS_ON()
        ,
        mutation_detector_(root)
#endif
  {
  }
  PaintLayerPaintOrderIterator(const PaintLayerPaintOrderIterator&) = delete;
  PaintLayerPaintOrderIterator& operator=(const PaintLayerPaintOrderIterator&) =
      delete;

  PaintLayer* Next();

  const GCedHeapVector<Member<PaintLayer>>*
  LayersPaintingOverlayOverflowControlsAfter(const PaintLayer* layer) const {
    return root_->stacking_node_
               ? root_->stacking_node_
                     ->LayersPaintingOverlayOverflowControlsAfter(layer)
               : nullptr;
  }

 private:
  const PaintLayer* root_;
  unsigned remaining_children_;
  unsigned index_;
  PaintLayer* current_normal_flow_child_;
#if DCHECK_IS_ON()
  PaintLayerListMutationDetector mutation_detector_;
#endif
};

// This iterator is similar to PaintLayerPaintOrderIterator but it walks the
// lists in reverse order (from the last item to the first one).
class CORE_EXPORT PaintLayerPaintOrderReverseIterator {
  STACK_ALLOCATED();

 public:
  PaintLayerPaintOrderReverseIterator(const PaintLayer* root,
                                      unsigned which_children)
      : root_(root),
        remaining_children_(which_children)
#if DCHECK_IS_ON()
        ,
        mutation_detector_(root)
#endif
  {
    SetIndexToLastItem();
  }
  PaintLayerPaintOrderReverseIterator(
      const PaintLayerPaintOrderReverseIterator&) = delete;
  PaintLayerPaintOrderReverseIterator& operator=(
      const PaintLayerPaintOrderReverseIterator&) = delete;

  PaintLayer* Next();

 private:
  void SetIndexToLastItem();

  const PaintLayer* root_;
  unsigned remaining_children_;
  int index_;
  PaintLayer* current_normal_flow_child_;
#if DCHECK_IS_ON()
  PaintLayerListMutationDetector mutation_detector_;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_PAINT_ORDER_ITERATOR_H_

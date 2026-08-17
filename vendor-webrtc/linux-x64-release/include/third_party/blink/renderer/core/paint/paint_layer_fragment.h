/*
 * Copyright (C) 2003, 2009, 2012 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_FRAGMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_FRAGMENT_H_

#include "third_party/blink/renderer/core/paint/clip_rect.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class FragmentData;
class PhysicalBoxFragment;

// PaintLayerFragment is the representation of a fragment of a PaintLayer.
// https://drafts.csswg.org/css-break/#fragment
//
// The fragments are collected by calling PaintLayer::CollectFragments()
// on every box once per hit-testing operation.
struct PaintLayerFragment {
  DISALLOW_NEW();

 public:
  // The coordinate space of |layer_offset|, |background_rect| and
  // |foreground_rect| is defined by the parameters of
  // PaintLayer::CollectFragments():
  // - |root_fragment| if it's not nullptr,
  // - or the first fragment of |root_layer|.

  // The PaintLayer's offset in the space defined above.
  PhysicalOffset layer_offset;

  // The rectangle used to clip the background.
  //
  // The rectangle is the rectangle-to-paint if no clip applies to the
  // fragment. It is the intersection of
  // - the visual overflow rect and
  // - all clips between |root_fragment_data->LocalBorderBoxProperties()
  //   .Clip()| (not included) and |fragment_data->PreClip()| (included).
  //
  // See PaintLayerClipper::CalculateRects.
  ClipRect background_rect;

  // The rectangle used to clip the content (foreground).
  //
  // The rectangle is the rectangle-to-paint if no clip applies to the
  // fragment. If the layer should apply overflow clip, the rectangle is the
  // intersection of |background_rect| and the overflow clip rect. Otherwise
  // it's the same as |background_rect|.
  //
  // See PaintLayerClipper::CalculateRects.
  ClipRect foreground_rect;

  // The fragment index of fragment_data / physical_fragment.
  wtf_size_t fragment_idx = WTF::kNotFound;

  // The corresponding FragmentData of this structure.
  Member<const FragmentData> fragment_data = nullptr;

  Member<const PhysicalBoxFragment> physical_fragment = nullptr;

  void Trace(Visitor*) const;
};

typedef HeapVector<PaintLayerFragment, 1> PaintLayerFragments;

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::PaintLayerFragment)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_FRAGMENT_H_

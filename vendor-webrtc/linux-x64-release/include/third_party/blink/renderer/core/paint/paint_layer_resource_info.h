/*
 * Copyright (C) 2012 Adobe Systems Incorporated. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer.
 * 2. Redistributions in binary form must reproduce the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer in the documentation and/or other materials
 *    provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDER "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY,
 * OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
 * TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
 * THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_RESOURCE_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_RESOURCE_INFO_H_

#include <optional>

#include "third_party/blink/renderer/core/svg/svg_resource_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class PaintLayer;

// PaintLayerResourceInfo holds the filter information for painting
// https://drafts.fxtf.org/filter-effects/. It also acts as the resource client
// for change notifications from <clipPath> elements for the clip-path property.
//
// Because PaintLayer is not allocated for SVG objects, SVG filters (both
// software and hardware-accelerated) use a different code path to paint the
// filters (SVGFilterPainter), but both code paths use the same abstraction for
// painting non-hardware accelerated filters (FilterEffect). Hardware
// accelerated CSS filters use CompositorFilterOperations, that is backed by cc.
class PaintLayerResourceInfo final
    : public GarbageCollected<PaintLayerResourceInfo>,
      public SVGResourceClient {
 public:
  explicit PaintLayerResourceInfo(PaintLayer*);
  PaintLayerResourceInfo(const PaintLayerResourceInfo&) = delete;
  PaintLayerResourceInfo& operator=(const PaintLayerResourceInfo&) = delete;
  ~PaintLayerResourceInfo() override;

  gfx::RectF FilterReferenceBox() const { return filter_reference_box_; }
  void SetFilterReferenceBox(const gfx::RectF& rect) {
    filter_reference_box_ = rect;
  }
  const std::optional<gfx::SizeF>& FilterViewport() const {
    return filter_viewport_;
  }
  void SetFilterViewport(std::optional<gfx::SizeF> viewport) {
    filter_viewport_ = viewport;
  }

  void ClearLayer() { layer_ = nullptr; }

  void ResourceContentChanged(SVGResource*) override;

  void Trace(Visitor* visitor) const override { visitor->Trace(layer_); }

 private:
  // |ClearLayer| must be called before *layer_ becomes invalid.
  Member<PaintLayer> layer_;
  gfx::RectF filter_reference_box_;
  std::optional<gfx::SizeF> filter_viewport_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_RESOURCE_INFO_H_

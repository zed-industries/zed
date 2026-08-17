/*
 * Copyright (C) 2004, 2006, 2007, 2009 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_HTML_CANVAS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_HTML_CANVAS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_replaced.h"

namespace blink {

class HTMLCanvasElement;

class CORE_EXPORT LayoutHTMLCanvas final : public LayoutReplaced {
 public:
  explicit LayoutHTMLCanvas(HTMLCanvasElement*);

  bool IsCanvas() const final {
    NOT_DESTROYED();
    return true;
  }

  void InvalidatePaint(const PaintInvalidatorContext&) const final;

  void CanvasSizeChanged();

  bool DrawsBackgroundOntoContentLayer() const final;

  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutHTMLCanvas";
  }

  void WillBeDestroyed() override;

  void Trace(Visitor*) const override;

  LayoutObject* FirstChild() const {
    NOT_DESTROYED();
    DCHECK_EQ(Children(), VirtualChildren());
    return Children()->FirstChild();
  }
  LayoutObject* LastChild() const {
    NOT_DESTROYED();
    DCHECK_EQ(Children(), VirtualChildren());
    return Children()->LastChild();
  }

  // As with LayoutMedia, use firstChild or lastChild instead.
  void SlowFirstChild() const = delete;
  void SlowLastChild() const = delete;

  const LayoutObjectChildList* Children() const {
    NOT_DESTROYED();
    return &children_;
  }
  LayoutObjectChildList* Children() {
    NOT_DESTROYED();
    return &children_;
  }

  void DidInvalidatePaintForPlacedElement(Element* placedElement);

 private:
  LayoutObjectChildList* VirtualChildren() final {
    NOT_DESTROYED();
    return Children();
  }
  const LayoutObjectChildList* VirtualChildren() const final {
    NOT_DESTROYED();
    return Children();
  }
  bool CanHaveChildren() const final {
    NOT_DESTROYED();
    return RuntimeEnabledFeatures::CanvasPlaceElementEnabled();
  }
  bool IsChildAllowed(LayoutObject*, const ComputedStyle&) const final;

  void PaintReplaced(const PaintInfo&,
                     const PhysicalOffset& paint_offset) const override;
  void NaturalSizeChanged() override {
    NOT_DESTROYED();
    CanvasSizeChanged();
  }
  PhysicalNaturalSizingInfo GetNaturalDimensions() const override;

  LayoutObjectChildList children_;
  PhysicalSize natural_size_;
};

template <>
struct DowncastTraits<LayoutHTMLCanvas> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsCanvas();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_HTML_CANVAS_H_

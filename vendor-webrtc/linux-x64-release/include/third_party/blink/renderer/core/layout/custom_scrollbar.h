/*
 * Copyright (C) 2008, 2009 Apple Inc. All Rights Reserved.
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
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_SCROLLBAR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_SCROLLBAR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/scroll/scrollbar.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/platform/geometry/physical_offset.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class ComputedStyle;
class GraphicsContext;
class LayoutObject;
class LayoutCustomScrollbarPart;

// Custom scrollbars are created when a box has -webkit-scrollbar* pseudo
// styles. The parts of a custom scrollbar are layout objects of class
// LayoutCustomScrollbarPart.
class CORE_EXPORT CustomScrollbar final : public Scrollbar {
 public:
  CustomScrollbar(ScrollableArea*,
                  ScrollbarOrientation,
                  const LayoutObject* style_source,
                  bool suppress_use_counters = false);
  ~CustomScrollbar() override;

  // Return the thickness that a custom scrollbar would have, before actually
  // constructing the real scrollbar.
  static int HypotheticalScrollbarThickness(const ScrollableArea*,
                                            ScrollbarOrientation,
                                            const LayoutObject* style_source);

  gfx::Rect ButtonRect(ScrollbarPart) const;
  gfx::Rect TrackRect(int start_length, int end_length) const;
  gfx::Rect TrackPieceRectWithMargins(ScrollbarPart, const gfx::Rect&) const;

  int MinimumThumbLength() const;

  bool IsOverlayScrollbar() const override { return false; }

  void OffsetDidChange(mojom::blink::ScrollType) override;

  void PositionScrollbarParts();

  // Custom scrollbars may be translucent.
  bool IsOpaque() const override { return false; }

  LayoutCustomScrollbarPart* GetPart(ScrollbarPart part_type) {
    auto it = parts_.find(part_type);
    return it != parts_.end() ? it->value.Get() : nullptr;
  }
  const LayoutCustomScrollbarPart* GetPart(ScrollbarPart part_type) const {
    auto it = parts_.find(part_type);
    return it != parts_.end() ? it->value.Get() : nullptr;
  }

  // Although this method returns an entire ComputedStyle, it is only used when
  // computing a cursor to use.
  // This method implements a cursor-specific, inheritance-like fallback for
  // ScrollbarParts that aren't used.
  // For example: without this fallback, hovering over a scrollbar-track on a
  // scrollbar styled only with `::-webkit-scrollbar` and
  // `::webkit-scroll-thumb` will surprisingly use the cursor style from the
  // originating element (the scroller) since the scrollbar-track will not have
  // a corresponding LayoutCustomScrollbarPart. In this case, it'd be
  // preferable to use the cursor style set in the `::webkit-scrollbar`
  const ComputedStyle* GetScrollbarPartStyleForCursor(
      ScrollbarPart part_type) const;

  void InvalidateDisplayItemClientsOfScrollbarParts();
  void ClearPaintFlags();

  void Paint(GraphicsContext&, const PhysicalOffset& paint_offset) const;

  void Trace(Visitor*) const override;

 private:
  friend class Scrollbar;

  void SetEnabled(bool) override;
  void DisconnectFromScrollableArea() override;

  void SetHoveredPart(ScrollbarPart) override;
  void SetPressedPart(ScrollbarPart, WebInputEvent::Type) override;

  void StyleChanged() override;

  bool IsCustomScrollbar() const override { return true; }

  void DestroyScrollbarParts();
  void UpdateScrollbarParts();
  const ComputedStyle* GetScrollbarPseudoElementStyle(ScrollbarPart, PseudoId);
  void UpdateScrollbarPart(ScrollbarPart);

  HeapHashMap<ScrollbarPart, Member<LayoutCustomScrollbarPart>> parts_;
  bool needs_position_scrollbar_parts_ = true;
  // When constructing a CustomScrollbar solely for the purpose of computing
  // hypothetical thickness, don't record feature usage.
  bool suppress_use_counters_ = false;
};

template <>
struct DowncastTraits<CustomScrollbar> {
  static bool AllowFrom(const Scrollbar& scrollbar) {
    return scrollbar.IsCustomScrollbar();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_SCROLLBAR_H_

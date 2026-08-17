// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_RESIZE_VIEWPORT_ANCHOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_RESIZE_VIEWPORT_ANCHOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/page/page.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class LocalFrameView;

// This class scrolls the viewports to compensate for bounds clamping caused by
// viewport size changes.
//
// It is needed when the layout viewport grows (causing its own scroll position
// to be clamped) and also when it shrinks (causing the visual viewport's scroll
// position to be clamped).
class CORE_EXPORT ResizeViewportAnchor final
    : public GarbageCollected<ResizeViewportAnchor> {
 public:
  explicit ResizeViewportAnchor(Page& page) : page_(page), scope_count_(0) {}
  ResizeViewportAnchor(const ResizeViewportAnchor&) = delete;
  ResizeViewportAnchor& operator=(const ResizeViewportAnchor&) = delete;

  class ResizeScope {
    STACK_ALLOCATED();

   public:
    explicit ResizeScope(ResizeViewportAnchor& anchor) : anchor_(&anchor) {
      anchor_->BeginScope();
    }
    ~ResizeScope() { anchor_->EndScope(); }

   private:
    ResizeViewportAnchor* anchor_;
  };

  void ResizeFrameView(const gfx::Size&);

  void Trace(Visitor* visitor) const { visitor->Trace(page_); }

 private:
  void BeginScope() { scope_count_++; }
  void EndScope();
  LocalFrameView* RootFrameView();

  // The amount of resize-induced clamping drift accumulated during the
  // ResizeScope.  Note that this should NOT include other kinds of scrolling
  // that may occur during layout, such as from ScrollAnchor.
  ScrollOffset drift_;
  Member<Page> page_;
  int scope_count_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_RESIZE_VIEWPORT_ANCHOR_H_

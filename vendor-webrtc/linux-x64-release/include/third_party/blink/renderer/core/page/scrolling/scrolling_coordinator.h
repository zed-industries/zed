/*
 * Copyright (C) 2011 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS''
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 * THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_SCROLLING_COORDINATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_SCROLLING_COORDINATOR_H_

#include "base/memory/weak_ptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/scroll/scroll_types.h"
#include "third_party/blink/renderer/platform/graphics/compositing/paint_artifact_compositor.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"

namespace blink {

class LocalFrame;
class Page;
class ScrollableArea;

// ScrollingCoordinator is a page-level object that mediates scroll-related
// interactions between Blink and the compositor.
class CORE_EXPORT ScrollingCoordinator final
    : public GarbageCollected<ScrollingCoordinator> {
 public:
  explicit ScrollingCoordinator(Page*);
  ScrollingCoordinator(const ScrollingCoordinator&) = delete;
  ScrollingCoordinator& operator=(const ScrollingCoordinator&) = delete;
  ~ScrollingCoordinator();
  void Trace(Visitor*) const;

  void WillBeDestroyed();

  // Updates scroll offset in cc scroll tree immediately. We don't wait for
  // a full document lifecycle update to propagate the scroll offset from blink
  // paint properties to cc paint properties because cc needs the scroll offset
  // to apply the impl-side scroll delta correctly at the beginning of the next
  // frame. The scroll offset in the transform node will still be updated
  // in normal document lifecycle update instead of here.
  // Returns whether the update is successful.
  bool UpdateCompositorScrollOffset(const LocalFrame&, const ScrollableArea&);

  // Traverses the frame tree to find the scrollable area using the element id.
  // This function only checks the local frames. This function does not check
  // the VisualViewport element id.
  ScrollableArea* ScrollableAreaWithElementIdInAllLocalFrames(
      const CompositorElementId&);

  // ScrollCallbacks implementation
  void DidCompositorScroll(CompositorElementId,
                           const gfx::PointF&,
                           const std::optional<cc::TargetSnapAreaElementIds>&);
  void DidChangeScrollbarsHidden(CompositorElementId, bool hidden);

  base::WeakPtr<CompositorScrollCallbacks> GetScrollCallbacks() {
    DCHECK(page_);
    if (!callbacks_) {
      callbacks_ = std::make_unique<CallbackProxy>(this);
    }
    return callbacks_->GetWeakPtr();
  }

 protected:
  Member<Page> page_;

 private:
  // This class adapts a base::WeakPtr into a GC-aware weak reference to the
  // ScrollingCoordinator. The cc::ScrollTree needs a WeakPtr since it lives
  // outside of Blink, but we cannot safely take a WeakPtr to a GC object
  // (crbug.com/1485318, crbug.com/1246423). So we hand out a WeakPtr to a
  // non-GC'ed proxy that holds a WeakPersistent to the ScrollingCoordinator.
  class CallbackProxy : public CompositorScrollCallbacks {
   public:
    explicit CallbackProxy(ScrollingCoordinator* sc)
        : scrolling_coordinator_(sc) {}
    base::WeakPtr<CompositorScrollCallbacks> GetWeakPtr() {
      return weak_ptr_factory_.GetWeakPtr();
    }
    void DidCompositorScroll(CompositorElementId element_id,
                             const gfx::PointF& offset,
                             const std::optional<cc::TargetSnapAreaElementIds>&
                                 snap_target_ids) override {
      if (ScrollingCoordinator* sc = scrolling_coordinator_.Get()) {
        sc->DidCompositorScroll(element_id, offset, snap_target_ids);
      }
    }
    void DidChangeScrollbarsHidden(CompositorElementId element_id,
                                   bool hidden) override {
      if (ScrollingCoordinator* sc = scrolling_coordinator_.Get()) {
        sc->DidChangeScrollbarsHidden(element_id, hidden);
      }
    }

   private:
    base::WeakPtrFactory<CompositorScrollCallbacks> weak_ptr_factory_{this};
    WeakPersistent<ScrollingCoordinator> scrolling_coordinator_;
  };
  std::unique_ptr<CallbackProxy> callbacks_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_SCROLLING_SCROLLING_COORDINATOR_H_

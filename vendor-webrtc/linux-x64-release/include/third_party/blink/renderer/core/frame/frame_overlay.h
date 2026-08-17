/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * 1. Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY GOOGLE INC. AND ITS CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL GOOGLE INC.
 * OR ITS CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_OVERLAY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_OVERLAY_H_

#include <memory>
#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class GraphicsContext;
class LocalFrame;
class PropertyTreeState;

// Manages a layer that is overlaid on a WebLocalFrame's content.
class CORE_EXPORT FrameOverlay : public GarbageCollected<FrameOverlay>,
                                 public DisplayItemClient {
 public:
  class Delegate {
   public:
    virtual ~Delegate() = default;

    // Paints frame overlay contents.
    virtual void PaintFrameOverlay(const FrameOverlay&,
                                   GraphicsContext&,
                                   const gfx::Size& view_size) const = 0;
    // Invalidates composited layers managed by the delegate, if any.
    virtual void Invalidate() {}

    // Service any animations managed by the delegate.
    virtual void ServiceScriptedAnimations(
        base::TimeTicks monotonic_frame_begin_time) {}
  };

  // |Destroy()| should be called when it is no longer used.
  FrameOverlay(LocalFrame*, std::unique_ptr<FrameOverlay::Delegate>);
  ~FrameOverlay() override;
  void Destroy();

  void UpdatePrePaint();

  void Paint(GraphicsContext&) const;

  // FrameOverlay is always the same size as the viewport.
  gfx::Size Size() const;

  const Delegate* GetDelegate() const { return delegate_.get(); }
  const LocalFrame& Frame() const { return *frame_; }

  // Services any animations that the overlay may be managing.
  void ServiceScriptedAnimations(base::TimeTicks monotonic_frame_begin_time);

  // DisplayItemClient.
  String DebugName() const final { return "FrameOverlay"; }

  void Trace(Visitor*) const override;

  PropertyTreeState DefaultPropertyTreeState() const;

 private:
  Member<LocalFrame> frame_;
  std::unique_ptr<FrameOverlay::Delegate> delegate_;

#if DCHECK_IS_ON()
  bool is_destroyed_ = false;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_OVERLAY_H_

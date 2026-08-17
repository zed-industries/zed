// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_OWNER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_OWNER_H_

#include "third_party/blink/public/common/frame/frame_policy.h"
#include "third_party/blink/public/mojom/css/preferred_color_scheme.mojom-blink.h"
#include "third_party/blink/public/mojom/frame/color_scheme.mojom-blink.h"
#include "third_party/blink/public/mojom/scroll/scrollbar_mode.mojom-blink.h"
#include "third_party/blink/public/mojom/timing/resource_timing.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class Frame;
class FrameSwapScope;

// Oilpan: all FrameOwner instances are GCed objects. FrameOwner additionally
// derives from GarbageCollectedMixin so that Member<FrameOwner> references can
// be kept (e.g., Frame::m_owner.)
class CORE_EXPORT FrameOwner : public GarbageCollectedMixin {
  friend class FrameSwapScope;

 public:
  virtual ~FrameOwner() = default;

  void Trace(Visitor* visitor) const override {}

  virtual bool IsLocal() const = 0;
  virtual bool IsRemote() const = 0;
  virtual bool IsPlugin() const { return false; }

  virtual Frame* ContentFrame() const = 0;
  virtual void SetContentFrame(Frame&) = 0;
  virtual void ClearContentFrame() = 0;
  virtual const FramePolicy& GetFramePolicy() const = 0;
  // Note: there is a subtle ordering dependency here: if a page load needs to
  // report resource timing information, it *must* do so before calling
  // DispatchLoad().
  virtual void AddResourceTiming(mojom::blink::ResourceTimingInfoPtr) = 0;
  virtual void DispatchLoad() = 0;

  // The natural dimensions of the embedded object changed. This is only
  // relevant for SVG documents that are embedded via <object> or <embed>.
  virtual void NaturalSizingInfoChanged() = 0;

  // Indicates that a child frame requires its parent frame to track whether the
  // child frame is occluded or has visual effects applied.
  virtual void SetNeedsOcclusionTracking(bool) = 0;

  // Returns the 'name' content attribute value of the browsing context
  // container.
  // https://html.spec.whatwg.org/C/#browsing-context-container
  virtual AtomicString BrowsingContextContainerName() const = 0;
  virtual mojom::blink::ScrollbarMode ScrollbarMode() const = 0;
  virtual int MarginWidth() const = 0;
  virtual int MarginHeight() const = 0;
  virtual bool AllowFullscreen() const = 0;
  virtual bool AllowPaymentRequest() const = 0;
  virtual bool IsDisplayNone() const = 0;
  virtual mojom::blink::ColorScheme GetColorScheme() const = 0;
  virtual mojom::blink::PreferredColorScheme GetPreferredColorScheme()
      const = 0;

  // Returns whether or not children of the owned frame should be lazily loaded.
  virtual bool ShouldLazyLoadChildren() const = 0;

  // Returns whether this is an iframe with the credentialless attribute set.
  // [spec]
  // https://wicg.github.io/anonymous-iframe/#dom-htmliframeelement-credentialless
  virtual bool Credentialless() const { return false; }

 protected:
  virtual void FrameOwnerPropertiesChanged() {}
  virtual void DidChangeAttributes() {}

 private:
  virtual void SetIsSwappingFrames(bool) {}
};

// The purpose of this class is to suppress the propagation of frame owner
// properties while a frame is being replaced. In particular, it prevents the
// erroneous propagation of is_display_none=true, which would otherwise happen
// when the old frame is detached prior to attaching the new frame. This class
// will postpone the propagation until the properties are in their new stable
// state.
//
// It is only intended to handle cases where one frame is detached and a new
// frame immediately attached. For normal frame unload/teardown, we don't need
// to suppress the propagation.
class FrameSwapScope {
  STACK_ALLOCATED();

 public:
  FrameSwapScope(FrameOwner* frame_owner) : frame_owner_(frame_owner) {
    if (frame_owner)
      frame_owner->SetIsSwappingFrames(true);
  }

  ~FrameSwapScope() {
    if (frame_owner_) {
      frame_owner_->SetIsSwappingFrames(false);
      frame_owner_->FrameOwnerPropertiesChanged();
      frame_owner_->DidChangeAttributes();
    }
  }

 private:
  FrameOwner* frame_owner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_OWNER_H_

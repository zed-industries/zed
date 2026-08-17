// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_CLIENT_H_

#include "base/unguessable_token.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace gfx {
class Rect;
}

namespace blink {

class LocalFrame;
enum class FrameDetachType;

class CORE_EXPORT FrameClient : public GarbageCollected<FrameClient> {
 public:
  virtual bool InShadowTree() const = 0;

  virtual void Detached(FrameDetachType) = 0;

  virtual unsigned BackForwardLength() = 0;

  // For the main frame, called when the main frame's dimensions have changed,
  // e.g. resizing a tab causes the document width to change; loading additional
  // content causes the document height to increase; explicitly changing the
  // height of the body element.
  //
  // For a subframe, called when the intersection rect between the main frame
  // and the subframe has changed, e.g. the subframe is initially added; the
  // subframe's position is updated explicitly or inherently (e.g. sticky
  // position while the page is being scrolled).
  virtual void OnMainFrameIntersectionChanged(
      const gfx::Rect& main_frame_intersection_rect) {}

  // Called when the main frame's viewport rectangle (the viewport dimensions
  // and the scroll position) changed, e.g. the user scrolled the main frame or
  // the viewport dimensions themselves changed. Only invoked on the main frame.
  virtual void OnMainFrameViewportRectangleChanged(
      const gfx::Rect& main_frame_viewport_rect) {}

  // Called when an image ad rectangle changed. An empty `image_ad_rect` is used
  // to signal the removal of the rectangle. Only invoked on the main frame.
  virtual void OnMainFrameImageAdRectangleChanged(
      DOMNodeId element_id,
      const gfx::Rect& image_ad_rect) {}

  virtual ~FrameClient() = default;

  virtual void Trace(Visitor* visitor) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_FRAME_CLIENT_H_

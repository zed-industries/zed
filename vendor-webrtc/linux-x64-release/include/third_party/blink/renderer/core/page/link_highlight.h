// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_LINK_HIGHLIGHT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_LINK_HIGHLIGHT_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace cc {
class AnimationHost;
class AnimationTimeline;
}

namespace blink {
class GraphicsContext;
class LinkHighlightImpl;
class LayoutObject;
class LocalFrame;
class Node;
class Page;
class PaintArtifactCompositor;

class CORE_EXPORT LinkHighlight final : public GarbageCollected<LinkHighlight> {
 public:
  explicit LinkHighlight(Page&);
  ~LinkHighlight();

  void Trace(Visitor*) const;

  void ResetForPageNavigation();

  void SetTapHighlight(Node*);

  void UpdateOpacityAndRequestAnimation();

  void AnimationHostInitialized(cc::AnimationHost&);
  void WillCloseAnimationHost();

  bool IsHighlighting(const LayoutObject& object) const {
    return impl_ && IsHighlightingInternal(object);
  }

  void UpdateBeforePrePaint();
  void UpdateAfterPrePaint();
  void Paint(GraphicsContext&) const;
  void UpdateAfterPaint(const PaintArtifactCompositor*);

 private:
  friend class LinkHighlightImplTest;

  void RemoveHighlight();

  LocalFrame* MainFrame() const;

  Page& GetPage() const {
    DCHECK(page_);
    return *page_;
  }

  bool IsHighlightingInternal(const LayoutObject& object) const;

  Member<Page> page_;
  std::unique_ptr<LinkHighlightImpl> impl_;
  cc::AnimationHost* animation_host_ = nullptr;
  scoped_refptr<cc::AnimationTimeline> timeline_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_LINK_HIGHLIGHT_H_

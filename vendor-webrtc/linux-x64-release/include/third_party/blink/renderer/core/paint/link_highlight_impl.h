/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_LINK_HIGHLIGHT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_LINK_HIGHLIGHT_IMPL_H_

#include <memory>

#include "base/time/time.h"
#include "cc/layers/content_layer_client.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/platform/animation/compositor_animation.h"
#include "third_party/blink/renderer/platform/animation/compositor_animation_client.h"
#include "third_party/blink/renderer/platform/animation/compositor_animation_delegate.h"
#include "third_party/blink/renderer/platform/geometry/path.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace cc {
class Layer;
class PictureLayer;
}  // namespace cc

namespace blink {

class EffectPaintPropertyNode;
class GraphicsContext;
class PaintArtifactCompositor;

class CORE_EXPORT LinkHighlightImpl final : public CompositorAnimationDelegate,
                                            public CompositorAnimationClient {
  USING_FAST_MALLOC(LinkHighlightImpl);

 public:
  explicit LinkHighlightImpl(Node*);
  ~LinkHighlightImpl() override;

  void UpdateOpacityAndRequestAnimation();

  // CompositorAnimationDelegate implementation.
  void NotifyAnimationStarted(base::TimeDelta monotonic_time,
                              int group) override {}
  void NotifyAnimationFinished(base::TimeDelta monotonic_time,
                               int group) override;
  void NotifyAnimationAborted(base::TimeDelta monotonic_time,
                              int group) override {}

  // CompositorAnimationClient implementation.
  CompositorAnimation* GetCompositorAnimation() const override;

  LayoutObject* GetLayoutObject() const {
    return node_ ? node_->GetLayoutObject() : nullptr;
  }

  CompositorElementId ElementIdForTesting() const { return element_id_; }

  const EffectPaintPropertyNode& Effect() const { return *effect_; }

  void UpdateBeforePrePaint();
  void UpdateAfterPrePaint();
  void Paint(GraphicsContext&);
  void UpdateAfterPaint(
      const PaintArtifactCompositor* paint_artifact_compositor);

  wtf_size_t FragmentCountForTesting() const { return fragments_.size(); }
  cc::PictureLayer* LayerForTesting(wtf_size_t index) const {
    return fragments_[index]->Layer();
  }

 private:
  void ReleaseResources();

  void StartCompositorAnimation();
  void StopCompositorAnimation();
  void SetNeedsRepaintAndCompositingUpdate();
  void UpdateOpacity(float opacity);

  class LinkHighlightFragment : public cc::ContentLayerClient {
   public:
    LinkHighlightFragment();
    ~LinkHighlightFragment() override;

    cc::PictureLayer* Layer() const { return layer_.get(); }
    const Path& GetPath() const { return path_; }
    void SetPath(const Path& path) { path_ = path; }
    void SetColor(const Color& color) { color_ = color; }

   private:
    // cc::ContentLayerClient implementation.
    scoped_refptr<cc::DisplayItemList> PaintContentsToDisplayList() override;
    bool FillsBoundsCompletely() const override { return false; }

    scoped_refptr<cc::PictureLayer> layer_;
    Path path_;
    Color color_;
  };
  Vector<std::unique_ptr<LinkHighlightFragment>> fragments_;

  WeakPersistent<Node> node_;
  std::unique_ptr<CompositorAnimation> compositor_animation_;
  Persistent<EffectPaintPropertyNode> effect_;

  // True if an animation has been requested.
  bool start_compositor_animation_ = false;
  bool is_animating_on_compositor_ = false;
  int compositor_keyframe_model_id_ = 0;
  base::TimeTicks start_time_;
  CompositorElementId element_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_LINK_HIGHLIGHT_IMPL_H_

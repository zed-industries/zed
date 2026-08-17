// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_EMBEDDED_CONTENT_VIEW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_EMBEDDED_CONTENT_VIEW_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/paint/paint_flags.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/vector2d.h"

namespace blink {

class CullRect;
class LayoutEmbeddedContent;
class LocalFrameView;
class GraphicsContext;

// EmbeddedContentView is a pure virtual class which is implemented by
// LocalFrameView, RemoteFrameView, and WebPluginContainerImpl.
class CORE_EXPORT EmbeddedContentView : public GarbageCollectedMixin {
 public:
  EmbeddedContentView(const gfx::Rect& frame_rect) : frame_rect_(frame_rect) {}
  virtual ~EmbeddedContentView() = default;

  virtual bool IsFrameView() const { return false; }
  virtual bool IsLocalFrameView() const { return false; }
  virtual bool IsPluginView() const { return false; }

  virtual LocalFrameView* ParentFrameView() const = 0;
  virtual LayoutEmbeddedContent* GetLayoutEmbeddedContent() const = 0;
  virtual void AttachToLayout() = 0;
  virtual void DetachFromLayout() = 0;
  // |cull_rect| is in the same coordinate space as Location() and FrameRect().
  // |paint_offset| is Location() mapped into the current coordinates space of
  // the current paint context.
  virtual void Paint(GraphicsContext&,
                     PaintFlags,
                     const CullRect& cull_rect,
                     const gfx::Vector2d& paint_offset) const = 0;
  // Called when the size of the view changes.  Implementations of
  // EmbeddedContentView should call LayoutEmbeddedContent::UpdateGeometry in
  // addition to any internal logic.
  virtual void UpdateGeometry() = 0;
  virtual void Show() = 0;
  virtual void Hide() = 0;
  virtual void Dispose() = 0;

  virtual void SetFrameRect(const gfx::Rect&);

  // This method pushes information about our frame rect to consumers.
  // Typically, it will be invoked by FrameRectsChanged; but it can also be
  // called directly to push frame rect information without changing it.
  virtual void PropagateFrameRects() = 0;

  // See WebFrameWidgetImpl::SetZoomLevel() for how this value is used.
  virtual void ZoomFactorChanged(float zoom_factor) {}

  gfx::Rect FrameRect() const { return gfx::Rect(Location(), Size()); }
  gfx::Point Location() const;
  int Width() const { return Size().width(); }
  int Height() const { return Size().height(); }
  gfx::Size Size() const { return frame_rect_.size(); }
  void Resize(int width, int height) { Resize(gfx::Size(width, height)); }
  void Resize(const gfx::Size& size) {
    SetFrameRect(gfx::Rect(frame_rect_.origin(), size));
  }
  bool IsAttached() const { return is_attached_; }
  // The visibility flags are set for iframes based on style properties of the
  // HTMLFrameOwnerElement in the embedding document.
  bool IsSelfVisible() const { return self_visible_; }
  void SetSelfVisible(bool);
  bool IsParentVisible() const { return parent_visible_; }
  void SetParentVisible(bool);
  bool IsVisible() const { return self_visible_ && parent_visible_; }

 protected:
  // Called when our frame rect changes (or the rect/scroll offset of an
  // ancestor changes).
  virtual void FrameRectsChanged(const gfx::Rect&) { PropagateFrameRects(); }
  virtual void SelfVisibleChanged() {}
  virtual void ParentVisibleChanged() {}
  void SetAttached(bool attached) { is_attached_ = attached; }

 private:
  // Note that frame_rect_ is actually in document coordinates, but the
  // FrameRect() and Location() methods convert to frame coordinates.
  gfx::Rect frame_rect_;
  bool self_visible_;
  bool parent_visible_;
  bool is_attached_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_EMBEDDED_CONTENT_VIEW_H_

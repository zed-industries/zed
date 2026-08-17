/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_INTERCEPTING_CANVAS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_INTERCEPTING_CANVAS_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_record.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/skia/include/core/SkBitmap.h"
#include "third_party/skia/include/core/SkCanvas.h"

namespace blink {

class InterceptingCanvasBase : public SkCanvas {
 public:
  template <typename DerivedCanvas>
  class CanvasInterceptorBase {
    STACK_ALLOCATED();

   public:
    CanvasInterceptorBase(const CanvasInterceptorBase&) = delete;
    CanvasInterceptorBase& operator=(const CanvasInterceptorBase&) = delete;

   protected:
    CanvasInterceptorBase(InterceptingCanvasBase* canvas) : canvas_(canvas) {
      ++canvas_->call_nesting_depth_;
    }

    ~CanvasInterceptorBase() {
      DCHECK_GT(canvas_->call_nesting_depth_, 0u);
      if (!--canvas_->call_nesting_depth_)
        canvas_->call_count_++;
    }

    DerivedCanvas* Canvas() { return static_cast<DerivedCanvas*>(canvas_); }
    bool TopLevelCall() const { return canvas_->CallNestingDepth() == 1; }
    InterceptingCanvasBase* canvas_;
  };

  InterceptingCanvasBase(const InterceptingCanvasBase&) = delete;
  InterceptingCanvasBase& operator=(const InterceptingCanvasBase&) = delete;

  void ResetStepCount() { call_count_ = 0; }

 protected:
  explicit InterceptingCanvasBase(SkBitmap bitmap)
      : SkCanvas(bitmap, SkSurfaceProps{}),
        call_nesting_depth_(0),
        call_count_(0) {}
  InterceptingCanvasBase(int width, int height)
      : SkCanvas(width, height), call_nesting_depth_(0), call_count_(0) {}

  void UnrollDrawPicture(const SkPicture*,
                         const SkMatrix*,
                         const SkPaint*,
                         SkPicture::AbortCallback*);

  void onDrawPaint(const SkPaint&) override = 0;
  void onDrawPoints(PointMode,
                    size_t count,
                    const SkPoint pts[],
                    const SkPaint&) override = 0;
  void onDrawRect(const SkRect&, const SkPaint&) override = 0;
  void onDrawOval(const SkRect&, const SkPaint&) override = 0;
  void onDrawRRect(const SkRRect&, const SkPaint&) override = 0;
  void onDrawPath(const SkPath&, const SkPaint&) override = 0;
  void onDrawImage2(const SkImage*,
                    SkScalar,
                    SkScalar,
                    const SkSamplingOptions&,
                    const SkPaint*) override = 0;
  void onDrawImageRect2(const SkImage*,
                        const SkRect& src,
                        const SkRect& dst,
                        const SkSamplingOptions&,
                        const SkPaint*,
                        SrcRectConstraint) override = 0;
  void onDrawVerticesObject(const SkVertices*,
                            SkBlendMode bmode,
                            const SkPaint&) override = 0;

  void onDrawDRRect(const SkRRect& outer,
                    const SkRRect& inner,
                    const SkPaint&) override = 0;
  void onDrawTextBlob(const SkTextBlob*,
                      SkScalar x,
                      SkScalar y,
                      const SkPaint&) override = 0;
  void onClipRect(const SkRect&, SkClipOp, ClipEdgeStyle) override = 0;
  void onClipRRect(const SkRRect&, SkClipOp, ClipEdgeStyle) override = 0;
  void onClipPath(const SkPath&, SkClipOp, ClipEdgeStyle) override = 0;
  void onClipRegion(const SkRegion&, SkClipOp) override = 0;
  void onDrawPicture(const SkPicture*,
                     const SkMatrix*,
                     const SkPaint*) override = 0;
  void didSetM44(const SkM44&) override = 0;
  void didConcat44(const SkM44&) override = 0;
  void didScale(SkScalar, SkScalar) override = 0;
  void didTranslate(SkScalar, SkScalar) override = 0;
  void willSave() override = 0;
  SaveLayerStrategy getSaveLayerStrategy(const SaveLayerRec&) override = 0;
  void willRestore() override = 0;

  unsigned CallNestingDepth() const { return call_nesting_depth_; }
  unsigned CallCount() const { return call_count_; }

 private:
  unsigned call_nesting_depth_;
  unsigned call_count_;
};

template <typename DerivedCanvas>
class CanvasInterceptor {};

template <typename DerivedCanvas,
          typename Interceptor = CanvasInterceptor<DerivedCanvas>>
class InterceptingCanvas : public InterceptingCanvasBase {
 protected:
  explicit InterceptingCanvas(SkBitmap bitmap)
      : InterceptingCanvasBase(bitmap) {}
  InterceptingCanvas(int width, int height)
      : InterceptingCanvasBase(width, height) {}

  void onDrawPaint(const SkPaint& paint) override {
    Interceptor interceptor(this);
    SkCanvas::onDrawPaint(paint);
  }

  void onDrawPoints(PointMode mode,
                    size_t count,
                    const SkPoint pts[],
                    const SkPaint& paint) override {
    Interceptor interceptor(this);
    SkCanvas::onDrawPoints(mode, count, pts, paint);
  }

  void onDrawRect(const SkRect& rect, const SkPaint& paint) override {
    Interceptor interceptor(this);
    SkCanvas::onDrawRect(rect, paint);
  }

  void onDrawOval(const SkRect& rect, const SkPaint& paint) override {
    Interceptor interceptor(this);
    SkCanvas::onDrawOval(rect, paint);
  }

  void onDrawRRect(const SkRRect& rrect, const SkPaint& paint) override {
    Interceptor interceptor(this);
    SkCanvas::onDrawRRect(rrect, paint);
  }

  void onDrawPath(const SkPath& path, const SkPaint& paint) override {
    Interceptor interceptor(this);
    SkCanvas::onDrawPath(path, paint);
  }

  void onDrawImage2(const SkImage* image,
                    SkScalar x,
                    SkScalar y,
                    const SkSamplingOptions& sampling,
                    const SkPaint* paint) override {
    Interceptor interceptor(this);
    SkCanvas::onDrawImage2(image, x, y, sampling, paint);
  }

  void onDrawImageRect2(const SkImage* image,
                        const SkRect& src,
                        const SkRect& dst,
                        const SkSamplingOptions& sampling,
                        const SkPaint* paint,
                        SrcRectConstraint constraint) override {
    Interceptor interceptor(this);
    SkCanvas::onDrawImageRect2(image, src, dst, sampling, paint, constraint);
  }

  void onDrawVerticesObject(const SkVertices* vertices,
                            SkBlendMode bmode,
                            const SkPaint& paint) override {
    Interceptor interceptor(this);
    SkCanvas::onDrawVerticesObject(vertices, bmode, paint);
  }

  void onDrawDRRect(const SkRRect& outer,
                    const SkRRect& inner,
                    const SkPaint& paint) override {
    Interceptor interceptor(this);
    SkCanvas::onDrawDRRect(outer, inner, paint);
  }

  void onDrawTextBlob(const SkTextBlob* blob,
                      SkScalar x,
                      SkScalar y,
                      const SkPaint& paint) override {
    Interceptor interceptor(this);
    SkCanvas::onDrawTextBlob(blob, x, y, paint);
  }

  void onClipRect(const SkRect& rect,
                  SkClipOp op,
                  ClipEdgeStyle edge_style) override {
    Interceptor interceptor(this);
    SkCanvas::onClipRect(rect, op, edge_style);
  }

  void onClipRRect(const SkRRect& rrect,
                   SkClipOp op,
                   ClipEdgeStyle edge_style) override {
    Interceptor interceptor(this);
    SkCanvas::onClipRRect(rrect, op, edge_style);
  }

  void onClipPath(const SkPath& path,
                  SkClipOp op,
                  ClipEdgeStyle edge_style) override {
    Interceptor interceptor(this);
    SkCanvas::onClipPath(path, op, edge_style);
  }

  void onClipRegion(const SkRegion& region, SkClipOp op) override {
    Interceptor interceptor(this);
    SkCanvas::onClipRegion(region, op);
  }

  void onDrawPicture(const SkPicture* picture,
                     const SkMatrix* matrix,
                     const SkPaint* paint) override {
    UnrollDrawPicture(picture, matrix, paint, nullptr);
  }

  void didSetM44(const SkM44&) override { Interceptor interceptor(this); }

  void didConcat44(const SkM44&) override { Interceptor interceptor(this); }

  void didScale(SkScalar x, SkScalar y) override {
    Interceptor interceptor(this);
  }

  void didTranslate(SkScalar x, SkScalar y) override {
    Interceptor interceptor(this);
  }

  void willSave() override {
    Interceptor interceptor(this);
    SkCanvas::willSave();
  }

  SkCanvas::SaveLayerStrategy getSaveLayerStrategy(
      const SaveLayerRec& rec) override {
    Interceptor interceptor(this);
    return SkCanvas::getSaveLayerStrategy(rec);
  }

  void willRestore() override {
    Interceptor interceptor(this);
    SkCanvas::willRestore();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_INTERCEPTING_CANVAS_H_

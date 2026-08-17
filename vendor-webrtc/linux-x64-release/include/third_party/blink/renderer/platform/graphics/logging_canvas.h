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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_LOGGING_CANVAS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_LOGGING_CANVAS_H_

#include <memory>
#include "third_party/blink/renderer/platform/graphics/intercepting_canvas.h"
#include "third_party/blink/renderer/platform/json/json_values.h"

namespace blink {

class LoggingCanvas : public InterceptingCanvasBase {
 public:
  LoggingCanvas();

  // Returns a snapshot of the current log data.
  std::unique_ptr<JSONArray> Log();

  void onDrawPaint(const SkPaint&) override;
  void onDrawPoints(PointMode,
                    size_t count,
                    const SkPoint pts[],
                    const SkPaint&) override;
  void onDrawRect(const SkRect&, const SkPaint&) override;
  void onDrawOval(const SkRect&, const SkPaint&) override;
  void onDrawRRect(const SkRRect&, const SkPaint&) override;
  void onDrawPath(const SkPath&, const SkPaint&) override;
  void onDrawImage2(const SkImage*,
                    SkScalar,
                    SkScalar,
                    const SkSamplingOptions&,
                    const SkPaint*) override;
  void onDrawImageRect2(const SkImage*,
                        const SkRect& src,
                        const SkRect& dst,
                        const SkSamplingOptions&,
                        const SkPaint*,
                        SrcRectConstraint) override;
  void onDrawVerticesObject(const SkVertices*,
                            SkBlendMode bmode,
                            const SkPaint&) override;

  void onDrawDRRect(const SkRRect& outer,
                    const SkRRect& inner,
                    const SkPaint&) override;
  void onDrawTextBlob(const SkTextBlob*,
                      SkScalar x,
                      SkScalar y,
                      const SkPaint&) override;
  void onClipRect(const SkRect&, SkClipOp, ClipEdgeStyle) override;
  void onClipRRect(const SkRRect&, SkClipOp, ClipEdgeStyle) override;
  void onClipPath(const SkPath&, SkClipOp, ClipEdgeStyle) override;
  void onClipRegion(const SkRegion&, SkClipOp) override;
  void onDrawPicture(const SkPicture*,
                     const SkMatrix*,
                     const SkPaint*) override;
  void didSetM44(const SkM44&) override;
  void didConcat44(const SkM44&) override;
  void didScale(SkScalar, SkScalar) override;
  void didTranslate(SkScalar, SkScalar) override;
  void willSave() override;
  SaveLayerStrategy getSaveLayerStrategy(const SaveLayerRec&) override;
  void willRestore() override;

 private:
  friend class AutoLogger;

  std::unique_ptr<JSONArray> log_;
};

PLATFORM_EXPORT std::unique_ptr<JSONArray> RecordAsJSON(const PaintRecord&);
PLATFORM_EXPORT String RecordAsDebugString(const PaintRecord&);
PLATFORM_EXPORT void ShowPaintRecord(const PaintRecord&);
PLATFORM_EXPORT std::unique_ptr<JSONArray> SkPictureAsJSON(const SkPicture&);
PLATFORM_EXPORT String SkPictureAsDebugString(const SkPicture&);
PLATFORM_EXPORT void ShowSkPicture(const SkPicture&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_LOGGING_CANVAS_H_

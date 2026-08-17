// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_CANVAS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_CANVAS_H_

#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/skia/include/core/SkCanvas.h"

namespace blink {

class SimCanvas : public SkCanvas {
 public:
  SimCanvas();

  enum CommandType {
    kRect,
    kText,
    kImage,
    kShape,
  };

  class Commands {
    DISALLOW_NEW();

   public:
    size_t DrawCount() const { return commands_.size(); }
    size_t DrawCount(CommandType, const String& color_string = String()) const;
    bool Contains(CommandType type,
                  const String& color_string = String()) const {
      return DrawCount(type, color_string) > 0;
    }

   private:
    struct Command {
      CommandType type;
      RGBA32 color;
    };
    Vector<Command> commands_;

    friend class SimCanvas;
  };

  const Commands& GetCommands() const { return commands_; }

 private:
  // Rect
  void onDrawRect(const SkRect&, const SkPaint&) override;

  // Shape
  void onDrawOval(const SkRect&, const SkPaint&) override;
  void onDrawRRect(const SkRRect&, const SkPaint&) override;
  void onDrawPath(const SkPath&, const SkPaint&) override;

  // Image
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

  // Text
  void onDrawTextBlob(const SkTextBlob*,
                      SkScalar x,
                      SkScalar y,
                      const SkPaint&) override;

  void AddCommand(CommandType, RGBA32 = 0);

  Commands commands_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_SIM_SIM_CANVAS_H_

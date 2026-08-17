// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_HYPHEN_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_HYPHEN_RESULT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/fonts/shaping/shape_result.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ComputedStyle;

class CORE_EXPORT HyphenResult {
  DISALLOW_NEW();

 public:
  HyphenResult() = default;
  explicit HyphenResult(const ComputedStyle& style) { Shape(style); }

  void Trace(Visitor* visitor) const { visitor->Trace(shape_result_); }

  explicit operator bool() const { return !text_.IsNull(); }

  const String& Text() const { return text_; }
  const ShapeResult& GetShapeResult() const { return *shape_result_; }
  LayoutUnit InlineSize() const {
    return shape_result_->SnappedWidth().ClampNegativeToZero();
  }

  void Shape(const ComputedStyle& style);

 private:
  String text_;
  Member<const ShapeResult> shape_result_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_INLINE_HYPHEN_RESULT_H_

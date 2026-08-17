// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_PAINT_WORKLET_INPUT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_PAINT_WORKLET_INPUT_H_

#include <memory>
#include <utility>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/paint_worklet_input.h"
#include "third_party/blink/renderer/core/css/cssom/paint_worklet_style_property_map.h"
#include "third_party/blink/renderer/platform/graphics/compositor_element_id.h"

namespace blink {

// CSSPaintWorkletInput encapsulates the necessary information to run a CSS
// Paint instance (a 'PaintWorklet') for a given target (e.g. the
// 'background-image' property of a particular element). It is used to enable
// Off-Thread PaintWorklet, allowing us to defer the actual JavaScript calls
// until the cc-Raster phase (and even then run the JavaScript on a separate
// worklet thread).
//
// TODO: WTF::Strings are now thread-safe. Consider refactoring this code.
class CORE_EXPORT CSSPaintWorkletInput : public PaintWorkletInput {
 public:
  CSSPaintWorkletInput(
      const String& name,
      const gfx::SizeF& container_size,
      float effective_zoom,
      int worklet_id,
      PaintWorkletStylePropertyMap::CrossThreadData values,
      Vector<std::unique_ptr<CrossThreadStyleValue>> parsed_input_args,
      cc::PaintWorkletInput::PropertyKeys property_keys);

  ~CSSPaintWorkletInput() override = default;

  // These accessors are safe on any thread.
  float EffectiveZoom() const { return effective_zoom_; }
  const Vector<std::unique_ptr<CrossThreadStyleValue>>& ParsedInputArguments()
      const {
    return parsed_input_arguments_;
  }

  // These should only be accessed on the PaintWorklet thread.
  String NameCopy() const { return name_; }
  PaintWorkletStylePropertyMap::CrossThreadData StyleMapData() const {
    return PaintWorkletStylePropertyMap::CopyCrossThreadData(style_map_data_);
  }

  PaintWorkletInputType GetType() const override {
    return PaintWorkletInputType::kCSS;
  }

  bool NeedsLayer() const override { return true; }

 private:
  const String name_;
  const float effective_zoom_;
  PaintWorkletStylePropertyMap::CrossThreadData style_map_data_;
  Vector<std::unique_ptr<CrossThreadStyleValue>> parsed_input_arguments_;
};

template <>
struct DowncastTraits<CSSPaintWorkletInput> {
  static bool AllowFrom(const cc::PaintWorkletInput& worklet_input) {
    auto* input = DynamicTo<PaintWorkletInput>(worklet_input);
    return input && AllowFrom(*input);
  }

  static bool AllowFrom(const PaintWorkletInput& worklet_input) {
    return worklet_input.GetType() ==
           PaintWorkletInput::PaintWorkletInputType::kCSS;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_PAINT_WORKLET_INPUT_H_

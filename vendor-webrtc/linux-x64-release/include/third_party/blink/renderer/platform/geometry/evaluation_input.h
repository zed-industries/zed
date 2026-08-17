// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_EVALUATION_INPUT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_EVALUATION_INPUT_H_

#include <optional>

#include "base/containers/flat_map.h"
#include "base/functional/function_ref.h"
#include "third_party/blink/renderer/platform/geometry/color_channel_keyword.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"

namespace blink {

class Length;

// When calcuating the min/max content-contribution we sometimes need to coerce
// a fit-content/stretch basis to auto.
enum class CalcSizeKeywordBehavior { kAsSpecified, kAsAuto };

struct EvaluationInput {
  STACK_ALLOCATED();

  using IntrinsicLengthEvaluator = base::FunctionRef<LayoutUnit(const Length&)>;

 public:
  std::optional<float> size_keyword_basis = std::nullopt;
  std::optional<IntrinsicLengthEvaluator> intrinsic_evaluator = std::nullopt;
  CalcSizeKeywordBehavior calc_size_keyword_behavior =
      CalcSizeKeywordBehavior::kAsSpecified;
  base::flat_map<ColorChannelKeyword, float> color_channel_keyword_values;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_EVALUATION_INPUT_H_

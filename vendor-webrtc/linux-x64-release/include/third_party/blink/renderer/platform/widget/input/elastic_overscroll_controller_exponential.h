// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_ELASTIC_OVERSCROLL_CONTROLLER_EXPONENTIAL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_ELASTIC_OVERSCROLL_CONTROLLER_EXPONENTIAL_H_

#include "cc/input/scroll_elasticity_helper.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/widget/input/elastic_overscroll_controller.h"

namespace blink {
// Manages scroller stretch and rebounds when overscrolling. This controller
// uses an Exponential curve.
class PLATFORM_EXPORT ElasticOverscrollControllerExponential
    : public ElasticOverscrollController {
 public:
  explicit ElasticOverscrollControllerExponential(
      cc::ScrollElasticityHelper* helper);
  ElasticOverscrollControllerExponential(
      const ElasticOverscrollControllerExponential&) = delete;
  ElasticOverscrollControllerExponential& operator=(
      const ElasticOverscrollControllerExponential&) = delete;
  ~ElasticOverscrollControllerExponential() override = default;

  void DidEnterMomentumAnimatedState() override;
  gfx::Vector2d StretchAmountForTimeDelta(
      const base::TimeDelta& delta) const override;
  gfx::Vector2d StretchAmountForAccumulatedOverscroll(
      const gfx::Vector2dF& accumulated_overscroll) const override;
  gfx::Vector2d AccumulatedOverscrollForStretchAmount(
      const gfx::Vector2dF& delta) const override;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_ELASTIC_OVERSCROLL_CONTROLLER_EXPONENTIAL_H_

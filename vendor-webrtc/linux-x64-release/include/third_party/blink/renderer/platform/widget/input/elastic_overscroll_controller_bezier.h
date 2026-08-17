// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_ELASTIC_OVERSCROLL_CONTROLLER_BEZIER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_ELASTIC_OVERSCROLL_CONTROLLER_BEZIER_H_

#include "base/gtest_prod_util.h"
#include "base/memory/ptr_util.h"
#include "base/time/time.h"
#include "cc/input/scroll_elasticity_helper.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/widget/input/elastic_overscroll_controller.h"
#include "ui/gfx/geometry/cubic_bezier.h"

namespace blink {
// Manages scroller stretch and rebounds when overscrolling. This controller
// uses a Bezier curve.
class PLATFORM_EXPORT ElasticOverscrollControllerBezier
    : public ElasticOverscrollController {
 public:
  explicit ElasticOverscrollControllerBezier(
      cc::ScrollElasticityHelper* helper);
  ElasticOverscrollControllerBezier(const ElasticOverscrollControllerBezier&) =
      delete;
  ElasticOverscrollControllerBezier& operator=(
      const ElasticOverscrollControllerBezier&) = delete;
  ~ElasticOverscrollControllerBezier() override = default;

  void DidEnterMomentumAnimatedState() override;
  gfx::Vector2d StretchAmountForTimeDelta(
      const base::TimeDelta& delta) const override;
  gfx::Vector2d StretchAmountForAccumulatedOverscroll(
      const gfx::Vector2dF& accumulated_overscroll) const override;
  gfx::Vector2d AccumulatedOverscrollForStretchAmount(
      const gfx::Vector2dF& delta) const override;
  gfx::Vector2dF OverscrollBoundary(const gfx::Size& scroller_bounds) const;
  double StretchAmountForForwardBounce(
      const gfx::CubicBezier bounce_forwards_curve,
      const base::TimeDelta& delta,
      const base::TimeDelta& bounce_forwards_duration,
      const double initial_stretch,
      const double bounce_forwards_distance) const;
  double StretchAmountForBackwardBounce(
      const gfx::CubicBezier bounce_backwards_curve,
      const base::TimeDelta& delta,
      const base::TimeDelta& bounce_backwards_duration,
      const double bounce_forwards_distance) const;

 private:
  FRIEND_TEST_ALL_PREFIXES(ElasticOverscrollControllerBezierTest,
                           VerifyInitialStretchDelta);
  FRIEND_TEST_ALL_PREFIXES(ElasticOverscrollControllerBezierTest,
                           VerifyForwardAnimationIsNotPlayed);

  // This is the velocity (in px/ms) of the fling at the moment the scroller
  // reaches its bounds. This is useful to tweak the Cubic Bezier curve for the
  // bounce animation that is about to get played.
  gfx::Vector2dF residual_velocity_;

  base::TimeDelta bounce_forwards_duration_x_;
  base::TimeDelta bounce_forwards_duration_y_;
  gfx::Vector2dF bounce_forwards_distance_;

  base::TimeDelta bounce_backwards_duration_x_;
  base::TimeDelta bounce_backwards_duration_y_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_INPUT_ELASTIC_OVERSCROLL_CONTROLLER_BEZIER_H_

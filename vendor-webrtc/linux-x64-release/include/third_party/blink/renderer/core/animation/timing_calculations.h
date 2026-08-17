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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TIMING_CALCULATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TIMING_CALCULATIONS_H_

#include <optional>

#include "base/debug/dump_without_crashing.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/core/animation/timing.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"

namespace blink {

class CORE_EXPORT TimingCalculations {
 public:
  static double TimingCalculationEpsilon();

  static AnimationTimeDelta TimeTolerance();

  static bool IsWithinAnimationTimeEpsilon(double a, double b);

  static bool IsWithinAnimationTimeTolerance(AnimationTimeDelta a,
                                             AnimationTimeDelta b);

  static bool LessThanOrEqualToWithinEpsilon(double a, double b);

  static bool LessThanOrEqualToWithinTimeTolerance(AnimationTimeDelta a,
                                                   AnimationTimeDelta b);

  static bool GreaterThanOrEqualToWithinEpsilon(double a, double b);

  static bool GreaterThanOrEqualToWithinTimeTolerance(AnimationTimeDelta a,
                                                      AnimationTimeDelta b);

  static bool GreaterThanWithinTimeTolerance(AnimationTimeDelta a,
                                             AnimationTimeDelta b);

  static double MultiplyZeroAlwaysGivesZero(double x, double y);

  static AnimationTimeDelta MultiplyZeroAlwaysGivesZero(AnimationTimeDelta x,
                                                        double y);

  // https://w3.org/TR/web-animations-1/#animation-effect-phases-and-states
  static Timing::Phase CalculatePhase(
      const Timing::NormalizedTiming& normalized,
      std::optional<AnimationTimeDelta>& local_time,
      Timing::AnimationDirection direction);

  // https://w3.org/TR/web-animations-1/#calculating-the-active-time
  static std::optional<AnimationTimeDelta> CalculateActiveTime(
      const Timing::NormalizedTiming& normalized,
      Timing::FillMode fill_mode,
      std::optional<AnimationTimeDelta> local_time,
      Timing::Phase phase);

  // Calculates the overall progress, which describes the number of iterations
  // that have completed (including partial iterations).
  // https://w3.org/TR/web-animations-1/#calculating-the-overall-progress
  static std::optional<double> CalculateOverallProgress(
      Timing::Phase phase,
      std::optional<AnimationTimeDelta> active_time,
      AnimationTimeDelta iteration_duration,
      double iteration_count,
      double iteration_start);

  // Calculates the simple iteration progress, which is a fraction of the
  // progress through the current iteration that ignores transformations to the
  // time introduced by the playback direction or timing functions applied to
  // the effect.
  // https://w3.org/TR/web-animations-1/#calculating-the-simple-iteration-progress
  static std::optional<double> CalculateSimpleIterationProgress(
      Timing::Phase phase,
      std::optional<double> overall_progress,
      double iteration_start,
      std::optional<AnimationTimeDelta> active_time,
      AnimationTimeDelta active_duration,
      double iteration_count);

  // https://w3.org/TR/web-animations-1/#calculating-the-current-iteration
  static std::optional<double> CalculateCurrentIteration(
      Timing::Phase phase,
      std::optional<AnimationTimeDelta> active_time,
      double iteration_count,
      std::optional<double> overall_progress,
      std::optional<double> simple_iteration_progress);

  // https://w3.org/TR/web-animations-1/#calculating-the-directed-progress
  static bool IsCurrentDirectionForwards(
      std::optional<double> current_iteration,
      Timing::PlaybackDirection direction);

  // https://w3.org/TR/web-animations-1/#calculating-the-directed-progress
  static std::optional<double> CalculateDirectedProgress(
      std::optional<double> simple_iteration_progress,
      std::optional<double> current_iteration,
      Timing::PlaybackDirection direction);

  // https://w3.org/TR/web-animations-1/#calculating-the-transformed-progress
  static std::optional<double> CalculateTransformedProgress(
      Timing::Phase phase,
      std::optional<double> directed_progress,
      bool is_current_direction_forward,
      scoped_refptr<TimingFunction> timing_function);

  // Offsets the active time by how far into the animation we start (i.e. the
  // product of the iteration start and iteration duration). This is not part of
  // the Web Animations spec; it is used for calculating the time until the next
  // iteration to optimize scheduling.
  static std::optional<AnimationTimeDelta> CalculateOffsetActiveTime(
      AnimationTimeDelta active_duration,
      std::optional<AnimationTimeDelta> active_time,
      AnimationTimeDelta start_offset);

  // Maps the offset active time into 'iteration time space'[0], aka the offset
  // into the current iteration. This is not part of the Web Animations spec
  // (note that the section linked below is non-normative); it is used for
  // calculating the time until the next iteration to optimize scheduling.
  //
  // [0] https://w3.org/TR/web-animations-1/#iteration-time-space
  static std::optional<AnimationTimeDelta> CalculateIterationTime(
      AnimationTimeDelta iteration_duration,
      AnimationTimeDelta active_duration,
      std::optional<AnimationTimeDelta> offset_active_time,
      AnimationTimeDelta start_offset,
      Timing::Phase phase,
      const Timing& specified);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_TIMING_CALCULATIONS_H_

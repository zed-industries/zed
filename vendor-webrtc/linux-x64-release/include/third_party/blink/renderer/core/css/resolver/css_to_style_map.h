/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc.
 * All rights reserved.
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CSS_TO_STYLE_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CSS_TO_STYLE_MAP_H_

#include "third_party/blink/renderer/core/animation/css/css_animation_data.h"
#include "third_party/blink/renderer/core/animation/css/css_transition_data.h"
#include "third_party/blink/renderer/core/animation/timing.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/style/style_timeline.h"
#include "third_party/blink/renderer/platform/animation/timing_function.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class FillLayer;
class CSSLengthResolver;
class CSSValue;
class StyleResolverState;
class NinePieceImage;
class BorderImageLengthBox;

class CSSToStyleMap {
  STATIC_ONLY(CSSToStyleMap);

 public:
  static void MapFillAttachment(StyleResolverState&,
                                FillLayer*,
                                const CSSValue&);
  static void MapFillClip(StyleResolverState&, FillLayer*, const CSSValue&);
  static void MapFillCompositingOperator(StyleResolverState&,
                                         FillLayer*,
                                         const CSSValue&);
  static void MapFillBlendMode(StyleResolverState&,
                               FillLayer*,
                               const CSSValue&);
  static void MapFillOrigin(StyleResolverState&, FillLayer*, const CSSValue&);
  static void MapFillImage(StyleResolverState&, FillLayer*, const CSSValue&);
  static void MapFillRepeat(StyleResolverState&, FillLayer*, const CSSValue&);
  static void MapFillMaskMode(StyleResolverState&, FillLayer*, const CSSValue&);
  static void MapFillSize(StyleResolverState&, FillLayer*, const CSSValue&);
  static void MapFillPositionX(StyleResolverState&,
                               FillLayer*,
                               const CSSValue&);
  static void MapFillPositionY(StyleResolverState&,
                               FillLayer*,
                               const CSSValue&);

  static Timing::Delay MapAnimationDelayStart(StyleResolverState&,
                                              const CSSValue&);
  static Timing::Delay MapAnimationDelayEnd(const CSSValue&);
  static Timing::Delay MapAnimationDelayEnd(StyleResolverState&,
                                            const CSSValue&);
  static Timing::PlaybackDirection MapAnimationDirection(StyleResolverState&,
                                                         const CSSValue&);
  static std::optional<double> MapAnimationDuration(StyleResolverState&,
                                                    const CSSValue&);
  static Timing::FillMode MapAnimationFillMode(StyleResolverState&,
                                               const CSSValue&);
  static double MapAnimationIterationCount(StyleResolverState&,
                                           const CSSValue&);
  static AtomicString MapAnimationName(StyleResolverState&, const CSSValue&);
  static CSSTransitionData::TransitionBehavior MapAnimationBehavior(
      StyleResolverState&,
      const CSSValue&);
  static StyleTimeline MapAnimationTimeline(StyleResolverState&,
                                            const CSSValue&);
  static EAnimPlayState MapAnimationPlayState(StyleResolverState&,
                                              const CSSValue&);
  static std::optional<TimelineOffset> MapAnimationRangeStart(
      StyleResolverState&,
      const CSSValue&);
  static std::optional<TimelineOffset> MapAnimationRangeEnd(StyleResolverState&,
                                                            const CSSValue&);
  static EffectModel::CompositeOperation MapAnimationComposition(
      StyleResolverState&,
      const CSSValue&);
  static CSSTransitionData::TransitionProperty MapAnimationProperty(
      StyleResolverState&,
      const CSSValue&);
  static scoped_refptr<TimingFunction> MapAnimationTimingFunction(
      const CSSLengthResolver&,
      const CSSValue&);
  static scoped_refptr<TimingFunction> MapAnimationTimingFunction(
      StyleResolverState&,
      const CSSValue&);

  static void MapNinePieceImage(StyleResolverState&,
                                CSSPropertyID,
                                const CSSValue&,
                                NinePieceImage&);
  static void MapNinePieceImageSlice(StyleResolverState&,
                                     const CSSValue&,
                                     NinePieceImage&);
  static BorderImageLengthBox MapNinePieceImageQuad(StyleResolverState&,
                                                    const CSSValue&);
  static void MapNinePieceImageRepeat(StyleResolverState&,
                                      const CSSValue&,
                                      NinePieceImage&);
  static EAnimationTriggerType MapAnimationTriggerType(StyleResolverState&,
                                                       const CSSValue&);
  static StyleTimeline MapAnimationTriggerTimeline(StyleResolverState&,
                                                   const CSSValue&);
  static std::optional<TimelineOffset> MapAnimationTriggerRangeStart(
      StyleResolverState&,
      const CSSValue&);
  static std::optional<TimelineOffset> MapAnimationTriggerRangeEnd(
      StyleResolverState&,
      const CSSValue&);
  static TimelineOffsetOrAuto MapAnimationTriggerExitRangeStart(
      StyleResolverState&,
      const CSSValue&);
  static TimelineOffsetOrAuto MapAnimationTriggerExitRangeEnd(
      StyleResolverState&,
      const CSSValue&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_CSS_TO_STYLE_MAP_H_

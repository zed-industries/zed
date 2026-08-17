/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2007, 2010 Apple Inc. All rights reserved.
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_MARQUEE_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_MARQUEE_ELEMENT_H_

#include "third_party/blink/renderer/core/animation/animation.h"
#include "third_party/blink/renderer/core/animation/keyframe_effect_model.h"
#include "third_party/blink/renderer/core/html/html_element.h"

namespace blink {

class HTMLMarqueeElement final : public HTMLElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  void Trace(Visitor*) const override;

  explicit HTMLMarqueeElement(Document&);

  InsertionNotificationRequest InsertedInto(ContainerNode&) final;
  void RemovedFrom(ContainerNode&) final;

  bool IsHorizontal() const;

  unsigned scrollAmount() const;
  void setScrollAmount(unsigned);

  unsigned scrollDelay() const;
  void setScrollDelay(unsigned);

  int loop() const;
  void setLoop(int, ExceptionState&);

  void start();
  void stop();

 private:
  void DidAddUserAgentShadowRoot(ShadowRoot&) override;

  bool IsPresentationAttribute(const QualifiedName&) const override;
  void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) override;

  class RequestAnimationFrameCallback;
  class AnimationFinished;

  struct AnimationParameters {
    String transform_begin;
    String transform_end;
    double distance;
  };

  struct Metrics {
    double content_width;
    double content_height;
    double marquee_width;
    double marquee_height;
  };

  StringKeyframeEffectModel* CreateEffectModel(const AnimationParameters&);

  void ContinueAnimation();
  bool ShouldContinue();

  enum Behavior { kScroll, kSlide, kAlternate };
  Behavior GetBehavior() const;

  enum Direction { kLeft, kRight, kUp, kDown };
  Direction GetDirection() const;

  Metrics GetMetrics();
  AnimationParameters GetAnimationParameters();
  AtomicString CreateTransform(double value) const;

  static const int kDefaultScrollAmount = 6;
  static const int kDefaultScrollDelayMS = 85;
  static const int kMinimumScrollDelayMS = 60;
  static const int kDefaultLoopLimit = -1;

  int continue_callback_request_id_ = 0;
  int loop_count_ = 0;
  Member<Element> mover_;
  Member<Animation> player_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_MARQUEE_ELEMENT_H_

// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_ANIMATION_PLAYBACK_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_ANIMATION_PLAYBACK_EVENT_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_union_cssnumericvalue_double.h"
#include "third_party/blink/renderer/core/dom/events/event.h"

namespace blink {

class AnimationPlaybackEventInit;

class AnimationPlaybackEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static AnimationPlaybackEvent* Create(
      const AtomicString& type,
      const AnimationPlaybackEventInit* initializer) {
    return MakeGarbageCollected<AnimationPlaybackEvent>(type, initializer);
  }

  AnimationPlaybackEvent(const AtomicString& type,
                         V8CSSNumberish* current_time,
                         V8CSSNumberish* timeline_time);
  AnimationPlaybackEvent(const AtomicString&,
                         const AnimationPlaybackEventInit*);
  ~AnimationPlaybackEvent() override;

  V8CSSNumberish* currentTime() const { return current_time_.Get(); }
  V8CSSNumberish* timelineTime() const { return timeline_time_.Get(); }

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  Member<V8CSSNumberish> current_time_;
  Member<V8CSSNumberish> timeline_time_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EVENTS_ANIMATION_PLAYBACK_EVENT_H_

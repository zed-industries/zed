/*
 * Copyright (C) 2011 Apple Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TRACK_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TRACK_EVENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event.h"
#include "third_party/blink/renderer/core/html/track/track_base.h"

namespace blink {

class TrackEventInit;
class V8UnionAudioTrackOrTextTrackOrVideoTrack;

class CORE_EXPORT TrackEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  TrackEvent();
  TrackEvent(const AtomicString& type, const TrackEventInit* initializer);
  TrackEvent(const AtomicString& type, TrackBase* track)
      : Event(type, Bubbles::kNo, Cancelable::kNo), track_(track) {}
  ~TrackEvent() override;

  static TrackEvent* Create() { return MakeGarbageCollected<TrackEvent>(); }

  static TrackEvent* Create(const AtomicString& type,
                            const TrackEventInit* initializer) {
    return MakeGarbageCollected<TrackEvent>(type, initializer);
  }

  static TrackEvent* Create(const AtomicString& type, TrackBase* track) {
    return MakeGarbageCollected<TrackEvent>(type, track);
  }

  const AtomicString& InterfaceName() const override;

  V8UnionAudioTrackOrTextTrackOrVideoTrack* track();

  void Trace(Visitor*) const override;

 private:
  Member<TrackBase> track_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TRACK_EVENT_H_

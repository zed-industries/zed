// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_TRACK_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_TRACK_EVENT_H_

#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class MediaStream;
class MediaStreamTrack;
class RTCRtpReceiver;
class RTCRtpTransceiver;
class RTCTrackEventInit;

// https://w3c.github.io/webrtc-pc/#rtctrackevent
class RTCTrackEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static RTCTrackEvent* Create(const AtomicString& type,
                               const RTCTrackEventInit* eventInitDict);
  RTCTrackEvent(RTCRtpReceiver*,
                MediaStreamTrack*,
                const HeapVector<Member<MediaStream>>&,
                RTCRtpTransceiver*);
  RTCTrackEvent(const AtomicString& type,
                const RTCTrackEventInit* eventInitDict);

  RTCRtpReceiver* receiver() const;
  MediaStreamTrack* track() const;
  const HeapVector<Member<MediaStream>>& streams() const;
  RTCRtpTransceiver* transceiver() const;

  void Trace(Visitor*) const override;

 private:
  Member<RTCRtpReceiver> receiver_;
  Member<MediaStreamTrack> track_;
  HeapVector<Member<MediaStream>> streams_;
  Member<RTCRtpTransceiver> transceiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_TRACK_EVENT_H_

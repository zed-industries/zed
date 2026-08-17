// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_SESSION_DESCRIPTION_PLATFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_SESSION_DESCRIPTION_PLATFORM_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class PLATFORM_EXPORT RTCSessionDescriptionPlatform final
    : public GarbageCollected<RTCSessionDescriptionPlatform> {
 public:
  RTCSessionDescriptionPlatform(const String& type, const String& sdp);

  String GetType() const { return type_; }
  void SetType(const String& type) { type_ = type; }

  String Sdp() const { return sdp_; }
  void SetSdp(const String& sdp) { sdp_ = sdp; }

  void Trace(Visitor* visitor) const {}

 private:
  String type_;
  String sdp_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_SESSION_DESCRIPTION_PLATFORM_H_

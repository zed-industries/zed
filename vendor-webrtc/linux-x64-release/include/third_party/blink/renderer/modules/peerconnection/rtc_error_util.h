// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ERROR_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ERROR_UTIL_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/webrtc/api/rtc_error.h"

namespace blink {

class DOMException;
class ExceptionState;
class ScriptPromiseResolverBase;

DOMException* CreateDOMExceptionFromRTCError(const webrtc::RTCError&);
void RejectPromiseFromRTCError(const webrtc::RTCError&,
                               ScriptPromiseResolverBase*);
void ThrowExceptionFromRTCError(const webrtc::RTCError&, ExceptionState&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ERROR_UTIL_H_

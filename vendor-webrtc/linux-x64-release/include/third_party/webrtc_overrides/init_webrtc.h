// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_INIT_WEBRTC_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_INIT_WEBRTC_H_

#include "third_party/webrtc/rtc_base/system/rtc_export.h"
#include "third_party/webrtc/rtc_base/trace_categories.h"

// Initialize WebRTC. Call this explicitly to initialize WebRTC module
// before initializing the sandbox in Chrome.
RTC_EXPORT bool InitializeWebRtcModuleBeforeSandbox();

// Hooks up Chrome+WebRTC integration such as logging and tracing. Should be
// run after tracing is initialized, otherwise WebRTC traces won't work.
RTC_EXPORT void InitializeWebRtcModule();

RTC_EXPORT const perfetto::internal::TrackEventCategoryRegistry&
GetWebRtcTrackEventCategoryRegistry();

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_INIT_WEBRTC_H_

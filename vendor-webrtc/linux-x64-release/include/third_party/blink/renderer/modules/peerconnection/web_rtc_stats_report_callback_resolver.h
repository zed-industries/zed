// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_WEB_RTC_STATS_REPORT_CALLBACK_RESOLVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_WEB_RTC_STATS_REPORT_CALLBACK_RESOLVER_H_

#include <memory>

#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_stats_report.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_stats.h"

namespace blink {

void WebRTCStatsReportCallbackResolver(ScriptPromiseResolver<RTCStatsReport>*,
                                       std::unique_ptr<RTCStatsReportPlatform>);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_WEB_RTC_STATS_REPORT_CALLBACK_RESOLVER_H_

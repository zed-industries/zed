/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_RTC_STATS_TRAVERSAL_H_
#define PC_RTC_STATS_TRAVERSAL_H_

#include <string>
#include <vector>

#include "api/scoped_refptr.h"
#include "api/stats/rtc_stats.h"
#include "api/stats/rtc_stats_report.h"

namespace webrtc {

// Traverses the stats graph, taking all stats objects that are directly or
// indirectly accessible from and including the stats objects identified by
// `ids`, returning them as a new stats report.
// This is meant to be used to implement the stats selection algorithm.
// https://w3c.github.io/webrtc-pc/#dfn-stats-selection-algorithm
scoped_refptr<RTCStatsReport> TakeReferencedStats(
    scoped_refptr<RTCStatsReport> report,
    const std::vector<std::string>& ids);

// Gets pointers to the string values of any members in `stats` that are used as
// references for looking up other stats objects in the same report by ID. The
// pointers are valid for the lifetime of `stats` assumings its members are not
// modified.
//
// For example, RTCCodecStats contains "transportId"
// (RTCCodecStats::transport_id) referencing an RTCTransportStats.
// https://w3c.github.io/webrtc-stats/#dom-rtccodecstats-transportid
std::vector<const std::string*> GetStatsReferencedIds(const RTCStats& stats);

}  // namespace webrtc

#endif  // PC_RTC_STATS_TRAVERSAL_H_

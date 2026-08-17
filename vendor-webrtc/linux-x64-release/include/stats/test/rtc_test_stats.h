/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef STATS_TEST_RTC_TEST_STATS_H_
#define STATS_TEST_RTC_TEST_STATS_H_

#include <cstdint>
#include <map>
#include <optional>
#include <string>
#include <vector>

#include "api/stats/rtc_stats.h"
#include "api/units/timestamp.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

class RTC_EXPORT RTCTestStats : public RTCStats {
 public:
  WEBRTC_RTCSTATS_DECL(RTCTestStats);
  RTCTestStats(const std::string& id, Timestamp timestamp);
  ~RTCTestStats() override;

  std::optional<bool> m_bool;
  std::optional<int32_t> m_int32;
  std::optional<uint32_t> m_uint32;
  std::optional<int64_t> m_int64;
  std::optional<uint64_t> m_uint64;
  std::optional<double> m_double;
  std::optional<std::string> m_string;
  std::optional<std::vector<bool>> m_sequence_bool;
  std::optional<std::vector<int32_t>> m_sequence_int32;
  std::optional<std::vector<uint32_t>> m_sequence_uint32;
  std::optional<std::vector<int64_t>> m_sequence_int64;
  std::optional<std::vector<uint64_t>> m_sequence_uint64;
  std::optional<std::vector<double>> m_sequence_double;
  std::optional<std::vector<std::string>> m_sequence_string;
  std::optional<std::map<std::string, uint64_t>> m_map_string_uint64;
  std::optional<std::map<std::string, double>> m_map_string_double;
};

}  // namespace webrtc

#endif  // STATS_TEST_RTC_TEST_STATS_H_

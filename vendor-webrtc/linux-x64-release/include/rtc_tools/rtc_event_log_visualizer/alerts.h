/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_ALERTS_H_
#define RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_ALERTS_H_

#include <stdio.h>

#include <functional>
#include <map>
#include <string>
#include <utility>

#include "absl/strings/string_view.h"
#include "logging/rtc_event_log/rtc_event_log_parser.h"
#include "rtc_tools/rtc_event_log_visualizer/analyzer_common.h"

namespace webrtc {

enum class TriageAlertType {
  kUnknown = 0,
  kIncomingRtpGap,
  kOutgoingRtpGap,
  kIncomingRtcpGap,
  kOutgoingRtcpGap,
  kIncomingSeqNumJump,
  kOutgoingSeqNumJump,
  kIncomingCaptureTimeJump,
  kOutgoingCaptureTimeJump,
  kOutgoingHighLoss,
  kLast,
};

struct TriageAlert {
  TriageAlertType type = TriageAlertType::kUnknown;
  int count = 0;
  float first_occurrence = -1;
  std::string explanation;
};

class TriageHelper {
 public:
  explicit TriageHelper(const AnalyzerConfig& config) : config_(config) {}

  TriageHelper(const TriageHelper&) = delete;
  TriageHelper& operator=(const TriageHelper&) = delete;

  void AnalyzeLog(const ParsedRtcEventLog& parsed_log);

  void AnalyzeStreamGaps(const ParsedRtcEventLog& parsed_log,
                         PacketDirection direction);
  void AnalyzeTransmissionGaps(const ParsedRtcEventLog& parsed_log,
                               PacketDirection direction);
  void Print(FILE* file);

  void ProcessAlerts(std::function<void(int, float, std::string)> f);

 private:
  AnalyzerConfig config_;
  std::map<TriageAlertType, TriageAlert> triage_alerts_;

  void Alert(TriageAlertType type,
             float time_seconds,
             absl::string_view explanation) {
    std::map<TriageAlertType, TriageAlert>::iterator it =
        triage_alerts_.find(type);

    if (it == triage_alerts_.end()) {
      TriageAlert alert;
      alert.type = type;
      alert.first_occurrence = time_seconds;
      alert.count = 1;
      alert.explanation = std::string(explanation);
      triage_alerts_.insert(std::make_pair(type, alert));
    } else {
      it->second.count += 1;
    }
  }
};

}  // namespace webrtc

#endif  // RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_ALERTS_H_

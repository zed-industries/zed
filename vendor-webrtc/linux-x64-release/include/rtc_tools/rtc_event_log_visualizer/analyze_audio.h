/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_ANALYZE_AUDIO_H_
#define RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_ANALYZE_AUDIO_H_

#include <cstdint>
#include <map>
#include <memory>
#include <string>

#include "api/function_view.h"
#include "api/neteq/neteq.h"
#include "logging/rtc_event_log/rtc_event_log_parser.h"
#include "modules/audio_coding/neteq/tools/neteq_stats_getter.h"
#include "rtc_tools/rtc_event_log_visualizer/analyzer_common.h"
#include "rtc_tools/rtc_event_log_visualizer/plot_base.h"

namespace webrtc {

void CreateAudioEncoderTargetBitrateGraph(const ParsedRtcEventLog& parsed_log,
                                          const AnalyzerConfig& config,
                                          Plot* plot);
void CreateAudioEncoderFrameLengthGraph(const ParsedRtcEventLog& parsed_log,
                                        const AnalyzerConfig& config,
                                        Plot* plot);
void CreateAudioEncoderPacketLossGraph(const ParsedRtcEventLog& parsed_log,
                                       const AnalyzerConfig& config,
                                       Plot* plot);
void CreateAudioEncoderEnableFecGraph(const ParsedRtcEventLog& parsed_log,
                                      const AnalyzerConfig& config,
                                      Plot* plot);
void CreateAudioEncoderEnableDtxGraph(const ParsedRtcEventLog& parsed_log,
                                      const AnalyzerConfig& config,
                                      Plot* plot);
void CreateAudioEncoderNumChannelsGraph(const ParsedRtcEventLog& parsed_log,
                                        const AnalyzerConfig& config,
                                        Plot* plot);

using NetEqStatsGetterMap =
    std::map<uint32_t, std::unique_ptr<test::NetEqStatsGetter>>;
NetEqStatsGetterMap SimulateNetEq(const ParsedRtcEventLog& parsed_log,
                                  const AnalyzerConfig& config,
                                  const std::string& replacement_file_name,
                                  int file_sample_rate_hz);

void CreateAudioJitterBufferGraph(const ParsedRtcEventLog& parsed_log,
                                  const AnalyzerConfig& config,
                                  uint32_t ssrc,
                                  const test::NetEqStatsGetter* stats_getter,
                                  Plot* plot);
void CreateNetEqNetworkStatsGraph(
    const ParsedRtcEventLog& parsed_log,
    const AnalyzerConfig& config,
    const NetEqStatsGetterMap& neteq_stats_getters,
    FunctionView<float(const NetEqNetworkStatistics&)> stats_extractor,
    const std::string& plot_name,
    Plot* plot);
void CreateNetEqLifetimeStatsGraph(
    const ParsedRtcEventLog& parsed_log,
    const AnalyzerConfig& config,
    const NetEqStatsGetterMap& neteq_stats_getters,
    FunctionView<float(const NetEqLifetimeStatistics&)> stats_extractor,
    const std::string& plot_name,
    Plot* plot);

}  // namespace webrtc

#endif  // RTC_TOOLS_RTC_EVENT_LOG_VISUALIZER_ANALYZE_AUDIO_H_

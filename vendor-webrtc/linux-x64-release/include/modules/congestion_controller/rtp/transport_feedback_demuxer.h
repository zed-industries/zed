/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_CONGESTION_CONTROLLER_RTP_TRANSPORT_FEEDBACK_DEMUXER_H_
#define MODULES_CONGESTION_CONTROLLER_RTP_TRANSPORT_FEEDBACK_DEMUXER_H_

#include <cstdint>
#include <map>
#include <utility>
#include <vector>

#include "api/sequence_checker.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "rtc_base/numerics/sequence_number_unwrapper.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// Implementation of StreamFeedbackProvider that provides a way for
// implementations of StreamFeedbackObserver to register for feedback callbacks
// for a given set of SSRCs.
// Registration methods need to be called from the same execution context
// (thread or task queue) and callbacks to
// StreamFeedbackObserver::OnPacketFeedbackVector will be made in that same
// context.
// TODO(tommi): This appears to be the only implementation of this interface.
// Do we need the interface?
class TransportFeedbackDemuxer final : public StreamFeedbackProvider {
 public:
  TransportFeedbackDemuxer();

  // Implements StreamFeedbackProvider interface
  void RegisterStreamFeedbackObserver(
      std::vector<uint32_t> ssrcs,
      StreamFeedbackObserver* observer) override;
  void DeRegisterStreamFeedbackObserver(
      StreamFeedbackObserver* observer) override;
  void AddPacket(const RtpPacketSendInfo& packet_info);
  void OnTransportFeedback(const rtcp::TransportFeedback& feedback);

 private:
  RTC_NO_UNIQUE_ADDRESS SequenceChecker observer_checker_;
  RtpSequenceNumberUnwrapper seq_num_unwrapper_
      RTC_GUARDED_BY(&observer_checker_);
  std::map<int64_t, StreamFeedbackObserver::StreamPacketInfo> history_
      RTC_GUARDED_BY(&observer_checker_);

  // Maps a set of ssrcs to corresponding observer. Vectors are used rather than
  // set/map to ensure that the processing order is consistent independently of
  // the randomized ssrcs.
  std::vector<std::pair<std::vector<uint32_t>, StreamFeedbackObserver*>>
      observers_ RTC_GUARDED_BY(&observer_checker_);
};
}  // namespace webrtc

#endif  // MODULES_CONGESTION_CONTROLLER_RTP_TRANSPORT_FEEDBACK_DEMUXER_H_

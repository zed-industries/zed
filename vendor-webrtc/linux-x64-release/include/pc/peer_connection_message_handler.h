/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_PEER_CONNECTION_MESSAGE_HANDLER_H_
#define PC_PEER_CONNECTION_MESSAGE_HANDLER_H_

#include <functional>

#include "api/jsep.h"
#include "api/legacy_stats_types.h"
#include "api/media_stream_interface.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_error.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "pc/legacy_stats_collector_interface.h"

namespace webrtc {

class PeerConnectionMessageHandler {
 public:
  explicit PeerConnectionMessageHandler(Thread* signaling_thread)
      : signaling_thread_(signaling_thread) {}
  ~PeerConnectionMessageHandler() = default;

  void PostSetSessionDescriptionSuccess(
      SetSessionDescriptionObserver* observer);
  void PostSetSessionDescriptionFailure(SetSessionDescriptionObserver* observer,
                                        RTCError&& error);
  void PostCreateSessionDescriptionFailure(
      CreateSessionDescriptionObserver* observer,
      RTCError error);
  void PostGetStats(StatsObserver* observer,
                    LegacyStatsCollectorInterface* legacy_stats,
                    MediaStreamTrackInterface* track);
  void RequestUsagePatternReport(std::function<void()>, int delay_ms);

 private:
  ScopedTaskSafety safety_;
  TaskQueueBase* const signaling_thread_;
};

}  // namespace webrtc

#endif  // PC_PEER_CONNECTION_MESSAGE_HANDLER_H_

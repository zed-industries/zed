/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Syncable is used by RtpStreamsSynchronizer in VideoReceiveStreamInterface,
// and implemented by AudioReceiveStreamInterface.

#ifndef CALL_SYNCABLE_H_
#define CALL_SYNCABLE_H_

#include <stdint.h>

#include <optional>

namespace webrtc {

class Syncable {
 public:
  struct Info {
    int64_t latest_receive_time_ms = 0;
    uint32_t latest_received_capture_timestamp = 0;
    uint32_t capture_time_ntp_secs = 0;
    uint32_t capture_time_ntp_frac = 0;
    uint32_t capture_time_source_clock = 0;
    int current_delay_ms = 0;
  };

  virtual ~Syncable();

  virtual uint32_t id() const = 0;
  virtual std::optional<Info> GetInfo() const = 0;
  virtual bool GetPlayoutRtpTimestamp(uint32_t* rtp_timestamp,
                                      int64_t* time_ms) const = 0;
  virtual bool SetMinimumPlayoutDelay(int delay_ms) = 0;
  virtual void SetEstimatedPlayoutNtpTimestampMs(int64_t ntp_timestamp_ms,
                                                 int64_t time_ms) = 0;
};
}  // namespace webrtc

#endif  // CALL_SYNCABLE_H_

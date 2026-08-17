/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_CONGESTION_CONTROLLER_REMB_THROTTLER_H_
#define MODULES_CONGESTION_CONTROLLER_REMB_THROTTLER_H_

#include <cstdint>
#include <functional>
#include <vector>

#include "api/units/data_rate.h"
#include "api/units/timestamp.h"
#include "modules/remote_bitrate_estimator/include/remote_bitrate_estimator.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {

// RembThrottler is a helper class used for throttling RTCP REMB messages.
// Throttles small changes to the received BWE within 200ms.
class RembThrottler : public RemoteBitrateObserver {
 public:
  using RembSender =
      std::function<void(int64_t bitrate_bps, std::vector<uint32_t> ssrcs)>;
  RembThrottler(RembSender remb_sender, Clock* clock);

  // Ensures the remote party is notified of the receive bitrate no larger than
  // `bitrate` using RTCP REMB.
  void SetMaxDesiredReceiveBitrate(DataRate bitrate);

  // Implements RemoteBitrateObserver;
  // Called every time there is a new bitrate estimate for a receive channel
  // group. This call will trigger a new RTCP REMB packet if the bitrate
  // estimate has decreased or if no RTCP REMB packet has been sent for
  // a certain time interval.
  void OnReceiveBitrateChanged(const std::vector<uint32_t>& ssrcs,
                               uint32_t bitrate_bps) override;

 private:
  const RembSender remb_sender_;
  Clock* const clock_;
  mutable Mutex mutex_;
  Timestamp last_remb_time_ RTC_GUARDED_BY(mutex_);
  DataRate last_send_remb_bitrate_ RTC_GUARDED_BY(mutex_);
  DataRate max_remb_bitrate_ RTC_GUARDED_BY(mutex_);
};

}  // namespace webrtc
#endif  // MODULES_CONGESTION_CONTROLLER_REMB_THROTTLER_H_

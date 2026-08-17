/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_DTMF_QUEUE_H_
#define MODULES_RTP_RTCP_SOURCE_DTMF_QUEUE_H_

#include <stdint.h>

#include <list>

#include "rtc_base/synchronization/mutex.h"

namespace webrtc {
class DtmfQueue {
 public:
  struct Event {
    uint16_t duration_ms = 0;
    uint8_t payload_type = 0;
    uint8_t key = 0;
    uint8_t level = 0;
  };

  DtmfQueue();
  ~DtmfQueue();

  bool AddDtmf(const Event& event);
  bool NextDtmf(Event* event);
  bool PendingDtmf() const;

 private:
  mutable Mutex dtmf_mutex_;
  std::list<Event> queue_;
};
}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_DTMF_QUEUE_H_

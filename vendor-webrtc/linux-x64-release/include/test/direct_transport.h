/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_DIRECT_TRANSPORT_H_
#define TEST_DIRECT_TRANSPORT_H_

#include <memory>

#include "api/call/transport.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "api/test/simulated_network.h"
#include "call/call.h"
#include "call/simulated_packet_receiver.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/task_utils/repeating_task.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class PacketReceiver;

namespace test {
class Demuxer {
 public:
  explicit Demuxer(const std::map<uint8_t, MediaType>& payload_type_map);
  ~Demuxer() = default;

  Demuxer(const Demuxer&) = delete;
  Demuxer& operator=(const Demuxer&) = delete;

  MediaType GetMediaType(const uint8_t* packet_data,
                         size_t packet_length) const;
  const std::map<uint8_t, MediaType> payload_type_map_;
};

// Objects of this class are expected to be allocated and destroyed  on the
// same task-queue - the one that's passed in via the constructor.
class DirectTransport : public Transport {
 public:
  DirectTransport(TaskQueueBase* task_queue,
                  std::unique_ptr<SimulatedPacketReceiverInterface> pipe,
                  Call* send_call,
                  const std::map<uint8_t, MediaType>& payload_type_map,
                  ArrayView<const RtpExtension> audio_extensions,
                  ArrayView<const RtpExtension> video_extensions);

  ~DirectTransport() override;

  // TODO(holmer): Look into moving this to the constructor.
  virtual void SetReceiver(PacketReceiver* receiver);

  // Backwards compatibility using statements.
  // TODO(https://bugs.webrtc.org/15410): Remove when not needed.
  using Transport::SendRtcp;
  using Transport::SendRtp;

  bool SendRtp(ArrayView<const uint8_t> data,
               const PacketOptions& options) override;
  bool SendRtcp(ArrayView<const uint8_t> data) override;

  int GetAverageDelayMs();

 private:
  void ProcessPackets() RTC_EXCLUSIVE_LOCKS_REQUIRED(&process_lock_);
  void LegacySendPacket(const uint8_t* data, size_t length);
  void Start();

  Call* const send_call_;

  TaskQueueBase* const task_queue_;

  Mutex process_lock_;
  RepeatingTaskHandle next_process_task_ RTC_GUARDED_BY(&process_lock_);

  const Demuxer demuxer_;
  const std::unique_ptr<SimulatedPacketReceiverInterface> fake_network_;
  const RtpHeaderExtensionMap audio_extensions_;
  const RtpHeaderExtensionMap video_extensions_;
};
}  // namespace test
}  // namespace webrtc

#endif  // TEST_DIRECT_TRANSPORT_H_

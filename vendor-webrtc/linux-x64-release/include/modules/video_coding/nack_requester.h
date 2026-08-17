/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_NACK_REQUESTER_H_
#define MODULES_VIDEO_CODING_NACK_REQUESTER_H_

#include <stdint.h>

#include <map>
#include <set>
#include <vector>

#include "api/field_trials_view.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "modules/include/module_common_types.h"
#include "modules/video_coding/histogram.h"
#include "rtc_base/numerics/sequence_number_util.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/task_utils/repeating_task.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {

class NackRequesterBase {
 public:
  virtual ~NackRequesterBase() = default;
  virtual void ProcessNacks() = 0;
};

class NackPeriodicProcessor {
 public:
  static constexpr TimeDelta kUpdateInterval = TimeDelta::Millis(20);
  explicit NackPeriodicProcessor(TimeDelta update_interval = kUpdateInterval);
  ~NackPeriodicProcessor();
  void RegisterNackModule(NackRequesterBase* module);
  void UnregisterNackModule(NackRequesterBase* module);

 private:
  void ProcessNackModules() RTC_RUN_ON(sequence_);

  const TimeDelta update_interval_;
  RepeatingTaskHandle repeating_task_ RTC_GUARDED_BY(sequence_);
  std::vector<NackRequesterBase*> modules_ RTC_GUARDED_BY(sequence_);
  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_;
};

class ScopedNackPeriodicProcessorRegistration {
 public:
  ScopedNackPeriodicProcessorRegistration(NackRequesterBase* module,
                                          NackPeriodicProcessor* processor);
  ~ScopedNackPeriodicProcessorRegistration();

 private:
  NackRequesterBase* const module_;
  NackPeriodicProcessor* const processor_;
};

class NackRequester final : public NackRequesterBase {
 public:
  NackRequester(TaskQueueBase* current_queue,
                NackPeriodicProcessor* periodic_processor,
                Clock* clock,
                NackSender* nack_sender,
                KeyFrameRequestSender* keyframe_request_sender,
                const FieldTrialsView& field_trials);
  ~NackRequester();

  void ProcessNacks() override;

  int OnReceivedPacket(uint16_t seq_num);
  int OnReceivedPacket(uint16_t seq_num, bool is_recovered);

  void ClearUpTo(uint16_t seq_num);
  void UpdateRtt(int64_t rtt_ms);

 private:
  // Which fields to consider when deciding which packet to nack in
  // GetNackBatch.
  enum NackFilterOptions { kSeqNumOnly, kTimeOnly, kSeqNumAndTime };

  // This class holds the sequence number of the packet that is in the nack list
  // as well as the meta data about when it should be nacked and how many times
  // we have tried to nack this packet.
  struct NackInfo {
    NackInfo();
    NackInfo(uint16_t seq_num,
             uint16_t send_at_seq_num,
             Timestamp created_at_time);

    uint16_t seq_num;
    uint16_t send_at_seq_num;
    Timestamp created_at_time;
    Timestamp sent_at_time;
    int retries;
  };

  void AddPacketsToNack(uint16_t seq_num_start, uint16_t seq_num_end)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(worker_thread_);

  std::vector<uint16_t> GetNackBatch(NackFilterOptions options)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(worker_thread_);

  // Update the reordering distribution.
  void UpdateReorderingStatistics(uint16_t seq_num)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(worker_thread_);

  // Returns how many packets we have to wait in order to receive the packet
  // with probability `probabilty` or higher.
  int WaitNumberOfPackets(float probability) const
      RTC_EXCLUSIVE_LOCKS_REQUIRED(worker_thread_);

  TaskQueueBase* const worker_thread_;
  Clock* const clock_;
  NackSender* const nack_sender_;
  KeyFrameRequestSender* const keyframe_request_sender_;

  // TODO(philipel): Some of the variables below are consistently used on a
  // known thread (e.g. see `initialized_`). Those probably do not need
  // synchronized access.
  std::map<uint16_t, NackInfo, DescendingSeqNumComp<uint16_t>> nack_list_
      RTC_GUARDED_BY(worker_thread_);
  std::set<uint16_t, DescendingSeqNumComp<uint16_t>> recovered_list_
      RTC_GUARDED_BY(worker_thread_);
  video_coding::Histogram reordering_histogram_ RTC_GUARDED_BY(worker_thread_);
  bool initialized_ RTC_GUARDED_BY(worker_thread_);
  TimeDelta rtt_ RTC_GUARDED_BY(worker_thread_);
  uint16_t newest_seq_num_ RTC_GUARDED_BY(worker_thread_);

  // Adds a delay before send nack on packet received.
  const TimeDelta send_nack_delay_;

  ScopedNackPeriodicProcessorRegistration processor_registration_;

  // Used to signal destruction to potentially pending tasks.
  ScopedTaskSafety task_safety_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_NACK_REQUESTER_H_

/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_TASK_QUEUE_FRAME_DECODE_SCHEDULER_H_
#define VIDEO_TASK_QUEUE_FRAME_DECODE_SCHEDULER_H_

#include "video/frame_decode_scheduler.h"

namespace webrtc {

// An implementation of FrameDecodeScheduler that is based on TaskQueues. This
// is the default implementation for general use.
class TaskQueueFrameDecodeScheduler : public FrameDecodeScheduler {
 public:
  TaskQueueFrameDecodeScheduler(Clock* clock,
                                TaskQueueBase* const bookkeeping_queue);
  ~TaskQueueFrameDecodeScheduler() override;
  TaskQueueFrameDecodeScheduler(const TaskQueueFrameDecodeScheduler&) = delete;
  TaskQueueFrameDecodeScheduler& operator=(
      const TaskQueueFrameDecodeScheduler&) = delete;

  // FrameDecodeScheduler implementation.
  std::optional<uint32_t> ScheduledRtpTimestamp() override;
  void ScheduleFrame(uint32_t rtp,
                     FrameDecodeTiming::FrameSchedule schedule,
                     FrameReleaseCallback cb) override;
  void CancelOutstanding() override;
  void Stop() override;

 private:
  Clock* const clock_;
  TaskQueueBase* const bookkeeping_queue_;

  std::optional<uint32_t> scheduled_rtp_;
  ScopedTaskSafetyDetached task_safety_;
  bool stopped_ = false;
};

}  // namespace webrtc

#endif  // VIDEO_TASK_QUEUE_FRAME_DECODE_SCHEDULER_H_

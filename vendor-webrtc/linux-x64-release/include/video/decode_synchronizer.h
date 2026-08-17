/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_DECODE_SYNCHRONIZER_H_
#define VIDEO_DECODE_SYNCHRONIZER_H_

#include <stdint.h>

#include <functional>
#include <memory>
#include <optional>
#include <set>
#include <utility>

#include "api/metronome/metronome.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "api/units/timestamp.h"
#include "rtc_base/checks.h"
#include "rtc_base/thread_annotations.h"
#include "video/frame_decode_scheduler.h"
#include "video/frame_decode_timing.h"

namespace webrtc {

// DecodeSynchronizer synchronizes the frame scheduling by coalescing decoding
// on the metronome.
//
// A video receive stream can use the DecodeSynchronizer by receiving a
// FrameDecodeScheduler instance with `CreateSynchronizedFrameScheduler()`.
// This instance implements FrameDecodeScheduler and can be used as a normal
// scheduler. This instance is owned by the receive stream, and is borrowed by
// the DecodeSynchronizer. The DecodeSynchronizer will stop borrowing the
// instance when `FrameDecodeScheduler::Stop()` is called, after which the
// scheduler may be destroyed by the receive stream.
//
// When a frame is scheduled for decode by a receive stream using the
// DecodeSynchronizer, it will instead be executed on the metronome during the
// tick interval where `max_decode_time` occurs. For example, if a frame is
// scheduled for decode in 50ms and the tick interval is 20ms, then the frame
// will be released for decoding in 2 ticks. See below for illustration,
//
// In the case where the decode time is in the past, or must occur before the
// next metronome tick then the frame will be released right away, allowing a
// delayed stream to catch up quickly.
//
// DecodeSynchronizer is single threaded - all method calls must run on the
// `worker_queue_`.
class DecodeSynchronizer {
 public:
  DecodeSynchronizer(Clock* clock,
                     Metronome* metronome,
                     TaskQueueBase* worker_queue);
  ~DecodeSynchronizer();
  DecodeSynchronizer(const DecodeSynchronizer&) = delete;
  DecodeSynchronizer& operator=(const DecodeSynchronizer&) = delete;

  std::unique_ptr<FrameDecodeScheduler> CreateSynchronizedFrameScheduler();

 private:
  class ScheduledFrame {
   public:
    ScheduledFrame(uint32_t rtp_timestamp,
                   FrameDecodeTiming::FrameSchedule schedule,
                   FrameDecodeScheduler::FrameReleaseCallback callback);

    // Disallow copy since `callback` should only be moved.
    ScheduledFrame(const ScheduledFrame&) = delete;
    ScheduledFrame& operator=(const ScheduledFrame&) = delete;
    ScheduledFrame(ScheduledFrame&&) = default;
    ScheduledFrame& operator=(ScheduledFrame&&) = default;

    // Executes `callback_`.
    void RunFrameReleaseCallback() &&;

    uint32_t rtp_timestamp() const { return rtp_timestamp_; }
    Timestamp LatestDecodeTime() const;

   private:
    uint32_t rtp_timestamp_;
    FrameDecodeTiming::FrameSchedule schedule_;
    FrameDecodeScheduler::FrameReleaseCallback callback_;
  };

  class SynchronizedFrameDecodeScheduler : public FrameDecodeScheduler {
   public:
    explicit SynchronizedFrameDecodeScheduler(DecodeSynchronizer* sync);
    ~SynchronizedFrameDecodeScheduler() override;

    // Releases the outstanding frame for decoding. This invalidates
    // `next_frame_`. There must be a frame scheduled.
    ScheduledFrame ReleaseNextFrame();

    // Returns `next_frame_.schedule.max_decode_time`. There must be a frame
    // scheduled when this is called.
    Timestamp LatestDecodeTime();

    // FrameDecodeScheduler implementation.
    std::optional<uint32_t> ScheduledRtpTimestamp() override;
    void ScheduleFrame(uint32_t rtp,
                       FrameDecodeTiming::FrameSchedule schedule,
                       FrameReleaseCallback cb) override;
    void CancelOutstanding() override;
    void Stop() override;

   private:
    DecodeSynchronizer* sync_;
    std::optional<ScheduledFrame> next_frame_;
    bool stopped_ = false;
  };

  void OnFrameScheduled(SynchronizedFrameDecodeScheduler* scheduler);
  void RemoveFrameScheduler(SynchronizedFrameDecodeScheduler* scheduler);

  void ScheduleNextTick();
  void OnTick();

  Clock* const clock_;
  TaskQueueBase* const worker_queue_;
  Metronome* const metronome_;

  Timestamp expected_next_tick_ = Timestamp::PlusInfinity();
  std::set<SynchronizedFrameDecodeScheduler*> schedulers_
      RTC_GUARDED_BY(worker_queue_);
  bool tick_scheduled_ = false;
  ScopedTaskSafetyDetached safety_;
};

}  // namespace webrtc

#endif  // VIDEO_DECODE_SYNCHRONIZER_H_

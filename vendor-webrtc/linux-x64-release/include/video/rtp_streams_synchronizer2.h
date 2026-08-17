/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_RTP_STREAMS_SYNCHRONIZER2_H_
#define VIDEO_RTP_STREAMS_SYNCHRONIZER2_H_

#include <memory>

#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/task_utils/repeating_task.h"
#include "video/stream_synchronization.h"

namespace webrtc {

class Syncable;

namespace internal {

// RtpStreamsSynchronizer is responsible for synchronizing audio and video for
// a given audio receive stream and video receive stream.
class RtpStreamsSynchronizer {
 public:
  RtpStreamsSynchronizer(TaskQueueBase* main_queue, Syncable* syncable_video);
  ~RtpStreamsSynchronizer();

  void ConfigureSync(Syncable* syncable_audio);

  // Gets the estimated playout NTP timestamp for the video frame with
  // `rtp_timestamp` and the sync offset between the current played out audio
  // frame and the video frame. Returns true on success, false otherwise.
  // The `estimated_freq_khz` is the frequency used in the RTP to NTP timestamp
  // conversion.
  bool GetStreamSyncOffsetInMs(uint32_t rtp_timestamp,
                               int64_t render_time_ms,
                               int64_t* video_playout_ntp_ms,
                               int64_t* stream_offset_ms,
                               double* estimated_freq_khz) const;

 private:
  void UpdateDelay();

  TaskQueueBase* const task_queue_;

  // Used to check if we're running on the main thread/task queue.
  // The reason we currently don't use RTC_DCHECK_RUN_ON(task_queue_) is because
  // we might be running on an webrtc::Thread implementation of TaskQueue, which
  // does not consistently set itself as the active TaskQueue.
  // Instead, we rely on a SequenceChecker for now.
  RTC_NO_UNIQUE_ADDRESS SequenceChecker main_checker_;

  Syncable* const syncable_video_;

  Syncable* syncable_audio_ RTC_GUARDED_BY(main_checker_) = nullptr;
  std::unique_ptr<StreamSynchronization> sync_ RTC_GUARDED_BY(main_checker_);
  StreamSynchronization::Measurements audio_measurement_
      RTC_GUARDED_BY(main_checker_);
  StreamSynchronization::Measurements video_measurement_
      RTC_GUARDED_BY(main_checker_);
  RepeatingTaskHandle repeating_task_ RTC_GUARDED_BY(main_checker_);
  int64_t last_stats_log_ms_ RTC_GUARDED_BY(&main_checker_);
};

}  // namespace internal
}  // namespace webrtc

#endif  // VIDEO_RTP_STREAMS_SYNCHRONIZER2_H_

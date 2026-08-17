/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_VIDEO_TRACK_SOURCE_H_
#define PC_VIDEO_TRACK_SOURCE_H_

#include <optional>

#include "api/media_stream_interface.h"
#include "api/notifier.h"
#include "api/sequence_checker.h"
#include "api/video/recordable_encoded_frame.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "api/video/video_source_interface.h"
#include "media/base/media_channel.h"
#include "rtc_base/checks.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// VideoTrackSource is a convenience base class for implementations of
// VideoTrackSourceInterface.
class RTC_EXPORT VideoTrackSource : public Notifier<VideoTrackSourceInterface> {
 public:
  explicit VideoTrackSource(bool remote);
  void SetState(SourceState new_state);

  SourceState state() const override {
    RTC_DCHECK_RUN_ON(&signaling_thread_checker_);
    return state_;
  }
  bool remote() const override { return remote_; }

  bool is_screencast() const override { return false; }
  std::optional<bool> needs_denoising() const override { return std::nullopt; }

  bool GetStats(Stats* stats) override { return false; }

  void AddOrUpdateSink(VideoSinkInterface<VideoFrame>* sink,
                       const VideoSinkWants& wants) override;
  void RemoveSink(VideoSinkInterface<VideoFrame>* sink) override;

  bool SupportsEncodedOutput() const override { return false; }
  void GenerateKeyFrame() override {}
  void AddEncodedSink(
      VideoSinkInterface<RecordableEncodedFrame>* sink) override {}
  void RemoveEncodedSink(
      VideoSinkInterface<RecordableEncodedFrame>* sink) override {}

 protected:
  virtual VideoSourceInterface<VideoFrame>* source() = 0;

 private:
  RTC_NO_UNIQUE_ADDRESS SequenceChecker worker_thread_checker_{
      SequenceChecker::kDetached};
  RTC_NO_UNIQUE_ADDRESS SequenceChecker signaling_thread_checker_;
  SourceState state_ RTC_GUARDED_BY(&signaling_thread_checker_);
  const bool remote_;
};

}  // namespace webrtc

#endif  //  PC_VIDEO_TRACK_SOURCE_H_

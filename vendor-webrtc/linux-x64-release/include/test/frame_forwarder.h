/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_FRAME_FORWARDER_H_
#define TEST_FRAME_FORWARDER_H_

#include "api/video/video_frame.h"
#include "api/video/video_source_interface.h"
#include "rtc_base/synchronization/mutex.h"

namespace webrtc {
namespace test {

// FrameForwarder can be used as an implementation
// of webrtc::VideoSourceInterface<VideoFrame> where the caller controls when
// a frame should be forwarded to its sink.
// Currently this implementation only support one sink.
class FrameForwarder : public VideoSourceInterface<VideoFrame> {
 public:
  FrameForwarder();
  ~FrameForwarder() override;
  // Forwards `video_frame` to the registered `sink_`.
  virtual void IncomingCapturedFrame(const VideoFrame& video_frame)
      RTC_LOCKS_EXCLUDED(mutex_);
  VideoSinkWants sink_wants() const RTC_LOCKS_EXCLUDED(mutex_);
  bool has_sinks() const RTC_LOCKS_EXCLUDED(mutex_);

 protected:
  VideoSinkWants sink_wants_locked() const RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  void AddOrUpdateSink(VideoSinkInterface<VideoFrame>* sink,
                       const VideoSinkWants& wants)
      RTC_LOCKS_EXCLUDED(mutex_) override;
  void AddOrUpdateSinkLocked(VideoSinkInterface<VideoFrame>* sink,
                             const VideoSinkWants& wants)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);
  void RemoveSink(VideoSinkInterface<VideoFrame>* sink)
      RTC_LOCKS_EXCLUDED(mutex_) override;

  mutable Mutex mutex_;
  VideoSinkInterface<VideoFrame>* sink_ RTC_GUARDED_BY(mutex_);
  VideoSinkWants sink_wants_ RTC_GUARDED_BY(mutex_);
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_FRAME_FORWARDER_H_

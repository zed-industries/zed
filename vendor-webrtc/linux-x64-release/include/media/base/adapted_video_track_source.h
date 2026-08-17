/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_BASE_ADAPTED_VIDEO_TRACK_SOURCE_H_
#define MEDIA_BASE_ADAPTED_VIDEO_TRACK_SOURCE_H_

#include <stdint.h>

#include <optional>

#include "api/media_stream_interface.h"
#include "api/notifier.h"
#include "api/video/recordable_encoded_frame.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "api/video/video_source_interface.h"
#include "api/video_track_source_constraints.h"
#include "media/base/video_adapter.h"
#include "media/base/video_broadcaster.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// Base class for sources which needs video adaptation, e.g., video
// capture sources. Sinks must be added and removed on one and only
// one thread, while AdaptFrame and OnFrame may be called on any
// thread.
class RTC_EXPORT AdaptedVideoTrackSource
    : public Notifier<VideoTrackSourceInterface> {
 public:
  AdaptedVideoTrackSource();
  ~AdaptedVideoTrackSource() override;

 protected:
  // Allows derived classes to initialize `video_adapter_` with a custom
  // alignment.
  explicit AdaptedVideoTrackSource(int required_alignment);
  // Checks the apply_rotation() flag. If the frame needs rotation, and it is a
  // plain memory frame, it is rotated. Subclasses producing native frames must
  // handle apply_rotation() themselves.
  void OnFrame(const VideoFrame& frame);
  // Indication from source that a frame was dropped.
  void OnFrameDropped();

  // Reports the appropriate frame size after adaptation. Returns true
  // if a frame is wanted. Returns false if there are no interested
  // sinks, or if the VideoAdapter decides to drop the frame.
  bool AdaptFrame(int width,
                  int height,
                  int64_t time_us,
                  int* out_width,
                  int* out_height,
                  int* crop_width,
                  int* crop_height,
                  int* crop_x,
                  int* crop_y);

  // Returns the current value of the apply_rotation flag, derived
  // from the VideoSinkWants of registered sinks. The value is derived
  // from sinks' wants, in AddOrUpdateSink and RemoveSink. Beware that
  // when using this method from a different thread, the value may
  // become stale before it is used.
  bool apply_rotation();

  VideoAdapter* video_adapter() { return &video_adapter_; }

 private:
  // Implements webrtc::VideoSourceInterface.
  void AddOrUpdateSink(VideoSinkInterface<VideoFrame>* sink,
                       const VideoSinkWants& wants) override;
  void RemoveSink(VideoSinkInterface<VideoFrame>* sink) override;

  // Part of VideoTrackSourceInterface.
  bool GetStats(Stats* stats) override;

  void OnSinkWantsChanged(const VideoSinkWants& wants);

  // Encoded sinks not implemented for AdaptedVideoTrackSource.
  bool SupportsEncodedOutput() const override { return false; }
  void GenerateKeyFrame() override {}
  void AddEncodedSink(
      VideoSinkInterface<RecordableEncodedFrame>* /* sink */) override {}
  void RemoveEncodedSink(
      VideoSinkInterface<RecordableEncodedFrame>* /* sink */) override {}
  void ProcessConstraints(
      const VideoTrackSourceConstraints& constraints) override;

  VideoAdapter video_adapter_;

  Mutex stats_mutex_;
  std::optional<Stats> stats_ RTC_GUARDED_BY(stats_mutex_);

  VideoBroadcaster broadcaster_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::AdaptedVideoTrackSource;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_BASE_ADAPTED_VIDEO_TRACK_SOURCE_H_

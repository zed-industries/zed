/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_VIDEO_RTP_TRACK_SOURCE_H_
#define PC_VIDEO_RTP_TRACK_SOURCE_H_

#include <vector>

#include "api/sequence_checker.h"
#include "api/video/recordable_encoded_frame.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "api/video/video_source_interface.h"
#include "media/base/video_broadcaster.h"
#include "pc/video_track_source.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// Video track source in use by VideoRtpReceiver
class VideoRtpTrackSource : public VideoTrackSource {
 public:
  class Callback {
   public:
    virtual ~Callback() = default;

    // Called when a keyframe should be generated
    virtual void OnGenerateKeyFrame() = 0;

    // Called when the implementor should eventually start to serve encoded
    // frames using BroadcastEncodedFrameBuffer.
    // The implementor should cause a keyframe to be eventually generated.
    virtual void OnEncodedSinkEnabled(bool enable) = 0;
  };

  explicit VideoRtpTrackSource(Callback* callback);

  VideoRtpTrackSource(const VideoRtpTrackSource&) = delete;
  VideoRtpTrackSource& operator=(const VideoRtpTrackSource&) = delete;

  // Call before the object implementing Callback finishes it's destructor. No
  // more callbacks will be fired after completion. Must be called on the
  // worker thread
  void ClearCallback();

  // Call to broadcast an encoded frame to registered sinks.
  // This method can be called on any thread or queue.
  void BroadcastRecordableEncodedFrame(
      const RecordableEncodedFrame& frame) const;

  // VideoTrackSource
  VideoSourceInterface<VideoFrame>* source() override;
  VideoSinkInterface<VideoFrame>* sink();

  // Returns true. This method can be called on any thread.
  bool SupportsEncodedOutput() const override;

  // Generates a key frame. Must be called on the worker thread.
  void GenerateKeyFrame() override;

  // Adds an encoded sink. Must be called on the worker thread.
  void AddEncodedSink(
      VideoSinkInterface<RecordableEncodedFrame>* sink) override;

  // Removes an encoded sink. Must be called on the worker thread.
  void RemoveEncodedSink(
      VideoSinkInterface<RecordableEncodedFrame>* sink) override;

 private:
  RTC_NO_UNIQUE_ADDRESS SequenceChecker worker_sequence_checker_{
      SequenceChecker::kDetached};
  // `broadcaster_` is needed since the decoder can only handle one sink.
  // It might be better if the decoder can handle multiple sinks and consider
  // the VideoSinkWants.
  VideoBroadcaster broadcaster_;
  mutable Mutex mu_;
  std::vector<VideoSinkInterface<RecordableEncodedFrame>*> encoded_sinks_
      RTC_GUARDED_BY(mu_);
  Callback* callback_ RTC_GUARDED_BY(worker_sequence_checker_);
};

}  // namespace webrtc

#endif  // PC_VIDEO_RTP_TRACK_SOURCE_H_

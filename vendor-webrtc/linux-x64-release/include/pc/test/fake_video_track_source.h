/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TEST_FAKE_VIDEO_TRACK_SOURCE_H_
#define PC_TEST_FAKE_VIDEO_TRACK_SOURCE_H_

#include "api/make_ref_counted.h"
#include "api/scoped_refptr.h"
#include "api/video/video_frame.h"
#include "api/video/video_source_interface.h"
#include "media/base/video_broadcaster.h"
#include "pc/video_track_source.h"

namespace webrtc {

// A minimal implementation of VideoTrackSource. Includes a VideoBroadcaster for
// injection of frames.
class FakeVideoTrackSource : public VideoTrackSource {
 public:
  static scoped_refptr<FakeVideoTrackSource> Create(bool is_screencast) {
    return make_ref_counted<FakeVideoTrackSource>(is_screencast);
  }

  static scoped_refptr<FakeVideoTrackSource> Create() { return Create(false); }

  bool is_screencast() const override { return is_screencast_; }

  void InjectFrame(const VideoFrame& frame) {
    video_broadcaster_.OnFrame(frame);
  }

 protected:
  explicit FakeVideoTrackSource(bool is_screencast)
      : VideoTrackSource(false /* remote */), is_screencast_(is_screencast) {}
  ~FakeVideoTrackSource() override = default;

  VideoSourceInterface<VideoFrame>* source() override {
    return &video_broadcaster_;
  }

 private:
  const bool is_screencast_;
  VideoBroadcaster video_broadcaster_;
};

}  // namespace webrtc

#endif  // PC_TEST_FAKE_VIDEO_TRACK_SOURCE_H_

/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TEST_FAKE_PERIODIC_VIDEO_TRACK_SOURCE_H_
#define PC_TEST_FAKE_PERIODIC_VIDEO_TRACK_SOURCE_H_

#include "api/video/video_frame.h"
#include "api/video/video_source_interface.h"
#include "pc/test/fake_periodic_video_source.h"
#include "pc/video_track_source.h"

namespace webrtc {

// A VideoTrackSource generating frames with configured size and frame interval.
class FakePeriodicVideoTrackSource : public VideoTrackSource {
 public:
  explicit FakePeriodicVideoTrackSource(bool remote)
      : FakePeriodicVideoTrackSource(FakePeriodicVideoSource::Config(),
                                     remote) {}

  FakePeriodicVideoTrackSource(FakePeriodicVideoSource::Config config,
                               bool remote)
      : VideoTrackSource(remote), source_(config) {}

  ~FakePeriodicVideoTrackSource() = default;

  FakePeriodicVideoSource& fake_periodic_source() { return source_; }
  const FakePeriodicVideoSource& fake_periodic_source() const {
    return source_;
  }

 protected:
  VideoSourceInterface<VideoFrame>* source() override { return &source_; }

 private:
  FakePeriodicVideoSource source_;
};

}  // namespace webrtc

#endif  // PC_TEST_FAKE_PERIODIC_VIDEO_TRACK_SOURCE_H_

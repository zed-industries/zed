/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_ADAPTATION_TEST_FAKE_VIDEO_STREAM_INPUT_STATE_PROVIDER_H_
#define CALL_ADAPTATION_TEST_FAKE_VIDEO_STREAM_INPUT_STATE_PROVIDER_H_

#include "call/adaptation/video_stream_input_state.h"
#include "call/adaptation/video_stream_input_state_provider.h"

namespace webrtc {

class FakeVideoStreamInputStateProvider : public VideoStreamInputStateProvider {
 public:
  FakeVideoStreamInputStateProvider();
  virtual ~FakeVideoStreamInputStateProvider();

  void SetInputState(int input_pixels, int input_fps, int min_pixels_per_frame);
  VideoStreamInputState InputState() override;

 private:
  VideoStreamInputState fake_input_state_;
};

}  // namespace webrtc

#endif  // CALL_ADAPTATION_TEST_FAKE_VIDEO_STREAM_INPUT_STATE_PROVIDER_H_

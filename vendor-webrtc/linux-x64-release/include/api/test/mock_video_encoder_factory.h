/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_VIDEO_ENCODER_FACTORY_H_
#define API_TEST_MOCK_VIDEO_ENCODER_FACTORY_H_

#include <memory>
#include <vector>

#include "api/environment/environment.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_encoder.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "test/gmock.h"

namespace webrtc {

class MockVideoEncoderFactory : public webrtc::VideoEncoderFactory {
 public:
  ~MockVideoEncoderFactory() override { Die(); }

  MOCK_METHOD(std::vector<SdpVideoFormat>,
              GetSupportedFormats,
              (),
              (const, override));
  MOCK_METHOD(std::unique_ptr<VideoEncoder>,
              Create,
              (const Environment&, const SdpVideoFormat&),
              (override));

  MOCK_METHOD(void, Die, ());
};

}  // namespace webrtc

#endif  // API_TEST_MOCK_VIDEO_ENCODER_FACTORY_H_

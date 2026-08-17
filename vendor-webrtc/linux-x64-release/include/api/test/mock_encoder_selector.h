/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_ENCODER_SELECTOR_H_
#define API_TEST_MOCK_ENCODER_SELECTOR_H_

#include <optional>

#include "api/units/data_rate.h"
#include "api/video/render_resolution.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "test/gmock.h"

namespace webrtc {

class MockEncoderSelector
    : public VideoEncoderFactory::EncoderSelectorInterface {
 public:
  MOCK_METHOD(void,
              OnCurrentEncoder,
              (const SdpVideoFormat& format),
              (override));

  MOCK_METHOD(std::optional<SdpVideoFormat>,
              OnAvailableBitrate,
              (const DataRate& rate),
              (override));

  MOCK_METHOD(std::optional<SdpVideoFormat>,
              OnResolutionChange,
              (const RenderResolution& resolution),
              (override));

  MOCK_METHOD(std::optional<SdpVideoFormat>, OnEncoderBroken, (), (override));
};

}  // namespace webrtc

#endif  // API_TEST_MOCK_ENCODER_SELECTOR_H_

/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_TEST_MOCK_RECORDABLE_ENCODED_FRAME_H_
#define API_VIDEO_TEST_MOCK_RECORDABLE_ENCODED_FRAME_H_

#include <optional>

#include "api/scoped_refptr.h"
#include "api/units/timestamp.h"
#include "api/video/color_space.h"
#include "api/video/encoded_image.h"
#include "api/video/recordable_encoded_frame.h"
#include "api/video/video_codec_type.h"
#include "api/video/video_rotation.h"
#include "test/gmock.h"

namespace webrtc {
class MockRecordableEncodedFrame : public RecordableEncodedFrame {
 public:
  MOCK_METHOD(scoped_refptr<const EncodedImageBufferInterface>,
              encoded_buffer,
              (),
              (const, override));
  MOCK_METHOD(std::optional<webrtc::ColorSpace>,
              color_space,
              (),
              (const, override));
  MOCK_METHOD(std::optional<webrtc::VideoRotation>,
              video_rotation,
              (),
              (const, override));
  MOCK_METHOD(VideoCodecType, codec, (), (const, override));
  MOCK_METHOD(bool, is_key_frame, (), (const, override));
  MOCK_METHOD(EncodedResolution, resolution, (), (const, override));
  MOCK_METHOD(Timestamp, render_time, (), (const, override));
};
}  // namespace webrtc
#endif  // API_VIDEO_TEST_MOCK_RECORDABLE_ENCODED_FRAME_H_

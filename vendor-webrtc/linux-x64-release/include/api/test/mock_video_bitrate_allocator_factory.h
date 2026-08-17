/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_VIDEO_BITRATE_ALLOCATOR_FACTORY_H_
#define API_TEST_MOCK_VIDEO_BITRATE_ALLOCATOR_FACTORY_H_

#include <memory>

#include "api/environment/environment.h"
#include "api/video/video_bitrate_allocator.h"
#include "api/video/video_bitrate_allocator_factory.h"
#include "api/video_codecs/video_codec.h"
#include "test/gmock.h"

namespace webrtc {

class MockVideoBitrateAllocatorFactory : public VideoBitrateAllocatorFactory {
 public:
  ~MockVideoBitrateAllocatorFactory() override { Die(); }
  MOCK_METHOD(std::unique_ptr<VideoBitrateAllocator>,
              Create,
              (const Environment&, const VideoCodec&),
              (override));
  MOCK_METHOD(void, Die, ());
};

}  // namespace webrtc

#endif  // API_TEST_MOCK_VIDEO_BITRATE_ALLOCATOR_FACTORY_H_

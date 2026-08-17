/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_VIDEO_BITRATE_ALLOCATOR_H_
#define API_TEST_MOCK_VIDEO_BITRATE_ALLOCATOR_H_

#include "api/video/video_bitrate_allocation.h"
#include "api/video/video_bitrate_allocator.h"
#include "test/gmock.h"

namespace webrtc {

class MockVideoBitrateAllocator : public webrtc::VideoBitrateAllocator {
  MOCK_METHOD(VideoBitrateAllocation,
              Allocate,
              (VideoBitrateAllocationParameters parameters),
              (override));
};

}  // namespace webrtc

#endif  // API_TEST_MOCK_VIDEO_BITRATE_ALLOCATOR_H_

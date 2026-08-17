/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef CALL_TEST_MOCK_BITRATE_ALLOCATOR_H_
#define CALL_TEST_MOCK_BITRATE_ALLOCATOR_H_


#include "call/bitrate_allocator.h"
#include "test/gmock.h"

namespace webrtc {
class MockBitrateAllocator : public BitrateAllocatorInterface {
 public:
  MOCK_METHOD(void,
              AddObserver,
              (BitrateAllocatorObserver*, MediaStreamAllocationConfig),
              (override));
  MOCK_METHOD(void, RemoveObserver, (BitrateAllocatorObserver*), (override));
  MOCK_METHOD(int,
              GetStartBitrate,
              (BitrateAllocatorObserver*),
              (const, override));
};
}  // namespace webrtc
#endif  // CALL_TEST_MOCK_BITRATE_ALLOCATOR_H_

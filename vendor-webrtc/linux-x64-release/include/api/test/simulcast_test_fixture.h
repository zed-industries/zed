/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_SIMULCAST_TEST_FIXTURE_H_
#define API_TEST_SIMULCAST_TEST_FIXTURE_H_

namespace webrtc {
namespace test {

class SimulcastTestFixture {
 public:
  virtual ~SimulcastTestFixture() = default;

  virtual void TestKeyFrameRequestsOnAllStreams() = 0;
  virtual void TestKeyFrameRequestsOnSpecificStreams() = 0;
  virtual void TestPaddingAllStreams() = 0;
  virtual void TestPaddingTwoStreams() = 0;
  virtual void TestPaddingTwoStreamsOneMaxedOut() = 0;
  virtual void TestPaddingOneStream() = 0;
  virtual void TestPaddingOneStreamTwoMaxedOut() = 0;
  virtual void TestSendAllStreams() = 0;
  virtual void TestDisablingStreams() = 0;
  virtual void TestActiveStreams() = 0;
  virtual void TestSwitchingToOneStream() = 0;
  virtual void TestSwitchingToOneOddStream() = 0;
  virtual void TestSwitchingToOneSmallStream() = 0;
  virtual void TestSpatioTemporalLayers333PatternEncoder() = 0;
  virtual void TestSpatioTemporalLayers321PatternEncoder() = 0;
  virtual void TestStrideEncodeDecode() = 0;
  virtual void TestDecodeWidthHeightSet() = 0;
  virtual void
  TestEncoderInfoForDefaultTemporalLayerProfileHasFpsAllocation() = 0;
};

}  // namespace test
}  // namespace webrtc

#endif  // API_TEST_SIMULCAST_TEST_FIXTURE_H_

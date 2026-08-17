/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_UTILITY_SIMULCAST_TEST_FIXTURE_IMPL_H_
#define MODULES_VIDEO_CODING_UTILITY_SIMULCAST_TEST_FIXTURE_IMPL_H_

#include <cstdint>
#include <memory>
#include <vector>

#include "api/environment/environment.h"
#include "api/scoped_refptr.h"
#include "api/test/mock_video_decoder.h"
#include "api/test/mock_video_encoder.h"
#include "api/test/simulcast_test_fixture.h"
#include "api/video/i420_buffer.h"
#include "api/video/video_codec_type.h"
#include "api/video/video_frame.h"
#include "api/video/video_frame_type.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_codec.h"
#include "api/video_codecs/video_decoder.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "modules/video_coding/utility/simulcast_rate_allocator.h"

namespace webrtc {
namespace test {

class SimulcastTestFixtureImpl final : public SimulcastTestFixture {
 public:
  SimulcastTestFixtureImpl(std::unique_ptr<VideoEncoderFactory> encoder_factory,
                           std::unique_ptr<VideoDecoderFactory> decoder_factory,
                           SdpVideoFormat video_format);
  ~SimulcastTestFixtureImpl() final;

  // Implements SimulcastTestFixture.
  void TestKeyFrameRequestsOnAllStreams() override;
  void TestKeyFrameRequestsOnSpecificStreams() override;
  void TestPaddingAllStreams() override;
  void TestPaddingTwoStreams() override;
  void TestPaddingTwoStreamsOneMaxedOut() override;
  void TestPaddingOneStream() override;
  void TestPaddingOneStreamTwoMaxedOut() override;
  void TestSendAllStreams() override;
  void TestDisablingStreams() override;
  void TestActiveStreams() override;
  void TestSwitchingToOneStream() override;
  void TestSwitchingToOneOddStream() override;
  void TestSwitchingToOneSmallStream() override;
  void TestSpatioTemporalLayers333PatternEncoder() override;
  void TestSpatioTemporalLayers321PatternEncoder() override;
  void TestStrideEncodeDecode() override;
  void TestDecodeWidthHeightSet() override;
  void TestEncoderInfoForDefaultTemporalLayerProfileHasFpsAllocation() override;

  static void DefaultSettings(VideoCodec* settings,
                              const int* temporal_layer_profile,
                              VideoCodecType codec_type,
                              bool reverse_layer_order = false);

 private:
  class TestEncodedImageCallback;
  class TestDecodedImageCallback;

  void SetUpCodec(const int* temporal_layer_profile);
  void SetUpRateAllocator();
  void SetRates(uint32_t bitrate_kbps, uint32_t fps);
  void RunActiveStreamsTest(std::vector<bool> active_streams);
  void UpdateActiveStreams(std::vector<bool> active_streams);
  void ExpectStream(VideoFrameType frame_type, int scaleResolutionDownBy);
  void ExpectStreams(VideoFrameType frame_type,
                     std::vector<bool> expected_streams_active);
  void ExpectStreams(VideoFrameType frame_type, int expected_video_streams);
  void VerifyTemporalIdxAndSyncForAllSpatialLayers(
      TestEncodedImageCallback* encoder_callback,
      const int* expected_temporal_idx,
      const bool* expected_layer_sync,
      int num_spatial_layers);
  void SwitchingToOneStream(int width, int height);

  const Environment env_;
  std::unique_ptr<VideoEncoder> encoder_;
  MockEncodedImageCallback encoder_callback_;
  std::unique_ptr<VideoDecoder> decoder_;
  MockDecodedImageCallback decoder_callback_;
  VideoCodec settings_;
  scoped_refptr<I420Buffer> input_buffer_;
  std::unique_ptr<VideoFrame> input_frame_;
  std::unique_ptr<SimulcastRateAllocator> rate_allocator_;
  VideoCodecType codec_type_;
};

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_UTILITY_SIMULCAST_TEST_FIXTURE_IMPL_H_

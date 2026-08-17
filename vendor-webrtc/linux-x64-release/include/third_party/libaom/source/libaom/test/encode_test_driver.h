/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */
#ifndef AOM_TEST_ENCODE_TEST_DRIVER_H_
#define AOM_TEST_ENCODE_TEST_DRIVER_H_

#include <string>
#include <vector>

#include "gtest/gtest.h"

#include "config/aom_config.h"

#if CONFIG_AV1_ENCODER
#include "aom/aomcx.h"
#endif
#include "aom/aom_encoder.h"

namespace libaom_test {

class CodecFactory;
class VideoSource;

enum TestMode { kRealTime, kOnePassGood, kTwoPassGood, kAllIntra };
#define ALL_TEST_MODES                                                     \
  ::testing::Values(::libaom_test::kRealTime, ::libaom_test::kOnePassGood, \
                    ::libaom_test::kTwoPassGood)

#define ONE_PASS_TEST_MODES \
  ::testing::Values(::libaom_test::kRealTime, ::libaom_test::kOnePassGood)

#define TWO_PASS_TEST_MODES ::testing::Values(::libaom_test::kTwoPassGood)

#define NONREALTIME_TEST_MODES \
  ::testing::Values(::libaom_test::kOnePassGood, ::libaom_test::kTwoPassGood)

// Provides an object to handle the libaom get_cx_data() iteration pattern
class CxDataIterator {
 public:
  explicit CxDataIterator(aom_codec_ctx_t *encoder)
      : encoder_(encoder), iter_(nullptr) {}

  const aom_codec_cx_pkt_t *Next() {
    return aom_codec_get_cx_data(encoder_, &iter_);
  }

 private:
  aom_codec_ctx_t *encoder_;
  aom_codec_iter_t iter_;
};

// Implements an in-memory store for libaom twopass statistics
class TwopassStatsStore {
 public:
  void Append(const aom_codec_cx_pkt_t &pkt) {
    buffer_.append(reinterpret_cast<char *>(pkt.data.twopass_stats.buf),
                   pkt.data.twopass_stats.sz);
  }

  aom_fixed_buf_t buf() {
    const aom_fixed_buf_t buf = { &buffer_[0], buffer_.size() };
    return buf;
  }

  void Reset() { buffer_.clear(); }

 protected:
  std::string buffer_;
};

// Provides a simplified interface to manage one video encoding pass, given
// a configuration and video source.
//
// TODO(jkoleszar): The exact services it provides and the appropriate
// level of abstraction will be fleshed out as more tests are written.
class Encoder {
 public:
  Encoder(aom_codec_enc_cfg_t cfg, const aom_codec_flags_t init_flags,
          TwopassStatsStore *stats)
      : cfg_(cfg), init_flags_(init_flags), stats_(stats) {
    memset(&encoder_, 0, sizeof(encoder_));
  }

  virtual ~Encoder() { aom_codec_destroy(&encoder_); }

  CxDataIterator GetCxData() { return CxDataIterator(&encoder_); }

  void InitEncoder(VideoSource *video);

  const aom_image_t *GetPreviewFrame() {
    return aom_codec_get_preview_frame(&encoder_);
  }
  // This is a thin wrapper around aom_codec_encode(), so refer to
  // aom_encoder.h for its semantics.
  void EncodeFrame(VideoSource *video, aom_enc_frame_flags_t frame_flags);

  // Convenience wrapper for EncodeFrame()
  void EncodeFrame(VideoSource *video) { EncodeFrame(video, 0); }

  void Control(int ctrl_id, int arg) {
    const aom_codec_err_t res = aom_codec_control(&encoder_, ctrl_id, arg);
    ASSERT_EQ(AOM_CODEC_OK, res) << EncoderError();
  }

  void Control(int ctrl_id, int *arg) {
    const aom_codec_err_t res = aom_codec_control(&encoder_, ctrl_id, arg);
    ASSERT_EQ(AOM_CODEC_OK, res) << EncoderError();
  }

  void Control(int ctrl_id, struct aom_scaling_mode *arg) {
    const aom_codec_err_t res = aom_codec_control(&encoder_, ctrl_id, arg);
    ASSERT_EQ(AOM_CODEC_OK, res) << EncoderError();
  }

  void Control(int ctrl_id, struct aom_svc_layer_id *arg) {
    const aom_codec_err_t res = aom_codec_control(&encoder_, ctrl_id, arg);
    ASSERT_EQ(AOM_CODEC_OK, res) << EncoderError();
  }

  void Control(int ctrl_id, struct aom_svc_ref_frame_config *arg) {
    const aom_codec_err_t res = aom_codec_control(&encoder_, ctrl_id, arg);
    ASSERT_EQ(AOM_CODEC_OK, res) << EncoderError();
  }

  void Control(int ctrl_id, struct aom_svc_ref_frame_comp_pred *arg) {
    const aom_codec_err_t res = aom_codec_control(&encoder_, ctrl_id, arg);
    ASSERT_EQ(AOM_CODEC_OK, res) << EncoderError();
  }

  void Control(int ctrl_id, struct aom_svc_params *arg) {
    const aom_codec_err_t res = aom_codec_control(&encoder_, ctrl_id, arg);
    ASSERT_EQ(AOM_CODEC_OK, res) << EncoderError();
  }

  void Control(int ctrl_id, struct aom_ext_part_funcs *arg) {
    const aom_codec_err_t res = aom_codec_control(&encoder_, ctrl_id, arg);
    ASSERT_EQ(AOM_CODEC_OK, res) << EncoderError();
  }

#if CONFIG_AV1_ENCODER
  void Control(int ctrl_id, aom_active_map_t *arg) {
    const aom_codec_err_t res = aom_codec_control(&encoder_, ctrl_id, arg);
    ASSERT_EQ(AOM_CODEC_OK, res) << EncoderError();
  }
#endif

  void SetOption(const char *name, const char *value) {
    const aom_codec_err_t res = aom_codec_set_option(&encoder_, name, value);
    ASSERT_EQ(AOM_CODEC_OK, res) << EncoderError();
  }

  void Config(const aom_codec_enc_cfg_t *cfg) {
    const aom_codec_err_t res = aom_codec_enc_config_set(&encoder_, cfg);
    ASSERT_EQ(AOM_CODEC_OK, res) << EncoderError();
    cfg_ = *cfg;
  }

 protected:
  virtual aom_codec_iface_t *CodecInterface() const = 0;

  const char *EncoderError() {
    const char *detail = aom_codec_error_detail(&encoder_);
    return detail ? detail : aom_codec_error(&encoder_);
  }

  // Encode an image
  void EncodeFrameInternal(const VideoSource &video,
                           aom_enc_frame_flags_t frame_flags);

  // Flush the encoder on EOS
  void Flush();

  aom_codec_ctx_t encoder_;
  aom_codec_enc_cfg_t cfg_;
  aom_codec_flags_t init_flags_;
  TwopassStatsStore *stats_;
};

// Common test functionality for all Encoder tests.
//
// This class is a mixin which provides the main loop common to all
// encoder tests. It provides hooks which can be overridden by subclasses
// to implement each test's specific behavior, while centralizing the bulk
// of the boilerplate. Note that it doesn't inherit the gtest testing
// classes directly, so that tests can be parameterized differently.
class EncoderTest {
 protected:
  explicit EncoderTest(const CodecFactory *codec)
      : codec_(codec), abort_(false), init_flags_(0), frame_flags_(0),
        mode_(kRealTime) {
    // Default to 1 thread.
    cfg_.g_threads = 1;
  }

  virtual ~EncoderTest() = default;

  // Initialize the cfg_ member with the default configuration for the
  // TestMode enum and maps the TestMode enum to the passes_ variable.
  void InitializeConfig(TestMode mode);

  // Set encoder flag.
  void set_init_flags(aom_codec_flags_t flag) { init_flags_ = flag; }

  // Main loop
  virtual void RunLoop(VideoSource *video);

  // Hook to be called at the beginning of a pass.
  virtual void BeginPassHook(unsigned int /*pass*/) {}

  // Hook to be called at the end of a pass.
  virtual void EndPassHook() {}

  // Hook to be called before encoding a frame.
  virtual void PreEncodeFrameHook(VideoSource * /*video*/,
                                  Encoder * /*encoder*/) {}

  virtual void PostEncodeFrameHook(Encoder * /*encoder*/) {}

  // Hook to be called on every compressed data packet.
  virtual void FramePktHook(const aom_codec_cx_pkt_t * /*pkt*/) {}

  // Hook to be called on every PSNR packet.
  virtual void PSNRPktHook(const aom_codec_cx_pkt_t * /*pkt*/) {}

  // Hook to be called on every first pass stats packet.
  virtual void StatsPktHook(const aom_codec_cx_pkt_t * /*pkt*/) {}

  // Calculates SSIM at frame level.
  virtual void CalculateFrameLevelSSIM(const aom_image_t * /*img_src*/,
                                       const aom_image_t * /*img_enc*/,
                                       aom_bit_depth_t /*bit_depth*/,
                                       unsigned int /*input_bit_depth*/) {}

  // Hook to determine whether the encode loop should continue.
  virtual bool Continue() const {
    return !(::testing::Test::HasFatalFailure() || abort_);
  }

  // Hook to determine whether to decode frame after encoding
  virtual bool DoDecode() const { return true; }

  // Hook to determine whether to decode invisible frames after encoding
  virtual bool DoDecodeInvisible() const { return true; }

  // Hook to handle encode/decode mismatch
  virtual void MismatchHook(const aom_image_t *img1, const aom_image_t *img2);

  // Hook to be called on every decompressed frame.
  virtual void DecompressedFrameHook(const aom_image_t & /*img*/,
                                     aom_codec_pts_t /*pts*/) {}

  // Hook to be called to handle decode result. Return true to continue.
  virtual bool HandleDecodeResult(const aom_codec_err_t res_dec,
                                  Decoder *decoder) {
    EXPECT_EQ(AOM_CODEC_OK, res_dec) << decoder->DecodeError();
    return AOM_CODEC_OK == res_dec;
  }

  virtual int GetNumSpatialLayers() { return 1; }

  // Hook that can modify the encoder's output data
  virtual const aom_codec_cx_pkt_t *MutateEncoderOutputHook(
      const aom_codec_cx_pkt_t *pkt) {
    return pkt;
  }

  const CodecFactory *codec_;
  bool abort_;
  aom_codec_enc_cfg_t cfg_;
  unsigned int passes_;
  TwopassStatsStore stats_;
  aom_codec_flags_t init_flags_;
  aom_enc_frame_flags_t frame_flags_;
  TestMode mode_;
};

}  // namespace libaom_test

#endif  // AOM_TEST_ENCODE_TEST_DRIVER_H_

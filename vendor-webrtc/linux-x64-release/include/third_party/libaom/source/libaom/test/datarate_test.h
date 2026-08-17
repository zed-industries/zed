/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#include "config/aom_config.h"

#include "gtest/gtest.h"
#include "test/codec_factory.h"
#include "test/encode_test_driver.h"
#include "test/i420_video_source.h"
#include "test/util.h"
#include "test/y4m_video_source.h"
#include "aom/aom_codec.h"

namespace datarate_test {
namespace {
class DatarateTest : public ::libaom_test::EncoderTest {
 public:
  explicit DatarateTest(const ::libaom_test::CodecFactory *codec)
      : EncoderTest(codec), set_cpu_used_(0), aq_mode_(0),
        speed_change_test_(false) {}

 protected:
  ~DatarateTest() override = default;

  virtual void ResetModel() {
    last_pts_ = 0;
    bits_in_buffer_model_ = cfg_.rc_target_bitrate * cfg_.rc_buf_initial_sz;
    frame_number_ = 0;
    tot_frame_number_ = 0;
    first_drop_ = 0;
    num_drops_ = 0;
    // Denoiser is off by default.
    denoiser_on_ = 0;
    bits_total_ = 0;
    denoiser_offon_test_ = 0;
    denoiser_offon_period_ = -1;
    tile_columns_ = 0;
    tile_rows_ = 0;
    auto_tiles_ = false;
    screen_mode_ = false;
    max_perc_spike_ = 1.0;
    max_perc_spike_high_ = 1.0;
    num_spikes_ = 0;
    num_spikes_high_ = 0;
    frame_update_bitrate_ = 0;
    for (int i = 0; i < 3; i++) {
      target_bitrate_update_[i] = 0;
      frame_number_dynamic_[i] = 0;
      bits_total_dynamic_[i] = 0;
      effective_datarate_dynamic_[i] = 0.0;
    }
    avif_mode_ = 0;
  }

  void PreEncodeFrameHook(::libaom_test::VideoSource *video,
                          ::libaom_test::Encoder *encoder) override {
    if (video->frame() == 0) {
      encoder->Control(AOME_SET_CPUUSED, set_cpu_used_);
      encoder->Control(AV1E_SET_AQ_MODE, aq_mode_);
      if (auto_tiles_) {
        encoder->Control(AV1E_SET_AUTO_TILES, 1);
      } else {
        encoder->Control(AV1E_SET_TILE_COLUMNS, tile_columns_);
        encoder->Control(AV1E_SET_TILE_ROWS, tile_rows_);
      }
      encoder->Control(AV1E_SET_ROW_MT, 1);
      if (cfg_.g_usage == AOM_USAGE_REALTIME) {
        encoder->Control(AV1E_SET_ENABLE_GLOBAL_MOTION, 0);
        encoder->Control(AV1E_SET_ENABLE_WARPED_MOTION, 0);
        encoder->Control(AV1E_SET_ENABLE_RESTORATION, 0);
        encoder->Control(AV1E_SET_ENABLE_OBMC, 0);
        encoder->Control(AV1E_SET_DELTAQ_MODE, 0);
        encoder->Control(AV1E_SET_ENABLE_TPL_MODEL, 0);
        encoder->Control(AV1E_SET_ENABLE_CDEF, 1);
        encoder->Control(AV1E_SET_COEFF_COST_UPD_FREQ, 2);
        encoder->Control(AV1E_SET_MODE_COST_UPD_FREQ, 2);
        encoder->Control(AV1E_SET_MV_COST_UPD_FREQ, 2);
        encoder->Control(AV1E_SET_DV_COST_UPD_FREQ, 2);
        encoder->Control(AV1E_SET_POSTENCODE_DROP_RTC, 1);
      }
      if (screen_mode_) {
        encoder->Control(AV1E_SET_TUNE_CONTENT, AOM_CONTENT_SCREEN);
        encoder->Control(AV1E_SET_ENABLE_PALETTE, 1);
        encoder->Control(AV1E_SET_ENABLE_INTRABC, 0);
      }
      if (avif_mode_) {
        encoder->Control(AV1E_SET_COEFF_COST_UPD_FREQ, 0);
        encoder->Control(AV1E_SET_MODE_COST_UPD_FREQ, 0);
        encoder->Control(AV1E_SET_MV_COST_UPD_FREQ, 0);
#if !CONFIG_REALTIME_ONLY
        encoder->Control(AV1E_SET_DELTAQ_MODE, 3);
#endif
#if CONFIG_QUANT_MATRIX
        encoder->Control(AV1E_SET_ENABLE_QM, 1);
#endif
        encoder->Control(AOME_SET_SHARPNESS, 1);
        encoder->Control(AV1E_SET_ENABLE_CHROMA_DELTAQ, 1);
        encoder->Control(AOME_SET_CQ_LEVEL, 0);
        encoder->Control(AV1E_SET_AQ_MODE, (aq_mode_ > 0) ? 1 : 0);
      }
    }

    if (speed_change_test_) {
      if (video->frame() == 0) {
        encoder->Control(AOME_SET_CPUUSED, 8);
      } else if (video->frame() == 30) {
        encoder->Control(AOME_SET_CPUUSED, 7);
      } else if (video->frame() == 60) {
        encoder->Control(AOME_SET_CPUUSED, 6);
      } else if (video->frame() == 90) {
        encoder->Control(AOME_SET_CPUUSED, 7);
      }
    }

    if (frame_update_bitrate_ > 0) {
      if (frame_number_ == frame_update_bitrate_) {
        cfg_.rc_target_bitrate = target_bitrate_update_[1];
        encoder->Config(&cfg_);
      } else if (frame_number_ == 2 * frame_update_bitrate_) {
        cfg_.rc_target_bitrate = target_bitrate_update_[2];
        encoder->Config(&cfg_);
      }
    }

    if (denoiser_offon_test_) {
      ASSERT_GT(denoiser_offon_period_, 0)
          << "denoiser_offon_period_ is not positive.";
      if ((video->frame() + 1) % denoiser_offon_period_ == 0) {
        // Flip denoiser_on_ periodically
        denoiser_on_ ^= 1;
      }
    }

    encoder->Control(AV1E_SET_NOISE_SENSITIVITY, denoiser_on_);

    const aom_rational_t tb = video->timebase();
    timebase_ = static_cast<double>(tb.num) / tb.den;
    duration_ = 0;
  }

  void FramePktHook(const aom_codec_cx_pkt_t *pkt) override {
    // Time since last timestamp = duration.
    aom_codec_pts_t duration = pkt->data.frame.pts - last_pts_;

    if (duration > 1) {
      // If first drop not set and we have a drop set it to this time.
      if (!first_drop_) first_drop_ = last_pts_ + 1;
      // Update the number of frame drops.
      num_drops_ += static_cast<int>(duration - 1);
      // Update counter for total number of frames (#frames input to encoder).
      // Needed for setting the proper layer_id below.
      tot_frame_number_ += static_cast<int>(duration - 1);
    }

    // Add to the buffer the bits we'd expect from a constant bitrate server.
    bits_in_buffer_model_ += static_cast<int64_t>(
        duration * timebase_ * cfg_.rc_target_bitrate * 1000);

    // Buffer should not go negative.
    ASSERT_GE(bits_in_buffer_model_, 0)
        << "Buffer Underrun at frame " << pkt->data.frame.pts;

    const size_t frame_size_in_bits = pkt->data.frame.sz * 8;

    // Update the total encoded bits.
    bits_total_ += frame_size_in_bits;

    // Update the most recent pts.
    last_pts_ = pkt->data.frame.pts;
    ++frame_number_;
    ++tot_frame_number_;
    const int per_frame_bandwidth = (cfg_.rc_target_bitrate * 1000) / 30;
    if (frame_size_in_bits > max_perc_spike_ * per_frame_bandwidth &&
        frame_number_ > 1)
      num_spikes_++;
    if (frame_size_in_bits > max_perc_spike_high_ * per_frame_bandwidth &&
        frame_number_ > 1)
      num_spikes_high_++;

    if (frame_update_bitrate_ > 0) {
      if (frame_number_ < frame_update_bitrate_) {
        bits_total_dynamic_[0] += frame_size_in_bits;
        frame_number_dynamic_[0]++;
      } else if (frame_number_ >= frame_update_bitrate_ &&
                 frame_number_ < 2 * frame_update_bitrate_) {
        bits_total_dynamic_[1] += frame_size_in_bits;
        frame_number_dynamic_[1]++;
      } else {
        bits_total_dynamic_[2] += frame_size_in_bits;
        frame_number_dynamic_[2]++;
      }
    }
  }

  void EndPassHook() override {
    duration_ = (last_pts_ + 1) * timebase_;
    // Effective file datarate:
    effective_datarate_ = (bits_total_ / 1000.0) / duration_;
    if (frame_update_bitrate_ > 0) {
      for (int i = 0; i < 3; i++)
        effective_datarate_dynamic_[i] =
            30 * (bits_total_dynamic_[i] / 1000.0) / frame_number_dynamic_[i];
    }
  }

  aom_codec_pts_t last_pts_;
  double timebase_;
  int frame_number_;      // Counter for number of non-dropped/encoded frames.
  int tot_frame_number_;  // Counter for total number of input frames.
  int64_t bits_total_;
  double duration_;
  double effective_datarate_;
  int set_cpu_used_;
  int64_t bits_in_buffer_model_;
  aom_codec_pts_t first_drop_;
  int num_drops_;
  int denoiser_on_;
  int denoiser_offon_test_;
  int denoiser_offon_period_;
  unsigned int aq_mode_;
  bool speed_change_test_;
  int tile_columns_;
  int tile_rows_;
  bool auto_tiles_;
  bool screen_mode_;
  double max_perc_spike_;
  double max_perc_spike_high_;
  int num_spikes_;
  int num_spikes_high_;
  // These are use for test with dynamic bitrate change.
  // Used to verify that the encoder can respond and hit bitrate that is updated
  // during the sequence.
  int frame_update_bitrate_;
  int target_bitrate_update_[3];
  double effective_datarate_dynamic_[3];
  int64_t bits_total_dynamic_[3];
  int frame_number_dynamic_[3];
  int avif_mode_;
};

}  // namespace
}  // namespace datarate_test

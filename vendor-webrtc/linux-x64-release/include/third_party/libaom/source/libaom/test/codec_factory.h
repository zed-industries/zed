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
#ifndef AOM_TEST_CODEC_FACTORY_H_
#define AOM_TEST_CODEC_FACTORY_H_

#include <tuple>

#include "config/aom_config.h"

#include "aom/aom_decoder.h"
#include "aom/aom_encoder.h"
#if CONFIG_AV1_ENCODER
#include "aom/aomcx.h"
#endif
#if CONFIG_AV1_DECODER
#include "aom/aomdx.h"
#endif

#include "test/decode_test_driver.h"
#include "test/encode_test_driver.h"
namespace libaom_test {

const int kCodecFactoryParam = 0;

class CodecFactory {
 public:
  CodecFactory() = default;

  virtual ~CodecFactory() = default;

  virtual Decoder *CreateDecoder(aom_codec_dec_cfg_t cfg) const = 0;

  virtual Decoder *CreateDecoder(aom_codec_dec_cfg_t cfg,
                                 const aom_codec_flags_t flags) const = 0;

  virtual Encoder *CreateEncoder(aom_codec_enc_cfg_t cfg,
                                 const aom_codec_flags_t init_flags,
                                 TwopassStatsStore *stats) const = 0;

  virtual aom_codec_err_t DefaultEncoderConfig(aom_codec_enc_cfg_t *cfg,
                                               unsigned int usage) const = 0;
};

/* Provide CodecTestWith<n>Params classes for a variable number of parameters
 * to avoid having to include a pointer to the CodecFactory in every test
 * definition.
 */
template <class T1>
class CodecTestWithParam
    : public ::testing::TestWithParam<
          std::tuple<const libaom_test::CodecFactory *, T1> > {};

template <class T1, class T2>
class CodecTestWith2Params
    : public ::testing::TestWithParam<
          std::tuple<const libaom_test::CodecFactory *, T1, T2> > {};

template <class T1, class T2, class T3>
class CodecTestWith3Params
    : public ::testing::TestWithParam<
          std::tuple<const libaom_test::CodecFactory *, T1, T2, T3> > {};

template <class T1, class T2, class T3, class T4>
class CodecTestWith4Params
    : public ::testing::TestWithParam<
          std::tuple<const libaom_test::CodecFactory *, T1, T2, T3, T4> > {};

template <class T1, class T2, class T3, class T4, class T5>
class CodecTestWith5Params
    : public ::testing::TestWithParam<
          std::tuple<const libaom_test::CodecFactory *, T1, T2, T3, T4, T5> > {
};

template <class T1, class T2, class T3, class T4, class T5, class T6>
class CodecTestWith6Params
    : public ::testing::TestWithParam<std::tuple<
          const libaom_test::CodecFactory *, T1, T2, T3, T4, T5, T6> > {};

/*
 * AV1 Codec Definitions
 */
class AV1Decoder : public Decoder {
 public:
  explicit AV1Decoder(aom_codec_dec_cfg_t cfg) : Decoder(cfg) {}

  AV1Decoder(aom_codec_dec_cfg_t cfg, const aom_codec_flags_t flag)
      : Decoder(cfg, flag) {}

 protected:
  aom_codec_iface_t *CodecInterface() const override {
#if CONFIG_AV1_DECODER
    return aom_codec_av1_dx();
#else
    return nullptr;
#endif
  }
};

class AV1Encoder : public Encoder {
 public:
  AV1Encoder(aom_codec_enc_cfg_t cfg, const aom_codec_flags_t init_flags,
             TwopassStatsStore *stats)
      : Encoder(cfg, init_flags, stats) {}

 protected:
  aom_codec_iface_t *CodecInterface() const override {
#if CONFIG_AV1_ENCODER
    return aom_codec_av1_cx();
#else
    return nullptr;
#endif
  }
};

class AV1CodecFactory : public CodecFactory {
 public:
  AV1CodecFactory() : CodecFactory() {}

  Decoder *CreateDecoder(aom_codec_dec_cfg_t cfg) const override {
    return CreateDecoder(cfg, 0);
  }

  Decoder *CreateDecoder(aom_codec_dec_cfg_t cfg,
                         const aom_codec_flags_t flags) const override {
#if CONFIG_AV1_DECODER
    return new AV1Decoder(cfg, flags);
#else
    (void)cfg;
    (void)flags;
    return nullptr;
#endif
  }

  Encoder *CreateEncoder(aom_codec_enc_cfg_t cfg,
                         const aom_codec_flags_t init_flags,
                         TwopassStatsStore *stats) const override {
#if CONFIG_AV1_ENCODER
    return new AV1Encoder(cfg, init_flags, stats);
#else
    (void)cfg;
    (void)init_flags;
    (void)stats;
    return nullptr;
#endif
  }

  aom_codec_err_t DefaultEncoderConfig(aom_codec_enc_cfg_t *cfg,
                                       unsigned int usage) const override {
#if CONFIG_AV1_ENCODER
    return aom_codec_enc_config_default(aom_codec_av1_cx(), cfg, usage);
#else
    (void)cfg;
    (void)usage;
    return AOM_CODEC_INCAPABLE;
#endif
  }
};

const libaom_test::AV1CodecFactory kAV1;

#define AV1_INSTANTIATE_TEST_SUITE(test, ...)                               \
  INSTANTIATE_TEST_SUITE_P(                                                 \
      AV1, test,                                                            \
      ::testing::Combine(                                                   \
          ::testing::Values(static_cast<const libaom_test::CodecFactory *>( \
              &libaom_test::kAV1)),                                         \
          __VA_ARGS__))

}  // namespace libaom_test
#endif  // AOM_TEST_CODEC_FACTORY_H_

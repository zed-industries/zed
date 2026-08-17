/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_CODECS_AUDIO_ENCODER_FACTORY_TEMPLATE_H_
#define API_AUDIO_CODECS_AUDIO_ENCODER_FACTORY_TEMPLATE_H_

#include <memory>
#include <optional>
#include <type_traits>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "api/audio_codecs/audio_codec_pair_id.h"
#include "api/audio_codecs/audio_encoder.h"
#include "api/audio_codecs/audio_encoder_factory.h"
#include "api/audio_codecs/audio_format.h"
#include "api/environment/environment.h"
#include "api/make_ref_counted.h"
#include "api/scoped_refptr.h"

namespace webrtc {

namespace audio_encoder_factory_template_impl {

template <typename... Ts>
struct Helper;

// Base case: 0 template parameters.
template <>
struct Helper<> {
  static void AppendSupportedEncoders(
      std::vector<AudioCodecSpec>* /* specs */) {}
  static std::optional<AudioCodecInfo> QueryAudioEncoder(
      const SdpAudioFormat& /* format */) {
    return std::nullopt;
  }
  static absl_nullable std::unique_ptr<AudioEncoder> CreateAudioEncoder(
      const Environment& /* env */,
      const SdpAudioFormat& /* format */,
      const AudioEncoderFactory::Options& /* options */) {
    return nullptr;
  }
};

// Use ranked overloads (abseil.io/tips/229) for dispatching.
struct Rank0 {};
struct Rank1 : Rank0 {};

template <typename Trait,
          typename = std::enable_if_t<std::is_convertible_v<
              decltype(Trait::MakeAudioEncoder(
                  std::declval<Environment>(),
                  std::declval<typename Trait::Config>(),
                  std::declval<AudioEncoderFactory::Options>())),
              std::unique_ptr<AudioEncoder>>>>
absl_nullable std::unique_ptr<AudioEncoder> CreateEncoder(
    Rank1,
    const Environment& env,
    const typename Trait::Config& config,
    const AudioEncoderFactory::Options& options) {
  return Trait::MakeAudioEncoder(env, config, options);
}

template <typename Trait,
          typename = std::enable_if_t<std::is_convertible_v<
              decltype(Trait::MakeAudioEncoder(
                  std::declval<typename Trait::Config>(),
                  int{},
                  std::declval<std::optional<AudioCodecPairId>>())),
              std::unique_ptr<AudioEncoder>>>>
absl_nullable std::unique_ptr<AudioEncoder> CreateEncoder(
    Rank0,
    const Environment& /* env */,
    const typename Trait::Config& config,
    const AudioEncoderFactory::Options& options) {
  return Trait::MakeAudioEncoder(config, options.payload_type,
                                 options.codec_pair_id);
}

// Inductive case: Called with n + 1 template parameters; calls subroutines
// with n template parameters.
template <typename T, typename... Ts>
struct Helper<T, Ts...> {
  static void AppendSupportedEncoders(std::vector<AudioCodecSpec>* specs) {
    T::AppendSupportedEncoders(specs);
    Helper<Ts...>::AppendSupportedEncoders(specs);
  }
  static std::optional<AudioCodecInfo> QueryAudioEncoder(
      const SdpAudioFormat& format) {
    auto opt_config = T::SdpToConfig(format);
    static_assert(std::is_same<decltype(opt_config),
                               std::optional<typename T::Config>>::value,
                  "T::SdpToConfig() must return a value of type "
                  "std::optional<T::Config>");
    return opt_config ? std::optional<AudioCodecInfo>(
                            T::QueryAudioEncoder(*opt_config))
                      : Helper<Ts...>::QueryAudioEncoder(format);
  }

  static absl_nullable std::unique_ptr<AudioEncoder> CreateAudioEncoder(
      const Environment& env,
      const SdpAudioFormat& format,
      const AudioEncoderFactory::Options& options) {
    if (auto opt_config = T::SdpToConfig(format); opt_config.has_value()) {
      return CreateEncoder<T>(Rank1{}, env, *opt_config, options);
    }
    return Helper<Ts...>::CreateAudioEncoder(env, format, options);
  }
};

template <typename... Ts>
class AudioEncoderFactoryT : public AudioEncoderFactory {
 public:
  std::vector<AudioCodecSpec> GetSupportedEncoders() override {
    std::vector<AudioCodecSpec> specs;
    Helper<Ts...>::AppendSupportedEncoders(&specs);
    return specs;
  }

  std::optional<AudioCodecInfo> QueryAudioEncoder(
      const SdpAudioFormat& format) override {
    return Helper<Ts...>::QueryAudioEncoder(format);
  }

  absl_nullable std::unique_ptr<AudioEncoder> Create(
      const Environment& env,
      const SdpAudioFormat& format,
      Options options) override {
    return Helper<Ts...>::CreateAudioEncoder(env, format, options);
  }
};

}  // namespace audio_encoder_factory_template_impl

// Make an AudioEncoderFactory that can create instances of the given encoders.
//
// Each encoder type is given as a template argument to the function; it should
// be a struct with the following static member functions:
//
//   // Converts `audio_format` to a ConfigType instance. Returns an empty
//   // optional if `audio_format` doesn't correctly specify an encoder of our
//   // type.
//   std::optional<ConfigType> SdpToConfig(const SdpAudioFormat& audio_format);
//
//   // Appends zero or more AudioCodecSpecs to the list that will be returned
//   // by AudioEncoderFactory::GetSupportedEncoders().
//   void AppendSupportedEncoders(std::vector<AudioCodecSpec>* specs);
//
//   // Returns information about how this format would be encoded. Used to
//   // implement AudioEncoderFactory::QueryAudioEncoder().
//   AudioCodecInfo QueryAudioEncoder(const ConfigType& config);
//
//   // Creates an AudioEncoder for the specified format. Used to implement
//   // AudioEncoderFactory::Create.
//   std::unique_ptr<AudioEncoder> MakeAudioEncoder(
//       const Environment& env,
//       const ConfigType& config,
//       const AudioEncoderFactory::Options& options);
//   or
//   std::unique_ptr<AudioEncoder> MakeAudioEncoder(
//       const ConfigType& config,
//       int payload_type,
//       std::optional<AudioCodecPairId> codec_pair_id);
//
// ConfigType should be a type that encapsulates all the settings needed to
// create an AudioEncoder. T::Config (where T is the encoder struct) should
// either be the config type, or an alias for it.
// When both MakeAudioEncoder signatures are present, 1st one is preferred.
//
// Whenever it tries to do something, the new factory will try each of the
// encoders in the order they were specified in the template argument list,
// stopping at the first one that claims to be able to do the job.
//
// TODO(kwiberg): Point at CreateBuiltinAudioEncoderFactory() for an example of
// how it is used.
template <typename... Ts>
scoped_refptr<AudioEncoderFactory> CreateAudioEncoderFactory() {
  // There's no technical reason we couldn't allow zero template parameters,
  // but such a factory couldn't create any encoders, and callers can do this
  // by mistake by simply forgetting the <> altogether. So we forbid it in
  // order to prevent caller foot-shooting.
  static_assert(sizeof...(Ts) >= 1,
                "Caller must give at least one template parameter");

  return make_ref_counted<
      audio_encoder_factory_template_impl::AudioEncoderFactoryT<Ts...>>();
}

}  // namespace webrtc

#endif  // API_AUDIO_CODECS_AUDIO_ENCODER_FACTORY_TEMPLATE_H_

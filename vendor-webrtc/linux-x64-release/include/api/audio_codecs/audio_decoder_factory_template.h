/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_CODECS_AUDIO_DECODER_FACTORY_TEMPLATE_H_
#define API_AUDIO_CODECS_AUDIO_DECODER_FACTORY_TEMPLATE_H_

#include <memory>
#include <optional>
#include <type_traits>
#include <utility>
#include <vector>

#include "absl/base/nullability.h"
#include "api/audio_codecs/audio_codec_pair_id.h"
#include "api/audio_codecs/audio_decoder.h"
#include "api/audio_codecs/audio_decoder_factory.h"
#include "api/audio_codecs/audio_format.h"
#include "api/environment/environment.h"
#include "api/make_ref_counted.h"
#include "api/scoped_refptr.h"

namespace webrtc {

namespace audio_decoder_factory_template_impl {

template <typename... Ts>
struct Helper;

// Base case: 0 template parameters.
template <>
struct Helper<> {
  static void AppendSupportedDecoders(
      std::vector<AudioCodecSpec>* /* specs */) {}
  static bool IsSupportedDecoder(const SdpAudioFormat& /* format */) {
    return false;
  }

  static absl_nullable std::unique_ptr<AudioDecoder> MakeAudioDecoder(
      const Environment& /* env */,
      const SdpAudioFormat& /* format */,
      std::optional<AudioCodecPairId> /* codec_pair_id */) {
    return nullptr;
  }
};

// Use ranked overloads (abseil.io/tips/229) for dispatching.
struct Rank0 {};
struct Rank1 : Rank0 {};

template <typename Trait,
          typename = std::enable_if_t<std::is_convertible_v<
              decltype(Trait::MakeAudioDecoder(
                  std::declval<Environment>(),
                  std::declval<typename Trait::Config>(),
                  std::declval<std::optional<AudioCodecPairId>>())),
              std::unique_ptr<AudioDecoder>>>>
absl_nullable std::unique_ptr<AudioDecoder> CreateDecoder(
    Rank1,
    const Environment& env,
    const typename Trait::Config& config,
    std::optional<AudioCodecPairId> codec_pair_id) {
  return Trait::MakeAudioDecoder(env, config, codec_pair_id);
}

template <typename Trait,
          typename = std::enable_if_t<std::is_convertible_v<
              decltype(Trait::MakeAudioDecoder(
                  std::declval<typename Trait::Config>(),
                  std::declval<std::optional<AudioCodecPairId>>())),
              std::unique_ptr<AudioDecoder>>>>
absl_nullable std::unique_ptr<AudioDecoder> CreateDecoder(
    Rank0,
    const Environment& /* env */,
    const typename Trait::Config& config,
    std::optional<AudioCodecPairId> codec_pair_id) {
  return Trait::MakeAudioDecoder(config, codec_pair_id);
}

// Inductive case: Called with n + 1 template parameters; calls subroutines
// with n template parameters.
template <typename T, typename... Ts>
struct Helper<T, Ts...> {
  static void AppendSupportedDecoders(std::vector<AudioCodecSpec>* specs) {
    T::AppendSupportedDecoders(specs);
    Helper<Ts...>::AppendSupportedDecoders(specs);
  }
  static bool IsSupportedDecoder(const SdpAudioFormat& format) {
    auto opt_config = T::SdpToConfig(format);
    static_assert(std::is_same<decltype(opt_config),
                               std::optional<typename T::Config>>::value,
                  "T::SdpToConfig() must return a value of type "
                  "std::optional<T::Config>");
    return opt_config ? true : Helper<Ts...>::IsSupportedDecoder(format);
  }

  static absl_nullable std::unique_ptr<AudioDecoder> MakeAudioDecoder(
      const Environment& env,
      const SdpAudioFormat& format,
      std::optional<AudioCodecPairId> codec_pair_id) {
    auto opt_config = T::SdpToConfig(format);
    return opt_config.has_value()
               ? CreateDecoder<T>(Rank1{}, env, *opt_config, codec_pair_id)
               : Helper<Ts...>::MakeAudioDecoder(env, format, codec_pair_id);
  }
};

template <typename... Ts>
class AudioDecoderFactoryT : public AudioDecoderFactory {
 public:
  std::vector<AudioCodecSpec> GetSupportedDecoders() override {
    std::vector<AudioCodecSpec> specs;
    Helper<Ts...>::AppendSupportedDecoders(&specs);
    return specs;
  }

  bool IsSupportedDecoder(const SdpAudioFormat& format) override {
    return Helper<Ts...>::IsSupportedDecoder(format);
  }

  absl_nullable std::unique_ptr<AudioDecoder> Create(
      const Environment& env,
      const SdpAudioFormat& format,
      std::optional<AudioCodecPairId> codec_pair_id) override {
    return Helper<Ts...>::MakeAudioDecoder(env, format, codec_pair_id);
  }
};

}  // namespace audio_decoder_factory_template_impl

// Make an AudioDecoderFactory that can create instances of the given decoders.
//
// Each decoder type is given as a template argument to the function; it should
// be a struct with the following static member functions:
//
//   // Converts `audio_format` to a ConfigType instance. Returns an empty
//   // optional if `audio_format` doesn't correctly specify a decoder of our
//   // type.
//   std::optional<ConfigType> SdpToConfig(const SdpAudioFormat& audio_format);
//
//   // Appends zero or more AudioCodecSpecs to the list that will be returned
//   // by AudioDecoderFactory::GetSupportedDecoders().
//   void AppendSupportedDecoders(std::vector<AudioCodecSpec>* specs);
//
//   // Creates an AudioDecoder for the specified format. Used to implement
//   // AudioDecoderFactory::Create().
//   std::unique_ptr<AudioEncoder> MakeAudioDecoder(
//       const Environment& env,
//       const ConfigType& config,
//       std::optional<AudioCodecPairId> codec_pair_id);
//   or
//   std::unique_ptr<AudioDecoder> MakeAudioDecoder(
//       const ConfigType& config,
//       std::optional<AudioCodecPairId> codec_pair_id);
//
// ConfigType should be a type that encapsulates all the settings needed to
// create an AudioDecoder. T::Config (where T is the decoder struct) should
// either be the config type, or an alias for it.
//
// Whenever it tries to do something, the new factory will try each of the
// decoder types in the order they were specified in the template argument
// list, stopping at the first one that claims to be able to do the job.
//
// TODO(kwiberg): Point at CreateBuiltinAudioDecoderFactory() for an example of
// how it is used.
template <typename... Ts>
scoped_refptr<AudioDecoderFactory> CreateAudioDecoderFactory() {
  // There's no technical reason we couldn't allow zero template parameters,
  // but such a factory couldn't create any decoders, and callers can do this
  // by mistake by simply forgetting the <> altogether. So we forbid it in
  // order to prevent caller foot-shooting.
  static_assert(sizeof...(Ts) >= 1,
                "Caller must give at least one template parameter");

  return make_ref_counted<
      audio_decoder_factory_template_impl::AudioDecoderFactoryT<Ts...>>();
}

}  // namespace webrtc

#endif  // API_AUDIO_CODECS_AUDIO_DECODER_FACTORY_TEMPLATE_H_

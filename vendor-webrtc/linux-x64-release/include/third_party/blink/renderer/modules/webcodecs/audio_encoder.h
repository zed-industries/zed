// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_AUDIO_ENCODER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_AUDIO_ENCODER_H_

#include "media/base/audio_codecs.h"
#include "media/base/audio_encoder.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_codec_state.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_encoded_audio_chunk_output_callback.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_webcodecs_error_callback.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webcodecs/audio_data.h"
#include "third_party/blink/renderer/modules/webcodecs/encoder_base.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ExceptionState;
class AudioEncoderConfig;
class AudioEncoderInit;
class AudioEncoderSupport;

class MODULES_EXPORT AudioEncoderTraits {
 public:
  struct ParsedConfig final : public GarbageCollected<ParsedConfig> {
    media::AudioEncoder::Options options;
    String codec_string;

    void Trace(Visitor*) const {}
  };

  struct AudioEncoderEncodeOptions
      : public GarbageCollected<AudioEncoderEncodeOptions> {
    void Trace(Visitor*) const {}
  };

  using Init = AudioEncoderInit;
  using Config = AudioEncoderConfig;
  using InternalConfig = ParsedConfig;
  using Input = AudioData;
  using EncodeOptions = AudioEncoderEncodeOptions;
  using OutputChunk = EncodedAudioChunk;
  using OutputCallback = V8EncodedAudioChunkOutputCallback;
  using MediaEncoder = media::AudioEncoder;

  // Can't be a virtual method, because it's used from base ctor.
  static const char* GetName();
};

class MODULES_EXPORT AudioEncoder final
    : public EncoderBase<AudioEncoderTraits> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static AudioEncoder* Create(ScriptState*,
                              const AudioEncoderInit*,
                              ExceptionState&);
  AudioEncoder(ScriptState*, const AudioEncoderInit*, ExceptionState&);
  ~AudioEncoder() override;

  void encode(AudioData* data, ExceptionState& exception_state) {
    return Base::encode(data, nullptr, exception_state);
  }

  // EventTarget interface
  const AtomicString& InterfaceName() const override;

  static ScriptPromise<AudioEncoderSupport>
  isConfigSupported(ScriptState*, const AudioEncoderConfig*, ExceptionState&);

 private:
  using Base = EncoderBase<AudioEncoderTraits>;
  using ParsedConfig = AudioEncoderTraits::ParsedConfig;

  void ProcessEncode(Request* request) override;
  void ProcessConfigure(Request* request) override;
  void ProcessReconfigure(Request* request) override;

  ParsedConfig* OnNewConfigure(const AudioEncoderConfig* opts,
                               ExceptionState&) override;
  bool VerifyCodecSupport(ParsedConfig*, String* js_error_message) override;
  void OnNewEncode(InputType* input, ExceptionState& exception_state) override;
  bool CanReconfigure(ParsedConfig& original_config,
                      ParsedConfig& new_config) override;

  std::unique_ptr<media::AudioEncoder> CreateMediaAudioEncoder(
      const ParsedConfig& config);
  void CallOutputCallback(
      ParsedConfig* active_config,
      uint32_t reset_count,
      media::EncodedAudioBuffer encoded_buffer,
      std::optional<media::AudioEncoder::CodecDescription> codec_desc);
  DOMException* MakeOperationError(std::string error_msg,
                                   media::EncoderStatus status);
  DOMException* MakeEncodingError(std::string error_msg,
                                  media::EncoderStatus status);

  // True if MojoAudioEncoder is being used.
  bool is_platform_encoder_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_AUDIO_ENCODER_H_

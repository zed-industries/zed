// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_AUDIO_DECODER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_AUDIO_DECODER_H_

#include <stdint.h>
#include <memory>

#include "media/base/audio_decoder.h"
#include "media/base/media_types.h"
#include "media/base/status.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_webcodecs_error_callback.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webcodecs/decoder_template.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace media {

class AudioBuffer;
class DecoderBuffer;
class MediaLog;

}  // namespace media

namespace blink {

class AudioData;
class AudioDecoderConfig;
class AudioDecoderSupport;
class EncodedAudioChunk;
class ExceptionState;
class AudioDecoderInit;
class V8AudioDataOutputCallback;

class MODULES_EXPORT AudioDecoderTraits {
 public:
  using InitType = AudioDecoderInit;
  using OutputType = AudioData;
  using MediaOutputType = media::AudioBuffer;
  using MediaDecoderType = media::AudioDecoder;
  using OutputCallbackType = V8AudioDataOutputCallback;
  using ConfigType = AudioDecoderConfig;
  using MediaConfigType = media::AudioDecoderConfig;
  using InputType = EncodedAudioChunk;

  static constexpr bool kNeedsGpuFactories = false;

  static std::unique_ptr<MediaDecoderType> CreateDecoder(
      ExecutionContext& execution_context,
      media::GpuVideoAcceleratorFactories* gpu_factories,
      media::MediaLog* media_log);
  static void InitializeDecoder(MediaDecoderType& decoder,
                                bool low_delay,
                                const MediaConfigType& media_config,
                                MediaDecoderType::InitCB init_cb,
                                MediaDecoderType::OutputCB output_cb);
  static int GetMaxDecodeRequests(const MediaDecoderType& decoder);
  static void UpdateDecoderLog(const MediaDecoderType& decoder,
                               const MediaConfigType& media_config,
                               media::MediaLog* media_log);
  static const char* GetName();
};

class MODULES_EXPORT AudioDecoder : public DecoderTemplate<AudioDecoderTraits> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static AudioDecoder* Create(ScriptState*,
                              const AudioDecoderInit*,
                              ExceptionState&);

  static ScriptPromise<AudioDecoderSupport>
  isConfigSupported(ScriptState*, const AudioDecoderConfig*, ExceptionState&);

  // Returns parsed AudioType if the configuration is valid.
  static std::optional<media::AudioType> IsValidAudioDecoderConfig(
      const AudioDecoderConfig& config,
      String* js_error_message);

  // For use by MediaSource and by ::MakeMediaConfig.
  static std::optional<media::AudioDecoderConfig> MakeMediaAudioDecoderConfig(
      const ConfigType& config,
      String* js_error_message);

  AudioDecoder(ScriptState*, const AudioDecoderInit*, ExceptionState&);
  ~AudioDecoder() override = default;

  // EventTarget interface
  const AtomicString& InterfaceName() const override;

 protected:
  bool IsValidConfig(const ConfigType& config,
                     String* js_error_message) override;
  std::optional<media::AudioDecoderConfig> MakeMediaConfig(
      const ConfigType& config,
      String* js_error_message) override;
  media::DecoderStatus::Or<scoped_refptr<media::DecoderBuffer>> MakeInput(
      const InputType& chunk,
      bool verify_key_frame) override;
  media::DecoderStatus::Or<OutputType*> MakeOutput(
      scoped_refptr<MediaOutputType> output,
      ExecutionContext* context) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_AUDIO_DECODER_H_

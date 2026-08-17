// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_FUZZER_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_FUZZER_UTILS_H_

#include "third_party/blink/renderer/bindings/core/v8/script_function.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_audio_decoder_config.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_audio_encoder_config.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_video_decoder_config.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_video_decoder_init.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_video_encoder_config.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_video_encoder_encode_options.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"
#include "third_party/blink/renderer/modules/webcodecs/array_buffer_util.h"
#include "third_party/blink/renderer/modules/webcodecs/audio_data.h"
#include "third_party/blink/renderer/modules/webcodecs/encoded_audio_chunk.h"
#include "third_party/blink/renderer/modules/webcodecs/encoded_video_chunk.h"
#include "third_party/blink/renderer/modules/webcodecs/fuzzer_inputs.pb.h"
#include "third_party/blink/renderer/modules/webcodecs/video_frame.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8-forward.h"

#include <string>

namespace base {
class ScopedClosureRunner;
}

namespace blink {

class DOMRectInit;
class PlaneLayout;

base::ScopedClosureRunner MakeScopedGarbageCollectionRequest(v8::Isolate*);

class FakeFunction : public ScriptFunction {
 public:
  explicit FakeFunction(std::string name);

  ScriptValue Call(ScriptState*, ScriptValue) override;

 private:
  const std::string name_;
};

VideoDecoderConfig* MakeVideoDecoderConfig(
    const wc_fuzzer::ConfigureVideoDecoder& proto);

AudioDecoderConfig* MakeAudioDecoderConfig(
    const wc_fuzzer::ConfigureAudioDecoder& proto);

VideoEncoderConfig* MakeVideoEncoderConfig(
    const wc_fuzzer::ConfigureVideoEncoder& proto);

AudioEncoderConfig* MakeAudioEncoderConfig(
    const wc_fuzzer::ConfigureAudioEncoder& proto);

EncodedVideoChunk* MakeEncodedVideoChunk(
    ScriptState* script_state,
    const wc_fuzzer::EncodedVideoChunk& proto);

EncodedAudioChunk* MakeEncodedAudioChunk(
    ScriptState* script_state,
    const wc_fuzzer::EncodedAudioChunk& proto);

struct BufferAndSource {
  UntracedMember<DOMArrayBuffer> buffer;
  UntracedMember<AllowSharedBufferSource> source;
};

BufferAndSource MakeAllowSharedBufferSource(
    const wc_fuzzer::AllowSharedBufferSource& proto);

PlaneLayout* MakePlaneLayout(const wc_fuzzer::PlaneLayout& proto);

DOMRectInit* MakeDOMRectInit(const wc_fuzzer::DOMRectInit& proto);

VideoColorSpaceInit* MakeVideoColorSpaceInit(
    const wc_fuzzer::VideoColorSpaceInit& proto);

VideoFrame* MakeVideoFrame(
    ScriptState* script_state,
    const wc_fuzzer::VideoFrameBufferInitInvocation& proto);

VideoFrame* MakeVideoFrame(ScriptState* script_state,
                           const wc_fuzzer::VideoFrameBitmapInit& proto);

AudioData* MakeAudioData(ScriptState* script_state,
                         const wc_fuzzer::AudioDataInit& proto);

AudioDataCopyToOptions* MakeAudioDataCopyToOptions(
    const wc_fuzzer::AudioDataCopyToOptions& proto);

VideoEncoderEncodeOptions* MakeEncodeOptions(
    const wc_fuzzer::EncodeVideo_EncodeOptions& proto);

String ToBitrateMode(
    wc_fuzzer::ConfigureVideoEncoder_VideoEncoderBitrateMode mode);

String ToScalabilityMode(wc_fuzzer::ConfigureVideoEncoder_ScalabilityMode mode);

String ToLatencyMode(wc_fuzzer::ConfigureVideoEncoder_LatencyMode mode);

String ToContentHint(wc_fuzzer::ConfigureVideoEncoder_ContentHint hint);

String ToAlphaOption(wc_fuzzer::ConfigureVideoEncoder_AlphaOption option);

String ToAacFormat(wc_fuzzer::AacFormat format);

String ToBitrateMode(wc_fuzzer::BitrateMode bitrate_mode);

String ToOpusSignal(wc_fuzzer::OpusSignal opus_signal);

String ToOpusApplication(wc_fuzzer::OpusApplication opus_application);

String ToAccelerationType(
    wc_fuzzer::ConfigureVideoEncoder_EncoderAccelerationPreference type);

String ToChunkType(wc_fuzzer::EncodedChunkType type);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_FUZZER_UTILS_H_

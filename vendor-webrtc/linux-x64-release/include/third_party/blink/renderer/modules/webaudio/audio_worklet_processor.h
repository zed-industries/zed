// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_PROCESSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_PROCESSOR_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webaudio/audio_worklet_processor_error_state.h"
#include "third_party/blink/renderer/platform/audio/audio_array.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

class AudioBus;
class AudioWorkletGlobalScope;
class AudioWorkletProcessorDefinition;
class MessagePort;
class ExecutionContext;
class V8BlinkAudioWorkletProcessCallback;

// AudioWorkletProcessor class represents the active instance created from
// AudioWorkletProcessorDefinition. AudioWorkletNodeHandler invokes `.process()`
// method in this object upon graph rendering.
//
// This is constructed and destroyed on a worker thread, and all methods also
// must be called on the worker thread.
class MODULES_EXPORT AudioWorkletProcessor : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // This static factory should be called after an instance of
  // AudioWorkletNode gets created by user-supplied JS code in the main
  // thread. This factory must not be called by user in
  // AudioWorkletGlobalScope.
  static AudioWorkletProcessor* Create(ExecutionContext*, ExceptionState&);

  AudioWorkletProcessor(AudioWorkletGlobalScope*,
                        const String& name,
                        MessagePort*);
  ~AudioWorkletProcessor() override;

  // `AudioWorkletHandler` invokes this method to process audio.
  bool Process(
      const Vector<scoped_refptr<AudioBus>>& inputs,
      Vector<scoped_refptr<AudioBus>>& outputs,
      const HashMap<String, std::unique_ptr<AudioFloatArray>>& param_value_map);

  const String& Name() const { return name_; }

  void SetErrorState(AudioWorkletProcessorErrorState);
  AudioWorkletProcessorErrorState GetErrorState() const;
  bool hasErrorOccurred() const;

  // IDL
  MessagePort* port() const;

  void Trace(Visitor*) const override;

 private:
  using BackingArrayBuffers =
    HeapVector<HeapVector<TraceWrapperV8Reference<v8::ArrayBuffer>>>;

  // An AudioPort is an array of one or more AudioBus objects, which is
  // represented by:
  // Vector<scoped_refptr<AudioBus>> or TraceWrapperV8Reference<v8::Array>.
  // An AudioBus can contain one or more AudioChannels, that are represented
  // with a Float32Array (V8) or an AudioFloatArray (Web Audio).

  // Returns true if the topology of two AudioPorts match. The first AudioPort
  // is given from AudioWorkletHandler (Blink) and the second AudioPort is from
  // AudioWorkletProcessor (V8).
  static bool PortTopologyMatches(
      v8::Isolate*, v8::Local<v8::Context>,
      const Vector<scoped_refptr<AudioBus>>& audio_port_1,
      const TraceWrapperV8Reference<v8::Array>& audio_port_2);

  // Freezes an AudioPort. After this operation the AudioPort will be locked and
  // cannot be altered. Returns false only if any V8 operation throws an
  // exception.
  static bool FreezeAudioPort(v8::Isolate*, v8::Local<v8::Context>,
                              v8::Local<v8::Array>& audio_port_array);

  // Clones the topology of `audio_port_1` and builds a new AudioPort to
  // `audio_port_2`. This call makes memory allocation and it should be avoided
  // in the hot audio stack as much as possible. If `array_buffers` is a valid
  // pointer, fill in `array_buffers` with new backing ArrayBuffers of
  // `audio_port_2`. Returns false only if any v8 operation throws an
  // exception.
  static bool ClonePortTopology(
      v8::Isolate*, v8::Local<v8::Context>,
      const Vector<scoped_refptr<AudioBus>>& audio_port_1,
      TraceWrapperV8Reference<v8::Array>& audio_port_2,
      BackingArrayBuffers& array_buffers);

  // Copies an AudioPort to a BackingArrayBuffers. The size of two args must
  // be identical.
  static void CopyPortToArrayBuffers(
      v8::Isolate*,
      const Vector<scoped_refptr<AudioBus>>& audio_port,
      BackingArrayBuffers& array_buffers);

  // Copies a BackingArrayBuffers to an AudioPort. The size of two args must
  // be identical.
  static void CopyArrayBuffersToPort(
      v8::Isolate*,
      const BackingArrayBuffers& array_buffers,
      Vector<scoped_refptr<AudioBus>>& audio_port);

  // Fills a given BackingArrayBuffers with zeros.
  static void ZeroArrayBuffers(v8::Isolate*,
                               const BackingArrayBuffers& array_buffers);

  // Returns true if the structure of `param_value_map` matches `params` object
  // and the underlying ArrayBuffers are not transferred.
  static bool ParamValueMapMatchesToParamsObject(
      v8::Isolate*, v8::Local<v8::Context>,
      const HashMap<String, std::unique_ptr<AudioFloatArray>>& param_value_map,
      const TraceWrapperV8Reference<v8::Object>& params);

  // Clones the structure of `param_value_map` to a given v8::Object, which
  // is an associated array of Float32Arrays.
  static bool CloneParamValueMapToObject(
      v8::Isolate*, v8::Local<v8::Context>,
      const HashMap<String, std::unique_ptr<AudioFloatArray>>& param_value_map,
      TraceWrapperV8Reference<v8::Object>& params);

  // Copies the content of float arrays from `param_value_map` to `params`
  // v8::Object.
  static bool CopyParamValueMapToObject(
      v8::Isolate*, v8::Local<v8::Context>,
      const HashMap<String, std::unique_ptr<AudioFloatArray>>& param_value_map,
      TraceWrapperV8Reference<v8::Object>& params);

  Member<AudioWorkletGlobalScope> global_scope_;
  Member<MessagePort> processor_port_;
  Member<V8BlinkAudioWorkletProcessCallback> cached_process_callback_;

  const String name_;

  TraceWrapperV8Reference<v8::Array> inputs_;
  TraceWrapperV8Reference<v8::Array> outputs_;
  TraceWrapperV8Reference<v8::Object> params_;

  BackingArrayBuffers input_array_buffers_;
  BackingArrayBuffers output_array_buffers_;

  AudioWorkletProcessorErrorState error_state_ =
      AudioWorkletProcessorErrorState::kNoError;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_PROCESSOR_H_

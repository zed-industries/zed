// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_PROCESSOR_DEFINITION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_PROCESSOR_DEFINITION_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_audio_param_descriptor.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

class V8BlinkAudioWorkletProcessorConstructor;

// Represents a JavaScript class definition registered in the
// AudioWorkletGlobalScope. After the registration, a definition class contains
// the V8 representation of class components (constructor, process callback,
// prototypes and parameter descriptors).
//
// This is constructed and destroyed on a worker thread, and all methods also
// must be called on the worker thread.
class MODULES_EXPORT AudioWorkletProcessorDefinition final
    : public GarbageCollected<AudioWorkletProcessorDefinition>,
      public NameClient {
 public:
  static AudioWorkletProcessorDefinition* Create(
      const String& name,
      V8BlinkAudioWorkletProcessorConstructor* constructor);

  explicit AudioWorkletProcessorDefinition(
      const String& name,
      V8BlinkAudioWorkletProcessorConstructor* constructor);
  ~AudioWorkletProcessorDefinition() final;

  const String& GetName() const { return name_; }
  V8BlinkAudioWorkletProcessorConstructor* ConstructorFunction() const {
    return constructor_.Get();
  }

  void SetAudioParamDescriptors(
      const HeapVector<Member<AudioParamDescriptor>>&);
  const Vector<String> GetAudioParamDescriptorNames() const;
  const AudioParamDescriptor* GetAudioParamDescriptor(const String& key) const;

  // Flag for data synchronization of definition between
  // AudioWorkletMessagingProxy and AudioWorkletGlobalScope.
  bool IsSynchronized() const { return is_synchronized_; }
  void MarkAsSynchronized() { is_synchronized_ = true; }

  void Trace(Visitor* visitor) const;

  const char* NameInHeapSnapshot() const override {
    return "AudioWorkletProcessorDefinition";
  }

 private:
  const String name_;
  bool is_synchronized_ = false;

  // The definition is per global scope. The active instance of
  // AudioProcessorWorklet should be passed into these to perform JS function.
  Member<V8BlinkAudioWorkletProcessorConstructor> constructor_;

  HeapVector<Member<AudioParamDescriptor>> audio_param_descriptors_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_PROCESSOR_DEFINITION_H_

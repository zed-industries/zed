/*
 * Copyright (C) 2010, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_SCRIPT_PROCESSOR_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_SCRIPT_PROCESSOR_NODE_H_

#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/waitable_event.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/modules/webaudio/script_processor_handler.h"
#include "third_party/blink/renderer/platform/audio/audio_bus.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

class AudioBuffer;
class BaseAudioContext;
class SharedAudioBuffer;

// ScriptProcessorNode is an AudioNode which allows for arbitrary synthesis or
// processing directly using JavaScript.  The API allows for a variable number
// of inputs and outputs, although it must have at least one input or output.
// This basic implementation supports no more than one input and output.  The
// "onaudioprocess" attribute is an event listener which will get called
// periodically with an AudioProcessingEvent which has AudioBuffers for each
// input and output.
class ScriptProcessorNode final
    : public AudioNode,
      public ActiveScriptWrappable<ScriptProcessorNode> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // bufferSize must be one of the following values: 256, 512, 1024, 2048,
  // 4096, 8192, 16384.
  // This value controls how frequently the onaudioprocess event handler is
  // called and how many sample-frames need to be processed each call.
  // Lower numbers for bufferSize will result in a lower (better)
  // latency. Higher numbers will be necessary to avoid audio breakup and
  // glitches.
  // The value chosen must carefully balance between latency and audio quality.
  static ScriptProcessorNode* Create(BaseAudioContext&, ExceptionState&);
  static ScriptProcessorNode* Create(BaseAudioContext&,
                                     uint32_t requested_buffer_size,
                                     ExceptionState&);
  static ScriptProcessorNode* Create(BaseAudioContext&,
                                     uint32_t requested_buffer_size,
                                     uint32_t number_of_input_channels,
                                     ExceptionState&);
  static ScriptProcessorNode* Create(BaseAudioContext&,
                                     uint32_t requested_buffer_size,
                                     uint32_t number_of_input_channels,
                                     uint32_t number_of_output_channels,
                                     ExceptionState&);

  ScriptProcessorNode(BaseAudioContext&,
                      float sample_rate,
                      uint32_t buffer_size,
                      uint32_t number_of_input_channels,
                      uint32_t number_of_output_channels);

  DEFINE_ATTRIBUTE_EVENT_LISTENER(audioprocess, kAudioprocess)
  uint32_t bufferSize() const;

  void DispatchEvent(double playback_time, uint32_t double_buffer_index);

  // ScriptWrappable
  bool HasPendingActivity() const final;

  void Trace(Visitor* visitor) const override;

  // InspectorHelperMixin
  void ReportDidCreate() final;
  void ReportWillBeDestroyed() final;

 private:
  HeapVector<Member<AudioBuffer>> input_buffers_;
  HeapVector<Member<AudioBuffer>> output_buffers_;
  Member<AudioBuffer> external_input_buffer_;
  Member<AudioBuffer> external_output_buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_SCRIPT_PROCESSOR_NODE_H_

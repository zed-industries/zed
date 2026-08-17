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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_BUFFER_SOURCE_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_BUFFER_SOURCE_NODE_H_

#include <atomic>
#include <memory>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/modules/webaudio/audio_buffer.h"
#include "third_party/blink/renderer/modules/webaudio/audio_buffer_source_handler.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param.h"
#include "third_party/blink/renderer/modules/webaudio/audio_scheduled_source_node.h"
#include "third_party/blink/renderer/modules/webaudio/panner_node.h"
#include "third_party/blink/renderer/platform/audio/audio_bus.h"
#include "third_party/blink/renderer/platform/wtf/threading.h"

namespace blink {

class AudioBufferSourceOptions;
class BaseAudioContext;

// AudioBufferSourceNode is an AudioNode representing an audio source from an
// in-memory audio asset represented by an AudioBuffer.  It generally will be
// used for short sounds which require a high degree of scheduling flexibility
// (can playback in rhythmically perfect ways).
class AudioBufferSourceNode final : public AudioScheduledSourceNode {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static AudioBufferSourceNode* Create(BaseAudioContext&, ExceptionState&);
  static AudioBufferSourceNode* Create(BaseAudioContext*,
                                       AudioBufferSourceOptions*,
                                       ExceptionState&);
  explicit AudioBufferSourceNode(BaseAudioContext&);
  void Trace(Visitor*) const override;
  AudioBufferSourceHandler& GetAudioBufferSourceHandler() const;

  AudioBuffer* buffer() const;
  void setBuffer(AudioBuffer*, ExceptionState&);
  AudioParam* playbackRate() const;
  AudioParam* detune() const;
  bool loop() const;
  void setLoop(bool);
  double loopStart() const;
  void setLoopStart(double);
  double loopEnd() const;
  void setLoopEnd(double);

  void start(ExceptionState&);
  void start(double when, ExceptionState&);
  void start(double when, double grain_offset, ExceptionState&);
  void start(double when,
             double grain_offset,
             double grain_duration,
             ExceptionState&);

  // InspectorHelperMixin
  void ReportDidCreate() final;
  void ReportWillBeDestroyed() final;

 private:
  Member<AudioParam> playback_rate_;
  Member<AudioParam> detune_;
  Member<AudioBuffer> buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_BUFFER_SOURCE_NODE_H_

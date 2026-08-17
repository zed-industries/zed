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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PROCESSING_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PROCESSING_EVENT_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_audio_processing_event_init.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/webaudio/audio_buffer.h"

namespace blink {

class AudioBuffer;
class AudioProcessingEventInit;

class AudioProcessingEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static AudioProcessingEvent* Create();
  static AudioProcessingEvent* Create(AudioBuffer* input_buffer,
                                      AudioBuffer* output_buffer,
                                      double playback_time);

  static AudioProcessingEvent* Create(const AtomicString& type,
                                      const AudioProcessingEventInit*);

  AudioProcessingEvent();
  AudioProcessingEvent(AudioBuffer* input_buffer,
                       AudioBuffer* output_buffer,
                       double playback_time);
  AudioProcessingEvent(const AtomicString& type,
                       const AudioProcessingEventInit*);
  ~AudioProcessingEvent() override;

  AudioBuffer* inputBuffer() { return input_buffer_.Get(); }
  AudioBuffer* outputBuffer() { return output_buffer_.Get(); }
  double playbackTime() const { return playback_time_; }

  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  Member<AudioBuffer> input_buffer_;
  Member<AudioBuffer> output_buffer_;
  double playback_time_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PROCESSING_EVENT_H_

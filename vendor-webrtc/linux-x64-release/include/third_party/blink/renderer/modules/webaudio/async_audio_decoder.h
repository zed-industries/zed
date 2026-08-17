/*
 * Copyright (C) 2011, Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_ASYNC_AUDIO_DECODER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_ASYNC_AUDIO_DECODER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_decode_error_callback.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_decode_success_callback.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer/array_buffer_contents.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_handle.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

class AudioBuffer;
class AudioBus;
class BaseAudioContext;
class DOMArrayBuffer;
class ExceptionContext;
class ExceptionState;

// AsyncAudioDecoder asynchronously decodes audio file data from a
// DOMArrayBuffer in the background thread. Upon successful decoding, a
// completion callback will be invoked with the decoded PCM data in an
// AudioBuffer.

class AsyncAudioDecoder {
  DISALLOW_NEW();

 public:
  AsyncAudioDecoder() = default;

  AsyncAudioDecoder(const AsyncAudioDecoder&) = delete;
  AsyncAudioDecoder& operator=(const AsyncAudioDecoder&) = delete;

  ~AsyncAudioDecoder() = default;

  // Must be called on the main thread.  `DecodeAsync` and callees must not
  // modify any of the parameters except `audio_data`.  They are used to
  // associate this decoding instance with the caller to process the decoding
  // appropriately when finished.
  void DecodeAsync(DOMArrayBuffer* audio_data,
                   float sample_rate,
                   V8DecodeSuccessCallback*,
                   V8DecodeErrorCallback*,
                   ScriptPromiseResolver<AudioBuffer>*,
                   BaseAudioContext*,
                   ExceptionState&);

 private:
  AudioBuffer* CreateAudioBufferFromAudioBus(AudioBus*);
  static void DecodeOnBackgroundThread(
      ArrayBufferContents audio_data_contents,
      float sample_rate,
      CrossThreadHandle<V8DecodeSuccessCallback>,
      CrossThreadHandle<V8DecodeErrorCallback>,
      CrossThreadHandle<ScriptPromiseResolver<AudioBuffer>>,
      CrossThreadHandle<BaseAudioContext>,
      scoped_refptr<base::SingleThreadTaskRunner>,
      const ExceptionContext&);
  static void NotifyComplete(ArrayBufferContents audio_data_contents,
                             V8DecodeSuccessCallback*,
                             V8DecodeErrorCallback*,
                             AudioBus*,
                             ScriptPromiseResolver<AudioBuffer>*,
                             BaseAudioContext*,
                             const ExceptionContext&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_ASYNC_AUDIO_DECODER_H_

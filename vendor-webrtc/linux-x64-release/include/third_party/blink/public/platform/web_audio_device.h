/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_AUDIO_DEVICE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_AUDIO_DEVICE_H_

#include "media/base/output_device_info.h"
#include "third_party/blink/public/platform/web_audio_sink_descriptor.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

// Abstract interface to the Chromium audio system.
class WebAudioDevice {
 public:
  virtual ~WebAudioDevice() = default;

  virtual void Start() = 0;
  virtual void Stop() = 0;

  // Pause an audio device so that the device doesn't produce
  // requests for audio frames. This is used when a web frame is
  // frozen. This shouldn't be observable to the web application.
  virtual void Pause() = 0;

  // Resume an audio device that was previously paused. Used
  // for frozen frames.
  virtual void Resume() = 0;

  virtual double SampleRate() = 0;
  virtual int FramesPerBuffer() = 0;

  // The maximum channel count of the current audio sink device.
  virtual int MaxChannelCount() = 0;

  // Sets the detect silence flag for |RendererWebAudioDeviceImpl|.
  virtual void SetDetectSilence(bool detect_silence) = 0;

  // Creates a new sink if one hasn't been created yet, and returns the sink
  // status.
  virtual media::OutputDeviceStatus MaybeCreateSinkAndGetStatus() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_AUDIO_DEVICE_H_

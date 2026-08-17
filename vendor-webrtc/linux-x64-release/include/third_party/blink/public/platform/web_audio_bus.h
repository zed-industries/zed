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
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_AUDIO_BUS_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_AUDIO_BUS_H_

#include "base/memory/raw_ptr.h"
#include "third_party/blink/public/platform/web_common.h"

#if INSIDE_BLINK
template <typename T>
class scoped_refptr;
#endif

namespace blink {

class AudioBus;

// A container for multi-channel linear PCM audio data.
//
// WARNING: It is not safe to pass a WebAudioBus across threads!!!
//
class BLINK_PLATFORM_EXPORT WebAudioBus {
 public:
  WebAudioBus() = default;
  ~WebAudioBus() { Reset(); }

  // Initialize() allocates memory of the given length for the given number of
  // channels.
  void Initialize(unsigned number_of_channels,
                  size_t length,
                  double sample_rate);

  // ResizeSmaller() can only be called after Initialize() with a new length <=
  // the initialization length.  The data stored in the bus will remain
  // undisturbed.
  void ResizeSmaller(size_t new_length);

  // Reset() releases the memory allocated from Initialize().
  void Reset();

  unsigned NumberOfChannels() const;
  size_t length() const;
  double SampleRate() const;

  float* ChannelData(unsigned channel_index);

#if INSIDE_BLINK
  scoped_refptr<AudioBus> Release();
#endif

 private:
  // Disallow copy and assign.
  WebAudioBus(const WebAudioBus&) = delete;
  void operator=(const WebAudioBus&) = delete;

  raw_ptr<AudioBus> private_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_AUDIO_BUS_H_

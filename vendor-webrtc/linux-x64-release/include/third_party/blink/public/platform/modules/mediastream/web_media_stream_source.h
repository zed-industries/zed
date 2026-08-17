/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_SOURCE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_SOURCE_H_

#include <memory>

#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_track.h"
#include "third_party/blink/public/platform/web_common.h"

#include "third_party/blink/public/platform/web_private_ptr.h"

namespace blink {

class MediaStreamSource;
class WebPlatformMediaStreamSource;
class WebString;

class BLINK_PLATFORM_EXPORT WebMediaStreamSource {
 public:
  enum Type { kTypeAudio, kTypeVideo };

  enum ReadyState {
    kReadyStateLive = 0,
    kReadyStateMuted = 1,
    kReadyStateEnded = 2
  };

  WebMediaStreamSource() = default;
  WebMediaStreamSource(const WebMediaStreamSource& other) { Assign(other); }
  ~WebMediaStreamSource() { Reset(); }

  WebMediaStreamSource& operator=(const WebMediaStreamSource& other) {
    Assign(other);
    return *this;
  }

  void Assign(const WebMediaStreamSource&);

  void Initialize(const WebString& id,
                  Type,
                  const WebString& name,
                  bool remote,
                  std::unique_ptr<WebPlatformMediaStreamSource>);
  void Reset();
  bool IsNull() const { return private_.IsNull(); }

  WebString Id() const;
  Type GetType() const;

  void SetReadyState(ReadyState);
  ReadyState GetReadyState() const;

  WebPlatformMediaStreamSource* GetPlatformSource() const;

#if INSIDE_BLINK
  explicit WebMediaStreamSource(MediaStreamSource*);
  WebMediaStreamSource& operator=(MediaStreamSource*);
  operator scoped_refptr<MediaStreamSource>() const;
  operator MediaStreamSource*() const;
#endif

 private:
  WebPrivatePtrForGC<MediaStreamSource> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_SOURCE_H_

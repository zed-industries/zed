/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_TRACK_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_TRACK_H_

#include "media/mojo/mojom/display_media_information.mojom-shared.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"

namespace blink {

class MediaStreamComponent;
class WebMediaStreamSource;

class BLINK_PLATFORM_EXPORT WebMediaStreamTrack {
 public:
  enum class FacingMode { kNone, kUser, kEnvironment, kLeft, kRight };

  static const char kResizeModeNone[];
  static const char kResizeModeRescale[];

  enum class ContentHintType {
    kNone,
    kAudioSpeech,
    kAudioMusic,
    kVideoMotion,
    kVideoDetail,
    kVideoText
  };

  WebMediaStreamTrack() = default;
  WebMediaStreamTrack(const WebMediaStreamTrack& other) { Assign(other); }
  ~WebMediaStreamTrack() { Reset(); }

  WebMediaStreamTrack& operator=(const WebMediaStreamTrack& other) {
    Assign(other);
    return *this;
  }
  void Assign(const WebMediaStreamTrack&);

  void Reset();
  bool IsNull() const { return private_.IsNull(); }

  WebMediaStreamSource Source() const;

#if INSIDE_BLINK
  explicit WebMediaStreamTrack(MediaStreamComponent*);
  WebMediaStreamTrack& operator=(MediaStreamComponent*);
  operator scoped_refptr<MediaStreamComponent>() const;
  operator MediaStreamComponent*() const;
#endif

 private:
  WebPrivatePtrForGC<MediaStreamComponent> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_TRACK_H_

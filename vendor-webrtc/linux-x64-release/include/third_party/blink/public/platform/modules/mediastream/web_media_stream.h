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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"

namespace blink {

class MediaStreamDescriptor;
class WebMediaStreamTrack;
class WebString;

class BLINK_PLATFORM_EXPORT WebMediaStreamObserver {
 public:
  // TrackAdded is called when the respective track is added to the observed
  // MediaStream.
  virtual void TrackAdded(const WebString& track_id) {}
  // TrackRemoved is called when the respective track is added to the observed
  // MediaStream.
  virtual void TrackRemoved(const WebString& track_id) {}
  // ActiveStateChanged is called when the observed MediaStream becomes either
  // active or inactive.
  virtual void ActiveStateChanged(bool is_active) {}

  // EnabledStateChangedForWebRtcAudio is called only when the observed
  // MediaStream that has a WebRTC remote audio track and that track is enabled
  // or disabled.
  virtual void EnabledStateChangedForWebRtcAudio(bool is_enabled) {}

 protected:
  virtual ~WebMediaStreamObserver() = default;
};

class BLINK_PLATFORM_EXPORT WebMediaStream {
 public:
  WebMediaStream() = default;
  WebMediaStream(const WebMediaStream& other) { Assign(other); }
  ~WebMediaStream() { Reset(); }

  WebMediaStream& operator=(const WebMediaStream& other) {
    Assign(other);
    return *this;
  }

  void Assign(const WebMediaStream&);

  void Reset();
  bool IsNull() const { return private_.IsNull(); }

  WebString Id() const;
  int UniqueId() const;

  // If a track is not found with the specified id, the returned track's
  // |IsNull| will return true.
  WebMediaStreamTrack GetAudioTrack(const WebString& track_id) const;
  WebMediaStreamTrack GetVideoTrack(const WebString& track_id) const;

  // These methods add/remove an observer to/from this WebMediaStream. The
  // caller is responsible for removing the observer before the destruction of
  // the WebMediaStream. Observers cannot be null, cannot be added or removed
  // more than once, and cannot invoke AddObserver/RemoveObserver in their
  // TrackAdded/TrackRemoved callbacks.
  void AddObserver(WebMediaStreamObserver*);
  void RemoveObserver(WebMediaStreamObserver*);

#if INSIDE_BLINK
  explicit WebMediaStream(MediaStreamDescriptor*);
  operator MediaStreamDescriptor*() const;
  WebMediaStream& operator=(MediaStreamDescriptor*);
#endif

 private:
  WebPrivatePtrForGC<MediaStreamDescriptor> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_WEB_MEDIA_STREAM_H_

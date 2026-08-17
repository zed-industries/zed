// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_REMOTE_PLAYBACK_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_REMOTE_PLAYBACK_OBSERVER_H_

#include "third_party/blink/public/mojom/presentation/presentation.mojom-blink.h"

namespace blink {

// Interface to be implemented by objects that intend to be notified by remote
// playback status changes on an HTMLMediaElement. The object should self-add
// itself to the RemotePlaybackController using the add/remove observer methods.
class RemotePlaybackObserver : public GarbageCollectedMixin {
 public:
  // Called when the remote playback state is changed. The state is related to
  // the connection to a remote device.
  virtual void OnRemotePlaybackStateChanged(
      mojom::blink::PresentationConnectionState) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_REMOTE_PLAYBACK_OBSERVER_H_

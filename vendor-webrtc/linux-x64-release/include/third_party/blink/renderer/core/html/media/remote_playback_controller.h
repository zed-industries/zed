// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_REMOTE_PLAYBACK_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_REMOTE_PLAYBACK_CONTROLLER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/media/html_media_element.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class RemotePlaybackObserver;

// Interface exposing RemotePlayback to core/. It is meant to replace
// WebRemotePlaybackClient in the long run when there will be no need to expose
// this outside of Blink.
class CORE_EXPORT RemotePlaybackController
    : public Supplement<HTMLMediaElement> {
 public:
  static const char kSupplementName[];

  static RemotePlaybackController* From(HTMLMediaElement&);

  virtual void AddObserver(RemotePlaybackObserver*) = 0;
  virtual void RemoveObserver(RemotePlaybackObserver*) = 0;

  // Exposes simplified internal methods for testing purposes.
  virtual void AvailabilityChangedForTesting(bool screen_is_available) = 0;
  virtual void StateChangedForTesting(bool is_connected) = 0;

  void Trace(Visitor*) const override;

 protected:
  explicit RemotePlaybackController(HTMLMediaElement&);

  // To be called by RemotePlayback implementation to register its
  // implementation.
  static void ProvideTo(HTMLMediaElement&, RemotePlaybackController*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_REMOTE_PLAYBACK_CONTROLLER_H_

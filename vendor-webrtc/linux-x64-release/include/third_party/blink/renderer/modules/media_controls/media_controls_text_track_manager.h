// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_TEXT_TRACK_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_TEXT_TRACK_MANAGER_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class HTMLMediaElement;
class TextTrack;

class MODULES_EXPORT MediaControlsTextTrackManager
    : public GarbageCollected<MediaControlsTextTrackManager> {
 public:
  explicit MediaControlsTextTrackManager(HTMLMediaElement&);

  MediaControlsTextTrackManager(const MediaControlsTextTrackManager&) = delete;
  MediaControlsTextTrackManager& operator=(
      const MediaControlsTextTrackManager&) = delete;

  // Returns the label for the track when a valid track is passed in and "Off"
  // when the parameter is null.
  String GetTextTrackLabel(TextTrack*) const;
  void ShowTextTrackAtIndex(unsigned);
  void DisableShowingTextTracks();

  virtual void Trace(Visitor*) const;

 private:
  Member<HTMLMediaElement> media_element_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CONTROLS_MEDIA_CONTROLS_TEXT_TRACK_MANAGER_H_

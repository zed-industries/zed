// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_AUDIO_TRACK_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_AUDIO_TRACK_LIST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/track/audio_track.h"
#include "third_party/blink/renderer/core/html/track/track_list_base.h"

namespace blink {

class CORE_EXPORT AudioTrackList final : public TrackListBase<AudioTrack> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit AudioTrackList(HTMLMediaElement&);
  ~AudioTrackList() override;

  bool HasEnabledTrack() const;

  // If we are enabling a track in exclusive mode, clear the enabled flag for
  // any other enabled tracks. Also clear the enabled flag for any other
  // exclusive tracks that are enabled.
  void TrackEnabled(const String& track_id, bool exclusive);

  // EventTarget
  const AtomicString& InterfaceName() const override;

  void Trace(Visitor* visitor) const override {
    TrackListBase<AudioTrack>::Trace(visitor);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_AUDIO_TRACK_LIST_H_

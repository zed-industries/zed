// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_AUTOMATIC_TRACK_SELECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_AUTOMATIC_TRACK_SELECTION_H_

#include "third_party/blink/renderer/core/html/track/text_track_kind_user_preference.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class TextTrackList;
class TrackGroup;

class AutomaticTrackSelection {
  STACK_ALLOCATED();

 public:
  struct Configuration {
    DISALLOW_NEW();
    Configuration()
        : disable_currently_enabled_tracks(false),
          force_enable_subtitle_or_caption_track(false),
          text_track_kind_user_preference(
              TextTrackKindUserPreference::kDefault) {}

    bool disable_currently_enabled_tracks;
    bool force_enable_subtitle_or_caption_track;
    TextTrackKindUserPreference text_track_kind_user_preference;
  };

  AutomaticTrackSelection(const Configuration&);

  void Perform(TextTrackList&);

 private:
  void PerformAutomaticTextTrackSelection(const TrackGroup&);
  void EnableDefaultMetadataTextTracks(const TrackGroup&);
  const AtomicString& PreferredTrackKind() const;

  const Configuration configuration_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_AUTOMATIC_TRACK_SELECTION_H_

// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VIDEO_TRACK_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VIDEO_TRACK_LIST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/track/track_list_base.h"
#include "third_party/blink/renderer/core/html/track/video_track.h"

namespace blink {

class CORE_EXPORT VideoTrackList final : public TrackListBase<VideoTrack> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit VideoTrackList(HTMLMediaElement&);
  ~VideoTrackList() override;

  int selectedIndex() const;

  // EventTarget
  const AtomicString& InterfaceName() const override;

  void TrackSelected(const String& selected_track_id);

  void Trace(Visitor* visitor) const override {
    TrackListBase<VideoTrack>::Trace(visitor);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VIDEO_TRACK_LIST_H_

// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VIDEO_TRACK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VIDEO_TRACK_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/track/track_base.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CORE_EXPORT VideoTrack final : public ScriptWrappable, public TrackBase {
  DEFINE_WRAPPERTYPEINFO();

 public:
  VideoTrack(const String& id,
             const AtomicString& kind,
             const AtomicString& label,
             const AtomicString& language,
             bool selected);
  ~VideoTrack() override;
  void Trace(Visitor*) const override;

  bool selected() const { return selected_; }
  void setSelected(bool);
  const AtomicString& kind() const { return kind_; }

  // Set selected to false without notifying the owner media element. Used when
  // another video track is selected, implicitly deselecting this one.
  void ClearSelected() { selected_ = false; }

  // Valid kind keywords.
  static const AtomicString& AlternativeKeyword();
  static const AtomicString& CaptionsKeyword();
  static const AtomicString& MainKeyword();
  static const AtomicString& SignKeyword();
  static const AtomicString& SubtitlesKeyword();
  static const AtomicString& CommentaryKeyword();

  static bool IsValidKindKeyword(const String&);

 private:
  bool selected_;
  const AtomicString kind_;
};

template <>
struct DowncastTraits<VideoTrack> {
  static bool AllowFrom(const TrackBase& track) {
    return track.GetType() == WebMediaPlayer::kVideoTrack;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VIDEO_TRACK_H_

// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_AUDIO_TRACK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_AUDIO_TRACK_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/track/track_base.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CORE_EXPORT AudioTrack final : public ScriptWrappable, public TrackBase {
  DEFINE_WRAPPERTYPEINFO();

 public:
  AudioTrack(const String& id,
             const AtomicString& kind,
             const AtomicString& label,
             const AtomicString& language,
             bool enabled,
             bool exclusive);
  ~AudioTrack() override;
  void Trace(Visitor*) const override;

  bool enabled() const { return enabled_; }
  void setEnabled(bool);

  // Set enabled to false without notifying the owner media element. Used when
  // an exclusive audio track is enabled, implicitly disabling this one.
  void ClearEnabled() { enabled_ = false; }

  bool IsExclusive() const { return exclusive_; }

  const AtomicString& kind() const { return kind_; }

  // Valid kind keywords.
  static const AtomicString& AlternativeKeyword();
  static const AtomicString& DescriptionsKeyword();
  static const AtomicString& MainKeyword();
  static const AtomicString& MainDescriptionsKeyword();
  static const AtomicString& TranslationKeyword();
  static const AtomicString& CommentaryKeyword();

  static bool IsValidKindKeyword(const String&);

 private:
  bool enabled_;
  bool exclusive_;
  const AtomicString kind_;
};

template <>
struct DowncastTraits<AudioTrack> {
  static bool AllowFrom(const TrackBase& track) {
    return track.GetType() == WebMediaPlayer::kAudioTrack;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_AUDIO_TRACK_H_

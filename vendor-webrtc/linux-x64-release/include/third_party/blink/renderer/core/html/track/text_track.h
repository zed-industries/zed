/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 * Copyright (C) 2011, 2012, 2013 Apple Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_text_track_kind.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_text_track_mode.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_style_sheet.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/html/track/track_base.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CueTimeline;
class ExceptionState;
class ExecutionContext;
class HTMLMediaElement;
class HTMLElement;
class TextTrack;
class TextTrackCue;
class TextTrackCueList;
class TextTrackList;

using TextTrackMode = V8TextTrackMode::Enum;

class CORE_EXPORT TextTrack : public EventTarget, public TrackBase {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum TextTrackType { kTrackElement, kAddTrack, kInBand };

  TextTrack(const V8TextTrackKind& kind,
            const AtomicString& label,
            const AtomicString& language,
            HTMLElement& source_element,
            const AtomicString& id = g_empty_atom,
            TextTrackType = kAddTrack);
  ~TextTrack() override;

  virtual void SetTrackList(TextTrackList*);
  TextTrackList* TrackList() { return track_list_.Get(); }

  bool IsVisualKind() const;
  bool IsSpokenKind() const;

  static const AtomicString& SubtitlesKeyword();
  static const AtomicString& CaptionsKeyword();
  static const AtomicString& DescriptionsKeyword();
  static const AtomicString& ChaptersKeyword();
  static const AtomicString& MetadataKeyword();
  static bool IsValidKindKeyword(const String&);

  V8TextTrackKind kind() const { return V8TextTrackKind(kind_); }
  void SetKind(const V8TextTrackKind& kind) { kind_ = kind.AsEnum(); }
  void SetLabel(const AtomicString& label) { label_ = label; }
  void SetLanguage(const AtomicString& language) { language_ = language; }
  void SetId(const String& id) { id_ = id; }

  V8TextTrackMode mode() const { return V8TextTrackMode(mode_); }
  virtual void setMode(const V8TextTrackMode&);
  void SetModeEnum(TextTrackMode mode);

  enum ReadinessState {
    kNotLoaded = 0,
    kLoading = 1,
    kLoaded = 2,
    kFailedToLoad = 3
  };
  ReadinessState GetReadinessState() const { return readiness_state_; }
  void SetReadinessState(ReadinessState state) { readiness_state_ = state; }

  TextTrackCueList* cues();
  TextTrackCueList* activeCues();

  HTMLMediaElement* MediaElement() const;
  Node* Owner() const;

  void addCue(TextTrackCue*);
  void removeCue(TextTrackCue*, ExceptionState&);

  void CueWillChange(TextTrackCue*);
  void CueDidChange(TextTrackCue*, bool update_cue_index);

  DEFINE_ATTRIBUTE_EVENT_LISTENER(cuechange, kCuechange)

  TextTrackType TrackType() const { return track_type_; }
  const AtomicString& Language() const { return language_; }

  int TrackIndex();
  void InvalidateTrackIndex();

  bool IsRendered() const;
  bool CanBeRendered() const;
  int TrackIndexRelativeToRenderedTracks();

  bool HasBeenConfigured() const { return has_been_configured_; }
  void SetHasBeenConfigured(bool flag) { has_been_configured_ = flag; }

  virtual bool IsDefault() const { return false; }

  void Reset();

  // EventTarget methods
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  void Trace(Visitor*) const override;

  const HeapVector<Member<CSSStyleSheet>>& GetCSSStyleSheets() const {
    return style_sheets_;
  }

  void SetCSSStyleSheets(HeapVector<Member<CSSStyleSheet>>);

 protected:
  void AddListOfCues(HeapVector<Member<TextTrackCue>>&);

 private:
  CueTimeline* GetCueTimeline() const;

  TextTrackCueList* EnsureTextTrackCueList();
  Member<TextTrackCueList> cues_;
  Member<TextTrackCueList> active_cues_;
  HeapVector<Member<CSSStyleSheet>> style_sheets_;

  Member<TextTrackList> track_list_;
  Member<HTMLElement> source_element_;
  TextTrackMode mode_ = TextTrackMode::kDisabled;
  TextTrackType track_type_;
  ReadinessState readiness_state_;
  int track_index_;
  int rendered_track_index_;
  bool has_been_configured_;
  V8TextTrackKind::Enum kind_ = V8TextTrackKind::Enum::kSubtitles;
};

template <>
struct DowncastTraits<TextTrack> {
  static bool AllowFrom(const TrackBase& track) {
    return track.GetType() == WebMediaPlayer::kTextTrack;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_H_

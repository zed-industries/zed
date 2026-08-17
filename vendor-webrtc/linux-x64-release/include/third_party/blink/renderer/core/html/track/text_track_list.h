/*
 * Copyright (C) 2011, 2012 Apple Inc.  All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_LIST_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/event_listener.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/html/media/html_media_element.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class TextTrack;

class CORE_EXPORT TextTrackList final : public EventTarget {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit TextTrackList(HTMLMediaElement*);
  ~TextTrackList() override;

  unsigned length() const;
  int GetTrackIndex(TextTrack*);
  int GetTrackIndexRelativeToRenderedTracks(TextTrack*);
  bool Contains(TextTrack*) const;

  TextTrack* AnonymousIndexedGetter(unsigned index);
  TextTrack* getTrackById(const AtomicString& id);
  void Append(TextTrack*);
  void Remove(TextTrack*);

  // EventTarget
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(addtrack, kAddtrack)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(change, kChange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(removetrack, kRemovetrack)

  HTMLMediaElement* Owner() const;

  void ScheduleChangeEvent();

  bool HasShowingTracks();

  void Trace(Visitor*) const override;

 private:
  void ScheduleTrackEvent(const AtomicString& event_name, TextTrack*);

  void ScheduleAddTrackEvent(TextTrack*);
  void ScheduleRemoveTrackEvent(TextTrack*);

  void InvalidateTrackIndexesAfterTrack(TextTrack*);

  Member<HTMLMediaElement> owner_;

  HeapVector<Member<TextTrack>> add_track_tracks_;
  HeapVector<Member<TextTrack>> element_tracks_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TEXT_TRACK_LIST_H_

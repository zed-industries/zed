// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TRACK_LIST_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TRACK_LIST_BASE_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/html/media/html_media_element.h"
#include "third_party/blink/renderer/core/html/track/track_event.h"

namespace blink {

template <class T>
class TrackListBase : public EventTarget {
 public:
  explicit TrackListBase(HTMLMediaElement* media_element)
      : media_element_(media_element) {}

  ~TrackListBase() override = default;

  unsigned length() const { return tracks_.size(); }
  T* AnonymousIndexedGetter(unsigned index) const {
    if (index >= tracks_.size())
      return nullptr;
    return tracks_[index].Get();
  }

  T* getTrackById(const String& id) const {
    for (const auto& track : tracks_) {
      if (String(track->id()) == id)
        return track.Get();
    }

    return nullptr;
  }

  DEFINE_ATTRIBUTE_EVENT_LISTENER(change, kChange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(addtrack, kAddtrack)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(removetrack, kRemovetrack)

  // EventTarget interface
  ExecutionContext* GetExecutionContext() const override {
    if (media_element_)
      return media_element_->GetExecutionContext();
    return nullptr;
  }

  void Add(T* track) {
    track->SetMediaElement(media_element_.Get());
    tracks_.push_back(track);
    ScheduleEvent(TrackEvent::Create(event_type_names::kAddtrack, track));
  }

  void Remove(const String& track_id) {
    for (unsigned i = 0; i < tracks_.size(); ++i) {
      if (tracks_[i]->id() != track_id)
        continue;

      tracks_[i]->SetMediaElement(nullptr);
      ScheduleEvent(
          TrackEvent::Create(event_type_names::kRemovetrack, tracks_[i].Get()));
      tracks_.EraseAt(i);
      return;
    }
    NOTREACHED();
  }

  void RemoveAll() {
    for (const auto& track : tracks_)
      track->SetMediaElement(nullptr);

    tracks_.clear();
  }

  void ScheduleChangeEvent() {
    ScheduleEvent(Event::Create(event_type_names::kChange));
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(tracks_);
    visitor->Trace(media_element_);
    EventTarget::Trace(visitor);
  }

 private:
  void ScheduleEvent(Event* event) {
    event->SetTarget(this);
    media_element_->ScheduleEvent(event);
  }

  HeapVector<Member<T>> tracks_;
  Member<HTMLMediaElement> media_element_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TRACK_LIST_BASE_H_

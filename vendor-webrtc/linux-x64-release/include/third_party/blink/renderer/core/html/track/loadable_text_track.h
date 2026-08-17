/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_LOADABLE_TEXT_TRACK_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_LOADABLE_TEXT_TRACK_H_

#include "third_party/blink/renderer/core/html/track/text_track.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class HTMLTrackElement;

class LoadableTextTrack final : public TextTrack {
 public:
  explicit LoadableTextTrack(HTMLTrackElement*);
  ~LoadableTextTrack() override;

  // TextTrack method.
  void setMode(const V8TextTrackMode&) override;

  using TextTrack::AddListOfCues;

  wtf_size_t TrackElementIndex() const;
  HTMLTrackElement* TrackElement() { return track_element_.Get(); }

  bool IsDefault() const override;

  void Trace(Visitor*) const override;

 private:
  Member<HTMLTrackElement> track_element_;
};

template <>
struct DowncastTraits<LoadableTextTrack> {
  static bool AllowFrom(const TextTrack& track) {
    return track.TrackType() == TextTrack::kTrackElement;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_LOADABLE_TEXT_TRACK_H_
